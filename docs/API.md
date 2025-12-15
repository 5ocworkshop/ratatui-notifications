<!-- FILE: API.md - Public API Reference for ratatui-notifications -->
<!-- VERSION: 1.1.0 -->
<!-- WCTX: Adding code generation feature -->
<!-- CLOG: Added generate_code() utility function documentation -->

# API Reference

This document describes the public API of `ratatui-notifications`.

## Core Types

### `Notifications`

The notification manager. Create one instance and use it throughout your application.

```rust
use ratatui_notifications::{Notifications, Overflow};

// Basic construction
let notifications = Notifications::new();

// With configuration
let notifications = Notifications::new()
    .max_concurrent(Some(5))
    .overflow(Overflow::DiscardOldest);
```

#### Methods

| Method | Signature | Description |
|--------|-----------|-------------|
| `new()` | `fn new() -> Self` | Create a new notification manager |
| `max_concurrent()` | `fn max_concurrent(self, max: Option<usize>) -> Self` | Set max simultaneous notifications (`None` = unlimited) |
| `overflow()` | `fn overflow(self, behavior: Overflow) -> Self` | Set behavior when max is exceeded |
| `add()` | `fn add(&mut self, notification: Notification) -> Result<u64, NotificationError>` | Add a notification, returns its ID |
| `remove()` | `fn remove(&mut self, id: u64)` | Remove a notification by ID |
| `clear()` | `fn clear(&mut self)` | Remove all notifications |
| `tick()` | `fn tick(&mut self, delta: Duration)` | Advance animation state (call each frame) |
| `render()` | `fn render(&self, frame: &mut Frame, area: Rect)` | Render all notifications |
| `active_count()` | `fn active_count(&self) -> usize` | Number of currently visible notifications |
| `is_empty()` | `fn is_empty(&self) -> bool` | Whether there are no active notifications |

---

### `Notification` / `NotificationBuilder`

Configuration for a single notification. Use the builder pattern.

```rust
use ratatui_notifications::{Notification, NotificationBuilder, Level, Anchor, Animation};
use std::time::Duration;

// Short form
let notification = Notification::new("Message content")
    .title("Title")
    .level(Level::Info)
    .build()
    .unwrap();

// Or use builder directly
let notification = NotificationBuilder::new("Message content")
    .title("Title")
    .anchor(Anchor::TopRight)
    .animation(Animation::Fade)
    .build()
    .unwrap();
```

#### Builder Methods

| Method | Type | Default | Description |
|--------|------|---------|-------------|
| `new(content)` | `impl Into<Text<'static>>` | — | Create builder with message content |
| `title()` | `impl Into<Line<'static>>` | `None` | Set notification title |
| `level()` | `Level` | `None` | Set log level (affects icon and colors) |
| `anchor()` | `Anchor` | `BottomRight` | Screen position |
| `animation()` | `Animation` | `Slide(Default)` | Animation style |
| `slide_direction()` | `SlideDirection` | `Default` | Direction for slide animation |
| `timing()` | `(entry, dwell, exit)` | Auto-calculated | Set animation durations |
| `auto_dismiss()` | `AutoDismiss` | `After(4s)` | When to automatically dismiss |
| `margin()` | `u16` | `1` | Margin from screen edge |
| `border_type()` | `BorderType` | `Rounded` | Border style |
| `border_style()` | `Style` | Level-based | Border color/style |
| `title_style()` | `Style` | Level-based | Title color/style |
| `content_style()` | `Style` | Default | Content text style |
| `fade()` | `bool` | `false` | Enable fade effect on slide |
| `entry_position()` | `Position` | Auto | Custom slide start position |
| `exit_position()` | `Position` | Auto | Custom slide end position |
| `build()` | — | — | Build the notification (validates content) |

---

## Utility Functions

### `generate_code()`

Generates Rust source code that recreates a notification configuration. Useful for tools that let users configure notifications visually and then export the code.

```rust
use ratatui_notifications::{Notification, generate_code, Level, Anchor};

let notification = Notification::new("Hello!")
    .title("Greeting")
    .level(Level::Info)
    .anchor(Anchor::TopRight)
    .build()
    .unwrap();

let code = generate_code(&notification);
// Returns:
// Notification::builder("Hello!")
//     .title("Greeting")
//     .anchor(Anchor::TopRight)
//     .build()

// Note: Level::Info is the default, so it's not included
```

#### Behavior

