<!-- FILE: README.md - Project overview and quick start guide -->
<!-- VERSION: 1.1.0 -->
<!-- WCTX: Adding code generation feature -->
<!-- CLOG: Added cookbook and code generation documentation -->

# ratatui-notifications

Animated notification widgets for [ratatui](https://ratatui.rs) terminal applications.

[![Crates.io](https://img.shields.io/crates/v/ratatui-notifications.svg)](https://crates.io/crates/ratatui-notifications)
[![License: MIT](https://img.shields.io/badge/License-MIT-blue.svg)](LICENSE)

## Features

- **Multiple animation styles** — Slide, Fade, Expand/Collapse
- **9 anchor positions** — Any corner, edge, or center of the screen
- **Auto-dismiss** — Configurable automatic dismissal with countdown
- **Smart stacking** — Multiple notifications stack without overlap
- **Level-based styling** — Info, Warn, Error, Debug, Trace with distinct icons
- **Fully customizable** — Colors, borders, timing, positions
- **Non-blocking** — Synchronous design integrates with any event loop
- **Code generation** — Export working Rust code for any notification configuration

## Try It Now

The fastest way to explore notifications is with the **interactive cookbook**:

```bash
cargo run --example cookbook
```

Press any key (`1-9`, `0`, `a-e`) to trigger a recipe and see exactly what code produces it. Press `i` to view the generated code, then `w` to save it to `notification_example.rs`.

For a full feature demo with all animation types and positions:

```bash
cargo run --example demo
```

> **Tip:** In both examples, press `i` after triggering any notification to see the exact Rust code needed to recreate it. Press `w` to write the code to a file you can copy into your project.

## Installation

Add to your `Cargo.toml`:

```toml
[dependencies]
ratatui-notifications = "0.1"
ratatui = "0.29"
```

## Quick Start

```rust
use ratatui_notifications::{Notification, Notifications, Level};
use std::time::Duration;

// Create the notification manager
let mut notifications = Notifications::new();

// Add a notification
let notif = Notification::new("Operation completed successfully!")
    .title("Success")
    .level(Level::Info)
    .build()
    .unwrap();

notifications.add(notif).unwrap();

// In your render loop:
loop {
    // Handle events...

    // Advance animations (call once per frame)
    notifications.tick(Duration::from_millis(16));

    terminal.draw(|frame| {
        // Render your app...

        // Render notifications on top
        notifications.render(frame, frame.area());
    })?;
}
```

## Animation Styles

### Slide (Default)

```rust
use ratatui_notifications::{Notification, Animation, SlideDirection};

let notif = Notification::new("Sliding in from the right!")
    .animation(Animation::Slide(SlideDirection::FromRight))
    .build()
    .unwrap();
```

### Fade

```rust
let notif = Notification::new("Fading in...")
    .animation(Animation::Fade)
    .build()
    .unwrap();
```

### Expand/Collapse

```rust
let notif = Notification::new("Expanding from center!")
    .animation(Animation::ExpandCollapse)
    .build()
    .unwrap();
```

## Positioning

Notifications can be anchored to any of 9 positions:

```rust
use ratatui_notifications::Anchor;

// Corners
Anchor::TopLeft, Anchor::TopRight, Anchor::BottomLeft, Anchor::BottomRight

// Edges
Anchor::TopCenter, Anchor::MiddleLeft, Anchor::MiddleRight, Anchor::BottomCenter

// Center
Anchor::MiddleCenter
```

## Stacking & Overflow

```rust
use ratatui_notifications::{Notifications, Overflow};

let notifications = Notifications::new()
    .max_concurrent(Some(5))           // Max 5 visible at once
    .overflow(Overflow::DiscardOldest); // Remove oldest when full
```

## Auto-Dismiss

```rust
use ratatui_notifications::AutoDismiss;
use std::time::Duration;

// Dismiss after 5 seconds
let notif = Notification::new("This will disappear")
    .auto_dismiss(AutoDismiss::After(Duration::from_secs(5)))
    .build()
    .unwrap();

// Never auto-dismiss (manual removal required)
let persistent = Notification::new("Click to dismiss")
    .auto_dismiss(AutoDismiss::Never)
    .build()
    .unwrap();
```

## Custom Styling

```rust
use ratatui::style::{Color, Style};
use ratatui::widgets::BorderType;

let notif = Notification::new("Styled notification")
    .title("Custom")
    .border_type(BorderType::Double)
    .border_style(Style::new().fg(Color::Cyan))
    .title_style(Style::new().fg(Color::Yellow))
    .build()
    .unwrap();
```

## Examples

### Cookbook (Recommended for Getting Started)

```bash
cargo run --example cookbook
```

15 ready-to-use recipes covering common patterns:
- Simple toasts, error alerts, warnings
- Persistent notifications, quick flashes
- Different animations (slide, fade, expand)
- Custom styling and positioning

**Keys:** `1-9`, `0`, `a-e` trigger recipes | `i` view code | `w` write to file | `q` quit

### Interactive Demo

```bash
cargo run --example demo
```

Full-featured demo showcasing all capabilities:

| Category | Keys | Description |
|----------|------|-------------|
| Positions | `1-9` | Numpad layout (7=TopLeft, 5=Center, 3=BottomRight) |
| Animations | `s` `e` `f` `c` | Slide directions, Expand, Fade, Combined |
| Slides | `t` `r` `u` `d` | Specific directions (top, right, bottom, left) |
| Content | `p` `m` `w` `x` | Path, Success, Warning, Error examples |
| Showcases | `l` `k` `o` | Log levels, Stacking, Overflow |
| Options | `b` | Cycle border styles |
| **Code** | `i` | **View generated code for last notification** |
| **Export** | `w` | **Write code to notification_example.rs** (in code modal) |
| Help | `?` | Show all controls |
| Quit | `q` | Exit |

### Code Generation

Both examples support exporting notification configurations as Rust code:

1. Trigger any notification
2. Press `i` to open the code modal
3. Press `w` to save to `notification_example.rs`

The generated code only includes non-default values, keeping it minimal and clean.

## API Documentation

See [API.md](API.md) for the complete API reference.

## License

MIT License. See [LICENSE](LICENSE) for details.

<!-- FILE: README.md - Project overview and quick start guide -->
<!-- END OF VERSION: 1.1.0 -->
