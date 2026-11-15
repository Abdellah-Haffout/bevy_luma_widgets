//! Chip / Tag widget with optional icon, avatar initials, and dismiss button.

use bevy::prelude::*;
use bevy::scene::{ResolveContext, ResolvedScene, ResolveSceneError};
use bevy_luma::prelude::*;

use crate::helpers::{dim_color, find_ancestor_or_self, interaction_colors, is_primary_click};
use crate::theme::{ChipStyle, WidgetColors};

/// Marker and state component for a chip widget.
#[derive(Component, Clone, Debug, Default, Reflect)]
pub struct LumaChip {
    pub label: String,
    pub disabled: bool,
    pub style: ChipStyle,
    pub has_explicit_style: bool,
}

/// Marker component for the label text inside a chip.
#[derive(Component, Clone, Debug, Default, Reflect)]
pub struct LumaChipLabel;

/// Marker component for the dismiss / close button on a removable chip.
#[derive(Component, Clone, Debug, Default, Reflect)]
pub struct LumaChipDismiss;

/// Event triggered when a removable chip is dismissed.
#[derive(Event, Clone, Copy, Debug)]
pub struct ChipDismissed(pub Entity);

const DISMISS_HOVER: Color = Color::srgba(1.0, 1.0, 1.0, 0.15);
const DISMISS_PRESSED: Color = Color::srgba(1.0, 1.0, 1.0, 0.25);

/// Declarative Chip widget struct with all configurable properties.
#[derive(Clone, Debug, Reflect)]
pub struct Chip {
    pub label: String,
    pub icon: Option<String>,
    pub removable: bool,
    pub disabled: bool,
    pub font: Handle<Font>,
    pub icon_font: Handle<Font>,
    pub style: Option<ChipStyle>,
}

impl Default for Chip {
    fn default() -> Self {
        Self {
            label: String::new(),
            icon: None,
            removable: false,
            disabled: false,
            font: Handle::default(),
            icon_font: Handle::default(),
            style: None,
        }
    }
}

impl Chip {
    pub fn new(label: impl Into<String>) -> Self {
        Self {
            label: label.into(),
            ..default()
        }
    }

    pub fn icon(mut self, icon: impl Into<String>) -> Self {
        self.icon = Some(icon.into());
        self
    }

    pub fn removable(mut self, removable: bool) -> Self {
        self.removable = removable;
        self
    }

