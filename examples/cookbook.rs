// FILE: examples/cookbook.rs - Curated notification recipes with code snippets
// VERSION: 1.1.0
// WCTX: Adding cookbook example for common configurations
// CLOG: Added notification feedback on file write, auto-incrementing filename
//
// Cookbook of common notification configurations.
// Run with: cargo run --example cookbook
//
// Each recipe shows the exact code needed - copy what you need!
// Press a number key to trigger a recipe and see the code.

use ratatui_notifications::{
    generate_code, Anchor, Animation, AutoDismiss, Level, Notification, NotificationBuilder,
    Notifications, Overflow, SizeConstraint, SlideDirection, Timing,
};

use color_eyre::Result;
use crossterm::{
    cursor,
    event::{self, DisableMouseCapture, EnableMouseCapture, Event, KeyCode, KeyEventKind},
    execute,
    terminal::{disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen},
};
use ratatui::{
    layout::{Alignment, Constraint, Direction, Layout, Rect},
    prelude::*,
    style::{Color, Modifier, Style},
    text::{Line, Span},
    widgets::{Block, BorderType, Borders, Clear, Paragraph, Wrap},
};
use std::{io, path::Path, time::Duration};

/// Find an available filename by incrementing a counter if the file exists.
/// Returns "notification_example.rs", "notification_example_1.rs", etc.
fn find_available_filename(base: &str, ext: &str) -> String {
    let first = format!("{}.{}", base, ext);
    if !Path::new(&first).exists() {
        return first;
    }
    for i in 1..100 {
        let name = format!("{}_{}.{}", base, i, ext);
        if !Path::new(&name).exists() {
            return name;
        }
    }
    // Fallback - just overwrite the base
    first
}

// ═══════════════════════════════════════════════════════════════════════════════
// RECIPES - Each returns (Notification, &'static str description)
// ═══════════════════════════════════════════════════════════════════════════════

/// Recipe 1: Simple Toast
/// Use case: Quick, non-intrusive feedback using all defaults
fn recipe_simple_toast() -> Notification {
    NotificationBuilder::new("Operation completed successfully")
        .build()
        .unwrap()
}

/// Recipe 2: Error Alert
/// Use case: Prominent error message at top-center
fn recipe_error_alert() -> Notification {
    NotificationBuilder::new("Failed to connect to database\nCheck your credentials")
        .title(" Error ")
        .level(Level::Error)
        .anchor(Anchor::TopCenter)
        .auto_dismiss(AutoDismiss::After(Duration::from_secs(8)))
        .build()
        .unwrap()
}

/// Recipe 3: Warning Banner
/// Use case: Warning that needs attention but isn't critical
fn recipe_warning_banner() -> Notification {
    NotificationBuilder::new("Your session will expire in 5 minutes")
        .title(" Warning ")
        .level(Level::Warn)
        .anchor(Anchor::TopRight)
        .build()
        .unwrap()
}

/// Recipe 4: Success Confirmation
/// Use case: Positive feedback after completing an action
fn recipe_success_confirmation() -> Notification {
    NotificationBuilder::new("Changes saved to disk")
        .title(" Success ")
        .level(Level::Info)
        .anchor(Anchor::BottomRight)
        .timing(
            Timing::Fixed(Duration::from_millis(300)),
            Timing::Fixed(Duration::from_secs(2)),
            Timing::Fixed(Duration::from_millis(400)),
        )
        .build()
        .unwrap()
}

/// Recipe 5: Persistent Notification
/// Use case: Important info that requires manual dismissal
fn recipe_persistent() -> Notification {
    NotificationBuilder::new("Press 'Enter' to continue\nThis won't auto-dismiss")
        .title(" Action Required ")
        .level(Level::Warn)
        .anchor(Anchor::MiddleCenter)
        .auto_dismiss(AutoDismiss::Never)
        .build()
        .unwrap()
}

/// Recipe 6: Quick Flash
/// Use case: Very brief confirmation (1 second)
fn recipe_quick_flash() -> Notification {
    NotificationBuilder::new("Copied!")
        .anchor(Anchor::BottomCenter)
        .timing(
            Timing::Fixed(Duration::from_millis(150)),
            Timing::Fixed(Duration::from_millis(800)),
            Timing::Fixed(Duration::from_millis(150)),
        )
        .auto_dismiss(AutoDismiss::After(Duration::from_secs(1)))
        .build()
        .unwrap()
}

/// Recipe 7: Slide from Left
/// Use case: Side-panel style notification
fn recipe_slide_from_left() -> Notification {
    NotificationBuilder::new("New message received")
        .title(" Inbox ")
        .anchor(Anchor::MiddleLeft)
        .slide_direction(SlideDirection::FromLeft)
        .build()
        .unwrap()
}

