// FILE: tests/notifications/functions/test_fnc_update_states.rs - Tests for fnc_update_states
// VERSION: 1.0.0
// WCTX: TDD implementation of update_states function
// CLOG: Initial creation with comprehensive state machine tests

use ratatui_notifications::notifications::classes::cls_notification_state::{NotificationState, ManagerDefaults};
use ratatui_notifications::notifications::classes::cls_notification::Notification;
use ratatui_notifications::notifications::functions::fnc_update_states::update_states;
use ratatui_notifications::notifications::types::{Animation, AnimationPhase, Timing, AutoDismiss};
use ratatui::prelude::*;
use std::collections::HashMap;
use std::time::Duration;

fn create_test_notification(animation: Animation) -> Notification {
    Notification {
        content: Text::raw("Test notification"),
        animation,
        slide_in_timing: Timing::Fixed(Duration::from_millis(100)),
        slide_out_timing: Timing::Fixed(Duration::from_millis(100)),
        auto_dismiss: AutoDismiss::After(Duration::from_millis(200)),
        ..Default::default()
    }
}

#[test]
fn test_pending_to_sliding_in_for_slide_animation() {
    let defaults = ManagerDefaults::default();
    let notification = create_test_notification(Animation::Slide);
    let mut states = HashMap::new();
    states.insert(1, NotificationState::new(1, notification, &defaults));

    assert_eq!(states[&1].current_phase, AnimationPhase::Pending);

    update_states(&mut states, Duration::from_millis(10));

    assert_eq!(states[&1].current_phase, AnimationPhase::SlidingIn);
    assert!(states[&1].animation_progress > 0.0);
}

#[test]
fn test_pending_to_expanding_for_expand_collapse_animation() {
    let defaults = ManagerDefaults::default();
    let notification = create_test_notification(Animation::ExpandCollapse);
    let mut states = HashMap::new();
    states.insert(1, NotificationState::new(1, notification, &defaults));

    assert_eq!(states[&1].current_phase, AnimationPhase::Pending);

    update_states(&mut states, Duration::from_millis(10));

    assert_eq!(states[&1].current_phase, AnimationPhase::Expanding);
    assert!(states[&1].animation_progress > 0.0);
}

#[test]
fn test_pending_to_fading_in_for_fade_animation() {
    let defaults = ManagerDefaults::default();
    let notification = create_test_notification(Animation::Fade);
    let mut states = HashMap::new();
    states.insert(1, NotificationState::new(1, notification, &defaults));

    assert_eq!(states[&1].current_phase, AnimationPhase::Pending);

    update_states(&mut states, Duration::from_millis(10));

    assert_eq!(states[&1].current_phase, AnimationPhase::FadingIn);
    assert!(states[&1].animation_progress > 0.0);
}

#[test]
fn test_progress_increases_correctly_with_delta_time() {
    let defaults = ManagerDefaults::default();
    let notification = create_test_notification(Animation::Slide);
    let mut states = HashMap::new();
    states.insert(1, NotificationState::new(1, notification, &defaults));

    // First update: Pending -> SlidingIn
    update_states(&mut states, Duration::from_millis(10));
    assert_eq!(states[&1].current_phase, AnimationPhase::SlidingIn);

    let initial_progress = states[&1].animation_progress;
    assert!(initial_progress > 0.0);

    // Second update: Progress should increase
    update_states(&mut states, Duration::from_millis(10));
    assert!(states[&1].animation_progress > initial_progress);
}

#[test]
fn test_entry_animation_completes_at_progress_one() {
    let defaults = ManagerDefaults::default();
    let notification = create_test_notification(Animation::Slide);
    let mut states = HashMap::new();
    states.insert(1, NotificationState::new(1, notification, &defaults));

    // Advance through entire entry animation
    update_states(&mut states, Duration::from_millis(100));

    assert_eq!(states[&1].current_phase, AnimationPhase::Dwelling);
    assert_eq!(states[&1].animation_progress, 1.0);
}

#[test]
fn test_transitions_to_dwelling_after_entry_complete() {
    let defaults = ManagerDefaults::default();

    // Test all three animation types
    for animation in [Animation::Slide, Animation::ExpandCollapse, Animation::Fade] {
        let notification = create_test_notification(animation);
        let mut states = HashMap::new();
        states.insert(1, NotificationState::new(1, notification, &defaults));

        // Complete entry animation
        update_states(&mut states, Duration::from_millis(100));

        assert_eq!(
            states[&1].current_phase,
            AnimationPhase::Dwelling,
            "Failed for {:?}", animation
        );
    }
}

#[test]
fn test_display_timer_counts_down_during_dwelling() {
    let defaults = ManagerDefaults::default();
    let notification = create_test_notification(Animation::Slide);
    let mut states = HashMap::new();
    states.insert(1, NotificationState::new(1, notification, &defaults));

    // Complete entry animation to reach Dwelling
    update_states(&mut states, Duration::from_millis(100));
    assert_eq!(states[&1].current_phase, AnimationPhase::Dwelling);

    let initial_time = states[&1].remaining_display_time;
    assert!(initial_time.is_some());

    // Count down timer
    update_states(&mut states, Duration::from_millis(50));

    let new_time = states[&1].remaining_display_time;
    assert!(new_time.is_some());
    assert!(new_time.unwrap() < initial_time.unwrap());
}

