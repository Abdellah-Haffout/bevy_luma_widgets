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

Every widget is designed from the ground up with **three core pillars**:
1. **Declarative Structs with `Scene`**: Every widget is a first-class Rust `struct` with public fields, `Default`, and fluent builder methods, implementing `bevy::scene::Scene` directly for **Bevy Scene Notation (`bsn!`)**.
2. **Android-Style Style Inheritance**: Hierarchical styling scopes (`LumaStyleScope`) allow subtrees and container components to inherit styles automatically (just like Android's XML theme/style inheritance), while still allowing individual widgets to override them.
3. **Zero-Overhead Convenience Functions**: Familiar helper wrappers (`luma_button(...)`, `luma_toggle(...)`, etc.) remain fully supported for quick prototyping.

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
| **Kbd** | Keyboard keycap badges | Raised 3D keycaps for hotkeys and shortcut combinations (`Ctrl + K`). |
| **StatCard** | Dashboard KPI metrics | Large display value, title, directional trend pill (`+14%` Up, `-8%` Down), and optional icon. |
| **Stepper** | Numeric input counter | Decrement `[-]` and increment `[+]` buttons, value clamping, and dynamic formatted display. |
| **Rating** | Star rating | 5-star rating widget in static display or interactive selectable mode (`Icon::STAR`). |
| **Breadcrumb** | Hierarchical navigation | Trail navigation with chevron separators and clickable non-current items. |
| **SegmentedControl** | Pill button group | Grouped segment options with animated active pill highlight. |
| **Chip** | Tags & chips | Rounded pills with optional leading icons and interactive dismiss `[x]` buttons. |
| **Toast** | Contextual notifications | Floating alert cards with variant status icons, close button, and auto-dismiss timer. |
| **Swatch** | Color preview & picker | Rounded color tiles with selection border highlight and optional label. |
| **Accordion** | Collapsible disclosure panels | Expandable content drawers with chevron animation. |
| **Alert** | Status banners | Callout cards with semantic variants (Info, Success, Warning, Danger). |
| **Avatar** | User initials & presence | Circular avatar initials with presence indicator dot (Online, Busy, Away). |
| **Skeleton** | Loading placeholders | Pulsing placeholder boxes for asynchronous content loading states. |
| **Tabs** | Switchable view switcher | Tab list triggers with active highlight and synchronized content panels. |
| **Tooltip** | Hover descriptions | Compact floating tooltip bubble with subtle borders. |

---

## Installation

Add `bevy_luma_widgets` to your `Cargo.toml`:

```toml
[dependencies]
bevy = "0.19"
bevy_luma_widgets = "0.1.0"
```

---

## Declarative Structs & Fluent Builders

Every widget can be created via:

1. **Fluent Builder Pattern (Recommended)**:
   ```rust
   ({ Button::new("Confirm").variant(ButtonVariant::Primary).size(ButtonSize::Medium).font(font.clone()) })
   ```
2. **Direct Struct Literal**:
   ```rust
   ({
       Button {
           label: "Confirm".into(),
           variant: ButtonVariant::Primary,
           size: ButtonSize::Medium,
           font: font.clone(),
           ..default()
       }
   })
   ```
3. **Convenience Function**:
   ```rust
   (luma_button("Confirm", ButtonVariant::Primary, ButtonSize::Medium, font.clone()))
   ```

---

## Android-Style Theming & Style Scoping

Just like in Android where you can define styles (`<style name="MyButton">`) and apply them to a layout or subtree so that all descendant views automatically inherit those visual attributes, `bevy_luma_widgets` provides `LumaStyleScope`:

```rust
// Apply custom styling across a whole branch:
luma_style_scope(
    LumaStyleScope::new()
        .button(ButtonStyle {
            bg_normal: Color::srgb(0.22, 0.14, 0.38),
            bg_hover: Color::srgb(0.32, 0.20, 0.52),
            text_color: Color::WHITE,
            radius: 20.0, // Fully rounded pill
            ..default()
        })
        .toggle(ToggleStyle {
            checked_bg: Color::srgb(0.65, 0.25, 0.90),
            ..default()
        }),
    bsn! {
        Children [
            // These buttons automatically inherit the scoped pill styling!
            ({ Button::new("Inherited Action A").font(font.clone()) }),
            ({ Button::new("Inherited Action B").font(font.clone()) }),

            // Individual widgets can still override the scope explicitly:
            ({ Button::new("Danger Override").variant(ButtonVariant::Danger).font(font.clone()) })
        ]
    }
)
```

Widgets inspect their ancestor hierarchy for `LumaStyleScope` using tree traversal. If no scope is found, they cleanly fall back to the global `LumaWidgetTheme` resource.

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
                { Button::new("Click Me").variant(ButtonVariant::Primary).size(ButtonSize::Medium).font(font.clone()) }
                on(|_event: On<Pointer<Click>>| {
                    info!("Button clicked!");
                })
            ),
            ({ Toggle::new(true).label("Active").font(font.clone()) })
        ]
    });
}
```

---

## Component Reference

### 1. Button

Buttons can be constructed declaratively as structs with fluent builder methods or convenience functions:

```rust
// Declarative builder with variant and size
({ Button::new("Confirm").variant(ButtonVariant::Primary).size(ButtonSize::Medium).font(font.clone()) })