/// Recipe 8: Fade Animation
/// Use case: Subtle, non-distracting appearance
fn recipe_fade() -> Notification {
    NotificationBuilder::new("Background sync complete")
        .animation(Animation::Fade)
        .anchor(Anchor::BottomRight)
        .build()
        .unwrap()
}

/// Recipe 9: Expand from Center
/// Use case: Dramatic center notification
fn recipe_expand_center() -> Notification {
    NotificationBuilder::new("Achievement Unlocked!\nFirst notification created")
        .title(" Congratulations ")
        .animation(Animation::ExpandCollapse)
        .anchor(Anchor::MiddleCenter)
        .level(Level::Info)
        .build()
        .unwrap()
}

/// Recipe 10: Slide + Fade Combined
/// Use case: Polished animation with smooth entrance
fn recipe_combined_animation() -> Notification {
    NotificationBuilder::new("Loading complete")
        .animation(Animation::Slide)
        .fade(true)
        .anchor(Anchor::TopRight)
        .build()
        .unwrap()
}

/// Recipe 11: Custom Border Style
/// Use case: Distinctive visual appearance
fn recipe_custom_border() -> Notification {
    NotificationBuilder::new("System status: All services operational")
        .title(" Status ")
        .border_type(BorderType::Double)
        .level(Level::Debug)
        .build()
        .unwrap()
}

/// Recipe 12: Compact Notification
/// Use case: Minimal space usage
fn recipe_compact() -> Notification {
    NotificationBuilder::new("OK")
        .max_size(SizeConstraint::Absolute(20), SizeConstraint::Absolute(3))
        .anchor(Anchor::BottomRight)
        .timing(
            Timing::Fixed(Duration::from_millis(200)),
            Timing::Fixed(Duration::from_secs(1)),
            Timing::Fixed(Duration::from_millis(200)),
        )
        .build()
        .unwrap()
}

/// Recipe 13: Multi-line with Margin
/// Use case: Longer messages with breathing room
fn recipe_multiline() -> Notification {
    NotificationBuilder::new("Build completed successfully\n\n  - 42 tests passed\n  - 0 warnings\n  - Time: 2.4s")
        .title(" Build Report ")
        .level(Level::Info)
        .margin(2)
        .build()
        .unwrap()
}

/// Recipe 14: Debug/Trace Style
/// Use case: Developer-time notifications
fn recipe_debug() -> Notification {
    NotificationBuilder::new("Query: SELECT * FROM users\nRows: 1,234 | Time: 45ms")
        .title(" SQL Debug ")
        .level(Level::Trace)
        .anchor(Anchor::BottomLeft)
        .build()
        .unwrap()
}

/// Recipe 15: Top-Left Corner
/// Use case: Unobtrusive corner placement
fn recipe_corner() -> Notification {
    NotificationBuilder::new("Auto-save enabled")
        .anchor(Anchor::TopLeft)
        .level(Level::Debug)
        .timing(
            Timing::Fixed(Duration::from_millis(300)),
            Timing::Fixed(Duration::from_secs(2)),
            Timing::Fixed(Duration::from_millis(300)),
        )
        .build()
        .unwrap()
}

// ═══════════════════════════════════════════════════════════════════════════════
// RECIPE METADATA
// ═══════════════════════════════════════════════════════════════════════════════

struct Recipe {
    key: char,
    name: &'static str,
    description: &'static str,
    create: fn() -> Notification,
}

const RECIPES: &[Recipe] = &[
    Recipe { key: '1', name: "Simple Toast", description: "Quick feedback, all defaults", create: recipe_simple_toast },
    Recipe { key: '2', name: "Error Alert", description: "Prominent error at top-center", create: recipe_error_alert },
    Recipe { key: '3', name: "Warning Banner", description: "Warning at top-right", create: recipe_warning_banner },
    Recipe { key: '4', name: "Success", description: "Positive feedback, fast timing", create: recipe_success_confirmation },
    Recipe { key: '5', name: "Persistent", description: "Manual dismiss only", create: recipe_persistent },
    Recipe { key: '6', name: "Quick Flash", description: "1-second confirmation", create: recipe_quick_flash },
    Recipe { key: '7', name: "Slide Left", description: "Side-panel style entry", create: recipe_slide_from_left },
    Recipe { key: '8', name: "Fade", description: "Subtle fade animation", create: recipe_fade },
    Recipe { key: '9', name: "Expand", description: "Dramatic center expand", create: recipe_expand_center },
    Recipe { key: '0', name: "Slide+Fade", description: "Combined animation", create: recipe_combined_animation },
    Recipe { key: 'a', name: "Custom Border", description: "Double border style", create: recipe_custom_border },
    Recipe { key: 'b', name: "Compact", description: "Minimal size", create: recipe_compact },
    Recipe { key: 'c', name: "Multi-line", description: "Longer content with margin", create: recipe_multiline },
    Recipe { key: 'd', name: "Debug", description: "Developer trace style", create: recipe_debug },
    Recipe { key: 'e', name: "Corner", description: "Top-left unobtrusive", create: recipe_corner },
];

