// FILE: src/notifications/classes/cls_notification_state.rs - NotificationState internal state management
// VERSION: 1.2.0
// WCTX: Fixing custom entry/exit position handling
// CLOG: Copy custom_entry_position and custom_exit_position from Notification to NotificationState; Added tests

use super::cls_notification::Notification;
use crate::notifications::types::{AnimationPhase, Timing, AutoDismiss};
use ratatui::prelude::*;
use std::time::{Duration, Instant};

/// Manager-level defaults for notification timing.
///
/// Provides fallback durations when notifications use `Timing::Auto`
/// or `AutoDismiss::After(Duration::ZERO)`.
#[derive(Debug, Clone, Copy)]
pub(crate) struct ManagerDefaults {
    pub default_entry_duration: Duration,
    pub default_dwell_duration: Duration,
    pub default_exit_duration: Duration,
    pub default_display_time: Duration,
}

impl Default for ManagerDefaults {
    fn default() -> Self {
        Self {
            default_entry_duration: Duration::from_millis(500),
            default_dwell_duration: Duration::from_secs(3),
            default_exit_duration: Duration::from_millis(750),
            default_display_time: Duration::from_secs(4),
        }
    }
}

/// Internal state for a single notification (pub(crate)).
///
/// Tracks animation progress, timing, and lifecycle for a notification
/// being managed by the AnimatedNotificationManager.
#[derive(Debug)]
pub(crate) struct NotificationState {
    /// Unique identifier for this notification
    pub(crate) id: u64,

    /// The original notification configuration
    pub(crate) notification: Notification,

    /// When this notification was created
    pub(crate) created_at: Instant,

    /// Current animation phase
    pub(crate) current_phase: AnimationPhase,

    /// Progress through current animation (0.0 to 1.0)
    pub(crate) animation_progress: f32,

    /// Target position/size (updated by render)
    pub(crate) full_rect: Rect,

    /// Remaining time until auto-dismiss (if applicable)
    pub(crate) remaining_display_time: Option<Duration>,

    /// Resolved entry animation duration
    pub(crate) actual_entry_duration: Duration,

    /// Resolved dwell duration
    pub(crate) actual_dwell_duration: Duration,

    /// Resolved exit animation duration
    pub(crate) actual_exit_duration: Duration,

    /// Custom entry position override (for slide animations)
    pub(crate) custom_entry_pos: Option<(f32, f32)>,

    /// Custom exit position override (for slide animations)
    pub(crate) custom_exit_pos: Option<(f32, f32)>,
}

impl NotificationState {
    /// Creates a new notification state.
    ///
    /// Resolves all timing durations based on the notification's configuration
    /// and the manager's defaults.
    ///
    /// # Arguments
    /// * `id` - Unique identifier for this notification
    /// * `notification` - The notification configuration
    /// * `defaults` - Manager-level default durations
    pub(crate) fn new(id: u64, notification: Notification, defaults: &ManagerDefaults) -> Self {
        // Resolve actual durations from Timing enum
        let actual_entry_duration = match notification.slide_in_timing {
            Timing::Fixed(d) => d,
            Timing::Auto => defaults.default_entry_duration,
        };

        let actual_dwell_duration = match notification.dwell_timing {
            Timing::Fixed(d) => d,
            Timing::Auto => defaults.default_dwell_duration,
        };

        let actual_exit_duration = match notification.slide_out_timing {
            Timing::Fixed(d) => d,
            Timing::Auto => defaults.default_exit_duration,
        };

        // Resolve remaining display time from AutoDismiss
        let remaining_display_time = match notification.auto_dismiss {
            AutoDismiss::Never => None,
            AutoDismiss::After(d) if d > Duration::ZERO => Some(d),
            AutoDismiss::After(_) => Some(defaults.default_display_time),
        };

        // Copy custom positions from notification (convert Position to (f32, f32))
        let custom_entry_pos = notification
            .custom_entry_position
            .map(|p| (p.x as f32, p.y as f32));
        let custom_exit_pos = notification
            .custom_exit_position
            .map(|p| (p.x as f32, p.y as f32));

        Self {
            id,
            notification,
            created_at: Instant::now(),
            current_phase: AnimationPhase::Pending,
            animation_progress: 0.0,
            full_rect: Rect::default(),
            remaining_display_time,
            actual_entry_duration,
            actual_dwell_duration,
            actual_exit_duration,
            custom_entry_pos,
            custom_exit_pos,
        }
    }

