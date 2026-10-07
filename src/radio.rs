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

use bevy::scene::{ResolveContext, ResolvedScene, ResolveSceneError};

/// Declarative Radio button widget struct with all configurable properties.
#[derive(Clone, Debug, Reflect)]
pub struct Radio {
    pub group: String,
    pub value: String,
    pub label: Option<String>,
    pub selected: bool,
    pub disabled: bool,
    pub font: Handle<Font>,
}

impl Default for Radio {
    fn default() -> Self {
        Self {
            group: String::new(),
            value: String::new(),
            label: None,
            selected: false,
            disabled: false,
            font: Handle::default(),
        }
    }
}

impl Radio {
    pub fn new(group: impl Into<String>, value: impl Into<String>) -> Self {
        Self {
            group: group.into(),
            value: value.into(),
            ..default()
        }
    }

    pub fn label(mut self, label: impl Into<String>) -> Self {
        self.label = Some(label.into());
        self
    }

    pub fn selected(mut self, selected: bool) -> Self {
        self.selected = selected;
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
}

impl Scene for Radio {
    fn resolve(
        self,
        context: &mut ResolveContext,
        scene: &mut ResolvedScene,
    ) -> Result<(), ResolveSceneError> {
        let border_color = if self.selected { SELECTED_COLOR } else { UNSELECTED_BORDER };
        let dot_color = if self.selected { SELECTED_COLOR } else { Color::NONE };
        let group = self.group;
        let value = self.value;
        let selected = self.selected;
        let disabled = self.disabled;

        let circle_node = bsn! {
            LumaRadio {
                group,
                value,
                selected,
                disabled,
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
        };

        if let Some(label_str) = self.label {
            let font = self.font;
            let container = bsn! {
                UNode::default()
                ULayout {
                    display: UDisplay::Flex,
                    flex_direction: UFlexDirection::Row,
                    align_items: UAlignItems::Center,
                    gap: 10.0,
                }
                Children [
                    circle_node,
                    (UText {
                        text: label_str,
                        font_size: 14.0,
                        font,
                        color: Color::srgb(0.92, 0.94, 0.98),
                    })
                ]
            };
            container.resolve(context, scene)
        } else {
            circle_node.resolve(context, scene)
        }
    }
}

/// Creates an isolated circular radio button scene.
pub fn luma_radio_circle(
    group: impl Into<String>,
    value: impl Into<String>,
    selected: bool,
) -> Radio {
    Radio::new(group, value).selected(selected)
}

/// Creates a radio button with an adjacent text label.
pub fn luma_radio(
    group: impl Into<String>,
    value: impl Into<String>,
    label: impl Into<String>,
    selected: bool,
    font: Handle<Font>,
) -> Radio {
    Radio::new(group, value).label(label).selected(selected).font(font)
}
fn select_radio(
    target_entity: Entity,
    commands: &mut Commands,
    all_radios: &mut Query<(
        Entity,
        &mut LumaRadio,
        &mut UBorder,
        Option<&Children>,
        Option<&mut UFocusVisual>,
    )>,
    dot_query: &mut Query<&mut UNode, With<LumaRadioDot>>,
) {
    // Check if target entity is a radio
    let Ok((_, target_radio, _, _, _)) = all_radios.get(target_entity) else {
        return;
    };

    if target_radio.disabled {
        return;
    }

    let target_group = target_radio.group.clone();
    let selected_value = target_radio.value.clone();

    // Synchronize the entire group: select target, deselect others in same group
    for (entity, mut radio, mut border, children, mut visual_opt) in all_radios.iter_mut() {
        if radio.group == target_group {
            let is_now_selected = entity == target_entity;
            radio.selected = is_now_selected;

            let target_border = if is_now_selected { SELECTED_COLOR } else { UNSELECTED_BORDER };

            if let Some(ref mut visual) = visual_opt {
                if visual.is_captured() {
                    visual.set_original_border_color(target_border);
                } else {
                    border.color = target_border;
                }
            } else {
                border.color = target_border;
            }

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
        entity: target_entity,
    });
}

fn on_radio_click(
    trigger: On<Pointer<Click>>,
    mut commands: Commands,
    mut all_radios: Query<(
        Entity,
        &mut LumaRadio,
        &mut UBorder,
        Option<&Children>,
        Option<&mut UFocusVisual>,
    )>,
    mut dot_query: Query<&mut UNode, With<LumaRadioDot>>,
) {
    select_radio(trigger.entity.entity(), &mut commands, &mut all_radios, &mut dot_query);
}

fn on_radio_activate(
    trigger: On<UFocusActivate>,
    mut commands: Commands,
    mut all_radios: Query<(
        Entity,
        &mut LumaRadio,
        &mut UBorder,
        Option<&Children>,
        Option<&mut UFocusVisual>,
    )>,
    mut dot_query: Query<&mut UNode, With<LumaRadioDot>>,
) {
    select_radio(trigger.entity, &mut commands, &mut all_radios, &mut dot_query);
}

pub struct LumaRadioPlugin;

impl Plugin for LumaRadioPlugin {
    fn build(&self, app: &mut App) {
        app.register_type::<LumaRadio>()
            .register_type::<LumaRadioDot>()
            .add_observer(on_radio_click)
            .add_observer(on_radio_activate);
    }
}