    pub fn disabled(mut self, disabled: bool) -> Self {
        self.disabled = disabled;
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

    pub fn style(mut self, style: ChipStyle) -> Self {
        self.style = Some(style);
        self
    }
}

impl Scene for Chip {
    fn resolve(
        self,
        context: &mut ResolveContext,
        scene: &mut ResolvedScene,
    ) -> Result<(), ResolveSceneError> {
        let has_explicit_style = self.style.is_some();
        let style = self.style.unwrap_or_default();
        let colors = WidgetColors::default();
        let chip_label = self.label.clone();
        let has_icon = self.icon.is_some();
        let icon_str = self.icon.unwrap_or_default();
        let removable = self.removable;
        let disabled = self.disabled;
        let icon_font = self.icon_font.clone();
        let label = self.label;
        let font = self.font;

        let bg_color = if disabled {
            colors.disabled_bg()
        } else {
            style.bg_color
        };
        let border_color = if disabled {
            colors.disabled_border()
        } else {
            style.border_color
        };
        let text_color = if disabled {
            colors.disabled_text()
        } else {
            style.text_color
        };
        let icon_color = if disabled {
            colors.disabled_text()
        } else {
            colors.primary
        };
        let dismiss_icon_color = if disabled {
            colors.disabled_text()
        } else {
            dim_color(style.text_color, 0.72)
        };
        let dismiss_colors = interaction_colors(
            Color::NONE,
            DISMISS_HOVER,
            DISMISS_PRESSED,
            disabled || !removable,
        );
        let radius = style.radius;
        let border_width = style.border_width;
        let padding = style.padding;
        let font_size = style.font_size;
        let focus_color = colors.focus_ring();
        let luma_chip = LumaChip {
            label: chip_label,
            disabled,
            style,
            has_explicit_style,
        };

        let s = bsn! {
            luma_chip
            UNode {
                height: UVal::Px(26.0),
                background_color: bg_color,
                border_radius: UCornerRadius::all(radius),
                padding,
            }
            UBorder {
                color: border_color,
                width: border_width,
                radius: UCornerRadius::all(radius),
            }
            ULayout {
                display: UDisplay::Flex,
                flex_direction: UFlexDirection::Row,
                align_items: UAlignItems::Center,
                gap: 5.0,
            }
            Children [
                UNode::default()
                ULayout {
                    display: { if has_icon { UDisplay::Flex } else { UDisplay::None } },
                }
                Children [
                    UText {
                        text: icon_str,
                        font_size: 13.0,
                        font: { icon_font.clone() },
                        color: icon_color,
                    }
                ]
                --
                LumaChipLabel
                UText {
                    text: label,
                    font_size,
                    font,
                    color: text_color,
                }
                --
                UNode::default()
                ULayout {
                    display: { if removable { UDisplay::Flex } else { UDisplay::None } },
                }
                Children [
                    LumaChipDismiss
                    UNode {
                        width: UVal::Px(16.0),
                        height: UVal::Px(16.0),
                        border_radius: UCornerRadius::all(8.0),
                    }
                    UInteraction::default()
                    dismiss_colors
                    UFocusable {
                        enabled: { removable && !disabled },
                        ..UFocusable::default()
                    }
                    UFocusVisual::border(focus_color, 1.0)
                    ULayout {
                        display: UDisplay::Flex,
                        justify_content: UJustifyContent::Center,
                        align_items: UAlignItems::Center,
                    }
                    Children [
                        UText {
                            text: { Icon::X.to_string() },
                            font_size: 11.0,
                            font: icon_font,
                            color: dismiss_icon_color,
                        }
                    ]
                ]
            ]
        };

        s.resolve(context, scene)
    }
}

/// Creates a standard text chip scene.
pub fn luma_chip(
    label: impl Into<String>,
    font: Handle<Font>,
) -> Chip {
    Chip::new(label).font(font)
}

/// Creates a chip scene with a leading icon.
pub fn luma_chip_with_icon(
    icon: &str,
    label: impl Into<String>,
    font: Handle<Font>,
    icon_font: Handle<Font>,
) -> Chip {
    Chip::new(label).icon(icon).font(font).icon_font(icon_font)
}

/// Creates a removable chip scene with a trailing dismiss [x] button.
pub fn luma_removable_chip(
    label: impl Into<String>,
    font: Handle<Font>,
    icon_font: Handle<Font>,
) -> Chip {
    Chip::new(label).removable(true).font(font).icon_font(icon_font)
}

/// Creates a removable chip scene with both a leading icon and a dismiss button.
pub fn luma_removable_chip_with_icon(
    icon: &str,
    label: impl Into<String>,
    font: Handle<Font>,
    icon_font: Handle<Font>,
) -> Chip {
    Chip::new(label)
        .icon(icon)
        .removable(true)
        .font(font)
        .icon_font(icon_font)
}

fn dismiss_chip(
    target_entity: Entity,
    commands: &mut Commands,
    parents_query: &Query<&ChildOf>,
    dismiss_query: &Query<&LumaChipDismiss>,
    chip_query: &Query<&LumaChip>,
) {
    let Some(dismiss_entity) = find_ancestor_or_self(target_entity, dismiss_query, parents_query) else {
        return;
    };
    let Some(chip_entity) = find_ancestor_or_self(dismiss_entity, chip_query, parents_query) else {
        return;
    };
    let Ok(chip) = chip_query.get(chip_entity) else {
        return;
    };
    if chip.disabled {
        return;
    }

    commands.trigger(ChipDismissed(chip_entity));
    if let Ok(mut cmd) = commands.get_entity(chip_entity) {
        cmd.try_despawn();
    }
}

fn on_chip_dismiss_click(
    trigger: On<PointerClick>,
    mut commands: Commands,
    parents_query: Query<&ChildOf>,
    dismiss_query: Query<&LumaChipDismiss>,
    chip_query: Query<&LumaChip>,
) {
    if !is_primary_click(&trigger) {
        return;
    }
    dismiss_chip(
        trigger.entity,
        &mut commands,
        &parents_query,
        &dismiss_query,
        &chip_query,
    );
}

fn on_chip_dismiss_activate(
    trigger: On<UFocusActivate>,
    mut commands: Commands,
    parents_query: Query<&ChildOf>,
    dismiss_query: Query<&LumaChipDismiss>,
    chip_query: Query<&LumaChip>,
) {
    dismiss_chip(
        trigger.entity,
        &mut commands,
        &parents_query,
        &dismiss_query,
        &chip_query,
    );
}

pub struct LumaChipPlugin;

impl Plugin for LumaChipPlugin {
    fn build(&self, app: &mut App) {
        app.register_type::<LumaChip>()
            .register_type::<LumaChipLabel>()
            .register_type::<LumaChipDismiss>()
            .add_observer(on_chip_dismiss_click)
            .add_observer(on_chip_dismiss_activate);
    }
}