    /// Updates the notification state based on elapsed time.
    ///
    /// Advances animation phases and progress based on timing configuration.
    /// Entry/exit animations use progress-based timing, while dwelling uses
    /// remaining_display_time countdown.
    ///
    /// # Arguments
    /// * `delta` - Time elapsed since last update
    pub(crate) fn update(&mut self, delta: Duration) {
        use crate::notifications::types::Animation;

        // Start animation if still pending
        if self.current_phase == AnimationPhase::Pending {
            self.current_phase = match self.notification.animation {
                Animation::Slide => AnimationPhase::SlidingIn,
                Animation::ExpandCollapse => AnimationPhase::Expanding,
                Animation::Fade => AnimationPhase::FadingIn,
            };
            self.animation_progress = 0.0;
        }

        // Update animation progress for entry/exit phases (NOT dwelling)
        let phase_duration = match self.current_phase {
            AnimationPhase::SlidingIn | AnimationPhase::FadingIn | AnimationPhase::Expanding => {
                self.actual_entry_duration
            }
            AnimationPhase::SlidingOut | AnimationPhase::FadingOut | AnimationPhase::Collapsing => {
                self.actual_exit_duration
            }
            // Dwelling phase uses remaining_display_time, not animation_progress
            _ => Duration::ZERO,
        };

        let mut progress_updated = false;
        if phase_duration > Duration::ZERO
            && matches!(
                self.current_phase,
                AnimationPhase::SlidingIn
                    | AnimationPhase::FadingIn
                    | AnimationPhase::Expanding
                    | AnimationPhase::SlidingOut
                    | AnimationPhase::FadingOut
                    | AnimationPhase::Collapsing
            )
        {
            let delta_progress = delta.as_secs_f32() / phase_duration.as_secs_f32();
            self.animation_progress = (self.animation_progress + delta_progress).min(1.0);
            progress_updated = true;
        }

        // Handle phase transitions when animation completes
        if progress_updated && self.animation_progress >= 1.0 {
            match self.current_phase {
                // Entry animation complete → Dwelling
                AnimationPhase::SlidingIn | AnimationPhase::Expanding | AnimationPhase::FadingIn => {
                    self.current_phase = AnimationPhase::Dwelling;
                    self.animation_progress = 0.0;
                }
                // Exit animation complete → Finished
                AnimationPhase::SlidingOut | AnimationPhase::Collapsing | AnimationPhase::FadingOut => {
                    self.current_phase = AnimationPhase::Finished;
                }
                _ => {}
            }
        }

        // Handle dwelling phase timer (separate from animation progress)
        if self.current_phase == AnimationPhase::Dwelling {
            if let Some(remaining) = self.remaining_display_time.as_mut() {
                *remaining = remaining.saturating_sub(delta);
                if remaining.is_zero() {
                    // Timer expired, transition to exit animation
                    self.current_phase = match self.notification.animation {
                        Animation::Slide => AnimationPhase::SlidingOut,
                        Animation::ExpandCollapse => AnimationPhase::Collapsing,
                        Animation::Fade => AnimationPhase::FadingOut,
                    };
                    self.animation_progress = 0.0;
                }
            }
            // If remaining_display_time is None, notification stays dwelling indefinitely
        }
    }
}

// Implement StackableNotification trait for render orchestrator
impl crate::notifications::orc_stacking::StackableNotification for NotificationState {
    fn id(&self) -> u64 {
        self.id
    }

    fn current_phase(&self) -> AnimationPhase {
        self.current_phase
    }

    fn created_at(&self) -> Instant {
        self.created_at
    }

    fn full_rect(&self) -> ratatui::prelude::Rect {
        self.full_rect
    }

    fn exterior_padding(&self) -> u16 {
        self.notification.exterior_margin
    }

    fn calculate_content_size(&self, frame_area: ratatui::prelude::Rect) -> (u16, u16) {
        crate::notifications::functions::fnc_calculate_size::calculate_size(&self.notification, frame_area)
    }
}

// Implement RenderableNotification trait for render orchestrator
impl crate::notifications::orc_render::RenderableNotification for NotificationState {
    fn level(&self) -> Option<crate::notifications::types::Level> {
        self.notification.level
    }

