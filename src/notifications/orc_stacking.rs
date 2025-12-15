// FILE: src/notifications/orc_stacking.rs - Orchestrates notification stacking logic
// VERSION: 1.0.1
// WCTX: TDD implementation of OFPF render orchestrators
// CLOG: Fixed result ordering to maintain chronological order

use crate::notifications::functions::fnc_calculate_anchor_position::calculate_anchor_position;
use crate::notifications::functions::fnc_calculate_rect::calculate_rect;
use crate::notifications::types::{Anchor, AnimationPhase};
use ratatui::prelude::*;
use std::collections::HashMap;
use std::time::Instant;

/// Vertical spacing between stacked notifications
const STACKING_VERTICAL_SPACING: u16 = 1;

/// Represents a notification with its calculated stacked position
#[derive(Debug, Clone)]
pub struct StackedNotification {
    pub id: u64,
    pub rect: Rect,
}

/// Trait for notification state that can be stacked.
///
/// This trait allows the stacking orchestrator to work with any notification state
/// implementation that provides the necessary information.
pub trait StackableNotification {
    fn id(&self) -> u64;
    fn current_phase(&self) -> AnimationPhase;
    fn created_at(&self) -> Instant;
    fn full_rect(&self) -> Rect;
    fn exterior_padding(&self) -> u16;
    /// Calculate the notification's content size based on frame area.
    /// Returns (width, height) tuple.
    fn calculate_content_size(&self, frame_area: Rect) -> (u16, u16);
}

/// Calculate stacking positions for notifications at a given anchor.
///
/// This function implements the core stacking algorithm:
/// 1. Filters to visible notifications (excludes Pending and Finished)
/// 2. Sorts by creation time (oldest first)
/// 3. Applies max_concurrent limit (keeps newest N)
/// 4. Determines stacking direction based on anchor
/// 5. Calculates accumulated heights and positions
/// 6. Returns list of (id, final_stacked_rect) pairs
///
/// # Arguments
///
/// * `notifications` - HashMap of all notification states
/// * `anchor` - The anchor position for this group
/// * `ids_at_anchor` - List of notification IDs at this anchor
/// * `frame_area` - The available frame area
/// * `max_concurrent` - Optional limit on concurrent visible notifications
///
/// # Returns
///
/// Vec of StackedNotification with calculated positions
///
/// # Type Parameters
///
/// * `T` - Any type implementing StackableNotification trait
pub fn calculate_stacking_positions<T: StackableNotification>(
    notifications: &HashMap<u64, T>,
    anchor: Anchor,
    ids_at_anchor: &[u64],
    frame_area: Rect,
    max_concurrent: Option<usize>,
) -> Vec<StackedNotification> {
    // 1. Filter to visible states and collect data (ID, Creation Time, Calculated Height, Width)
    let mut visible_states_data: Vec<(u64, Instant, u16, u16)> = ids_at_anchor
        .iter()
        .filter_map(|id| {
            notifications.get(id).and_then(|state| {
                let phase = state.current_phase();
                if phase != AnimationPhase::Finished && phase != AnimationPhase::Pending {
                    let rect = state.full_rect();
                    let (width, height) = if rect.height > 0 && rect.width > 0 {
                        (rect.width, rect.height)
                    } else {
                        // Calculate size from content if not yet set
                        state.calculate_content_size(frame_area)
                    };
                    if height > 0 {
                        Some((*id, state.created_at(), height, width))
                    } else {
                        None
                    }
                } else {
                    None
                }
            })
        })
        .collect();

    // 2. Sort by creation time (oldest first)
    visible_states_data.sort_unstable_by_key(|&(_, created_at, _, _)| created_at);

    // 3. Apply max_concurrent limit (take the newest N items)
    let max_concurrent = max_concurrent.unwrap_or(usize::MAX);
    let num_to_render = visible_states_data.len().min(max_concurrent);
    let candidate_data = &visible_states_data[visible_states_data.len() - num_to_render..];

    // 4. Determine stacking direction & available height
    let is_stacking_up = matches!(
        anchor,
        Anchor::BottomLeft | Anchor::BottomCenter | Anchor::BottomRight
    );
    let anchor_pos = calculate_anchor_position(anchor, frame_area);
    let available_height = if is_stacking_up {
        anchor_pos.y.saturating_sub(frame_area.y)
    } else {
        frame_area.bottom().saturating_sub(anchor_pos.y)
    };

    // 5. Calculate stack positions and filter by fit
    //
    // Strategy: Iterate in visual order based on anchor position.
    // For bottom anchors (stacking up): iterate newest-to-oldest so newest appears at anchor
    // For top anchors (stacking down): iterate oldest-to-newest so oldest appears at anchor
    let mut accumulated_height: u16 = 0;
    let mut result_list: Vec<StackedNotification> = Vec::with_capacity(num_to_render);

    // Create iterator in correct order for visual stacking
    let iter_order: Box<dyn Iterator<Item = &(u64, Instant, u16, u16)>> = if is_stacking_up {
        Box::new(candidate_data.iter().rev()) // Newest first visually appears at bottom
    } else {
        Box::new(candidate_data.iter()) // Oldest first visually appears at top
    };

    for &(id, _, height, width) in iter_order {
        let spacing = if accumulated_height > 0 {
            STACKING_VERTICAL_SPACING
        } else {
            0
        };
        let needed_height = height.saturating_add(spacing);

        if accumulated_height.saturating_add(needed_height) <= available_height {
            // Get the notification state to calculate base rect
            if let Some(state) = notifications.get(&id) {
                // Calculate base rect (X position and unstacked Y)
                let base_full_rect = calculate_rect(
                    anchor,
                    anchor_pos,
                    width,
                    height,
                    frame_area,
                    state.exterior_padding(),
                );

                // Calculate stacked Y based on accumulated height of items already placed
                // For bottom anchors: newer (later) items stack upward (subtract from base Y)
                // For top anchors: newer (later) items stack downward (add to base Y)
                let stacked_y = if is_stacking_up {
                    base_full_rect.y.saturating_sub(accumulated_height)
                } else {
                    base_full_rect.y.saturating_add(accumulated_height)
                };

                // Create the final Rect for this notification
                let final_stacked_rect = Rect {
                    x: base_full_rect.x,
                    y: stacked_y
                        .max(frame_area.y)
                        .min(frame_area.bottom().saturating_sub(height)),
                    width: base_full_rect.width,
                    height,
                }
                .intersection(frame_area);

                if final_stacked_rect.width > 0 && final_stacked_rect.height > 0 {
                    result_list.push(StackedNotification {
                        id,
                        rect: final_stacked_rect,
                    });
                    accumulated_height = accumulated_height.saturating_add(needed_height);
                } else {
                    break; // Break if clamping resulted in zero size
                }
            }
        } else {
            // Doesn't fit, stop adding notifications for this anchor
            break;
        }
    }

    result_list
}

// FILE: src/notifications/orc_stacking.rs - Orchestrates notification stacking logic
// END OF VERSION: 1.0.1
