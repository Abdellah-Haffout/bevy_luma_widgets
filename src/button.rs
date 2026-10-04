//! Interactive button widget with variants, sizes, and icon support.

use bevy::prelude::*;
use bevy_luma::prelude::*;

/// Styling variant for buttons.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Default, Reflect)]
pub enum ButtonVariant {
    #[default]
    Primary,
    Secondary,
    Outline,
    Ghost,
    Danger,
}

/// Sizing preset for buttons.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Default, Reflect)]
pub enum ButtonSize {
    Small,
    #[default]
    Medium,
    Large,
}

impl ButtonSize {
    pub fn padding(&self) -> USides {
        match self {
            ButtonSize::Small => USides::axes(10.0, 6.0),
            ButtonSize::Medium => USides::axes(16.0, 9.0),
            ButtonSize::Large => USides::axes(22.0, 13.0),
        }
    }

    pub fn font_size(&self) -> f32 {
        match self {
            ButtonSize::Small => 12.0,
            ButtonSize::Medium => 14.0,
            ButtonSize::Large => 16.0,
        }
    }

    pub fn icon_size(&self) -> f32 {
        match self {
            ButtonSize::Small => 14.0,
            ButtonSize::Medium => 16.0,
            ButtonSize::Large => 18.0,
        }
    }

    pub fn radius(&self) -> f32 {
        match self {
            ButtonSize::Small => 6.0,
            ButtonSize::Medium => 8.0,
            ButtonSize::Large => 10.0,
        }
    }

    pub fn min_height(&self) -> f32 {
        match self {
            ButtonSize::Small => 30.0,
            ButtonSize::Medium => 38.0,
            ButtonSize::Large => 46.0,
        }
    }
}

/// Marker and configuration component for a Luma button.
#[derive(Component, Clone, Debug, Reflect)]
pub struct LumaButton {
    pub variant: ButtonVariant,
    pub size: ButtonSize,
    pub disabled: bool,
}

impl Default for LumaButton {
    fn default() -> Self {
        Self {
            variant: ButtonVariant::Primary,
            size: ButtonSize::Medium,
            disabled: false,
        }
    }
}

/// Event triggered when a button is clicked.
#[derive(Event, Clone, Copy, Debug)]
pub struct ButtonClicked(pub Entity);

/// Button colors computed for styling.
pub struct ButtonStyleColors {
    pub bg_normal: Color,
    pub bg_hover: Color,
    pub bg_pressed: Color,
    pub border_color: Color,
    pub border_width: f32,
    pub text_color: Color,
}

impl ButtonVariant {
    pub fn colors(&self) -> ButtonStyleColors {
        match self {
            ButtonVariant::Primary => ButtonStyleColors {
                bg_normal: Color::srgb(0.20, 0.45, 0.90),
                bg_hover: Color::srgb(0.28, 0.55, 1.00),
                bg_pressed: Color::srgb(0.14, 0.35, 0.75),
                border_color: Color::srgb(0.30, 0.55, 1.00),
                border_width: 1.0,
                text_color: Color::WHITE,
            },
            ButtonVariant::Secondary => ButtonStyleColors {
                bg_normal: Color::srgb(0.18, 0.20, 0.27),
                bg_hover: Color::srgb(0.24, 0.27, 0.36),
                bg_pressed: Color::srgb(0.13, 0.15, 0.20),
                border_color: Color::srgb(0.25, 0.28, 0.38),
                border_width: 1.0,
                text_color: Color::srgb(0.95, 0.96, 0.98),
            },
            ButtonVariant::Outline => ButtonStyleColors {
                bg_normal: Color::NONE,
                bg_hover: Color::srgba(1.0, 1.0, 1.0, 0.08),
                bg_pressed: Color::srgba(1.0, 1.0, 1.0, 0.14),
                border_color: Color::srgb(0.35, 0.40, 0.52),
                border_width: 1.0,
                text_color: Color::srgb(0.90, 0.92, 0.96),
            },
            ButtonVariant::Ghost => ButtonStyleColors {
                bg_normal: Color::NONE,
                bg_hover: Color::srgba(1.0, 1.0, 1.0, 0.08),
                bg_pressed: Color::srgba(1.0, 1.0, 1.0, 0.14),
                border_color: Color::NONE,
                border_width: 0.0,
                text_color: Color::srgb(0.90, 0.92, 0.96),
            },
            ButtonVariant::Danger => ButtonStyleColors {
                bg_normal: Color::srgb(0.85, 0.22, 0.28),
                bg_hover: Color::srgb(0.95, 0.30, 0.36),
                bg_pressed: Color::srgb(0.70, 0.16, 0.22),
                border_color: Color::srgb(0.95, 0.35, 0.40),
                border_width: 1.0,
                text_color: Color::WHITE,
            },
        }
    }
}

