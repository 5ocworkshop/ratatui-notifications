// FILE: tests/test_fnc_resolve_styles_integration.rs - Integration tests for style resolution function
// VERSION: 1.0.0
// WCTX: TDD implementation of OFPF notification functions
// CLOG: Initial creation

use ratatui::style::{Color, Style};
use ratatui_notifications::notifications::functions::fnc_resolve_styles::resolve_styles;
use ratatui_notifications::notifications::types::Level;

#[test]
fn test_no_level_returns_default_styles() {
    let (block_style, border_style, title_style) =
        resolve_styles(None, None, None, None);

    // Default block style should be empty/default
    assert_eq!(block_style, Style::new());

    // Default border style should be dark gray
    assert_eq!(border_style, Style::new().fg(Color::DarkGray));

    // Default title style should be empty/default
    assert_eq!(title_style, Style::new());
}

#[test]
fn test_level_info_returns_green_border() {
    let (block_style, border_style, title_style) =
        resolve_styles(Some(Level::Info), None, None, None);

    // Block style should still be default
    assert_eq!(block_style, Style::new());

    // Border style should be green for Info
    assert_eq!(border_style, Style::new().fg(Color::Green));

    // Title style should also have green (patched from border)
    assert_eq!(title_style, Style::new().fg(Color::Green));
}

#[test]
fn test_level_warn_returns_yellow_border() {
    let (block_style, border_style, title_style) =
        resolve_styles(Some(Level::Warn), None, None, None);

    assert_eq!(block_style, Style::new());
    assert_eq!(border_style, Style::new().fg(Color::Yellow));
    assert_eq!(title_style, Style::new().fg(Color::Yellow));
}

#[test]
fn test_level_error_returns_red_border() {
    let (block_style, border_style, title_style) =
        resolve_styles(Some(Level::Error), None, None, None);

    assert_eq!(block_style, Style::new());
    assert_eq!(border_style, Style::new().fg(Color::Red));
    assert_eq!(title_style, Style::new().fg(Color::Red));
}

#[test]
fn test_level_debug_returns_blue_border() {
    let (block_style, border_style, title_style) =
        resolve_styles(Some(Level::Debug), None, None, None);

    assert_eq!(block_style, Style::new());
    assert_eq!(border_style, Style::new().fg(Color::Blue));
    assert_eq!(title_style, Style::new().fg(Color::Blue));
}

#[test]
fn test_level_trace_returns_magenta_border() {
    let (block_style, border_style, title_style) =
        resolve_styles(Some(Level::Trace), None, None, None);

    assert_eq!(block_style, Style::new());
    assert_eq!(border_style, Style::new().fg(Color::Magenta));
    assert_eq!(title_style, Style::new().fg(Color::Magenta));
}

#[test]
fn test_custom_block_style_overrides_default() {
    let custom_block = Style::new().bg(Color::Cyan);
    let (block_style, border_style, title_style) =
        resolve_styles(Some(Level::Info), Some(custom_block), None, None);

    // Custom block style should be used
    assert_eq!(block_style, custom_block);

    // Border and title should still use level color
    assert_eq!(border_style, Style::new().fg(Color::Green));
    assert_eq!(title_style, Style::new().fg(Color::Green));
}

#[test]
fn test_custom_border_style_overrides_level() {
    let custom_border = Style::new().fg(Color::Cyan);
    let (block_style, border_style, title_style) =
        resolve_styles(Some(Level::Error), None, Some(custom_border), None);

    assert_eq!(block_style, Style::new());

    // Custom border style should override level-based color
    assert_eq!(border_style, custom_border);

    // Title should patch from custom border, not level
    assert_eq!(title_style, Style::new().fg(Color::Cyan));
}

#[test]
fn test_custom_title_style_overrides_all() {
    let custom_title = Style::new().fg(Color::Magenta).bg(Color::Black);
    let (block_style, border_style, title_style) =
        resolve_styles(
            Some(Level::Info),
            None,
            Some(Style::new().fg(Color::Yellow)),
            Some(custom_title)
        );

    assert_eq!(block_style, Style::new());
    assert_eq!(border_style, Style::new().fg(Color::Yellow));

    // Custom title style should completely override
    assert_eq!(title_style, custom_title);
}

#[test]
fn test_all_custom_styles_provided() {
    let custom_block = Style::new().bg(Color::Blue);
    let custom_border = Style::new().fg(Color::White);
    let custom_title = Style::new().fg(Color::Red).bg(Color::Yellow);

    let (block_style, border_style, title_style) =
        resolve_styles(
            Some(Level::Debug), // Should be ignored
            Some(custom_block),
            Some(custom_border),
            Some(custom_title)
        );

    // All custom styles should be used
    assert_eq!(block_style, custom_block);
    assert_eq!(border_style, custom_border);
    assert_eq!(title_style, custom_title);
}

// FILE: tests/test_fnc_resolve_styles_integration.rs - Integration tests for style resolution function
// END OF VERSION: 1.0.0
