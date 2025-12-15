// FILE: src/notifications/classes/cls_notification.rs - Notification class with builder
// VERSION: 2.2.0
// WCTX: Adding code generation support
// CLOG: Added content() and title() getters for code generation feature

use ratatui::prelude::*;
use ratatui::widgets::{BorderType, Padding};

use crate::notifications::types::{
    Anchor, Animation, AutoDismiss, Level, NotificationError, SlideDirection, SizeConstraint, Timing,
};

/// Maximum allowed characters in notification content.
const MAX_CONTENT_CHARS: usize = 1000;

/// A notification with content, styling, and animation configuration.
///
/// Notifications are created using the builder pattern via `NotificationBuilder`.
/// Each notification has content, optional title, and extensive styling/animation options.
///
/// # Example
///
/// ```no_run
/// use ratatui_notifications::notifications::{NotificationBuilder, Level};
///
/// let notification = NotificationBuilder::new("Hello, world!")
///     .title("Greeting")
///     .level(Level::Info)
///     .build()
///     .unwrap();
/// ```
#[derive(Debug, Clone)]
pub struct Notification {
    /// The notification content (body text).
    pub(crate) content: Text<'static>,

    /// Optional title line displayed at the top.
    pub(crate) title: Option<Line<'static>>,

    /// Severity level affecting visual styling.
    pub(crate) level: Option<Level>,

    /// Screen position from which notification expands.
    pub(crate) anchor: Anchor,

    /// Animation style for entry and exit.
    pub(crate) animation: Animation,

    /// Direction from which notification slides (for Slide animation).
    pub(crate) slide_direction: SlideDirection,

    /// Duration for slide-in animation.
    pub(crate) slide_in_timing: Timing,

    /// Duration notification remains visible before auto-dismiss.
    pub(crate) dwell_timing: Timing,

    /// Duration for slide-out animation.
    pub(crate) slide_out_timing: Timing,

    /// Auto-dismissal behavior.
    pub(crate) auto_dismiss: AutoDismiss,

    /// Maximum width constraint.
    pub(crate) max_width: Option<SizeConstraint>,

    /// Maximum height constraint.
    pub(crate) max_height: Option<SizeConstraint>,

    /// Inner padding around content.
    pub(crate) padding: Padding,

    /// Outer margin from screen edge.
    pub(crate) exterior_margin: u16,

    /// Overall block style.
    pub(crate) block_style: Option<Style>,

    /// Border style.
    pub(crate) border_style: Option<Style>,

    /// Title style.
    pub(crate) title_style: Option<Style>,

    /// Border type (single, double, thick, etc.).
    pub(crate) border_type: Option<BorderType>,

    /// Custom entry position (overrides anchor-based calculation).
    pub(crate) custom_entry_position: Option<Position>,

    /// Custom exit position (overrides anchor-based calculation).
    pub(crate) custom_exit_position: Option<Position>,

    /// Whether to apply fade effect during animation.
    pub(crate) fade_effect: bool,
}

impl Notification {
    /// Creates a new notification builder with the given content.
    ///
    /// This is a convenience method that returns a `NotificationBuilder`.
    ///
    /// # Example
    ///
    /// ```no_run
    /// use ratatui_notifications::Notification;
    ///
    /// let notification = Notification::new("Hello, world!")
    ///     .title("Greeting")
    ///     .build()
    ///     .unwrap();
    /// ```
    #[allow(clippy::new_ret_no_self)]
    pub fn new(content: impl Into<Text<'static>>) -> NotificationBuilder {
        NotificationBuilder::new(content)
    }

    // ========================================================================
    // Public Getters - Allow inspection of notification configuration
    // ========================================================================