// Direct struct initialization
({
    Button {
        label: "Confirm".into(),
        variant: ButtonVariant::Primary,
        font: font.clone(),
        ..default()
    }
})

// Leading icon button
({ Button::new("Play").icon(Icon::PLAY).variant(ButtonVariant::Primary).font(font.clone()).icon_font(icon_font.clone()) })

// Backwards-compatible helper
(luma_button("Confirm", ButtonVariant::Primary, ButtonSize::Medium, font.clone()))
```

### 2. Toggle Switch

Interactive switches with smooth animated knob transitions:

```rust
// Declarative builder
({ Toggle::new(true).label("V-Sync Lock").font(font.clone()) })

// Standalone switch without label
({ Toggle::new(false) })
```

Listen to state changes via the `ToggleChanged` observer event:

```rust
app.add_observer(|trigger: On<ToggleChanged>| {
    info!("Toggle {:?} is now: {}", trigger.entity, trigger.is_checked);
});
```

### 3. Checkbox

```rust
// Declarative builder
({ Checkbox::new("Enable Motion Blur", true).font(font.clone()).icon_font(icon_font.clone()) })

// Checkbox without label (box only)
({ Checkbox::box_only(true).icon_font(icon_font.clone()) })

// Convenience function
(luma_checkbox("Enable Motion Blur", true, font.clone(), icon_font.clone()))
```

### 4. Slider

Sliders track pointer clicks and dragging, automatically synchronizing the fill bar and thumb knob:

```rust
// Declarative builder with range and width
({ Slider::new(75.0, 0.0, 100.0, 240.0) })

// Stepped slider
({ Slider::stepped(20.0, 0.0, 100.0, 10.0, 240.0) })

// Direct builder chaining
({ Slider::default().value(50.0).min(0.0).max(100.0).width(200.0) })
```

Listen for changes via `SliderChanged`:

```rust
app.add_observer(|trigger: On<SliderChanged>| {
    info!("Slider value changed to: {}", trigger.value);
});
```

### 5. Progress Bar

```rust
// Declarative builder
({ ProgressBar::new(0.65).width(UVal::Px(280.0)).height(8.0).variant(ProgressVariant::Success) })

// Convenience function
(luma_progress_bar(0.65, UVal::Px(280.0), 8.0, ProgressVariant::Success))
```

Variants: `ProgressVariant::Default`, `ProgressVariant::Success`, `ProgressVariant::Warning`, `ProgressVariant::Danger`.

### 6. Badge

```rust
// Declarative builder
({ Badge::new("ONLINE").variant(BadgeVariant::Success).font(font.clone()) })
({ Badge::new("VERIFIED").icon(Icon::CHECK).variant(BadgeVariant::Info).font(font.clone()).icon_font(icon_font.clone()) })

// Convenience functions
(luma_badge("ONLINE", BadgeVariant::Success, font.clone()))
(luma_badge_with_icon(Icon::CHECK, "VERIFIED", BadgeVariant::Info, font.clone(), icon_font.clone()))
```

### 7. Divider

```rust
// Horizontal divider
({ Divider::horizontal() })

// Horizontal divider with centered text label
({ Divider::horizontal().label("OR").font(font.clone()) })

// Fixed vertical divider
({ Divider::vertical(40.0) })
```

### 8. Modal Dialog

```rust
({
    Modal::new("Discard Changes?", "Any unsaved edits will be lost permanently.")
        .confirm("Discard")
        .cancel("Cancel")
        .font(font.clone())
})
```

### 9. Keyboard Keycap (Kbd)

```rust
// Single keycap
({ Kbd::new("Ctrl").size(KbdSize::Medium).font(font.clone()) })

// Multi-key combination
(luma_kbd_shortcut((
    { Kbd::new("Ctrl").size(KbdSize::Medium).font(font.clone()) },
    luma_kbd_separator(font.clone()),
    { Kbd::new("K").size(KbdSize::Medium).font(font.clone()) }
)))
```

### 10. Stat / Metric Card

```rust
// Basic metric card
({ StatCard::new("Active Users", "14,820").font(font.clone()) })

