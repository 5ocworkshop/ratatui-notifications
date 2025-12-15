// FILE: tests/test_fnc_get_level_icon_integration.rs - Integration tests for level icon lookup function
// VERSION: 1.0.0
// WCTX: TDD implementation of OFPF notification functions
// CLOG: Initial creation

use ratatui_notifications::notifications::functions::fnc_get_level_icon::get_level_icon;
use ratatui_notifications::notifications::types::Level;

#[test]
fn test_level_info_returns_info_icon() {
    let icon = get_level_icon(Some(Level::Info));
    assert_eq!(icon, Some(" ℹ"));
}

#[test]
fn test_level_warn_returns_warning_icon() {
    let icon = get_level_icon(Some(Level::Warn));
    assert_eq!(icon, Some(" ⚠"));
}

#[test]
fn test_level_error_returns_error_icon() {
    let icon = get_level_icon(Some(Level::Error));
    assert_eq!(icon, Some(" ✖"));
}

#[test]
fn test_level_debug_returns_debug_icon() {
    let icon = get_level_icon(Some(Level::Debug));
    assert_eq!(icon, Some(" 🐞"));
}

#[test]
fn test_level_trace_returns_trace_icon() {
    let icon = get_level_icon(Some(Level::Trace));
    assert_eq!(icon, Some(" ⊙"));
}

#[test]
fn test_none_returns_none() {
    let icon = get_level_icon(None);
    assert_eq!(icon, None);
}

// FILE: tests/test_fnc_get_level_icon_integration.rs - Integration tests for level icon lookup function
// END OF VERSION: 1.0.0