    /// Returns the notification's content text.
    pub fn content(&self) -> &Text<'static> {
        &self.content
    }

    /// Returns the notification's title, if set.
    pub fn title(&self) -> Option<&Line<'static>> {
        self.title.as_ref()
    }

    /// Returns the notification's severity level.
    pub fn level(&self) -> Option<Level> {
        self.level
    }

    /// Returns the notification's anchor position.
    pub fn anchor(&self) -> Anchor {
        self.anchor
    }

    /// Returns the notification's animation type.
    pub fn animation(&self) -> Animation {
        self.animation
    }

    /// Returns the notification's slide direction.
    pub fn slide_direction(&self) -> SlideDirection {
        self.slide_direction
    }

    /// Returns the slide-in timing configuration.
    pub fn slide_in_timing(&self) -> Timing {
        self.slide_in_timing
    }

    /// Returns the dwell timing configuration.
    pub fn dwell_timing(&self) -> Timing {
        self.dwell_timing
    }

    /// Returns the slide-out timing configuration.
    pub fn slide_out_timing(&self) -> Timing {
        self.slide_out_timing
    }

    /// Returns the auto-dismiss configuration.
    pub fn auto_dismiss(&self) -> AutoDismiss {
        self.auto_dismiss
    }

    /// Returns the maximum width constraint.
    pub fn max_width(&self) -> Option<SizeConstraint> {
        self.max_width
    }

    /// Returns the maximum height constraint.
    pub fn max_height(&self) -> Option<SizeConstraint> {
        self.max_height
    }

    /// Returns the inner padding.
    pub fn padding(&self) -> Padding {
        self.padding
    }

    /// Returns the exterior margin.
    pub fn exterior_margin(&self) -> u16 {
        self.exterior_margin
    }

    /// Returns the border type.
    pub fn border_type(&self) -> Option<BorderType> {
        self.border_type
    }

    /// Returns the custom entry position.
    pub fn custom_entry_position(&self) -> Option<Position> {
        self.custom_entry_position
    }

    /// Returns the custom exit position.
    pub fn custom_exit_position(&self) -> Option<Position> {
        self.custom_exit_position
    }

    /// Returns whether fade effect is enabled.
    pub fn fade_effect(&self) -> bool {
        self.fade_effect
    }
}

impl Default for Notification {
    fn default() -> Self {
        Self {
            content: Text::from(""),
            title: None,
            level: Some(Level::Info),
            anchor: Anchor::default(),
            animation: Animation::default(),
            slide_direction: SlideDirection::default(),
            slide_in_timing: Timing::default(),
            dwell_timing: Timing::default(),
            slide_out_timing: Timing::default(),
            auto_dismiss: AutoDismiss::default(),
            max_width: Some(SizeConstraint::Percentage(0.4)),
            max_height: Some(SizeConstraint::Percentage(0.2)),
            padding: Padding::horizontal(1),
            exterior_margin: 0,
            block_style: None,
            border_style: None,
            title_style: None,
            border_type: Some(BorderType::Rounded),
            custom_entry_position: None,
            custom_exit_position: None,
            fade_effect: false,
        }
    }
}

/// Builder for constructing notifications with fluent API.
///
/// # Example
///
/// ```no_run
/// use ratatui_notifications::notifications::{NotificationBuilder, Level, Anchor};
/// use std::time::Duration;
///
/// let notification = NotificationBuilder::new("Important message")
///     .title("Alert")
///     .level(Level::Warn)
///     .anchor(Anchor::TopCenter)
///     .build()
///     .unwrap();
/// ```
#[derive(Debug, Clone)]
pub struct NotificationBuilder {
    notification: Notification,
}

impl NotificationBuilder {
    /// Creates a new notification builder with the specified content.
    ///
    /// # Arguments
    ///
    /// * `content` - The notification content text
    ///
    /// # Example
    ///
    /// ```no_run
    /// use ratatui_notifications::notifications::NotificationBuilder;
    ///
    /// let builder = NotificationBuilder::new("Hello!");
    /// ```
    pub fn new(content: impl Into<Text<'static>>) -> Self {
        Self {
            notification: Notification {
                content: content.into(),
                ..Default::default()
            },
        }
    }

    /// Sets the notification title.
    ///
    /// # Arguments
    ///
    /// * `title` - Title text displayed at the top of the notification
    ///
    /// # Example
    ///
    /// ```no_run
    /// use ratatui_notifications::notifications::NotificationBuilder;
    ///
    /// let notification = NotificationBuilder::new("Content")
    ///     .title("My Title")
    ///     .build()
    ///     .unwrap();
    /// ```
    pub fn title(mut self, title: impl Into<Line<'static>>) -> Self {
        self.notification.title = Some(title.into());
        self
    }

