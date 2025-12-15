// FILE: src/notifications/orc_manager.rs - Notifications manager orchestrator
// VERSION: 1.0.0
// WCTX: Implementing Notifications manager orchestrator using TDD
// CLOG: Initial creation with manager coordination logic

use crate::notifications::classes::{Notification, NotificationState, ManagerDefaults};
use crate::notifications::orc_render::render_notifications;
use crate::notifications::types::{Anchor, NotificationError, Overflow};
use ratatui::prelude::{Frame, Rect};
use std::collections::HashMap;
use std::time::Duration;

/// Manager for animated notifications.
///
/// # Example
/// ```no_run
/// use ratatui_notifications::notifications::{Notifications, NotificationBuilder, Level};
///
/// let mut notifications = Notifications::new();
///
/// let notif = NotificationBuilder::new("Task completed!")
///     .level(Level::Info)
///     .build()
///     .unwrap();
/// notifications.add(notif).unwrap();
///
/// // In render loop:
/// notifications.tick(std::time::Duration::from_millis(16));
/// // notifications.render(&mut frame, frame.area());
/// ```
#[derive(Debug)]
pub struct Notifications {
    /// Active notification states keyed by ID
    states: HashMap<u64, NotificationState>,

    /// Notifications grouped by anchor position
    by_anchor: HashMap<Anchor, Vec<u64>>,

    /// Next available ID for new notifications
    next_id: u64,

    /// Default timing values for notifications
    defaults: ManagerDefaults,

    /// Maximum concurrent notifications per anchor (None = unlimited)
    max_concurrent: Option<usize>,

    /// Overflow behavior when max_concurrent is reached
    overflow: Overflow,
}

impl Notifications {
    /// Creates a new notifications manager with default settings.
    ///
    /// Default configuration:
    /// - Unlimited concurrent notifications
    /// - DiscardOldest overflow behavior
    /// - Standard animation timings
    pub fn new() -> Self {
        Self {
            states: HashMap::new(),
            by_anchor: HashMap::new(),
            next_id: 0,
            defaults: ManagerDefaults::default(),
            max_concurrent: None,
            overflow: Overflow::default(),
        }
    }

    /// Sets the maximum number of concurrent notifications per anchor.
    ///
    /// # Arguments
    /// * `max` - Maximum concurrent notifications (None = unlimited)
    ///
    /// # Example
    /// ```no_run
    /// use ratatui_notifications::notifications::Notifications;
    ///
    /// let manager = Notifications::new()
    ///     .max_concurrent(Some(3));
    /// ```
    pub fn max_concurrent(mut self, max: Option<usize>) -> Self {
        self.max_concurrent = max;
        self
    }

    /// Sets the overflow behavior when max_concurrent is reached.
    ///
    /// # Arguments
    /// * `behavior` - Overflow behavior (DiscardOldest or DiscardNewest)
    ///
    /// # Example
    /// ```no_run
    /// use ratatui_notifications::notifications::{Notifications, Overflow};
    ///
    /// let manager = Notifications::new()
    ///     .overflow(Overflow::DiscardNewest);
    /// ```
    pub fn overflow(mut self, behavior: Overflow) -> Self {
        self.overflow = behavior;
        self
    }

    /// Adds a notification and returns its unique ID.
    ///
    /// If max_concurrent limit is reached for the notification's anchor,
    /// applies the configured overflow behavior.
    ///
    /// # Arguments
    /// * `notification` - The notification to add
    ///
    /// # Returns
    /// * `Ok(u64)` - The unique ID assigned to the notification
    /// * `Err(NotificationError)` - If the notification is invalid
    ///
    /// # Example
    /// ```no_run
    /// use ratatui_notifications::notifications::{Notifications, NotificationBuilder};
    ///
    /// let mut manager = Notifications::new();
    /// let notif = NotificationBuilder::new("Hello!").build().unwrap();
    /// let id = manager.add(notif).unwrap();
    /// ```
    pub fn add(&mut self, notification: Notification) -> Result<u64, NotificationError> {
        // Generate ID
        let id = self.next_id;
        self.next_id = self.next_id.checked_add(1).unwrap_or(0);

        let anchor = notification.anchor;

        // Check and enforce limits
        self.enforce_limit(anchor);

        // Create state
        let state = NotificationState::new(id, notification, &self.defaults);

        // Add to maps
        self.states.insert(id, state);
        self.by_anchor.entry(anchor).or_default().push(id);

        Ok(id)
    }

