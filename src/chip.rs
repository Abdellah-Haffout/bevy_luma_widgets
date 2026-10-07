//! Chip / Tag widget with optional icon, avatar initials, and dismiss button.

use bevy::ecs::relationship::Relationship;
use bevy::prelude::*;
use bevy::scene::{ResolveContext, ResolvedScene, ResolveSceneError};
use bevy_luma::prelude::*;

/// Marker and state component for a chip widget.
#[derive(Component, Clone, Debug, Default, Reflect)]
pub struct LumaChip {
    pub label: String,
    pub disabled: bool,
}

/// Marker component for the dismiss / close button on a removable chip.
#[derive(Component, Clone, Debug, Default, Reflect)]
pub struct LumaChipDismiss;

/// Event triggered when a removable chip is dismissed.
#[derive(Event, Clone, Copy, Debug)]
pub struct ChipDismissed(pub Entity);

const CHIP_BG: Color = Color::srgb(0.14, 0.17, 0.23);
const CHIP_BORDER: Color = Color::srgb(0.24, 0.29, 0.40);
const CHIP_TEXT: Color = Color::srgb(0.90, 0.93, 0.98);
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
}

impl Scene for Chip {
    fn resolve(
        self,
        context: &mut ResolveContext,
        scene: &mut ResolvedScene,
    ) -> Result<(), ResolveSceneError> {
        let chip_label = self.label.clone();
        let has_icon = self.icon.is_some();
        let icon_str = self.icon.unwrap_or_default();
        let removable = self.removable;
        let disabled = self.disabled;
        let icon_font = self.icon_font.clone();
        let label = self.label;
        let font = self.font;

        let s = bsn! {
            LumaChip {
                label: chip_label,
                disabled,
            }
            UNode {
                height: UVal::Px(26.0),
                background_color: CHIP_BG,
                border_radius: UCornerRadius::all(13.0),
                padding: USides::axes(8.0, 2.0),
            }
            UBorder {
                color: CHIP_BORDER,
                width: 1.0,
                radius: UCornerRadius::all(13.0),
            }
            ULayout {
                display: UDisplay::Flex,
                flex_direction: UFlexDirection::Row,
                align_items: UAlignItems::Center,
                gap: 5.0,
            }
            Children [
                (
                    UNode::default()
                    ULayout {
                        display: { if has_icon { UDisplay::Flex } else { UDisplay::None } },
                    }
                    Children [
                        (UText {
                            text: icon_str,
                            font_size: 13.0,
                            font: { icon_font.clone() },
                            color: Color::srgb(0.45, 0.70, 1.00),
                        })
                    ]
                ),
                (UText {
                    text: label,
                    font_size: 12.0,
                    font,
                    color: CHIP_TEXT,
                }),
                (
                    UNode::default()
                    ULayout {
                        display: { if removable { UDisplay::Flex } else { UDisplay::None } },
                    }
                    Children [
                        (
                            LumaChipDismiss
                            UNode {
                                width: UVal::Px(16.0),
                                height: UVal::Px(16.0),
                                border_radius: UCornerRadius::all(8.0),
                            }
                            UInteraction::default()
                            UInteractionColors {
                                normal: Color::NONE,
                                hovered: DISMISS_HOVER,
                                pressed: DISMISS_PRESSED,
                            }
                            UFocusable::new()
                            UFocusVisual::border(Color::srgb(0.35, 0.65, 1.0), 1.0)
                            ULayout {
                                display: UDisplay::Flex,
                                justify_content: UJustifyContent::Center,
                                align_items: UAlignItems::Center,
                            }
                            Children [
                                (UText {
                                    text: { Icon::X.to_string() },
                                    font_size: 11.0,
                                    font: icon_font,
                                    color: Color::srgb(0.65, 0.70, 0.80),
                                })
                            ]
                        )
                    ]
                )
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
    let dismiss_entity = if dismiss_query.contains(target_entity) {
        target_entity
    } else if let Ok(parent) = parents_query.get(target_entity) {
        if dismiss_query.contains(parent.get()) {
            parent.get()
        } else {
            return;
        }
    } else {
        return;
    };

    let mut current = dismiss_entity;
    while let Ok(parent) = parents_query.get(current) {
        current = parent.get();
        if chip_query.contains(current) {
            commands.trigger(ChipDismissed(current));
            commands.entity(current).despawn();
            break;
        }
    }
}

fn on_chip_dismiss_click(
    trigger: On<Pointer<Click>>,
    mut commands: Commands,
    parents_query: Query<&ChildOf>,
    dismiss_query: Query<&LumaChipDismiss>,
    chip_query: Query<&LumaChip>,
) {
    dismiss_chip(
        trigger.entity.entity(),
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
            .register_type::<LumaChipDismiss>()
            .add_observer(on_chip_dismiss_click)
            .add_observer(on_chip_dismiss_activate);
    }
}