// Metric with trend indicator (+14.2% Up)
({
    StatCard::new("Active Users", "14,820")
        .trend(StatTrend::Up("+14.2%".into()))
        .font(font.clone())
        .icon_font(icon_font.clone())
})

// Full KPI card with icon and description
({
    StatCard::new("Frame Rate", "144 FPS")
        .trend(StatTrend::Up("+12 FPS".into()))
        .description("VSync lock enabled")
        .icon(Icon::CPU)
        .font(font.clone())
        .icon_font(icon_font.clone())
})
```

### 11. Numeric Stepper

Interactive counter with decrement `[-]` and increment `[+]` buttons:

```rust
// value, min, max, step
({ Stepper::new(3.0, 1.0, 20.0, 1.0).font(font.clone()).icon_font(icon_font.clone()) })
```

Listen for changes via `StepperChanged`:

```rust
app.add_observer(|trigger: On<StepperChanged>| {
    info!("Stepper value: {}", trigger.value);
});
```

### 12. Star Rating

```rust
// Read-only display (5 stars)
({ Rating::new(5).icon_font(icon_font.clone()) })

// Interactive clickable 5-star selector
({ Rating::new(4).interactive(true).icon_font(icon_font.clone()) })
```

Listen for user selections via `RatingChanged`:

```rust
app.add_observer(|trigger: On<RatingChanged>| {
    info!("New rating selected: {} stars", trigger.value);
});
```

### 13. Breadcrumbs

```rust
({
    BreadcrumbTrail::new((
        BreadcrumbItem::new(0, "home", "Home", false).font(font.clone()),
        BreadcrumbSeparator::new(icon_font.clone()),
        BreadcrumbItem::new(1, "components", "Components", false).font(font.clone()),
        BreadcrumbSeparator::new(icon_font.clone()),
        BreadcrumbItem::new(2, "widgets", "Gallery", true).font(font.clone()),
    ))
})
```

Listen for breadcrumb clicks via `BreadcrumbClicked`:

```rust
app.add_observer(|trigger: On<BreadcrumbClicked>| {
    info!("Navigating to crumb {}: {}", trigger.index, trigger.value);
});
```

### 14. Segmented Control

```rust
({
    SegmentedControl::new(
        "view_mode",
        "analytics",
        (
            SegmentOption::new("view_mode", "overview", "Overview", false).font(font.clone()),
            SegmentOption::new("view_mode", "analytics", "Analytics", true).font(font.clone()),
            SegmentOption::new("view_mode", "settings", "Settings", false).font(font.clone()),
        ),
    )
})
```

Listen for segment changes via `SegmentChanged`:

```rust
app.add_observer(|trigger: On<SegmentChanged>| {
    info!("Selected segment '{}': {}", trigger.group, trigger.value);
});
```

### 15. Chips & Tags

```rust
// Standard text chip
({ Chip::new("Rust 2024").font(font.clone()) })

// Chip with icon
({ Chip::new("Multi-Threaded").icon(Icon::CPU).font(font.clone()).icon_font(icon_font.clone()) })

// Removable chip with dismiss [x] button
({ Chip::new("Bevy 0.19").removable(true).font(font.clone()).icon_font(icon_font.clone()) })
```

Listen for dismiss actions via `ChipDismissed`:

```rust
app.add_observer(|trigger: On<ChipDismissed>| {
    info!("Chip {:?} was dismissed", trigger.0);
});
```

### 16. Toast Notifications

```rust
// Interactive notification banner
({
    Toast::new(
        "Download Complete",
        "The assets have finished downloading.",
    )
    .variant(ToastVariant::Success)
    .font(font.clone())
    .icon_font(icon_font.clone())
})

// Auto-dismiss notification after 5 seconds
({
    Toast::new(
        "Notice",
        "Temporary notification message",
    )
    .variant(ToastVariant::Info)
    .duration(5.0)
    .font(font.clone())
    .icon_font(icon_font.clone())
})
```

### 17. Color Swatch

```rust
// Color preview tile
({ Swatch::new(Color::srgb(0.20, 0.45, 0.90)).selected(true) })

// Swatch with label
({ Swatch::new(Color::srgb(0.16, 0.75, 0.45)).label("Emerald").selected(false).font(font.clone()) })
```

Listen for selection changes via `SwatchSelected`:

```rust
app.add_observer(|trigger: On<SwatchSelected>| {
    info!("Swatch clicked: {:?}", trigger.color);
});
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
