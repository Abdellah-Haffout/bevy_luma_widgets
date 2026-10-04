//! Radio button widget with grouped single-choice selection.

use bevy::prelude::*;
use bevy_luma::prelude::*;

/// Marker and state component for a radio button.
#[derive(Component, Clone, Debug, Reflect)]
pub struct LumaRadio {
    pub group: String,
    pub value: String,
    pub selected: bool,
    pub disabled: bool,
}

impl Default for LumaRadio {
    fn default() -> Self {
        Self {
            group: String::new(),
            value: String::new(),
            selected: false,
            disabled: false,
        }
    }
}

/// Marker component for the internal radio indicator dot.
#[derive(Component, Clone, Debug, Default, Reflect)]
pub struct LumaRadioDot;

/// Event triggered when a radio button in a group is selected.
#[derive(Event, Clone, Debug)]
pub struct RadioChanged {
    pub group: String,
    pub value: String,
    pub entity: Entity,
}

const SELECTED_COLOR: Color = Color::srgb(0.20, 0.45, 0.90);
const UNSELECTED_BORDER: Color = Color::srgb(0.30, 0.35, 0.45);
const CIRCLE_BG: Color = Color::srgba(0.12, 0.15, 0.20, 0.8);

/// Creates an isolated circular radio button scene.
pub fn luma_radio_circle(
    group: impl Into<String>,
    value: impl Into<String>,
    selected: bool,
) -> impl Scene {
    let border_color = if selected { SELECTED_COLOR } else { UNSELECTED_BORDER };
    let dot_color = if selected { SELECTED_COLOR } else { Color::NONE };
    let group = group.into();
    let value = value.into();

    bsn! {
        LumaRadio {
            group,
            value,
            selected,
            disabled: false,
        }
        UNode {
            width: UVal::Px(20.0),
            height: UVal::Px(20.0),
            background_color: CIRCLE_BG,
            border_radius: UCornerRadius::all(10.0),
        }
        UBorder {
            color: border_color,
            width: 1.5,
            radius: UCornerRadius::all(10.0),
        }
        UInteraction::default()
        UFocusable::new()
        UFocusVisual::border(Color::srgb(0.35, 0.65, 1.0), 2.0)
        ULayout {
            display: UDisplay::Flex,
            justify_content: UJustifyContent::Center,
            align_items: UAlignItems::Center,
        }
        Children [
            (
                LumaRadioDot
                UNode {
                    width: UVal::Px(10.0),
                    height: UVal::Px(10.0),
                    background_color: dot_color,
                    border_radius: UCornerRadius::all(5.0),
                }
            )
        ]
    }
}

/// Creates a radio button with an adjacent text label.
pub fn luma_radio(
    group: impl Into<String>,
    value: impl Into<String>,
    label: impl Into<String>,
    selected: bool,
    font: Handle<Font>,
) -> impl Scene {
    let label = label.into();

    bsn! {
        UNode::default()
        ULayout {
            display: UDisplay::Flex,
            flex_direction: UFlexDirection::Row,
            align_items: UAlignItems::Center,
            gap: 10.0,
        }
        Children [
            (
                luma_radio_circle(group, value, selected)
            ),
            (
                UText {
                    text: label,
                    font_size: 14.0,
                    font,
                    color: Color::srgb(0.92, 0.94, 0.98),
                }
            )
        ]
    }
}

fn on_radio_click(
    trigger: On<Pointer<Click>>,
    mut commands: Commands,
    mut all_radios: Query<(Entity, &mut LumaRadio, &mut UBorder, Option<&Children>)>,
    mut dot_query: Query<&mut UNode, With<LumaRadioDot>>,
) {
    let clicked_entity = trigger.entity.entity();

    // Check if clicked entity is a radio
    let Ok((_, clicked_radio, _, _)) = all_radios.get(clicked_entity) else {
        return;
    };

    if clicked_radio.disabled {
        return;
    }

    let target_group = clicked_radio.group.clone();
    let selected_value = clicked_radio.value.clone();

    // Synchronize the entire group: select clicked, deselect others in same group
    for (entity, mut radio, mut border, children) in all_radios.iter_mut() {
        if radio.group == target_group {
            let is_now_selected = entity == clicked_entity;
            radio.selected = is_now_selected;

            border.color = if is_now_selected { SELECTED_COLOR } else { UNSELECTED_BORDER };

            if let Some(children) = children {
                for child in children.iter() {
                    if let Ok(mut dot) = dot_query.get_mut(child) {
                        dot.background_color = if is_now_selected { SELECTED_COLOR } else { Color::NONE };
                    }
                }
            }
        }
    }

    commands.trigger(RadioChanged {
        group: target_group,
        value: selected_value,
        entity: clicked_entity,
    });
}

pub struct LumaRadioPlugin;

impl Plugin for LumaRadioPlugin {
    fn build(&self, app: &mut App) {
        app.register_type::<LumaRadio>()
            .register_type::<LumaRadioDot>()
            .add_observer(on_radio_click);
    }
}