- **Minimal output**: Only includes non-default values to keep code clean
- **String escaping**: Content with quotes or newlines is properly escaped
- **Formatted**: Each builder method is on its own indented line
- **Valid Rust**: Output can be copy-pasted directly into code

#### Signature

```rust
pub fn generate_code(notification: &Notification) -> String
```

---

## Enums

### `Anchor`

Screen position for the notification.

```rust
pub enum Anchor {
    TopLeft,      TopCenter,      TopRight,
    MiddleLeft,   MiddleCenter,   MiddleRight,
    BottomLeft,   BottomCenter,   BottomRight,  // default
}
```

---

### `Animation`

Animation style for entry and exit.

```rust
pub enum Animation {
    Slide(SlideDirection),  // default: Slide(Default)
    ExpandCollapse,         // Grow from center
    Fade,                   // Fade in/out
}
```

---

### `SlideDirection`

Direction for slide animation. `Default` chooses based on anchor position.

```rust
pub enum SlideDirection {
    Default,     // Auto-select based on anchor
    FromTop,
    FromBottom,
    FromLeft,
    FromRight,
    FromTopLeft,
    FromTopRight,
    FromBottomLeft,
    FromBottomRight,
}
```

---

### `Level`

Log level for automatic styling.

```rust
pub enum Level {
    Info,   // default, blue icon
    Warn,   // yellow/orange icon
    Error,  // red icon
    Debug,  // gray icon
    Trace,  // dim gray icon
}
```

Each level has a distinct icon and color scheme applied automatically.

---

### `AutoDismiss`

When to automatically remove the notification.

```rust
pub enum AutoDismiss {
    Never,                    // Manual removal only
    After(Duration),          // Auto-dismiss after duration (default: 4 seconds)
}
```

---

### `Overflow`

Behavior when `max_concurrent` is exceeded.

```rust
pub enum Overflow {
    DiscardOldest,  // default: remove oldest notification
    DiscardNewest,  // reject new notification
}
```

---

### `Timing`

Duration specification for animation phases.

```rust
pub enum Timing {
    Auto,              // Calculate based on content size
    Fixed(Duration),   // Use exact duration
}
```

---

### `SizeConstraint`

Notification size constraints.

```rust
pub enum SizeConstraint {
    Absolute(u16),      // Fixed pixel size
    Percentage(f32),    // Percentage of container (0.0-1.0)
}
```

---

## Error Types

### `NotificationError`

```rust
pub enum NotificationError {
    ContentTooLong { max: usize, actual: usize },
    InvalidConfiguration(String),
}
```

Content is limited to 1000 characters. Use the error's `Display` impl for user-friendly messages.

---

## Usage Pattern

### Basic Integration

```rust
use ratatui_notifications::{Notification, Notifications, Level};
use std::time::Duration;

fn main() {
    let mut notifications = Notifications::new();

    // Add notification when something happens
    let notif = Notification::new("File saved successfully")
        .title("Success")
        .level(Level::Info)
        .build()
        .unwrap();
    notifications.add(notif).unwrap();

    // In your render loop (60fps = 16ms per frame)
    loop {
        // ... handle events ...

        notifications.tick(Duration::from_millis(16));

        terminal.draw(|frame| {
            // ... render your app ...

            // Render notifications last (on top)
            notifications.render(frame, frame.area());
        })?;
    }
}
```

### Custom Styling

```rust
use ratatui::style::{Color, Style};
use ratatui::widgets::BorderType;

let notification = Notification::new("Custom styled notification")
    .title("Custom")
    .border_type(BorderType::Double)
    .border_style(Style::new().fg(Color::Cyan))
    .title_style(Style::new().fg(Color::Yellow))
    .content_style(Style::new().fg(Color::White))
    .build()
    .unwrap();
```

### Custom Animation Path

```rust
use ratatui::layout::Position;

let notification = Notification::new("Slides from custom position")
    .animation(Animation::Slide(SlideDirection::FromLeft))
    .entry_position(Position::new(0, 10))
    .exit_position(Position::new(100, 10))
    .fade(true)  // Add fade effect to slide
    .build()
    .unwrap();
```

---

## Threading Model

The library is **synchronous and non-blocking**:

- `tick(delta)` advances animation state based on elapsed time
- `render()` draws current state without blocking
- No threads or async runtime required
- Integrates with any event loop (crossterm, termion, etc.)

Call `tick()` once per frame with the actual elapsed time for smooth animations.

<!-- FILE: API.md - Public API Reference for ratatui-notifications -->
<!-- END OF VERSION: 1.1.0 -->
