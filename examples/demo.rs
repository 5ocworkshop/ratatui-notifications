// FILE: examples/demo.rs - Interactive demonstration of ratatui-notifications crate features
// VERSION: 2.3.0
// WCTX: Adding code generation feature
// CLOG: Fixed generate_code for all demos, added success notification on file write

use ratatui_notifications::{
    generate_code, NotificationBuilder, Notifications,
    Anchor, Animation, Level, Overflow,
    SlideDirection, Timing, SizeConstraint,
};
use color_eyre::Result;
use crossterm::{
    cursor,
    event::{self, DisableMouseCapture, EnableMouseCapture, Event, KeyCode, KeyEventKind},
    execute,
    terminal::{disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen},
};
use ratatui::{
    layout::{Alignment, Constraint, Direction, Layout, Position as RatatuiPosition, Rect},
    prelude::*,
    style::{Color, Modifier, Style},
    text::{Line, Span},
    widgets::{Block, BorderType, Borders, Clear, Paragraph, Wrap},
};
use std::{collections::VecDeque, io, path::Path, time::Duration};

const MAX_LOG_MESSAGES: usize = 8;

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

// Realistic demo content
const PATH_EXAMPLES: &[&str] = &[
    "/home/user/projects/rust-app/src/main.rs",
    "~/.config/myapp/settings.toml",
    "/var/log/application.log",
    "C:\\Users\\Dev\\Documents\\project\\build.rs",
];

const SUCCESS_MESSAGES: &[&str] = &[
    "Changes saved successfully",
    "Build completed in 2.4s",
    "All tests passed (42/42)",
    "Connection established",
    "File uploaded: report.pdf",
];

const WARNING_MESSAGES: &[(&str, &str)] = &[
    ("Low disk space", "Only 2.1 GB remaining on /dev/sda1"),
    ("Deprecated API", "Method `old_fn()` will be removed in v3.0"),
    ("Slow query", "Query took 3.2s - consider adding an index"),
];

const ERROR_MESSAGES: &[(&str, &str)] = &[
    ("Connection failed", "Could not reach api.example.com\nRetrying in 5 seconds..."),
    ("Build error", "src/lib.rs:42:15\n  expected `String`, found `&str`"),
    ("Permission denied", "/etc/shadow: EACCES\nRun with elevated privileges"),
];

struct App {
    notifications: Notifications,
    should_quit: bool,
    log_messages: VecDeque<String>,
    last_frame_area: Rect,
    current_border_type: BorderType,
    demo_index: usize,
    overflow_count: u32,
    // Code generation modal
    show_code_modal: bool,
    last_notification_code: String,
    // Help modal
    show_help_modal: bool,
}

impl App {
    fn new() -> Self {
        let notifications = Notifications::new()
            .max_concurrent(Some(5))
            .overflow(Overflow::DiscardOldest);

        App {
            notifications,
            should_quit: false,
            log_messages: VecDeque::with_capacity(MAX_LOG_MESSAGES),
            last_frame_area: Rect::default(),
            current_border_type: BorderType::Rounded,
            demo_index: 0,
            overflow_count: 0,
            show_code_modal: false,
            last_notification_code: String::new(),
            show_help_modal: false,
        }
    }

    fn add_log(&mut self, message: impl Into<String>) {
        let msg = message.into();
        if self.log_messages.len() >= MAX_LOG_MESSAGES {
            self.log_messages.pop_front();
        }
        self.log_messages.push_back(msg);
    }

    fn on_tick(&mut self) {
        self.notifications.tick(Duration::from_millis(16));
    }

    fn next_demo_content<T: Copy>(&mut self, items: &[T]) -> T {
        let item = items[self.demo_index % items.len()];
        self.demo_index = self.demo_index.wrapping_add(1);
        item
    }

    // ═══════════════════════════════════════════════════════════════════════════
    // POSITION DEMOS - Numpad layout: 7-8-9 (top), 4-5-6 (mid), 1-2-3 (bottom)
    // ═══════════════════════════════════════════════════════════════════════════

