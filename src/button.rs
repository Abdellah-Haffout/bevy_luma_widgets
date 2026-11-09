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
            ButtonSize::Small => USides::axes(10.0, 5.0),
            ButtonSize::Medium => USides::axes(14.0, 8.0),
            ButtonSize::Large => USides::axes(20.0, 11.0),
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
            ButtonSize::Medium => 15.0,
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
            ButtonSize::Small => 28.0,
            ButtonSize::Medium => 34.0,
            ButtonSize::Large => 42.0,
        }
    }
}

/// Marker and configuration component for a Luma button.
#[derive(Component, Clone, Debug, Reflect)]
pub struct LumaButton {
    pub variant: ButtonVariant,
    pub size: ButtonSize,
    pub disabled: bool,
    pub has_explicit_style: bool,
}

impl Default for LumaButton {
    fn default() -> Self {
        Self {
            variant: ButtonVariant::Primary,
            size: ButtonSize::Medium,
            disabled: false,
            has_explicit_style: false,
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
                bg_normal: Color::srgb_u8(0x1F, 0xD6, 0x9A),
                bg_hover: Color::srgb_u8(0x3D, 0xE2, 0xAC),
                bg_pressed: Color::srgb_u8(0x17, 0xB8, 0x84),
                border_color: Color::srgb_u8(0x1F, 0xD6, 0x9A),
                border_width: 1.0,
                text_color: Color::srgb_u8(0x06, 0x22, 0x18),
            },
            ButtonVariant::Secondary => ButtonStyleColors {
                bg_normal: Color::srgb_u8(0x18, 0x18, 0x1B),
                bg_hover: Color::srgb_u8(0x24, 0x24, 0x28),
                bg_pressed: Color::srgb_u8(0x2C, 0x2C, 0x31),
                border_color: Color::srgb_u8(0x2B, 0x2B, 0x30),
                border_width: 1.0,
                text_color: Color::srgb_u8(0xEC, 0xEC, 0xEE),
            },
            ButtonVariant::Outline => ButtonStyleColors {
                bg_normal: Color::srgb_u8(0x12, 0x12, 0x14),
                bg_hover: Color::srgb_u8(0x24, 0x24, 0x28),
                bg_pressed: Color::srgb_u8(0x2C, 0x2C, 0x31),
                border_color: Color::srgb_u8(0x2B, 0x2B, 0x30),
                border_width: 1.0,
                text_color: Color::srgb_u8(0xEC, 0xEC, 0xEE),
            },
            ButtonVariant::Ghost => ButtonStyleColors {
                bg_normal: Color::NONE,
                bg_hover: Color::srgba(1.0, 1.0, 1.0, 0.06),
                bg_pressed: Color::srgba(1.0, 1.0, 1.0, 0.12),
                border_color: Color::NONE,
                border_width: 0.0,
                text_color: Color::srgb_u8(0xEC, 0xEC, 0xEE),
            },
            ButtonVariant::Danger => ButtonStyleColors {
                bg_normal: Color::srgb_u8(0x3B, 0x12, 0x13),
                bg_hover: Color::srgb_u8(0x4A, 0x17, 0x18),
                bg_pressed: Color::srgb_u8(0x30, 0x0E, 0x0F),
                border_color: Color::srgb_u8(0x52, 0x1C, 0x1D),
                border_width: 1.0,
                text_color: Color::srgb_u8(0xF4, 0x6B, 0x6B),
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
    pub loading: bool,
    pub icon_only: bool,
    pub fab: bool,
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
            loading: false,
            icon_only: false,
            fab: false,
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

    pub fn loading(mut self, loading: bool) -> Self {
        self.loading = loading;
        self
    }

    pub fn icon_only(mut self, icon: impl Into<String>) -> Self {
        self.icon = Some(icon.into());
        self.icon_only = true;
        self
    }

    pub fn fab(mut self, icon: impl Into<String>) -> Self {
        self.icon = Some(icon.into());
        self.fab = true;
        self.icon_only = true;
        self
    }

    pub fn icon_button(icon: impl Into<String>) -> Self {
        Self::default().icon_only(icon)
    }

    pub fn fab_button(icon: impl Into<String>) -> Self {
        Self::default().fab(icon)
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

        let (radius, padding, min_height, width) = if self.fab {
            (28.0, USides::all(0.0), 56.0, UVal::Px(56.0))
        } else if self.icon_only {
            (radius, USides::all(0.0), min_height, UVal::Px(min_height))
        } else {
            (radius, padding, min_height, UVal::Auto)
        };

        let has_explicit_style = self.style.is_some();
        let icon_size = self.size.icon_size();
        let icon_text = self.icon.clone().unwrap_or_default();
        let has_icon = self.icon.is_some() && !self.loading;
        let has_label = !self.label.is_empty();
        let loading = self.loading;
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
                has_explicit_style,
            }
            UNode {
                width,
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
                UNode::default()
                ULayout {
                    display: { if loading { UDisplay::Flex } else { UDisplay::None } },
                }
                Children [
                    @{
                        crate::spinner::Spinner::new()
                            .size(icon_size)
                            .thickness(2.0)
                            .color(text_color)
                    }
                ]
                --
                UNode::default()
                ULayout {
                    display: { if has_icon { UDisplay::Flex } else { UDisplay::None } },
                }
                Children [
                    UText {
                        text: icon_text,
                        font_size: icon_size,
                        font: icon_font,
                        color: text_color,
                    }
                ]
                --
                UNode::default()
                ULayout {
                    display: { if has_label { UDisplay::Flex } else { UDisplay::None } },
                }
                Children [
                    UText {
                        text: label,
                        font_size,
                        font,
                        color: text_color,
                    }
                ]
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
    trigger: On<PointerClick>,
    mut commands: Commands,
    query: Query<&LumaButton>,
    parents_query: Query<&ChildOf>,
) {
    if trigger.event().button != PointerButton::Primary {
        return;
    }
    let target = if query.contains(trigger.entity) {
        Some(trigger.entity)
    } else {
        parents_query.iter_ancestors(trigger.entity).find(|&e| query.contains(e))
    };
    if let Some(entity) = target {
        if let Ok(button) = query.get(entity) {
            if !button.disabled {
                commands.trigger(ButtonClicked(entity));
            }
        }
    }
}

fn on_button_focus_activate(
    trigger: On<UFocusActivate>,
    mut commands: Commands,
    query: Query<&LumaButton>,
    parents_query: Query<&ChildOf>,
) {
    let target = if query.contains(trigger.entity) {
        Some(trigger.entity)
    } else {
        parents_query.iter_ancestors(trigger.entity).find(|&e| query.contains(e))
    };
    if let Some(entity) = target {
        if let Ok(button) = query.get(entity) {
            if !button.disabled {
                commands.trigger(ButtonClicked(entity));
            }
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
