//! Interactive button widget with variants, sizes, styles, and icon support.

use bevy::picking::pointer::PointerButton;
use bevy::prelude::*;
use bevy::scene::{ResolveContext, ResolvedScene, ResolveSceneError};
use bevy_luma::prelude::*;

use crate::theme::ButtonStyle;

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

/// Declarative Button widget struct with all configurable properties.
#[derive(Clone, Debug, Reflect)]
pub struct Button {
    pub label: String,
    pub variant: ButtonVariant,
    pub size: ButtonSize,
    pub icon: Option<String>,
    pub font: Handle<Font>,
    pub icon_font: Handle<Font>,
    pub style: Option<ButtonStyle>,
    pub disabled: bool,
}

impl Default for Button {
    fn default() -> Self {
        Self {
            label: String::new(),
            variant: ButtonVariant::Primary,
            size: ButtonSize::Medium,
            icon: None,
            font: Handle::default(),
            icon_font: Handle::default(),
            style: None,
            disabled: false,
        }
    }
}

impl Button {
    pub fn new(label: impl Into<String>) -> Self {
        Self {
            label: label.into(),
            ..default()
        }
    }

    pub fn variant(mut self, variant: ButtonVariant) -> Self {
        self.variant = variant;
        self
    }

    pub fn primary(self) -> Self {
        self.variant(ButtonVariant::Primary)
    }

    pub fn secondary(self) -> Self {
        self.variant(ButtonVariant::Secondary)
    }

    pub fn outline(self) -> Self {
        self.variant(ButtonVariant::Outline)
    }

    pub fn ghost(self) -> Self {
        self.variant(ButtonVariant::Ghost)
    }

    pub fn danger(self) -> Self {
        self.variant(ButtonVariant::Danger)
    }

    pub fn size(mut self, size: ButtonSize) -> Self {
        self.size = size;
        self
    }

    pub fn small(self) -> Self {
        self.size(ButtonSize::Small)
    }

    pub fn medium(self) -> Self {
        self.size(ButtonSize::Medium)
    }

    pub fn large(self) -> Self {
        self.size(ButtonSize::Large)
    }

    pub fn icon(mut self, icon: impl Into<String>) -> Self {
        self.icon = Some(icon.into());
        self
    }

    pub fn font(mut self, font: Handle<Font>) -> Self {
        self.font = font;
        self
    }

    pub fn icon_font(mut self, icon_font: Handle<Font>) -> Self {
        self.icon_font = icon_font;
        self
    }

    pub fn style(mut self, style: ButtonStyle) -> Self {
        self.style = Some(style);
        self
    }

    pub fn disabled(mut self, disabled: bool) -> Self {
        self.disabled = disabled;
        self
    }
}

impl Scene for Button {
    fn resolve(
        self,
        context: &mut ResolveContext,
        scene: &mut ResolvedScene,
    ) -> Result<(), ResolveSceneError> {
        let (bg_normal, bg_hover, bg_pressed, border_color, border_width, text_color, radius, padding, min_height, font_size) =
            if let Some(ref s) = self.style {
                (
                    s.bg_normal,
                    s.bg_hover,
                    s.bg_pressed,
                    s.border_color,
                    s.border_width,
                    s.text_color,
                    s.radius,
                    s.padding,
                    s.min_height,
                    s.font_size,
                )
            } else {
                let c = self.variant.colors();
                (
                    c.bg_normal,
                    c.bg_hover,
                    c.bg_pressed,
                    c.border_color,
                    c.border_width,
                    c.text_color,
                    self.size.radius(),
                    self.size.padding(),
                    self.size.min_height(),
                    self.size.font_size(),
                )
            };

        let icon_size = self.size.icon_size();
        let icon_text = self.icon.clone().unwrap_or_default();
        let has_icon = self.icon.is_some();
        let has_label = !self.label.is_empty();
        let variant = self.variant;
        let size = self.size;
        let disabled = self.disabled;
        let icon_font = self.icon_font;
        let label = self.label;
        let font = self.font;

        let s = bsn! {
            LumaButton {
                variant,
                size,
                disabled,
            }
            UNode {
                min_height,
                background_color: bg_normal,
                border_radius: UCornerRadius::all(radius),
                padding,
            }
            UBorder {
                color: border_color,
                width: border_width,
                radius: UCornerRadius::all(radius),
            }
            UInteraction::default()
            UInteractionColors {
                normal: bg_normal,
                hovered: bg_hover,
                pressed: bg_pressed,
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
                (
                    UNode::default()
                    ULayout {
                        display: { if has_icon { UDisplay::Flex } else { UDisplay::None } },
                    }
                    Children [
                        (UText {
                            text: icon_text,
                            font_size: icon_size,
                            font: icon_font,
                            color: text_color,
                        })
                    ]
                ),
                (
                    UNode::default()
                    ULayout {
                        display: { if has_label { UDisplay::Flex } else { UDisplay::None } },
                    }
                    Children [
                        (UText {
                            text: label,
                            font_size,
                            font,
                            color: text_color,
                        })
                    ]
                )
            ]
        };

        s.resolve(context, scene)
    }
}

/// Creates a standard text button scene.
pub fn luma_button(
    label: impl Into<String>,
    variant: ButtonVariant,
    size: ButtonSize,
    font: Handle<Font>,
) -> Button {
    Button::new(label).variant(variant).size(size).font(font)
}

/// Creates a button scene with a leading icon.
pub fn luma_button_with_icon(
    icon: &str,
    label: impl Into<String>,
    variant: ButtonVariant,
    size: ButtonSize,
    font: Handle<Font>,
    icon_font: Handle<Font>,
) -> Button {
    Button::new(label)
        .icon(icon)
        .variant(variant)
        .size(size)
        .font(font)
        .icon_font(icon_font)
}

/// Creates an icon-only button scene.
pub fn luma_icon_button(
    icon: &str,
    variant: ButtonVariant,
    size: ButtonSize,
    icon_font: Handle<Font>,
) -> Button {
    Button::default()
        .icon(icon)
        .variant(variant)
        .size(size)
        .icon_font(icon_font)
}

fn on_button_pointer_click(
    trigger: On<Pointer<Click>>,
    mut commands: Commands,
    query: Query<&LumaButton>,
) {
    if trigger.event().button != PointerButton::Primary {
        return;
    }
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