    fn demo_position(&mut self, anchor: Anchor) {
        let anchor_name = format!("{:?}", anchor);
        let content = format!("Position: {}\nSlide animation", anchor_name);

        let notification = NotificationBuilder::new(content)
            .anchor(anchor)
            .title(format!(" {} ", anchor_name))
            .level(Level::Info)
            .border_type(self.current_border_type)
            .timing(
                Timing::Fixed(Duration::from_millis(400)),
                Timing::Fixed(Duration::from_secs(3)),
                Timing::Fixed(Duration::from_millis(500)),
            )
            .build();

        match notification {
            Ok(n) => {
                self.last_notification_code = generate_code(&n);
                if let Ok(id) = self.notifications.add(n) {
                    self.add_log(format!("{} → ID {}", anchor_name, id));
                }
            }
            Err(e) => self.add_log(format!("Error: {}", e)),
        }
    }

    // ═══════════════════════════════════════════════════════════════════════════
    // ANIMATION DEMOS
    // ═══════════════════════════════════════════════════════════════════════════

    fn demo_slide_directions(&mut self) {
        // Show slides from each direction
        let demos = [
            (Anchor::TopCenter, SlideDirection::FromTop, "Slides down"),
            (Anchor::BottomCenter, SlideDirection::FromBottom, "Slides up"),
            (Anchor::MiddleLeft, SlideDirection::FromLeft, "Slides right"),
            (Anchor::MiddleRight, SlideDirection::FromRight, "Slides left"),
        ];

        for (i, (anchor, direction, desc)) in demos.iter().enumerate() {
            let notification = NotificationBuilder::new(desc.to_string())
                .anchor(*anchor)
                .title(format!(" {:?} ", direction))
                .level(Level::Debug)
                .border_type(self.current_border_type)
                .slide_direction(*direction)
                .timing(
                    Timing::Fixed(Duration::from_millis(500)),
                    Timing::Fixed(Duration::from_secs(3 + i as u64)),
                    Timing::Fixed(Duration::from_millis(500)),
                )
                .build();

            if let Ok(n) = notification {
                self.last_notification_code = generate_code(&n);
                self.notifications.add(n).ok();
            }
        }
        self.add_log("Slide directions showcase");
    }

    fn demo_expand(&mut self) {
        let notification = NotificationBuilder::new("Expands from center point\nand collapses back")
            .anchor(Anchor::MiddleCenter)
            .title(" Expand ")
            .level(Level::Info)
            .border_type(self.current_border_type)
            .animation(Animation::ExpandCollapse)
            .timing(
                Timing::Fixed(Duration::from_millis(350)),
                Timing::Fixed(Duration::from_secs(2)),
                Timing::Fixed(Duration::from_millis(350)),
            )
            .build();

        match notification {
            Ok(n) => {
                self.last_notification_code = generate_code(&n);
                if let Ok(id) = self.notifications.add(n) {
                    self.add_log(format!("Expand → ID {}", id));
                }
            }
            Err(e) => self.add_log(format!("Error: {}", e)),
        }
    }

    fn demo_fade(&mut self) {
        // Neon magenta for that eye-catching fade effect
        let border_color = Color::Rgb(187, 0, 187);
        let title_color = Color::Rgb(255, 100, 255);

        let notification = NotificationBuilder::new("Fades in smoothly\nthen fades out")
            .anchor(Anchor::MiddleLeft)
            .title(" Fading... ")
            .border_type(self.current_border_type)
            .border_style(Style::new().fg(border_color))
            .title_style(Style::new().fg(title_color))
            .animation(Animation::Fade)
            .timing(
                Timing::Fixed(Duration::from_millis(600)),
                Timing::Fixed(Duration::from_secs(3)),
                Timing::Fixed(Duration::from_millis(600)),
            )
            .build();

        match notification {
            Ok(n) => {
                self.last_notification_code = generate_code(&n);
                if let Ok(id) = self.notifications.add(n) {
                    self.add_log(format!("Fade → ID {}", id));
                }
            }
            Err(e) => self.add_log(format!("Error: {}", e)),
        }
    }

    // ═══════════════════════════════════════════════════════════════════════════
    // REAL-WORLD CONTENT EXAMPLES
    // ═══════════════════════════════════════════════════════════════════════════

