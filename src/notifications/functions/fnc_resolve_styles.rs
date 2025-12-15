// FILE: src/notifications/functions/fnc_resolve_styles.rs - Resolves notification styles based on level and custom overrides
// VERSION: 1.0.0
// WCTX: TDD implementation of OFPF notification functions
// CLOG: Initial creation

use crate::notifications::types::Level;
use ratatui::style::{Color, Style};

// Default styles
const DEFAULT_BLOCK_STYLE: Style = Style::new();
const DEFAULT_TITLE_STYLE: Style = Style::new();
const DEFAULT_BORDER_STYLE: Style = Style::new().fg(Color::DarkGray);

// Level-based border styles
const INFO_BORDER_STYLE: Style = Style::new().fg(Color::Green);
const WARN_BORDER_STYLE: Style = Style::new().fg(Color::Yellow);
const ERROR_BORDER_STYLE: Style = Style::new().fg(Color::Red);
const DEBUG_BORDER_STYLE: Style = Style::new().fg(Color::Blue);
const TRACE_BORDER_STYLE: Style = Style::new().fg(Color::Magenta);

/// Resolves the final styles for block, border, and title based on level and custom overrides.
///
/// # Arguments
///
/// * `level` - Optional notification level that determines default border/title colors
/// * `block_style` - Optional custom block style (overrides default)
/// * `border_style` - Optional custom border style (overrides level-based default)
/// * `title_style` - Optional custom title style (overrides all defaults)
///
/// # Returns
///
/// A tuple of (block_style, border_style, title_style) with all defaults and overrides applied.
///
/// # Style Resolution Order
///
/// 1. Start with default styles
/// 2. If level is provided, apply level-based border color and patch title
/// 3. If custom block_style is provided, use it
/// 4. If custom border_style is provided, use it and patch title
/// 5. If custom title_style is provided, use it (final override)
pub fn resolve_styles(
    level: Option<Level>,
    block_style: Option<Style>,
    border_style: Option<Style>,
    title_style: Option<Style>,
) -> (Style, Style, Style) {
    let mut final_block_style = DEFAULT_BLOCK_STYLE;
    let mut final_border_style = DEFAULT_BORDER_STYLE;
    let mut final_title_style = DEFAULT_TITLE_STYLE;

    // Apply level-based styling
    if let Some(lvl) = level {
        let level_border_style = match lvl {
            Level::Info => INFO_BORDER_STYLE,
            Level::Warn => WARN_BORDER_STYLE,
            Level::Error => ERROR_BORDER_STYLE,
            Level::Debug => DEBUG_BORDER_STYLE,
            Level::Trace => TRACE_BORDER_STYLE,
        };
        final_border_style = level_border_style;
        final_title_style = final_title_style.patch(level_border_style);
    }

    // Apply custom block style
    if let Some(bs) = block_style {
        final_block_style = bs;
    }

    // Apply custom border style (and patch title)
    if let Some(bs) = border_style {
        final_border_style = bs;
        final_title_style = final_title_style.patch(bs);
    }

    // Apply custom title style (final override)
    if let Some(ts) = title_style {
        final_title_style = ts;
    }

    (final_block_style, final_border_style, final_title_style)
}

// FILE: src/notifications/functions/fnc_resolve_styles.rs - Resolves notification styles based on level and custom overrides
// END OF VERSION: 1.0.0
