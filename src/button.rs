//! Interactive button widget with variants, sizes, styles, and icon support.

use bevy::prelude::*;
use bevy::scene::{ResolveContext, ResolvedScene, ResolveSceneError};
use bevy_luma::prelude::*;

use crate::helpers::{ConditionalScene, find_ancestor_or_self, interaction_colors, is_primary_click};
use crate::theme::{ButtonStyle, WidgetColors};

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
    pub loading: bool,
    pub is_icon_only: bool,
    pub has_explicit_size: bool,
    pub has_explicit_style: bool,
}

impl Default for LumaButton {
    fn default() -> Self {
        Self {
            variant: ButtonVariant::Primary,
            size: ButtonSize::Medium,
            disabled: false,
            loading: false,
            is_icon_only: false,
            has_explicit_size: false,
            has_explicit_style: false,
        }
    }
}

/// Marker component on the label text entity inside a [`Button`].
#[derive(Component, Clone, Copy, Debug, Default, Reflect)]
pub struct LumaButtonLabel;

/// Marker component on the leading/icon-only text entity inside a [`Button`].
#[derive(Component, Clone, Copy, Debug, Default, Reflect)]
pub struct LumaButtonIcon;

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

impl From<ButtonStyle> for ButtonStyleColors {
    fn from(style: ButtonStyle) -> Self {
        Self {
            bg_normal: style.bg_normal,
            bg_hover: style.bg_hover,
            bg_pressed: style.bg_pressed,
            border_color: style.border_color,
            border_width: style.border_width,
            text_color: style.text_color,
        }
    }
}