/// Creates a standard text button scene.
pub fn luma_button(
    label: impl Into<String>,
    variant: ButtonVariant,
    size: ButtonSize,
    font: Handle<Font>,
) -> impl Scene {
    let label = label.into();
    let colors = variant.colors();
    let padding = size.padding();
    let font_size = size.font_size();
    let radius = size.radius();
    let min_height = size.min_height();

    bsn! {
        LumaButton {
            variant,
            size,
            disabled: false,
        }
        UNode {
            min_height,
            background_color: { colors.bg_normal },
            border_radius: { UCornerRadius::all(radius) },
            padding: { padding },
        }
        UBorder {
            color: { colors.border_color },
            width: { colors.border_width },
            radius: { UCornerRadius::all(radius) },
        }
        UInteraction::default()
        UInteractionColors {
            normal: { colors.bg_normal },
            hovered: { colors.bg_hover },
            pressed: { colors.bg_pressed },
        }
        UFocusable::new()
        UFocusVisual::border(Color::srgb(0.35, 0.65, 1.0), 2.0)
        ULayout {
            display: UDisplay::Flex,
            flex_direction: UFlexDirection::Row,
            justify_content: UJustifyContent::Center,
            align_items: UAlignItems::Center,
        }
        Children [
            (UText {
                text: label,
                font_size,
                font,
                color: { colors.text_color },
            })
        ]
    }
}

/// Creates a button scene with a leading icon.
pub fn luma_button_with_icon(
    icon: &str,
    label: impl Into<String>,
    variant: ButtonVariant,
    size: ButtonSize,
    font: Handle<Font>,
    icon_font: Handle<Font>,
) -> impl Scene {
    let label = label.into();
    let icon = icon.to_string();
    let colors = variant.colors();
    let padding = size.padding();
    let font_size = size.font_size();
    let icon_size = size.icon_size();
    let radius = size.radius();
    let min_height = size.min_height();

    bsn! {
        LumaButton {
            variant,
            size,
            disabled: false,
        }
        UNode {
            min_height,
            background_color: { colors.bg_normal },
            border_radius: { UCornerRadius::all(radius) },
            padding: { padding },
        }
        UBorder {
            color: { colors.border_color },
            width: { colors.border_width },
            radius: { UCornerRadius::all(radius) },
        }
        UInteraction::default()
        UInteractionColors {
            normal: { colors.bg_normal },
            hovered: { colors.bg_hover },
            pressed: { colors.bg_pressed },
        }
        UFocusable::new()
        UFocusVisual::border(Color::srgb(0.35, 0.65, 1.0), 2.0)
        ULayout {
            display: UDisplay::Flex,
            flex_direction: UFlexDirection::Row,
            justify_content: UJustifyContent::Center,
            align_items: UAlignItems::Center,
            gap: 8.0,
        }
        Children [
            (UText {
                text: icon,
                font_size: icon_size,
                font: icon_font,
                color: { colors.text_color },
            }),
            (UText {
                text: label,
                font_size,
                font,
                color: { colors.text_color },
            })
        ]
    }
}

/// Creates an icon-only square button scene.
pub fn luma_icon_button(
    icon: &str,
    variant: ButtonVariant,
    size: ButtonSize,
    icon_font: Handle<Font>,
) -> impl Scene {
    let icon = icon.to_string();
    let colors = variant.colors();
    let icon_size = size.icon_size();
    let radius = size.radius();
    let dimension = size.min_height();

    bsn! {
        LumaButton {
            variant,
            size,
            disabled: false,
        }
        UNode {
            width: { UVal::Px(dimension) },
            height: { UVal::Px(dimension) },
            background_color: { colors.bg_normal },
            border_radius: { UCornerRadius::all(radius) },
        }
        UBorder {
            color: { colors.border_color },
            width: { colors.border_width },
            radius: { UCornerRadius::all(radius) },
        }
        UInteraction::default()
        UInteractionColors {
            normal: { colors.bg_normal },
            hovered: { colors.bg_hover },
            pressed: { colors.bg_pressed },
        }
        UFocusable::new()
        UFocusVisual::border(Color::srgb(0.35, 0.65, 1.0), 2.0)
        ULayout {
            display: UDisplay::Flex,
            justify_content: UJustifyContent::Center,
            align_items: UAlignItems::Center,
        }
        Children [
            (UText {
                text: icon,
                font_size: icon_size,
                font: icon_font,
                color: { colors.text_color },
            })
        ]
    }
}

fn on_button_pointer_click(
    trigger: On<Pointer<Click>>,
    mut commands: Commands,
    query: Query<&LumaButton>,
) {
    let entity = trigger.entity.entity();
    if let Ok(button) = query.get(entity) {
        if !button.disabled {
            commands.trigger(ButtonClicked(entity));
        }
    }
}

fn on_button_focus_activate(
    trigger: On<UFocusActivate>,
    mut commands: Commands,
    query: Query<&LumaButton>,
) {
    let entity = trigger.entity;
    if let Ok(button) = query.get(entity) {
        if !button.disabled {
            commands.trigger(ButtonClicked(entity));
        }
    }
}

pub struct LumaButtonPlugin;

impl Plugin for LumaButtonPlugin {
    fn build(&self, app: &mut App) {
        app.register_type::<LumaButton>()
            .register_type::<ButtonVariant>()
            .register_type::<ButtonSize>()
            .add_observer(on_button_pointer_click)
            .add_observer(on_button_focus_activate);
    }
}