    fn demo_file_path(&mut self) {
        let path = self.next_demo_content(PATH_EXAMPLES);

        // Wide, short notification for file paths
        let notification = NotificationBuilder::new(path)
            .anchor(Anchor::BottomCenter)
            .title(" File ")
            .level(Level::Trace)
            .border_type(self.current_border_type)
            .max_size(SizeConstraint::Percentage(0.6), SizeConstraint::Absolute(4))
            .slide_direction(SlideDirection::FromBottom)
            .timing(
                Timing::Fixed(Duration::from_millis(300)),
                Timing::Fixed(Duration::from_secs(3)),
                Timing::Fixed(Duration::from_millis(400)),
            )
            .build();

        match notification {
            Ok(n) => {
                self.last_notification_code = generate_code(&n);
                if let Ok(id) = self.notifications.add(n) {
                    self.add_log(format!("Path → ID {}", id));
                }
            }
            Err(e) => self.add_log(format!("Error: {}", e)),
        }
    }

    fn demo_success(&mut self) {
        let message = self.next_demo_content(SUCCESS_MESSAGES);

        // Compact success notification
        let notification = NotificationBuilder::new(message)
            .anchor(Anchor::TopRight)
            .title(" Success ")
            .level(Level::Info)
            .border_type(self.current_border_type)
            .border_style(Style::new().fg(Color::Green))
            .max_size(SizeConstraint::Percentage(0.35), SizeConstraint::Percentage(0.2))
            .timing(
                Timing::Fixed(Duration::from_millis(250)),
                Timing::Fixed(Duration::from_secs(2)),
                Timing::Fixed(Duration::from_millis(350)),
            )
            .build();

        match notification {
            Ok(n) => {
                self.last_notification_code = generate_code(&n);
                if let Ok(id) = self.notifications.add(n) {
                    self.add_log(format!("Success → ID {}", id));
                }
            }
            Err(e) => self.add_log(format!("Error: {}", e)),
        }
    }

    fn demo_warning(&mut self) {
        let (title, detail) = self.next_demo_content(WARNING_MESSAGES);

        let notification = NotificationBuilder::new(detail.to_string())
            .anchor(Anchor::TopCenter)
            .title(format!(" {} ", title))
            .level(Level::Warn)
            .border_type(self.current_border_type)
            .max_size(SizeConstraint::Percentage(0.45), SizeConstraint::Percentage(0.2))
            .timing(
                Timing::Fixed(Duration::from_millis(400)),
                Timing::Fixed(Duration::from_secs(4)),
                Timing::Fixed(Duration::from_millis(500)),
            )
            .build();

        match notification {
            Ok(n) => {
                self.last_notification_code = generate_code(&n);
                if let Ok(id) = self.notifications.add(n) {
                    self.add_log(format!("Warning → ID {}", id));
                }
            }
            Err(e) => self.add_log(format!("Error: {}", e)),
        }
    }

    fn demo_error(&mut self) {
        let (title, detail) = self.next_demo_content(ERROR_MESSAGES);

        // Taller notification for error details
        let notification = NotificationBuilder::new(detail.to_string())
            .anchor(Anchor::MiddleRight)
            .title(format!(" {} ", title))
            .level(Level::Error)
            .border_type(self.current_border_type)
            .max_size(SizeConstraint::Percentage(0.4), SizeConstraint::Percentage(0.3))
            .slide_direction(SlideDirection::FromRight)
            .timing(
                Timing::Fixed(Duration::from_millis(350)),
                Timing::Fixed(Duration::from_secs(5)),
                Timing::Fixed(Duration::from_millis(450)),
            )
            .build();

        match notification {
            Ok(n) => {
                self.last_notification_code = generate_code(&n);
                if let Ok(id) = self.notifications.add(n) {
                    self.add_log(format!("Error → ID {}", id));
                }
            }
            Err(e) => self.add_log(format!("Error: {}", e)),
        }
    }

    // ═══════════════════════════════════════════════════════════════════════════
    // SHOWCASE FEATURES
    // ═══════════════════════════════════════════════════════════════════════════

    fn demo_all_levels(&mut self) {
        let levels = [
            (Level::Trace, Anchor::BottomLeft, "Trace: verbose debugging"),
            (Level::Debug, Anchor::BottomCenter, "Debug: dev info"),
            (Level::Info, Anchor::BottomRight, "Info: general messages"),
            (Level::Warn, Anchor::TopLeft, "Warn: potential issues"),
            (Level::Error, Anchor::TopRight, "Error: failures"),
        ];

        for (i, (level, anchor, content)) in levels.iter().enumerate() {
            let notification = NotificationBuilder::new(*content)
                .anchor(*anchor)
                .title(format!(" {:?} ", level))
                .level(*level)
                .border_type(self.current_border_type)
                .timing(
                    Timing::Fixed(Duration::from_millis(300 + i as u64 * 100)),
                    Timing::Fixed(Duration::from_secs(3 + i as u64)),
                    Timing::Fixed(Duration::from_millis(400)),
                )
                .build();

            if let Ok(n) = notification {
                self.last_notification_code = generate_code(&n);
                self.notifications.add(n).ok();
            }
        }
        self.add_log("All log levels displayed");
    }