    fn title(&self) -> Option<ratatui::text::Line<'static>> {
        self.notification.title.clone()
    }

    fn content(&self) -> ratatui::prelude::Text<'static> {
        self.notification.content.clone()
    }

    fn border_type(&self) -> ratatui::widgets::BorderType {
        self.notification.border_type.unwrap_or(ratatui::widgets::BorderType::Plain)
    }

    fn fade_effect(&self) -> bool {
        self.notification.fade_effect
    }

    fn animation_type(&self) -> crate::notifications::types::Animation {
        self.notification.animation
    }

    fn animation_progress(&self) -> f32 {
        self.animation_progress
    }

    fn block_style(&self) -> Option<ratatui::prelude::Style> {
        self.notification.block_style
    }

    fn border_style(&self) -> Option<ratatui::prelude::Style> {
        self.notification.border_style
    }

    fn title_style(&self) -> Option<ratatui::prelude::Style> {
        self.notification.title_style
    }

    fn padding(&self) -> ratatui::widgets::Padding {
        self.notification.padding
    }

    fn set_full_rect(&mut self, rect: ratatui::prelude::Rect) {
        self.full_rect = rect;
    }

    fn calculate_animation_rect(&self, frame_area: ratatui::prelude::Rect) -> ratatui::prelude::Rect {
        use crate::notifications::types::Animation;

        match self.notification.animation {
            Animation::Slide => {
                crate::notifications::functions::fnc_slide_calculate_rect::slide_calculate_rect(
                    self.full_rect,
                    frame_area,
                    self.animation_progress,
                    self.current_phase,
                    self.notification.anchor,
                    self.notification.slide_direction,
                    self.custom_entry_pos,
                    self.custom_exit_pos,
                )
            }
            Animation::ExpandCollapse => {
                crate::notifications::functions::fnc_expand_calculate_rect::calculate_rect(
                    self.full_rect,
                    frame_area,
                    self.current_phase,
                    self.animation_progress,
                )
            }
            Animation::Fade => {
                crate::notifications::functions::fnc_fade_calculate_rect::calculate_rect(
                    self.full_rect,
                    frame_area,
                    self.current_phase,
                    self.animation_progress,
                )
            }
        }
    }

    fn apply_animation_block_effect<'a>(
        &self,
        block: ratatui::widgets::Block<'a>,
        frame_area: ratatui::prelude::Rect,
        base_set: &'a ratatui::symbols::border::Set,
    ) -> ratatui::widgets::Block<'a> {
        use crate::notifications::types::Animation;

        match self.notification.animation {
            Animation::Slide => {
                crate::notifications::functions::fnc_slide_apply_border_effect::slide_apply_border_effect(
                    block,
                    self.notification.anchor,
                    self.notification.slide_direction,
                    self.animation_progress,
                    self.current_phase,
                    self.full_rect,
                    self.custom_entry_pos,
                    self.custom_exit_pos,
                    frame_area,
                    base_set,
                )
            }
            _ => block,
        }
    }

    fn interpolate_frame_foreground(
        &self,
        base_fg: Option<ratatui::prelude::Color>,
        phase: AnimationPhase,
        progress: f32,
    ) -> Option<ratatui::prelude::Color> {
        use crate::notifications::types::Animation;
        use crate::notifications::functions::fnc_fade_interpolate_color::FadeHandler;

        match self.notification.animation {
            Animation::Fade => {
                FadeHandler.interpolate_frame_foreground(base_fg, phase, progress)
            }
            _ if self.notification.fade_effect => {
                FadeHandler.interpolate_frame_foreground(base_fg, phase, progress)
            }
            _ => base_fg,
        }
    }

    fn interpolate_content_foreground(
        &self,
        base_fg: Option<ratatui::prelude::Color>,
        phase: AnimationPhase,
        progress: f32,
    ) -> Option<ratatui::prelude::Color> {
        use crate::notifications::types::Animation;
        use crate::notifications::functions::fnc_fade_interpolate_color::FadeHandler;

        match self.notification.animation {
            Animation::Fade => {
                FadeHandler.interpolate_content_foreground(base_fg, phase, progress)
            }
            _ if self.notification.fade_effect => {
                FadeHandler.interpolate_content_foreground(base_fg, phase, progress)
            }
            _ => base_fg.or(Some(ratatui::prelude::Color::White)),
        }
    }
}


#[cfg(test)]
mod tests {
    use super::*;
    use crate::notifications::types::{Timing, AutoDismiss};

    fn create_test_notification() -> Notification {
        // Use Default to create a simple test notification
        Notification {
            content: Text::raw("Test notification"),
            ..Default::default()
        }
    }

    #[test]
    fn test_new_state_starts_in_pending_phase() {
        let defaults = ManagerDefaults::default();
        let notification = create_test_notification();
        let state = NotificationState::new(1, notification, &defaults);
        assert_eq!(state.current_phase, AnimationPhase::Pending);
    }

    #[test]
    fn test_progress_starts_at_zero() {
        let defaults = ManagerDefaults::default();
        let notification = create_test_notification();
        let state = NotificationState::new(1, notification, &defaults);
        assert_eq!(state.animation_progress, 0.0);
    }

    #[test]
    fn test_timing_fixed_duration_resolved_correctly() {
        let defaults = ManagerDefaults::default();
        let mut notification = create_test_notification();
        notification.slide_in_timing = Timing::Fixed(Duration::from_millis(300));

        let state = NotificationState::new(1, notification, &defaults);
        assert_eq!(state.actual_entry_duration, Duration::from_millis(300));
    }

