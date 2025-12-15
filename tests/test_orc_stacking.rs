// FILE: tests/notifications/test_orc_stacking.rs - Tests for stacking orchestrator
// VERSION: 1.0.0
// WCTX: TDD implementation of OFPF render orchestrators
// CLOG: Initial creation with comprehensive stacking tests

use ratatui::prelude::*;
use std::collections::HashMap;
use std::time::{Duration, Instant};

// Import types and structures we'll need
use ratatui_notifications::notifications::types::{Anchor, AnimationPhase};
use ratatui_notifications::notifications::orc_stacking::calculate_stacking_positions;

// Helper struct to simulate NotificationState for testing
#[derive(Clone)]
struct MockNotificationState {
    id: u64,
    current_phase: AnimationPhase,
    created_at: Instant,
    full_rect: Rect,
    exterior_padding: u16,
}

impl MockNotificationState {
    fn new(id: u64, phase: AnimationPhase, width: u16, height: u16) -> Self {
        Self {
            id,
            current_phase: phase,
            created_at: Instant::now(),
            full_rect: Rect::new(0, 0, width, height),
            exterior_padding: 0,
        }
    }

    fn with_created_at(mut self, created_at: Instant) -> Self {
        self.created_at = created_at;
        self
    }
}

impl ratatui_notifications::notifications::orc_stacking::StackableNotification for MockNotificationState {
    fn id(&self) -> u64 {
        self.id
    }

    fn current_phase(&self) -> AnimationPhase {
        self.current_phase
    }

    fn created_at(&self) -> Instant {
        self.created_at
    }

    fn full_rect(&self) -> Rect {
        self.full_rect
    }

    fn exterior_padding(&self) -> u16 {
        self.exterior_padding
    }

    fn calculate_content_size(&self, _frame_area: Rect) -> (u16, u16) {
        // Mock implementation: return full_rect dimensions
        (self.full_rect.width, self.full_rect.height)
    }
}

#[test]
fn test_empty_notifications_returns_empty() {
    let notifications: HashMap<u64, MockNotificationState> = HashMap::new();
    let ids_at_anchor: Vec<u64> = vec![];
    let frame_area = Rect::new(0, 0, 100, 100);

    let result = calculate_stacking_positions(
        &notifications,
        Anchor::BottomRight,
        &ids_at_anchor,
        frame_area,
        None,
    );

    assert!(result.is_empty(), "Empty notifications should return empty result");
}

#[test]
fn test_single_notification_correct_position() {
    let mut notifications = HashMap::new();
    let state = MockNotificationState::new(1, AnimationPhase::Dwelling, 40, 10);
    notifications.insert(1, state);

    let ids_at_anchor = vec![1];
    let frame_area = Rect::new(0, 0, 100, 100);

    let result = calculate_stacking_positions(
        &notifications,
        Anchor::BottomRight,
        &ids_at_anchor,
        frame_area,
        None,
    );

    assert_eq!(result.len(), 1, "Should return one stacked notification");
    assert_eq!(result[0].id, 1, "Should have correct ID");
    assert!(result[0].rect.width > 0 && result[0].rect.height > 0, "Should have valid dimensions");
}

#[test]
fn test_multiple_notifications_stack_correctly() {
    let now = Instant::now();
    let mut notifications = HashMap::new();

    // Create 3 notifications with different creation times
    let state1 = MockNotificationState::new(1, AnimationPhase::Dwelling, 40, 10)
        .with_created_at(now);
    let state2 = MockNotificationState::new(2, AnimationPhase::Dwelling, 40, 10)
        .with_created_at(now + Duration::from_millis(100));
    let state3 = MockNotificationState::new(3, AnimationPhase::Dwelling, 40, 10)
        .with_created_at(now + Duration::from_millis(200));

    notifications.insert(1, state1);
    notifications.insert(2, state2);
    notifications.insert(3, state3);

    let ids_at_anchor = vec![1, 2, 3];
    let frame_area = Rect::new(0, 0, 100, 100);

    let result = calculate_stacking_positions(
        &notifications,
        Anchor::BottomRight,
        &ids_at_anchor,
        frame_area,
        None,
    );

    assert_eq!(result.len(), 3, "Should return all three notifications");

    // For bottom anchors: newest is first (at anchor), oldest is last (furthest from anchor)
    // This is the visual stacking order - newest appears at the bottom corner
    assert_eq!(result[0].id, 3, "First (at anchor) should be newest");
    assert_eq!(result[1].id, 2, "Second should be middle");
    assert_eq!(result[2].id, 1, "Third (furthest from anchor) should be oldest");

    // Verify they stack correctly: newer notifications closer to anchor (higher Y for bottom)
    // result[0] is at anchor (highest Y), result[2] is furthest up (lowest Y)
    assert!(result[2].rect.y < result[1].rect.y, "Oldest should be higher (lower Y)");
    assert!(result[1].rect.y < result[0].rect.y, "Middle should be between oldest and newest");
}