    /// Sets the notification severity level.
    ///
    /// # Arguments
    ///
    /// * `level` - Severity level (Info, Warn, Error, Debug, Trace)
    pub fn level(mut self, level: Level) -> Self {
        self.notification.level = Some(level);
        self
    }

    /// Sets the screen anchor position.
    ///
    /// # Arguments
    ///
    /// * `anchor` - Position from which notification expands
    pub fn anchor(mut self, anchor: Anchor) -> Self {
        self.notification.anchor = anchor;
        self
    }

    /// Sets the animation type.
    ///
    /// # Arguments
    ///
    /// * `animation` - Animation style (Slide, ExpandCollapse, Fade)
    pub fn animation(mut self, animation: Animation) -> Self {
        self.notification.animation = animation;
        self
    }

    /// Sets the slide direction.
    ///
    /// # Arguments
    ///
    /// * `direction` - Direction from which notification slides in
    pub fn slide_direction(mut self, direction: SlideDirection) -> Self {
        self.notification.slide_direction = direction;
        self
    }

    /// Sets the animation timings.
    ///
    /// # Arguments
    ///
    /// * `slide_in` - Duration for slide-in animation
    /// * `dwell` - Duration notification remains visible
    /// * `slide_out` - Duration for slide-out animation
    pub fn timing(mut self, slide_in: Timing, dwell: Timing, slide_out: Timing) -> Self {
        self.notification.slide_in_timing = slide_in;
        self.notification.dwell_timing = dwell;
        self.notification.slide_out_timing = slide_out;
        self
    }

    /// Sets auto-dismiss behavior.
    ///
    /// # Arguments
    ///
    /// * `auto_dismiss` - When to automatically dismiss the notification
    pub fn auto_dismiss(mut self, auto_dismiss: AutoDismiss) -> Self {
        self.notification.auto_dismiss = auto_dismiss;
        self
    }

    /// Sets maximum size constraints.
    ///
    /// # Arguments
    ///
    /// * `width` - Maximum width constraint
    /// * `height` - Maximum height constraint
    pub fn max_size(mut self, width: SizeConstraint, height: SizeConstraint) -> Self {
        self.notification.max_width = Some(width);
        self.notification.max_height = Some(height);
        self
    }

    /// Sets inner padding.
    ///
    /// # Arguments
    ///
    /// * `padding` - Padding around content
    pub fn padding(mut self, padding: Padding) -> Self {
        self.notification.padding = padding;
        self
    }

    /// Sets exterior margin.
    ///
    /// # Arguments
    ///
    /// * `margin` - Margin from screen edge
    pub fn margin(mut self, margin: u16) -> Self {
        self.notification.exterior_margin = margin;
        self
    }

    /// Sets block style.
    ///
    /// # Arguments
    ///
    /// * `style` - Overall block style
    pub fn style(mut self, style: Style) -> Self {
        self.notification.block_style = Some(style);
        self
    }

    /// Sets border style.
    ///
    /// # Arguments
    ///
    /// * `style` - Border style
    pub fn border_style(mut self, style: Style) -> Self {
        self.notification.border_style = Some(style);
        self
    }

    /// Sets title style.
    ///
    /// # Arguments
    ///
    /// * `style` - Title text style
    pub fn title_style(mut self, style: Style) -> Self {
        self.notification.title_style = Some(style);
        self
    }

    /// Sets border type.
    ///
    /// # Arguments
    ///
    /// * `border_type` - Border type (Single, Double, Thick, etc.)
    pub fn border_type(mut self, border_type: BorderType) -> Self {
        self.notification.border_type = Some(border_type);
        self
    }

    /// Sets custom entry position.
    ///
    /// # Arguments
    ///
    /// * `position` - Custom position for notification entry
    pub fn entry_position(mut self, position: Position) -> Self {
        self.notification.custom_entry_position = Some(position);
        self
    }

    /// Sets custom exit position.
    ///
    /// # Arguments
    ///
    /// * `position` - Custom position for notification exit
    pub fn exit_position(mut self, position: Position) -> Self {
        self.notification.custom_exit_position = Some(position);
        self
    }