    fn demo_stacking(&mut self) {
        // Rapidly add notifications to show stacking behavior
        for i in 1..=6 {
            let notification = NotificationBuilder::new(format!("Stacked notification #{}", i))
                .anchor(Anchor::TopRight)
                .title(format!(" Stack {} ", i))
                .level(if i > 3 { Level::Warn } else { Level::Info })
                .border_type(self.current_border_type)
                .timing(
                    Timing::Fixed(Duration::from_millis(200)),
                    Timing::Fixed(Duration::from_secs(2 + i as u64)),
                    Timing::Fixed(Duration::from_millis(300)),
                )
                .build();

            if let Ok(n) = notification {
                self.last_notification_code = generate_code(&n);
                self.notifications.add(n).ok();
            }
        }
        self.add_log("Stacking demo (max 5 shown, oldest discarded)");
    }

    fn demo_combined_effects(&mut self) {
        // Slide + fade combined
        let notification = NotificationBuilder::new("Slides in while fading\nthen fades out while sliding")
            .anchor(Anchor::MiddleLeft)
            .title(" Slide + Fade ")
            .border_type(self.current_border_type)
            .border_style(Style::new().fg(Color::Rgb(255, 180, 100)))
            .slide_direction(SlideDirection::FromLeft)
            .fade(true)
            .timing(
                Timing::Fixed(Duration::from_millis(600)),
                Timing::Fixed(Duration::from_secs(3)),
                Timing::Fixed(Duration::from_millis(700)),
            )
            .build();

        match notification {
            Ok(n) => {
                self.last_notification_code = generate_code(&n);
                if let Ok(id) = self.notifications.add(n) {
                    self.add_log(format!("Slide+Fade → ID {}", id));
                }
            }
            Err(e) => self.add_log(format!("Error: {}", e)),
        }
    }

    // ═══════════════════════════════════════════════════════════════════════════
    // SPECIFIC SLIDE DIRECTIONS - Shows anchor vs direction independence
    // ═══════════════════════════════════════════════════════════════════════════

    fn demo_specific_slide(&mut self, anchor: Anchor, direction: SlideDirection) {
        // Demonstrates that anchor position and slide direction are independent
        let title = format!(" {:?} ", direction);
        let content = format!(
            "Anchor: {:?}\nSlide: {:?}\nShows direction independence!",
            anchor, direction
        );

        let notification = NotificationBuilder::new(content)
            .anchor(anchor)
            .title(title)
            .level(Level::Debug)
            .border_type(self.current_border_type)
            .slide_direction(direction)
            .timing(
                Timing::Fixed(Duration::from_millis(600)),
                Timing::Fixed(Duration::from_secs(4)),
                Timing::Fixed(Duration::from_millis(600)),
            )
            .build();

        match notification {
            Ok(n) => {
                self.last_notification_code = generate_code(&n);
                if let Ok(id) = self.notifications.add(n) {
                    self.add_log(format!("{:?} from {:?} → ID {}", anchor, direction, id));
                }
            }
            Err(e) => self.add_log(format!("Error: {}", e)),
        }
    }

    // ═══════════════════════════════════════════════════════════════════════════
    // CUSTOM PATH - Entry/exit positions with fade
    // ═══════════════════════════════════════════════════════════════════════════