    /// Removes a notification by ID.
    ///
    /// # Arguments
    /// * `id` - The notification ID to remove
    ///
    /// # Returns
    /// * `true` - If the notification existed and was removed
    /// * `false` - If the notification didn't exist
    ///
    /// # Example
    /// ```no_run
    /// use ratatui_notifications::notifications::{Notifications, NotificationBuilder};
    ///
    /// let mut manager = Notifications::new();
    /// let notif = NotificationBuilder::new("Test").build().unwrap();
    /// let id = manager.add(notif).unwrap();
    /// assert!(manager.remove(id));
    /// ```
    pub fn remove(&mut self, id: u64) -> bool {
        if let Some(state) = self.states.remove(&id) {
            // Remove from anchor map
            let anchor = state.notification.anchor;
            if let Some(ids) = self.by_anchor.get_mut(&anchor) {
                ids.retain(|&existing_id| existing_id != id);
            }
            true
        } else {
            false
        }
    }

    /// Removes all notifications.
    ///
    /// # Example
    /// ```no_run
    /// use ratatui_notifications::notifications::Notifications;
    ///
    /// let mut manager = Notifications::new();
    /// // ... add notifications ...
    /// manager.clear();
    /// ```
    pub fn clear(&mut self) {
        self.states.clear();
        self.by_anchor.clear();
    }

    /// Updates all notification animations.
    ///
    /// Call this once per frame with the elapsed time since the last update.
    ///
    /// # Arguments
    /// * `delta` - Time elapsed since last tick
    ///
    /// # Example
    /// ```no_run
    /// use ratatui_notifications::notifications::Notifications;
    /// use std::time::Duration;
    ///
    /// let mut manager = Notifications::new();
    /// manager.tick(Duration::from_millis(16)); // ~60 FPS
    /// ```
    pub fn tick(&mut self, delta: Duration) {
        // Update all notification states
        let states_to_update: Vec<u64> = self.states.keys().copied().collect();

        for id in states_to_update {
            if let Some(state) = self.states.get_mut(&id) {
                state.update(delta);
            }
        }

        // Remove finished notifications
        let finished: Vec<u64> = self.states
            .iter()
            .filter_map(|(id, state)| {
                if state.current_phase == crate::notifications::types::AnimationPhase::Finished {
                    Some(*id)
                } else {
                    None
                }
            })
            .collect();

        for id in finished {
            self.remove(id);
        }
    }

    /// Renders all active notifications to the frame.
    ///
    /// # Arguments
    /// * `frame` - The frame to render to
    /// * `area` - The area to render within
    ///
    /// # Example
    /// ```no_run
    /// use ratatui_notifications::notifications::Notifications;
    /// use ratatui::backend::TestBackend;
    /// use ratatui::Terminal;
    ///
    /// let mut manager = Notifications::new();
    /// let backend = TestBackend::new(80, 24);
    /// let mut terminal = Terminal::new(backend).unwrap();
    ///
    /// terminal.draw(|frame| {
    ///     manager.render(frame, frame.area());
    /// }).unwrap();
    /// ```
    pub fn render(&mut self, frame: &mut Frame<'_>, _area: Rect) {
        render_notifications(&mut self.states, &self.by_anchor, frame, self.max_concurrent);
    }

    /// Enforces max_concurrent limit for the given anchor.
    ///
    /// Removes oldest or newest notification as needed based on overflow behavior.
    fn enforce_limit(&mut self, anchor: Anchor) {
        if let Some(max) = self.max_concurrent {
            let current_count = self.by_anchor
                .get(&anchor)
                .map_or(0, |ids| ids.len());

            if current_count >= max {
                // Remove one notification based on overflow behavior
                let id_to_remove = match self.overflow {
                    Overflow::DiscardOldest => self.find_oldest_at_anchor(anchor),
                    Overflow::DiscardNewest => self.find_newest_at_anchor(anchor),
                };

                if let Some(id) = id_to_remove {
                    self.remove(id);
                }
            }
        }
    }

    /// Finds the oldest notification at the given anchor.
    fn find_oldest_at_anchor(&self, anchor: Anchor) -> Option<u64> {
        self.by_anchor
            .get(&anchor)?
            .iter()
            .filter_map(|id| {
                self.states
                    .get(id)
                    .map(|state| (id, state.created_at))
            })
            .min_by_key(|&(_, created_at)| created_at)
            .map(|(&id, _)| id)
    }

    /// Finds the newest notification at the given anchor.
    fn find_newest_at_anchor(&self, anchor: Anchor) -> Option<u64> {
        self.by_anchor
            .get(&anchor)?
            .iter()
            .filter_map(|id| {
                self.states
                    .get(id)
                    .map(|state| (id, state.created_at))
            })
            .max_by_key(|&(_, created_at)| created_at)
            .map(|(&id, _)| id)
    }
}

impl Default for Notifications {
    fn default() -> Self {
        Self::new()
    }
}

// FILE: src/notifications/orc_manager.rs - Notifications manager orchestrator
// END OF VERSION: 1.0.0
