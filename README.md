<div align="center">

# bevy_luma_widgets

**A modern, high-level UI widget component suite for the bevy_luma framework in Bevy.**

[![Crates.io](https://img.shields.io/badge/crates.io-v0.1.0-orange?style=flat-square)](https://crates.io/crates/bevy_luma_widgets)
[![Docs.rs](https://img.shields.io/badge/docs.rs-bevy__luma__widgets-blue?style=flat-square)](https://docs.rs/bevy_luma_widgets)
[![Bevy](https://img.shields.io/badge/Bevy-0.19-purple?style=flat-square)](https://bevyengine.org)
[![License](https://img.shields.io/badge/License-MIT%20OR%20Apache--2.0-green?style=flat-square)](#license)

</div>

---

## Overview

`bevy_luma_widgets` provides a battery-included collection of responsive, accessible, theme-aware UI components built on top of the `bevy_luma` signed distance field (SDF) engine.

Every widget is designed from the ground up for **Bevy Scene Notation (`bsn!`)**, enabling declarative, compositional UI tree creation with zero runtime baggage.

### Included Widgets

| Widget | Purpose | Key Features |
|---|---|---|
| **Button** | Interactive triggers | Primary, Secondary, Outline, Ghost, Danger variants; leading/trailing icons; Small, Medium, Large sizes. |
| **Toggle** | Boolean switches | Animated sliding knob with spring lerp, automatic track color shift, pointer events. |
| **Checkbox** | Multiple choice toggles | Checkmark indicator powered by Lucide icons, configurable text label. |
| **Slider** | Continuous range inputs | Drag & click track hit-testing, dynamic fill bar, thumb knob, optional step rounding. |
| **ProgressBar** | Visual progress indicators | Clamped 0.0 to 1.0 range, Default, Success, Warning, and Danger semantic color variants. |
| **Badge** | Status pill indicators | Compact tags with Info, Success, Warning, Danger, and Neutral styles; optional icons. |
| **Divider** | Content separation | Full-width horizontal, fixed vertical, and horizontal with embedded text labels. |
| **Card** | Surface grouping | Elevated panel containers with GPU anti-aliased corner radiuses and subtle borders. |
| **Modal** | Overlay dialogs | Fullscreen darkened backdrop with centered dialog, message body, and action buttons. |
| **Radio** | Mutually exclusive choices | Grouped single-selection radios with synchronized indicator dots. |
| **Input** | Text editing fields | Styled container with focus borders, placeholder handling, and caret editing. |

---

## Installation

Add `bevy_luma_widgets` to your `Cargo.toml`:

```toml
[dependencies]
bevy = "0.19"
bevy_luma_widgets = "0.1.0"
```

---

## Quickstart

```rust
use bevy::prelude::*;
use bevy_luma_widgets::prelude::*;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_plugins(LumaWidgetsPlugin)
        .add_systems(Startup, setup_ui)
        .run();
}

fn setup_ui(mut commands: Commands, theme: Res<Theme>) {
    let font = theme.text.font.inter_regular.clone();

    commands.spawn(Camera2d);
    commands.spawn_scene(bsn! {
        URootUi::screen()
        UNode {
            width: UVal::Percent(1.0),
            height: UVal::Percent(1.0),
            background_color: Color::srgb(0.08, 0.09, 0.12),
        }
        ULayout {
            display: UDisplay::Flex,
            justify_content: UJustifyContent::Center,
            align_items: UAlignItems::Center,
            gap: 16.0,
        }
        Children [
            (
                luma_button("Click Me", ButtonVariant::Primary, ButtonSize::Medium, font.clone())
                on(|_event: On<Pointer<Click>>| {
                    info!("Button clicked!");
                })
            ),
            (
                luma_toggle_with_label("Active", true, font.clone())
            )
        ]
    });
}
```

---

## Component Reference

### 1. Button

Buttons support multiple styling presets, sizing variants, and optional icons:

```rust
// Standard text button
luma_button("Confirm", ButtonVariant::Primary, ButtonSize::Medium, font.clone())

// Leading icon button
luma_button_with_icon(Icon::PLAY, "Start Game", ButtonVariant::Primary, ButtonSize::Large, font.clone(), icon_font.clone())

// Square icon-only button
luma_icon_button(Icon::SETTINGS, ButtonVariant::Secondary, ButtonSize::Medium, icon_font.clone())
```

### 2. Toggle Switch

Interactive switches with smooth animated knob transitions:

```rust
// Standalone switch
luma_toggle(true)

// Switch with text label
luma_toggle_with_label("V-Sync Lock", false, font.clone())
```

Listen to state changes via the `ToggleChanged` observer event:

```rust
app.add_observer(|trigger: On<ToggleChanged>| {
    info!("Toggle {:?} is now: {}", trigger.entity, trigger.is_checked);
});
```

### 3. Checkbox

```rust
luma_checkbox("Enable Motion Blur", true, font.clone(), icon_font.clone())
```

### 4. Slider

Sliders track pointer clicks and dragging, automatically synchronizing the fill bar and thumb knob:

```rust
// value, min, max, width in pixels
luma_slider(75.0, 0.0, 100.0, 240.0)
```

Listen for changes via `SliderChanged`:

```rust
app.add_observer(|trigger: On<SliderChanged>| {
    info!("Slider value changed to: {}", trigger.value);
});
```

### 5. Progress Bar

```rust
luma_progress_bar(0.65, UVal::Px(280.0), 8.0, ProgressVariant::Success)
```

Variants: `ProgressVariant::Default`, `ProgressVariant::Success`, `ProgressVariant::Warning`, `ProgressVariant::Danger`.

### 6. Badge

```rust
luma_badge("ONLINE", BadgeVariant::Success, font.clone())
luma_badge_with_icon(Icon::CHECK, "VERIFIED", BadgeVariant::Info, font.clone(), icon_font.clone())
```

### 7. Divider

```rust
// Simple horizontal separator
luma_divider()

// Horizontal separator with centered text label
luma_divider_with_label("OR", font.clone())

// Fixed vertical separator
luma_divider_vertical(40.0)
```

### 8. Modal Dialog

```rust
luma_modal_dialog(
    "Discard Changes?",
    "Any unsaved edits will be lost permanently.",
    "Cancel",
    "Discard",
    font.clone(),
)
```

---

## Examples

Run the included showcase examples:

```bash
# Complete widget gallery
cargo run --example widgets_gallery

# Realistic game settings dialog
cargo run --example settings_dialog
```

---

## Architecture

`bevy_luma_widgets` builds directly on `bevy_luma` primitives:

```
+--------------------------------------------------------+
|                   bevy_luma_widgets                    |
|   (Buttons, Toggles, Sliders, Checkboxes, Dialogs)     |
+--------------------------------------------------------+
                           |
+--------------------------------------------------------+
|                       bevy_luma                        |
|  +---------------+  +---------------+  +-------------+ |
|  |  luma_layout  |  |  luma_engine  |  | luma_style  | |
|  | (Grid / Flex) |  | (SDF Shaders) |  | (Typography)| |
|  +---------------+  +---------------+  +-------------+ |
+--------------------------------------------------------+
                           |
+--------------------------------------------------------+
|                          Bevy                          |
|         (ECS, Scene Notation bsn!, Winit, WGPU)        |
+--------------------------------------------------------+
```

---

## License

Dual-licensed under either of:

- Apache License, Version 2.0 ([LICENSE-APACHE](LICENSE-APACHE) or http://www.apache.org/licenses/LICENSE-2.0)
- MIT license ([LICENSE-MIT](LICENSE-MIT) or http://opensource.org/licenses/MIT)

at your option.