    fn demo_custom_path(&mut self) {
        let frame_area = self.last_frame_area;
        if frame_area.width == 0 || frame_area.height == 0 {
            self.add_log("Frame not ready yet");
            return;
        }

        // Calculate custom positions: start left, end right, same height
        let start_x = (frame_area.width as f32 * 0.15).round() as u16;
        let start_y = frame_area.height / 2;
        let start_pos = RatatuiPosition::new(
            start_x.min(frame_area.right().saturating_sub(1)),
            start_y,
        );

        let end_x = (frame_area.width as f32 * 0.60).round() as u16;
        let end_pos = RatatuiPosition::new(end_x.max(frame_area.x), start_y);

        let notification = NotificationBuilder::new(
            "Custom entry → exit path\nwith fade effect!\nSlides across screen",
        )
        .anchor(Anchor::MiddleCenter)
        .title(" Custom Path ")
        .border_type(self.current_border_type)
        .border_style(Style::new().fg(Color::Rgb(255, 165, 0)))
        .slide_direction(SlideDirection::FromLeft)
        .entry_position(start_pos)
        .exit_position(end_pos)
        .fade(true)
        .timing(
            Timing::Fixed(Duration::from_millis(800)),
            Timing::Fixed(Duration::from_secs(3)),
            Timing::Fixed(Duration::from_millis(800)),
        )
        .build();

        match notification {
            Ok(n) => {
                self.last_notification_code = generate_code(&n);
                if let Ok(id) = self.notifications.add(n) {
                    self.add_log(format!("Custom path+fade → ID {}", id));
                }
            }
            Err(e) => self.add_log(format!("Error: {}", e)),
        }
    }

    // ═══════════════════════════════════════════════════════════════════════════
    // OVERFLOW DEMOS - Compare discard behaviors
    // ═══════════════════════════════════════════════════════════════════════════

    fn demo_overflow(&mut self) {
        self.overflow_count += 1;
        let notification = NotificationBuilder::new(format!(
            "Overflow test #{}\nMax concurrent: 5\nOldest discarded",
            self.overflow_count
        ))
        .anchor(Anchor::TopRight)
        .title(format!(" Overflow {} ", self.overflow_count))
        .level(if self.overflow_count > 3 {
            Level::Warn
        } else {
            Level::Info
        })
        .border_type(self.current_border_type)
        .timing(
            Timing::Fixed(Duration::from_millis(200)),
            Timing::Fixed(Duration::from_secs(5)),
            Timing::Fixed(Duration::from_millis(300)),
        )
        .build();

        match notification {
            Ok(n) => {
                self.last_notification_code = generate_code(&n);
                if let Ok(id) = self.notifications.add(n) {
                    self.add_log(format!("Overflow #{} → ID {}", self.overflow_count, id));
                }
            }
            Err(e) => self.add_log(format!("Error: {}", e)),
        }
    }

    fn cycle_border(&mut self) {
        self.current_border_type = match self.current_border_type {
            BorderType::Rounded => BorderType::Double,
            BorderType::Double => BorderType::Thick,
            BorderType::Thick => BorderType::Plain,
            BorderType::Plain => BorderType::Rounded,
            _ => BorderType::Rounded,
        };
        self.add_log(format!("Border: {:?}", self.current_border_type));
    }
}