// ═══════════════════════════════════════════════════════════════════════════════
// APPLICATION
// ═══════════════════════════════════════════════════════════════════════════════

struct App {
    notifications: Notifications,
    should_quit: bool,
    current_code: String,
    current_recipe_name: String,
    show_code_modal: bool,
}

impl App {
    fn new() -> Self {
        App {
            notifications: Notifications::new()
                .max_concurrent(Some(3))
                .overflow(Overflow::DiscardOldest),
            should_quit: false,
            current_code: String::new(),
            current_recipe_name: String::new(),
            show_code_modal: false,
        }
    }

    fn trigger_recipe(&mut self, recipe: &Recipe) {
        let notification = (recipe.create)();
        self.current_code = generate_code(&notification);
        self.current_recipe_name = recipe.name.to_string();
        let _ = self.notifications.add(notification);
    }

    fn on_tick(&mut self) {
        self.notifications.tick(Duration::from_millis(16));
    }

    fn write_code_to_file(&mut self) -> Option<String> {
        if self.current_code.is_empty() {
            return None;
        }
        let filename = find_available_filename("notification_example", "rs");
        match std::fs::write(&filename, &self.current_code) {
            Ok(_) => {
                // Show success notification
                let notif = NotificationBuilder::new(format!("Saved to {}", filename))
                    .title(" File Written ")
                    .level(Level::Info)
                    .anchor(Anchor::BottomCenter)
                    .timing(
                        Timing::Fixed(Duration::from_millis(200)),
                        Timing::Fixed(Duration::from_secs(2)),
                        Timing::Fixed(Duration::from_millis(300)),
                    )
                    .build();
                if let Ok(n) = notif {
                    let _ = self.notifications.add(n);
                }
                Some(filename)
            }
            Err(_) => None,
        }
    }
}

fn main() -> Result<()> {
    color_eyre::install()?;

    enable_raw_mode()?;
    let mut stdout = io::stdout();
    execute!(stdout, EnterAlternateScreen, EnableMouseCapture, cursor::Hide)?;
    let backend = CrosstermBackend::new(stdout);
    let mut terminal = Terminal::new(backend)?;
    let mut app = App::new();

    let res = run_app(&mut terminal, &mut app);

    disable_raw_mode()?;
    execute!(
        terminal.backend_mut(),
        LeaveAlternateScreen,
        DisableMouseCapture,
        cursor::Show
    )?;

    if let Err(err) = res {
        println!("Error: {:?}", err);
        return Err(color_eyre::Report::new(err));
    }
    Ok(())
}

fn run_app(terminal: &mut Terminal<CrosstermBackend<io::Stdout>>, app: &mut App) -> io::Result<()> {
    let tick_rate = Duration::from_millis(16);

    loop {
        terminal.draw(|f| ui(f, app))?;

        if event::poll(tick_rate)? {
            if let Event::Key(key) = event::read()? {
                if key.kind == KeyEventKind::Press {
                    // Handle modal keys first
                    if app.show_code_modal {
                        match key.code {
                            KeyCode::Char('w') => {
                                app.write_code_to_file();
                                app.show_code_modal = false;
                            }
                            KeyCode::Char('i') | KeyCode::Esc => {
                                app.show_code_modal = false;
                            }
                            _ => {}
                        }
                    } else {
                        match key.code {
                            KeyCode::Char('q') | KeyCode::Esc => {
                                app.should_quit = true;
                            }
                            KeyCode::Char('i') => {
                                if !app.current_code.is_empty() {
                                    app.show_code_modal = true;
                                }
                            }
                            KeyCode::Char(c) => {
                                // Find matching recipe
                                if let Some(recipe) = RECIPES.iter().find(|r| r.key == c) {
                                    app.trigger_recipe(recipe);
                                }
                            }
                            _ => {}
                        }
                    }
                }
            }
        }

        app.on_tick();

        if app.should_quit {
            return Ok(());
        }
    }
}

fn ui(f: &mut Frame<'_>, app: &mut App) {
    let frame_area = f.area();

    // Main layout
    let main_layout = Layout::default()
        .direction(Direction::Vertical)
        .constraints([
            Constraint::Length(3),  // Title
            Constraint::Min(10),    // Recipes
            Constraint::Length(5),  // Current code preview
        ])
        .margin(1)
        .split(frame_area);

    render_title(f, main_layout[0]);
    render_recipes(f, main_layout[1]);
    render_code_preview(f, main_layout[2], app);

    // Render notifications
    app.notifications.render(f, frame_area);

    // Render modal on top if visible
    if app.show_code_modal {
        render_code_modal(f, frame_area, app);
    }
}