#[test]
fn test_bottom_anchors_stack_upward() {
    let now = Instant::now();
    let mut notifications = HashMap::new();

    let state1 = MockNotificationState::new(1, AnimationPhase::Dwelling, 40, 10)
        .with_created_at(now);
    let state2 = MockNotificationState::new(2, AnimationPhase::Dwelling, 40, 10)
        .with_created_at(now + Duration::from_millis(100));

    notifications.insert(1, state1);
    notifications.insert(2, state2);

    let ids_at_anchor = vec![1, 2];
    let frame_area = Rect::new(0, 0, 100, 100);

    let result = calculate_stacking_positions(
        &notifications,
        Anchor::BottomRight,
        &ids_at_anchor,
        frame_area,
        None,
    );

    assert_eq!(result.len(), 2);

    // For bottom anchors, newer notifications should be higher (lower y coordinate)
    assert!(result[1].rect.y < result[0].rect.y,
        "Bottom anchor: newer notification should have lower Y (stack upward)");
}

#[test]
fn test_top_anchors_stack_downward() {
    let now = Instant::now();
    let mut notifications = HashMap::new();

    let state1 = MockNotificationState::new(1, AnimationPhase::Dwelling, 40, 10)
        .with_created_at(now);
    let state2 = MockNotificationState::new(2, AnimationPhase::Dwelling, 40, 10)
        .with_created_at(now + Duration::from_millis(100));

    notifications.insert(1, state1);
    notifications.insert(2, state2);

    let ids_at_anchor = vec![1, 2];
    let frame_area = Rect::new(0, 0, 100, 100);

    let result = calculate_stacking_positions(
        &notifications,
        Anchor::TopRight,
        &ids_at_anchor,
        frame_area,
        None,
    );

    assert_eq!(result.len(), 2);

    // For top anchors, newer notifications should be lower (higher y coordinate)
    assert!(result[1].rect.y > result[0].rect.y,
        "Top anchor: newer notification should have higher Y (stack downward)");
}

#[test]
fn test_stacking_respects_available_height() {
    let now = Instant::now();
    let mut notifications = HashMap::new();

    // Create 10 notifications, each 15 pixels tall
    for i in 1..=10 {
        let state = MockNotificationState::new(i, AnimationPhase::Dwelling, 40, 15)
            .with_created_at(now + Duration::from_millis(i as u64 * 10));
        notifications.insert(i, state);
    }

    let ids_at_anchor: Vec<u64> = (1..=10).collect();

    // Frame area only 50 pixels tall - can only fit ~3 notifications with spacing
    let frame_area = Rect::new(0, 0, 100, 50);

    let result = calculate_stacking_positions(
        &notifications,
        Anchor::BottomRight,
        &ids_at_anchor,
        frame_area,
        None,
    );

    // Should return fewer than 10 due to height constraint
    assert!(result.len() < 10, "Should limit based on available height");
    assert!(result.len() > 0, "Should return at least some notifications");

    // All returned notifications should fit within frame
    for stacked in &result {
        assert!(stacked.rect.y >= frame_area.y, "Y should be within frame");
        assert!(stacked.rect.bottom() <= frame_area.bottom(), "Bottom should be within frame");
    }
}

#[test]
fn test_max_concurrent_limit_respected() {
    let now = Instant::now();
    let mut notifications = HashMap::new();

    // Create 10 notifications
    for i in 1..=10 {
        let state = MockNotificationState::new(i, AnimationPhase::Dwelling, 40, 10)
            .with_created_at(now + Duration::from_millis(i as u64 * 10));
        notifications.insert(i, state);
    }

    let ids_at_anchor: Vec<u64> = (1..=10).collect();
    let frame_area = Rect::new(0, 0, 100, 200); // Plenty of space

    // Set max_concurrent to 3
    let result = calculate_stacking_positions(
        &notifications,
        Anchor::BottomRight,
        &ids_at_anchor,
        frame_area,
        Some(3),
    );

    assert_eq!(result.len(), 3, "Should respect max_concurrent limit of 3");

    // Should keep the newest 3 (IDs 8, 9, 10)
    // For bottom anchors: newest first (at anchor), oldest last (furthest from anchor)
    assert_eq!(result[0].id, 10, "First (at anchor) should be newest");
    assert_eq!(result[1].id, 9, "Second should be second newest");
    assert_eq!(result[2].id, 8, "Third (furthest from anchor) should be third newest");
}

#[test]
fn test_pending_and_finished_notifications_excluded() {
    let now = Instant::now();
    let mut notifications = HashMap::new();

    let state1 = MockNotificationState::new(1, AnimationPhase::Pending, 40, 10)
        .with_created_at(now);
    let state2 = MockNotificationState::new(2, AnimationPhase::Dwelling, 40, 10)
        .with_created_at(now + Duration::from_millis(100));
    let state3 = MockNotificationState::new(3, AnimationPhase::Finished, 40, 10)
        .with_created_at(now + Duration::from_millis(200));

    notifications.insert(1, state1);
    notifications.insert(2, state2);
    notifications.insert(3, state3);

    let ids_at_anchor = vec![1, 2, 3];
    let frame_area = Rect::new(0, 0, 100, 100);

    let result = calculate_stacking_positions(
        &notifications,
        Anchor::BottomRight,
        &ids_at_anchor,
        frame_area,
        None,
    );

    assert_eq!(result.len(), 1, "Should only include visible notifications");
    assert_eq!(result[0].id, 2, "Should only include the Dwelling notification");
}

// FILE: tests/notifications/test_orc_stacking.rs - Tests for stacking orchestrator
// END OF VERSION: 1.0.0