fn main() -> Result<()> {
    env_logger::init();
    enable_raw_mode()?;
    let mut stdout = io::stdout();
    execute!(stdout, EnterAlternateScreen, EnableMouseCapture, cursor::Hide)?;
    let backend = CrosstermBackend::new(stdout);
    let mut terminal = Terminal::new(backend)?;
    let mut app = App::new();
    app.add_log("Press any highlighted key to trigger a demo");

    let res = run_app(&mut terminal, &mut app);

    disable_raw_mode()?;
    execute!(
        terminal.backend_mut(),
        LeaveAlternateScreen,
        DisableMouseCapture,
        cursor::Show
    )?;
    terminal.show_cursor()?;

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
                    match key.code {
                        // Quit
                        KeyCode::Char('q') | KeyCode::Esc => {
                            app.should_quit = true;
                        }

                        // ═══ POSITIONS (Numpad layout) ═══
                        // 7 8 9  →  TopLeft    TopCenter    TopRight
                        // 4 5 6  →  MiddleLeft MiddleCenter MiddleRight
                        // 1 2 3  →  BottomLeft BottomCenter BottomRight
                        KeyCode::Char('7') => app.demo_position(Anchor::TopLeft),
                        KeyCode::Char('8') => app.demo_position(Anchor::TopCenter),
                        KeyCode::Char('9') => app.demo_position(Anchor::TopRight),
                        KeyCode::Char('4') => app.demo_position(Anchor::MiddleLeft),
                        KeyCode::Char('5') => app.demo_position(Anchor::MiddleCenter),
                        KeyCode::Char('6') => app.demo_position(Anchor::MiddleRight),
                        KeyCode::Char('1') => app.demo_position(Anchor::BottomLeft),
                        KeyCode::Char('2') => app.demo_position(Anchor::BottomCenter),
                        KeyCode::Char('3') => app.demo_position(Anchor::BottomRight),

                        // ═══ ANIMATIONS ═══
                        KeyCode::Char('s') => app.demo_slide_directions(),
                        KeyCode::Char('e') => app.demo_expand(),
                        KeyCode::Char('f') => app.demo_fade(),
                        KeyCode::Char('c') => app.demo_combined_effects(),

                        // ═══ SPECIFIC SLIDES (anchor vs direction independence) ═══
                        KeyCode::Char('t') => {
                            app.demo_specific_slide(Anchor::TopRight, SlideDirection::FromTop)
                        }
                        KeyCode::Char('r') => {
                            app.demo_specific_slide(Anchor::TopRight, SlideDirection::FromRight)
                        }
                        KeyCode::Char('u') => {
                            app.demo_specific_slide(Anchor::BottomLeft, SlideDirection::FromBottom)
                        }
                        KeyCode::Char('d') => {
                            app.demo_specific_slide(Anchor::BottomLeft, SlideDirection::FromLeft)
                        }

                        // ═══ CUSTOM PATH ═══
                        KeyCode::Char('g') => app.demo_custom_path(),

                        // ═══ REAL CONTENT ═══
                        KeyCode::Char('p') => app.demo_file_path(),
                        KeyCode::Char('m') => app.demo_success(),
                        KeyCode::Char('w') => app.demo_warning(),
                        KeyCode::Char('x') => app.demo_error(),

                        // ═══ SHOWCASES ═══
                        KeyCode::Char('l') => app.demo_all_levels(),
                        KeyCode::Char('k') => app.demo_stacking(),
                        KeyCode::Char('o') => app.demo_overflow(),

                        // ═══ OPTIONS ═══
                        KeyCode::Char('b') => app.cycle_border(),

                        // ═══ CODE MODAL ═══
                        KeyCode::Char('i') => {
                            if app.show_code_modal {
                                app.show_code_modal = false;
                            } else if !app.last_notification_code.is_empty() {
                                app.show_code_modal = true;
                            }
                        }

                        // ═══ HELP MODAL ═══
                        KeyCode::Char('?') => {
                            app.show_help_modal = !app.show_help_modal;
                        }

                        _ => {}
                    }

                    // Handle modal-specific keys when code modal is open
                    if app.show_code_modal {
                        match key.code {
                            KeyCode::Char('w') => {
                                let filename = find_available_filename("notification_example", "rs");
                                match std::fs::write(&filename, &app.last_notification_code) {
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
                                            let _ = app.notifications.add(n);
                                        }
                                        app.add_log(format!("Code written to {}", filename));
                                    }
                                    Err(e) => {
                                        app.add_log(format!("Write error: {}", e));
                                    }
                                }
                                app.show_code_modal = false;
                            }
                            KeyCode::Esc => {
                                app.show_code_modal = false;
                            }
                            _ => {}
                        }
                    }

                    // Handle help modal close
                    if app.show_help_modal && key.code == KeyCode::Esc {
                        app.show_help_modal = false;
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
    app.last_frame_area = f.area();
    let frame_area = f.area();

    // Layout: menu in center, log at bottom
    let main_layout = Layout::default()
        .direction(Direction::Vertical)
        .constraints([Constraint::Min(16), Constraint::Length(6)])
        .margin(1)
        .split(frame_area);

    let menu_area = main_layout[0];
    let log_area = main_layout[1];

    // Center the menu horizontally
    let menu_width = 72.min(menu_area.width);
    let menu_x = menu_area.x + (menu_area.width.saturating_sub(menu_width)) / 2;
    let centered_menu = Rect::new(menu_x, menu_area.y, menu_width, menu_area.height);

    render_menu(f, centered_menu, app);
    render_log(f, log_area, app);

    // Render notifications on top of everything
    app.notifications.render(f, frame_area);

    // Render modals on top if visible
    if app.show_code_modal {
        render_code_modal(f, frame_area, app);
    }
    if app.show_help_modal {
        render_help_modal(f, frame_area);
    }
}