fn render_title(f: &mut Frame<'_>, area: Rect) {
    let title = Paragraph::new(Line::from(vec![
        Span::styled("Notification Cookbook", Style::new().fg(Color::White).add_modifier(Modifier::BOLD)),
        Span::styled(" - Press a key to see recipe, ", Style::new().fg(Color::DarkGray)),
        Span::styled("[i]", Style::new().fg(Color::Cyan).add_modifier(Modifier::BOLD)),
        Span::styled(" for code, ", Style::new().fg(Color::DarkGray)),
        Span::styled("[q]", Style::new().fg(Color::Cyan).add_modifier(Modifier::BOLD)),
        Span::styled(" to quit", Style::new().fg(Color::DarkGray)),
    ]))
    .alignment(Alignment::Center);
    f.render_widget(title, area);
}

fn render_recipes(f: &mut Frame<'_>, area: Rect) {
    let block = Block::default()
        .borders(Borders::ALL)
        .border_type(BorderType::Rounded)
        .border_style(Style::new().fg(Color::DarkGray))
        .title(" Recipes ");

    let inner = block.inner(area);
    f.render_widget(block, area);

    // Split into 3 columns
    let columns = Layout::default()
        .direction(Direction::Horizontal)
        .constraints([Constraint::Ratio(1, 3), Constraint::Ratio(1, 3), Constraint::Ratio(1, 3)])
        .split(inner);

    let key_style = Style::new().fg(Color::Cyan).add_modifier(Modifier::BOLD);
    let name_style = Style::new().fg(Color::White);
    let desc_style = Style::new().fg(Color::DarkGray);

    // Render recipes in columns (5 per column)
    for (col_idx, col_area) in columns.iter().enumerate() {
        let start = col_idx * 5;
        let end = (start + 5).min(RECIPES.len());

        let lines: Vec<Line> = RECIPES[start..end]
            .iter()
            .map(|r| {
                Line::from(vec![
                    Span::styled(format!("[{}] ", r.key), key_style),
                    Span::styled(r.name, name_style),
                    Span::styled(format!(" - {}", r.description), desc_style),
                ])
            })
            .collect();

        let paragraph = Paragraph::new(lines).wrap(Wrap { trim: true });
        f.render_widget(paragraph, *col_area);
    }
}

fn render_code_preview(f: &mut Frame<'_>, area: Rect, app: &App) {
    let title = if app.current_recipe_name.is_empty() {
        " Code Preview ".to_string()
    } else {
        format!(" {} - Press [i] to expand ", app.current_recipe_name)
    };

    let block = Block::default()
        .borders(Borders::ALL)
        .border_type(BorderType::Rounded)
        .border_style(Style::new().fg(Color::DarkGray))
        .title(title);

    let inner = block.inner(area);
    f.render_widget(block, area);

    let preview = if app.current_code.is_empty() {
        "Press a recipe key to see the code...".to_string()
    } else {
        // Show first few lines
        app.current_code.lines().take(3).collect::<Vec<_>>().join("\n") + "\n    ..."
    };

    let paragraph = Paragraph::new(preview)
        .style(Style::new().fg(Color::Green))
        .wrap(Wrap { trim: false });
    f.render_widget(paragraph, inner);
}

fn render_code_modal(f: &mut Frame<'_>, frame_area: Rect, app: &App) {
    // Center the modal
    let modal_width = 70.min(frame_area.width.saturating_sub(4));
    let modal_height = 20.min(frame_area.height.saturating_sub(4));
    let modal_x = (frame_area.width.saturating_sub(modal_width)) / 2;
    let modal_y = (frame_area.height.saturating_sub(modal_height)) / 2;
    let modal_area = Rect::new(modal_x, modal_y, modal_width, modal_height);

    // Clear the area behind the modal
    f.render_widget(Clear, modal_area);

    let block = Block::default()
        .borders(Borders::ALL)
        .border_type(BorderType::Double)
        .border_style(Style::new().fg(Color::Cyan))
        .title(format!(" {} - Generated Code ", app.current_recipe_name))
        .title_bottom(Line::from(" [w] Write to notification_example.rs | [i]/[Esc] Close ").alignment(Alignment::Center));

    let inner = block.inner(modal_area);
    f.render_widget(block, modal_area);

    let code_paragraph = Paragraph::new(app.current_code.clone())
        .style(Style::new().fg(Color::Green))
        .wrap(Wrap { trim: false });
    f.render_widget(code_paragraph, inner);
}

// FILE: examples/cookbook.rs - Curated notification recipes with code snippets
// END OF VERSION: 1.1.0