impl ButtonVariant {
    pub fn colors(&self) -> ButtonStyleColors {
        match self {
            ButtonVariant::Primary => ButtonStyle::primary().into(),
            ButtonVariant::Secondary => ButtonStyle::secondary().into(),
            ButtonVariant::Outline => ButtonStyle::outline().into(),
            ButtonVariant::Ghost => ButtonStyle::ghost().into(),
            ButtonVariant::Danger => ButtonStyle::danger().into(),
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
        let has_explicit_size = self.size != ButtonSize::Medium;
        let icon_size = self.size.icon_size();
        let has_icon = self.icon.is_some() && !self.loading;
        let icon_text = self.icon.clone().unwrap_or_default();
        let has_label = !self.label.is_empty();
        let loading = self.loading;
        let variant = self.variant;
        let size = self.size;
        let disabled = self.disabled;
        let is_icon_only = self.icon_only || self.fab;
        let icon_font = self.icon_font;
        let label = self.label;
        let font = self.font;

        let theme_colors = WidgetColors::default();
        let (effective_bg, effective_border, effective_fg) = if disabled {
            let bg = if variant == ButtonVariant::Ghost {
                Color::NONE
            } else {
                theme_colors.disabled_bg()
            };
            let border = if variant == ButtonVariant::Ghost {
                Color::NONE
            } else {
                theme_colors.disabled_border()
            };
            (bg, border, theme_colors.disabled_text())
        } else {
            (bg_normal, border_color, text_color)
        };

        let i_colors = interaction_colors(
            effective_bg,
            bg_hover,
            bg_pressed,
            disabled || loading,
        );
        let focus_color = theme_colors.focus_ring();

        let spinner_child = ConditionalScene(loading.then(|| {
            crate::spinner::Spinner::new()
                .size(icon_size)
                .thickness(2.0)
                .color(effective_fg)
        }));
        let icon_child = ConditionalScene(has_icon.then(|| {
            bsn! {
                LumaButtonIcon
                UText {
                    text: icon_text,
                    font_size: icon_size,
                    font: icon_font,
                    color: effective_fg,
                }
            }
        }));
        let label_child = ConditionalScene(has_label.then(|| {
            bsn! {
                LumaButtonLabel
                UText {
                    text: label,
                    font_size,
                    font,
                    color: effective_fg,
                }
            }
        }));

        let s = bsn! {
            LumaButton {
                variant,
                size,
                disabled,
                loading,
                is_icon_only,
                has_explicit_size,
                has_explicit_style,
            }
            UNode {
                width,
                min_height,
                background_color: effective_bg,
                border_radius: UCornerRadius::all(radius),
                padding,
            }
            USelf {
                flex_shrink: { Some(0.0) },
            }
            UBorder {
                color: effective_border,
                width: border_width,
                radius: UCornerRadius::all(radius),
            }
            UInteraction::default()
            i_colors
            UFocusable {
                enabled: { !disabled },
                ..UFocusable::new()
            }
            UFocusVisual::border(focus_color, 2.0)
            ULayout {
                display: UDisplay::Flex,
                flex_direction: UFlexDirection::Row,
                justify_content: UJustifyContent::Center,
                align_items: UAlignItems::Center,
                gap: 8.0,
            }
            Children [
                @{spinner_child}
                --
                @{icon_child}
                --
                @{label_child}
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
        .icon_only(icon)
        .variant(variant)
        .size(size)
        .icon_font(icon_font)
}

fn trigger_button_if_active(entity: Entity, query: &Query<&LumaButton>, commands: &mut Commands) {
    if let Ok(button) = query.get(entity) {
        if !button.disabled && !button.loading {
            commands.trigger(ButtonClicked(entity));
        }
    }
}

/// Applies `variant` visual colors onto a spawned [`Button`] entity and its label/icon children.
pub(crate) fn apply_button_variant_visuals(
    variant: ButtonVariant,
    luma_btn: &mut LumaButton,
    node: &mut UNode,
    border: &mut UBorder,
    interaction_colors_comp: &mut UInteractionColors,
    children: Option<&Children>,
    children_query: &Query<&Children>,
    text_query: &mut Query<&mut UText>,
    label_or_icon_query: &Query<(), Or<(With<LumaButtonLabel>, With<LumaButtonIcon>)>>,
) {
    luma_btn.variant = variant;
    let c = variant.colors();
    let theme_colors = WidgetColors::default();
    let (bg, brd, fg) = if luma_btn.disabled {
        let bg = if variant == ButtonVariant::Ghost {
            Color::NONE
        } else {
            theme_colors.disabled_bg()
        };
        let brd = if variant == ButtonVariant::Ghost {
            Color::NONE
        } else {
            theme_colors.disabled_border()
        };
        (bg, brd, theme_colors.disabled_text())
    } else {
        (c.bg_normal, c.border_color, c.text_color)
    };

    node.background_color = bg;
    border.color = brd;
    border.width = c.border_width;
    *interaction_colors_comp = interaction_colors(
        bg,
        c.bg_hover,
        c.bg_pressed,
        luma_btn.disabled || luma_btn.loading,
    );

    if let Some(children) = children {
        for &child in children {
            crate::helpers::visit_descendants(child, children_query, |e| {
                if label_or_icon_query.contains(e) {
                    if let Ok(mut txt) = text_query.get_mut(e) {
                        txt.color = fg;
                    }
                }
                crate::helpers::DescendantVisit::Continue
            });
        }
    }
}

fn on_button_pointer_click(
    trigger: On<PointerClick>,
    mut commands: Commands,
    query: Query<&LumaButton>,
    parents_query: Query<&ChildOf>,
) {
    if !is_primary_click(&trigger) {
        return;
    }
    if let Some(entity) = find_ancestor_or_self(trigger.entity, &query, &parents_query) {
        trigger_button_if_active(entity, &query, &mut commands);
    }
}

fn on_button_focus_activate(
    trigger: On<UFocusActivate>,
    mut commands: Commands,
    query: Query<&LumaButton>,
    parents_query: Query<&ChildOf>,
) {
    if let Some(entity) = find_ancestor_or_self(trigger.entity, &query, &parents_query) {
        trigger_button_if_active(entity, &query, &mut commands);
    }
}

pub struct LumaButtonPlugin;

impl Plugin for LumaButtonPlugin {
    fn build(&self, app: &mut App) {
        app.register_type::<LumaButton>()
            .register_type::<LumaButtonLabel>()
            .register_type::<LumaButtonIcon>()
            .register_type::<ButtonVariant>()
            .register_type::<ButtonSize>()
            .add_observer(on_button_pointer_click)
            .add_observer(on_button_focus_activate);
    }
}