fn render_menu(f: &mut Frame<'_>, area: Rect, app: &App) {
    let block = Block::default()
        .borders(Borders::ALL)
        .border_type(BorderType::Rounded)
        .border_style(Style::new().fg(Color::DarkGray));

    let inner = block.inner(area);
    f.render_widget(block, area);

    let layout = Layout::default()
        .direction(Direction::Vertical)
        .constraints([
            Constraint::Length(2), // Title
            Constraint::Length(1), // Spacer
            Constraint::Min(0),    // Content
        ])
        .split(inner);

    // Title
    let title = Line::from(vec![
        Span::styled("───", Style::new().fg(Color::DarkGray)),
        Span::styled(" Ratatui Notifications ", Style::new().fg(Color::White).add_modifier(Modifier::BOLD)),
        Span::styled("───", Style::new().fg(Color::DarkGray)),
    ]).alignment(Alignment::Center);
    f.render_widget(Paragraph::new(title), layout[0]);

    // Menu columns
    let content_layout = Layout::default()
        .direction(Direction::Horizontal)
        .constraints([Constraint::Ratio(1, 3), Constraint::Ratio(1, 3), Constraint::Ratio(1, 3)])
        .split(layout[2]);

    let key_style = Style::new().fg(Color::Cyan).add_modifier(Modifier::BOLD);
    let section_style = Style::new().fg(Color::Yellow);
    let dim_style = Style::new().fg(Color::DarkGray);

    // Column 1: Positions and Animations
    let col1 = vec![
        Line::from(Span::styled("POSITIONS", section_style)),
        Line::from(vec![
            Span::styled("7", key_style), Span::raw(" "), Span::styled("8", key_style), Span::raw(" "), Span::styled("9", key_style),
            Span::styled("  top", dim_style),
        ]),
        Line::from(vec![
            Span::styled("4", key_style), Span::raw(" "), Span::styled("5", key_style), Span::raw(" "), Span::styled("6", key_style),
            Span::styled("  mid", dim_style),
        ]),
        Line::from(vec![
            Span::styled("1", key_style), Span::raw(" "), Span::styled("2", key_style), Span::raw(" "), Span::styled("3", key_style),
            Span::styled("  bot", dim_style),
        ]),
        Line::raw(""),
        Line::from(Span::styled("ANIMATIONS", section_style)),
        Line::from(vec![Span::styled("s", key_style), Span::raw(" all directions")]),
        Line::from(vec![Span::styled("e", key_style), Span::raw(" expand")]),
        Line::from(vec![Span::styled("f", key_style), Span::raw(" fade")]),
        Line::from(vec![Span::styled("c", key_style), Span::raw(" slide+fade")]),
        Line::from(vec![Span::styled("g", key_style), Span::raw(" custom path")]),
    ];

    // Column 2: Specific slides and content
    let col2 = vec![
        Line::from(Span::styled("SPECIFIC SLIDES", section_style)),
        Line::from(vec![Span::styled("t", key_style), Span::raw(" TR←top")]),
        Line::from(vec![Span::styled("r", key_style), Span::raw(" TR←right")]),
        Line::from(vec![Span::styled("u", key_style), Span::raw(" BL←bottom")]),
        Line::from(vec![Span::styled("d", key_style), Span::raw(" BL←left")]),
        Line::raw(""),
        Line::from(Span::styled("CONTENT", section_style)),
        Line::from(vec![Span::styled("p", key_style), Span::raw(" file path")]),
        Line::from(vec![Span::styled("m", key_style), Span::raw(" success")]),
        Line::from(vec![Span::styled("w", key_style), Span::raw(" warning")]),
        Line::from(vec![Span::styled("x", key_style), Span::raw(" error")]),
    ];

    // Column 3: Showcases and options
    let col3 = vec![
        Line::from(Span::styled("SHOWCASES", section_style)),
        Line::from(vec![Span::styled("l", key_style), Span::raw(" log levels")]),
        Line::from(vec![Span::styled("k", key_style), Span::raw(" stacking")]),
        Line::from(vec![Span::styled("o", key_style), Span::raw(" overflow")]),
        Line::raw(""),
        Line::from(Span::styled("OPTIONS", section_style)),
        Line::from(vec![Span::styled("b", key_style), Span::raw(" border")]),
        Line::from(vec![
            Span::raw("  "),
            Span::styled(format!("{:?}", app.current_border_type), dim_style),
        ]),
        Line::from(vec![Span::styled("i", key_style), Span::raw(" show code")]),
        Line::from(vec![Span::styled("?", key_style), Span::raw(" help")]),
        Line::raw(""),
        Line::from(vec![Span::styled("q", key_style), Span::raw(" quit")]),
    ];

    f.render_widget(Paragraph::new(col1), content_layout[0]);
    f.render_widget(Paragraph::new(col2), content_layout[1]);
    f.render_widget(Paragraph::new(col3), content_layout[2]);
}