    #[test]
    fn test_timing_auto_uses_default_duration() {
        let defaults = ManagerDefaults {
            default_entry_duration: Duration::from_millis(600),
            default_dwell_duration: Duration::from_secs(3),
            default_exit_duration: Duration::from_millis(800),
            default_display_time: Duration::from_secs(5),
        };
        let mut notification = create_test_notification();
        notification.slide_in_timing = Timing::Auto;

        let state = NotificationState::new(1, notification, &defaults);
        assert_eq!(state.actual_entry_duration, Duration::from_millis(600));
    }

    #[test]
    fn test_auto_dismiss_never_sets_none() {
        let defaults = ManagerDefaults::default();
        let mut notification = create_test_notification();
        notification.auto_dismiss = AutoDismiss::Never;

        let state = NotificationState::new(1, notification, &defaults);
        assert!(state.remaining_display_time.is_none());
    }

    #[test]
    fn test_auto_dismiss_after_sets_duration() {
        let defaults = ManagerDefaults::default();
        let mut notification = create_test_notification();
        notification.auto_dismiss = AutoDismiss::After(Duration::from_secs(10));

        let state = NotificationState::new(1, notification, &defaults);
        assert_eq!(state.remaining_display_time, Some(Duration::from_secs(10)));
    }

    #[test]
    fn test_auto_dismiss_zero_uses_default() {
        let defaults = ManagerDefaults {
            default_entry_duration: Duration::from_millis(500),
            default_dwell_duration: Duration::from_secs(3),
            default_exit_duration: Duration::from_millis(750),
            default_display_time: Duration::from_secs(7),
        };
        let mut notification = create_test_notification();
        notification.auto_dismiss = AutoDismiss::After(Duration::ZERO);

        let state = NotificationState::new(1, notification, &defaults);
        assert_eq!(state.remaining_display_time, Some(Duration::from_secs(7)));
    }

    #[test]
    fn test_created_at_timestamp_is_set() {
        let defaults = ManagerDefaults::default();
        let notification = create_test_notification();
        let before = Instant::now();
        let state = NotificationState::new(1, notification, &defaults);
        let after = Instant::now();

        // Timestamp should be between before and after
        assert!(state.created_at >= before);
        assert!(state.created_at <= after);
    }

    #[test]
    fn test_manager_defaults_provides_sensible_values() {
        let defaults = ManagerDefaults::default();

        assert_eq!(defaults.default_entry_duration, Duration::from_millis(500));
        assert_eq!(defaults.default_dwell_duration, Duration::from_secs(3));
        assert_eq!(defaults.default_exit_duration, Duration::from_millis(750));
        assert_eq!(defaults.default_display_time, Duration::from_secs(4));
    }

    #[test]
    fn test_id_is_stored_correctly() {
        let defaults = ManagerDefaults::default();
        let notification = create_test_notification();
        let state = NotificationState::new(42, notification, &defaults);

        assert_eq!(state.id, 42);
    }

    #[test]
    fn test_custom_positions_none_when_notification_has_none() {
        let defaults = ManagerDefaults::default();
        let notification = create_test_notification();
        let state = NotificationState::new(1, notification, &defaults);

        assert!(state.custom_entry_pos.is_none());
        assert!(state.custom_exit_pos.is_none());
    }

    #[test]
    fn test_custom_positions_copied_from_notification() {
        use ratatui::layout::Position;

        let defaults = ManagerDefaults::default();
        let mut notification = create_test_notification();
        notification.custom_entry_position = Some(Position::new(10, 20));
        notification.custom_exit_position = Some(Position::new(100, 50));

        let state = NotificationState::new(1, notification, &defaults);

        assert_eq!(state.custom_entry_pos, Some((10.0, 20.0)));
        assert_eq!(state.custom_exit_pos, Some((100.0, 50.0)));
    }

    #[test]
    fn test_all_timing_fields_resolved() {
        let defaults = ManagerDefaults::default();
        let mut notification = create_test_notification();
        notification.slide_in_timing = Timing::Fixed(Duration::from_millis(100));
        notification.dwell_timing = Timing::Fixed(Duration::from_millis(200));
        notification.slide_out_timing = Timing::Fixed(Duration::from_millis(300));

        let state = NotificationState::new(1, notification, &defaults);

        assert_eq!(state.actual_entry_duration, Duration::from_millis(100));
        assert_eq!(state.actual_dwell_duration, Duration::from_millis(200));
        assert_eq!(state.actual_exit_duration, Duration::from_millis(300));
    }
}

// FILE: src/notifications/classes/cls_notification_state.rs - NotificationState internal state management
// END OF VERSION: 1.2.0