    /// Enables or disables fade effect.
    ///
    /// # Arguments
    ///
    /// * `enable` - Whether to apply fade effect during animation
    pub fn fade(mut self, enable: bool) -> Self {
        self.notification.fade_effect = enable;
        self
    }

    /// Builds the notification, validating content size.
    ///
    /// # Returns
    ///
    /// * `Ok(Notification)` if validation passes
    /// * `Err(NotificationError::ContentTooLarge)` if content exceeds limit
    ///
    /// # Errors
    ///
    /// Returns error if content exceeds `MAX_CONTENT_CHARS` (1000) characters.
    pub fn build(self) -> Result<Notification, NotificationError> {
        // Validate content size
        let content_str = self.notification.content.to_string();
        let char_count = content_str.chars().count();

        if char_count > MAX_CONTENT_CHARS {
            return Err(NotificationError::ContentTooLarge(
                char_count,
                MAX_CONTENT_CHARS,
            ));
        }

        Ok(self.notification)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::time::Duration;

    #[test]
    fn test_default_notification_has_sensible_defaults() {
        let notification = Notification::default();

        // Check default values match specification
        assert_eq!(notification.anchor, Anchor::BottomRight);
        assert_eq!(notification.animation, Animation::Slide);
        assert_eq!(notification.slide_direction, SlideDirection::Default);
        assert_eq!(notification.auto_dismiss, AutoDismiss::After(Duration::from_secs(4)));
        assert_eq!(notification.level, Some(Level::Info));
        assert_eq!(notification.title, None);
        assert_eq!(notification.fade_effect, false);
        assert_eq!(notification.exterior_margin, 0);
        assert_eq!(notification.max_width, Some(SizeConstraint::Percentage(0.4)));
        assert_eq!(notification.max_height, Some(SizeConstraint::Percentage(0.2)));
        assert_eq!(notification.border_type, Some(BorderType::Rounded));
        assert_eq!(notification.padding, Padding::horizontal(1));

        // Content should be empty by default
        assert_eq!(notification.content.to_string(), "");
    }

    #[test]
    fn test_builder_sets_content_correctly() {
        let notification = NotificationBuilder::new("Hello, world!")
            .build()
            .unwrap();

        assert_eq!(notification.content.to_string(), "Hello, world!");
    }

    #[test]
    fn test_builder_sets_title_correctly() {
        let notification = NotificationBuilder::new("Content")
            .title("My Title")
            .build()
            .unwrap();

        assert!(notification.title.is_some());
        assert_eq!(notification.title.unwrap().to_string(), "My Title");
    }

    #[test]
    fn test_builder_sets_level() {
        let notification = NotificationBuilder::new("Error message")
            .level(Level::Error)
            .build()
            .unwrap();

        assert_eq!(notification.level, Some(Level::Error));
    }

    #[test]
    fn test_builder_sets_anchor() {
        let notification = NotificationBuilder::new("Test")
            .anchor(Anchor::TopLeft)
            .build()
            .unwrap();

        assert_eq!(notification.anchor, Anchor::TopLeft);
    }

    #[test]
    fn test_builder_sets_animation() {
        let notification = NotificationBuilder::new("Test")
            .animation(Animation::Fade)
            .build()
            .unwrap();

        assert_eq!(notification.animation, Animation::Fade);
    }

    #[test]
    fn test_builder_sets_slide_direction() {
        let notification = NotificationBuilder::new("Test")
            .slide_direction(SlideDirection::FromTop)
            .build()
            .unwrap();

        assert_eq!(notification.slide_direction, SlideDirection::FromTop);
    }

    #[test]
    fn test_builder_sets_timings() {
        let slide_in = Timing::Fixed(Duration::from_millis(200));
        let dwell = Timing::Fixed(Duration::from_millis(3000));
        let slide_out = Timing::Fixed(Duration::from_millis(150));

        let notification = NotificationBuilder::new("Test")
            .timing(slide_in, dwell, slide_out)
            .build()
            .unwrap();

        assert_eq!(notification.slide_in_timing, slide_in);
        assert_eq!(notification.dwell_timing, dwell);
        assert_eq!(notification.slide_out_timing, slide_out);
    }

    #[test]
    fn test_builder_sets_auto_dismiss() {
        let notification = NotificationBuilder::new("Test")
            .auto_dismiss(AutoDismiss::Never)
            .build()
            .unwrap();

        assert_eq!(notification.auto_dismiss, AutoDismiss::Never);
    }

    #[test]
    fn test_builder_sets_max_size() {
        let notification = NotificationBuilder::new("Test")
            .max_size(SizeConstraint::Absolute(50), SizeConstraint::Percentage(0.3))
            .build()
            .unwrap();

        assert_eq!(notification.max_width, Some(SizeConstraint::Absolute(50)));
        assert_eq!(notification.max_height, Some(SizeConstraint::Percentage(0.3)));
    }

    #[test]
    fn test_builder_sets_padding() {
        let padding = Padding::new(1, 2, 3, 4);

        let notification = NotificationBuilder::new("Test")
            .padding(padding)
            .build()
            .unwrap();

        assert_eq!(notification.padding, padding);
    }

    #[test]
    fn test_builder_sets_margin() {
        let notification = NotificationBuilder::new("Test")
            .margin(5)
            .build()
            .unwrap();

        assert_eq!(notification.exterior_margin, 5);
    }

    #[test]
    fn test_builder_sets_block_style() {
        let style = Style::default().fg(Color::Red);

        let notification = NotificationBuilder::new("Test")
            .style(style)
            .build()
            .unwrap();

        assert_eq!(notification.block_style, Some(style));
    }

    #[test]
    fn test_builder_sets_border_style() {
        let style = Style::default().fg(Color::Blue);

        let notification = NotificationBuilder::new("Test")
            .border_style(style)
            .build()
            .unwrap();

        assert_eq!(notification.border_style, Some(style));
    }

    #[test]
    fn test_builder_sets_title_style() {
        let style = Style::default().fg(Color::Green);

        let notification = NotificationBuilder::new("Test")
            .title_style(style)
            .build()
            .unwrap();

        assert_eq!(notification.title_style, Some(style));
    }

    #[test]
    fn test_builder_sets_border_type() {
        let notification = NotificationBuilder::new("Test")
            .border_type(BorderType::Double)
            .build()
            .unwrap();

        assert_eq!(notification.border_type, Some(BorderType::Double));
    }

    #[test]
    fn test_builder_sets_custom_entry_position() {
        let pos = Position::new(10, 20);

        let notification = NotificationBuilder::new("Test")
            .entry_position(pos)
            .build()
            .unwrap();

        assert_eq!(notification.custom_entry_position, Some(pos));
    }

    #[test]
    fn test_builder_sets_custom_exit_position() {
        let pos = Position::new(5, 15);

        let notification = NotificationBuilder::new("Test")
            .exit_position(pos)
            .build()
            .unwrap();

        assert_eq!(notification.custom_exit_position, Some(pos));
    }

    #[test]
    fn test_builder_sets_fade_effect() {
        let notification = NotificationBuilder::new("Test")
            .fade(true)
            .build()
            .unwrap();

        assert_eq!(notification.fade_effect, true);
    }

    #[test]
    fn test_builder_builds_with_all_options() {
        let padding = Padding::uniform(2);
        let style = Style::default().fg(Color::Yellow);
        let border_style = Style::default().fg(Color::Cyan);
        let title_style = Style::default().fg(Color::Magenta);
        let entry_pos = Position::new(0, 0);
        let exit_pos = Position::new(100, 100);

        let notification = NotificationBuilder::new("Full config test")
            .title("Test Title")
            .level(Level::Warn)
            .anchor(Anchor::TopCenter)
            .animation(Animation::ExpandCollapse)
            .slide_direction(SlideDirection::FromBottom)
            .timing(
                Timing::Fixed(Duration::from_millis(100)),
                Timing::Fixed(Duration::from_millis(2000)),
                Timing::Fixed(Duration::from_millis(100)),
            )
            .auto_dismiss(AutoDismiss::After(Duration::from_secs(5)))
            .max_size(SizeConstraint::Percentage(0.5), SizeConstraint::Absolute(10))
            .padding(padding)
            .margin(3)
            .style(style)
            .border_style(border_style)
            .title_style(title_style)
            .border_type(BorderType::Thick)
            .entry_position(entry_pos)
            .exit_position(exit_pos)
            .fade(true)
            .build()
            .unwrap();

        // Verify all fields
        assert_eq!(notification.content.to_string(), "Full config test");
        assert_eq!(notification.title.unwrap().to_string(), "Test Title");
        assert_eq!(notification.level, Some(Level::Warn));
        assert_eq!(notification.anchor, Anchor::TopCenter);
        assert_eq!(notification.animation, Animation::ExpandCollapse);
        assert_eq!(notification.slide_direction, SlideDirection::FromBottom);
        assert_eq!(notification.auto_dismiss, AutoDismiss::After(Duration::from_secs(5)));
        assert_eq!(notification.max_width, Some(SizeConstraint::Percentage(0.5)));
        assert_eq!(notification.max_height, Some(SizeConstraint::Absolute(10)));
        assert_eq!(notification.padding, padding);
        assert_eq!(notification.exterior_margin, 3);
        assert_eq!(notification.block_style, Some(style));
        assert_eq!(notification.border_style, Some(border_style));
        assert_eq!(notification.title_style, Some(title_style));
        assert_eq!(notification.border_type, Some(BorderType::Thick));
        assert_eq!(notification.custom_entry_position, Some(entry_pos));
        assert_eq!(notification.custom_exit_position, Some(exit_pos));
        assert_eq!(notification.fade_effect, true);
    }

    #[test]
    fn test_content_validation_accepts_valid_content() {
        // Create content just under the limit (1000 chars)
        let content = "a".repeat(999);

        let result = NotificationBuilder::new(Text::from(content)).build();

        assert!(result.is_ok());
    }

    #[test]
    fn test_content_validation_rejects_oversized_content() {
        // Create content exceeding the limit (1000 chars)
        let content = "a".repeat(1001);

        let result = NotificationBuilder::new(Text::from(content)).build();

        assert!(result.is_err());
        match result {
            Err(NotificationError::ContentTooLarge(size, limit)) => {
                assert!(size > 1000);
                assert_eq!(limit, 1000);
            }
            _ => panic!("Expected ContentTooLarge error"),
        }
    }

    #[test]
    fn test_content_validation_at_boundary() {
        // Exactly 1000 chars should be accepted
        let content = "a".repeat(1000);

        let result = NotificationBuilder::new(Text::from(content)).build();

        assert!(result.is_ok());
    }

    #[test]
    fn test_notification_implements_debug() {
        let notification = NotificationBuilder::new("Test")
            .title("Debug Test")
            .build()
            .unwrap();

        // Should be able to format with Debug
        let debug_str = format!("{:?}", notification);
        assert!(debug_str.contains("Notification"));
    }

    #[test]
    fn test_notification_implements_clone() {
        let notification = NotificationBuilder::new("Original")
            .title("Clone Test")
            .level(Level::Info)
            .build()
            .unwrap();

        let cloned = notification.clone();

        assert_eq!(notification.content.to_string(), cloned.content.to_string());
        assert_eq!(notification.title.as_ref().map(|l| l.to_string()),
                   cloned.title.as_ref().map(|l| l.to_string()));
        assert_eq!(notification.level, cloned.level);
    }

    #[test]
    fn test_builder_fluent_interface() {
        // Test that methods can be chained
        let _notification = NotificationBuilder::new("Fluent test")
            .title("Title")
            .level(Level::Debug)
            .anchor(Anchor::MiddleCenter)
            .animation(Animation::Slide)
            .fade(false)
            .build()
            .unwrap();

        // If this compiles, fluent interface works
    }

    #[test]
    fn test_multiline_content() {
        let content = "Line 1\nLine 2\nLine 3";

        let notification = NotificationBuilder::new(content)
            .build()
            .unwrap();

        assert_eq!(notification.content.to_string(), content);
    }

    #[test]
    fn test_empty_content() {
        let notification = NotificationBuilder::new("")
            .build()
            .unwrap();

        assert_eq!(notification.content.to_string(), "");
    }
}

// FILE: src/notifications/classes/cls_notification.rs - Notification class with builder
// END OF VERSION: 2.2.0