fn render_log(f: &mut Frame<'_>, area: Rect, app: &App) {
    let log_lines: Vec<Line> = app
        .log_messages
        .iter()
        .map(|msg| Line::styled(msg.clone(), Style::new().fg(Color::DarkGray)))
        .collect();

    let log_widget = Paragraph::new(log_lines)
        .block(
            Block::default()
                .borders(Borders::ALL)
                .border_type(BorderType::Rounded)
                .border_style(Style::new().fg(Color::DarkGray))
                .title(Span::styled(" Log ", Style::new().fg(Color::DarkGray))),
        )
        .wrap(Wrap { trim: true });

    f.render_widget(Clear, area);
    f.render_widget(log_widget, area);
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
        .title(" Generated Code ")
        .title_bottom(Line::from(" [w] Write to notification_example.rs | [i]/[Esc] Close ").alignment(Alignment::Center));

    let inner = block.inner(modal_area);
    f.render_widget(block, modal_area);

    let code_paragraph = Paragraph::new(app.last_notification_code.clone())
        .style(Style::new().fg(Color::Green))
        .wrap(Wrap { trim: false });
    f.render_widget(code_paragraph, inner);
}

fn render_help_modal(f: &mut Frame<'_>, frame_area: Rect) {
    let modal_width = 60.min(frame_area.width.saturating_sub(4));
    let modal_height = 18.min(frame_area.height.saturating_sub(4));
    let modal_x = (frame_area.width.saturating_sub(modal_width)) / 2;
    let modal_y = (frame_area.height.saturating_sub(modal_height)) / 2;
    let modal_area = Rect::new(modal_x, modal_y, modal_width, modal_height);

    f.render_widget(Clear, modal_area);

    let block = Block::default()
        .borders(Borders::ALL)
        .border_type(BorderType::Double)
        .border_style(Style::new().fg(Color::Yellow))
        .title(" Help ")
        .title_bottom(Line::from(" [?]/[Esc] Close ").alignment(Alignment::Center));

    let inner = block.inner(modal_area);
    f.render_widget(block, modal_area);

    let key_style = Style::new().fg(Color::Cyan).add_modifier(Modifier::BOLD);
    let help_text = vec![
        Line::from(vec![
            Span::styled("1-9", key_style),
            Span::raw("     Trigger notification at position (numpad layout)"),
        ]),
        Line::from(vec![
            Span::styled("s/e/f/c", key_style),
            Span::raw(" Animation demos (slide/expand/fade/combined)"),
        ]),
        Line::from(vec![
            Span::styled("g", key_style),
            Span::raw("       Custom path animation"),
        ]),
        Line::from(vec![
            Span::styled("t/r/u/d", key_style),
            Span::raw(" Specific slide directions"),
        ]),
        Line::from(vec![
            Span::styled("p/m/w/x", key_style),
            Span::raw(" Content demos (path/success/warning/error)"),
        ]),
        Line::from(vec![
            Span::styled("l/k/o", key_style),
            Span::raw("   Showcases (levels/stacking/overflow)"),
        ]),
        Line::raw(""),
        Line::from(vec![
            Span::styled("b", key_style),
            Span::raw("       Cycle border type"),
        ]),
        Line::from(vec![
            Span::styled("i", key_style),
            Span::raw("       Show generated code for last notification"),
        ]),
        Line::from(vec![
            Span::styled("w", key_style),
            Span::raw("       (in code modal) Write code to file"),
        ]),
        Line::raw(""),
        Line::from(vec![
            Span::styled("q/Esc", key_style),
            Span::raw("   Quit"),
        ]),
    ];

    let help_paragraph = Paragraph::new(help_text).wrap(Wrap { trim: false });
    f.render_widget(help_paragraph, inner);
}

// FILE: examples/demo.rs - Interactive demonstration of ratatui-notifications crate features
// END OF VERSION: 2.3.0