#[test]
fn test_timer_expiry_triggers_exit_animation() {
    let defaults = ManagerDefaults::default();

    // Test all three animation types
    let test_cases = vec![
        (Animation::Slide, AnimationPhase::SlidingOut),
        (Animation::ExpandCollapse, AnimationPhase::Collapsing),
        (Animation::Fade, AnimationPhase::FadingOut),
    ];

    for (animation, expected_exit_phase) in test_cases {
        let notification = create_test_notification(animation);
        let mut states = HashMap::new();
        states.insert(1, NotificationState::new(1, notification, &defaults));

        // Complete entry animation
        update_states(&mut states, Duration::from_millis(100));
        assert_eq!(states[&1].current_phase, AnimationPhase::Dwelling);

        // Complete display timer (200ms total, already spent 100ms)
        update_states(&mut states, Duration::from_millis(200));

        assert_eq!(
            states[&1].current_phase,
            expected_exit_phase,
            "Failed for {:?}", animation
        );
        assert_eq!(states[&1].animation_progress, 0.0);
    }
}

#[test]
fn test_exit_animation_completes_to_finished() {
    let defaults = ManagerDefaults::default();
    let notification = create_test_notification(Animation::Slide);
    let mut states = HashMap::new();
    states.insert(1, NotificationState::new(1, notification, &defaults));

    // Complete entry animation
    update_states(&mut states, Duration::from_millis(100));

    // Complete display timer
    update_states(&mut states, Duration::from_millis(200));
    assert_eq!(states[&1].current_phase, AnimationPhase::SlidingOut);

    // Complete exit animation
    update_states(&mut states, Duration::from_millis(100));

    assert_eq!(states[&1].current_phase, AnimationPhase::Finished);
    assert_eq!(states[&1].animation_progress, 1.0);
}

#[test]
fn test_returns_ids_of_finished_notifications() {
    let defaults = ManagerDefaults::default();
    let notification = create_test_notification(Animation::Slide);
    let mut states = HashMap::new();
    states.insert(1, NotificationState::new(1, notification, &defaults));

    // Go through full lifecycle
    update_states(&mut states, Duration::from_millis(100)); // Entry
    update_states(&mut states, Duration::from_millis(200)); // Dwell
    let finished_ids = update_states(&mut states, Duration::from_millis(100)); // Exit

    assert_eq!(finished_ids.len(), 1);
    assert!(finished_ids.contains(&1));
}

#[test]
fn test_multiple_notifications_at_different_phases() {
    let defaults = ManagerDefaults::default();
    let mut states = HashMap::new();

    // Notification 1: Pending
    let notif1 = create_test_notification(Animation::Slide);
    states.insert(1, NotificationState::new(1, notif1, &defaults));

    // Notification 2: Already in SlidingIn
    let notif2 = create_test_notification(Animation::Fade);
    let mut state2 = NotificationState::new(2, notif2, &defaults);
    state2.update(Duration::from_millis(10)); // Start it
    states.insert(2, state2);

    update_states(&mut states, Duration::from_millis(10));

    // Both should have progressed
    assert_eq!(states[&1].current_phase, AnimationPhase::SlidingIn);
    assert_eq!(states[&2].current_phase, AnimationPhase::FadingIn);
}

#[test]
fn test_no_finished_ids_when_none_finish() {
    let defaults = ManagerDefaults::default();
    let notification = create_test_notification(Animation::Slide);
    let mut states = HashMap::new();
    states.insert(1, NotificationState::new(1, notification, &defaults));

    let finished_ids = update_states(&mut states, Duration::from_millis(10));

    assert!(finished_ids.is_empty());
}

#[test]
fn test_multiple_notifications_finish_simultaneously() {
    let defaults = ManagerDefaults::default();
    let mut states = HashMap::new();

    for id in 1..=3 {
        let notification = create_test_notification(Animation::Slide);
        states.insert(id, NotificationState::new(id, notification, &defaults));
    }

    // Run full lifecycle for all
    update_states(&mut states, Duration::from_millis(100)); // Entry
    update_states(&mut states, Duration::from_millis(200)); // Dwell
    let finished_ids = update_states(&mut states, Duration::from_millis(100)); // Exit

    assert_eq!(finished_ids.len(), 3);
    assert!(finished_ids.contains(&1));
    assert!(finished_ids.contains(&2));
    assert!(finished_ids.contains(&3));
}

#[test]
fn test_dwelling_without_auto_dismiss() {
    let defaults = ManagerDefaults::default();
    let mut notification = create_test_notification(Animation::Slide);
    notification.auto_dismiss = AutoDismiss::Never;

    let mut states = HashMap::new();
    states.insert(1, NotificationState::new(1, notification, &defaults));

    // Complete entry animation
    update_states(&mut states, Duration::from_millis(100));
    assert_eq!(states[&1].current_phase, AnimationPhase::Dwelling);

    // Should stay in Dwelling indefinitely
    update_states(&mut states, Duration::from_millis(1000));
    assert_eq!(states[&1].current_phase, AnimationPhase::Dwelling);
}

// FILE: tests/notifications/functions/test_fnc_update_states.rs - Tests for fnc_update_states
// END OF VERSION: 1.0.0
