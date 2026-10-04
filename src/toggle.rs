//! Animated toggle switch widget.

use bevy::prelude::*;
use bevy_luma::prelude::*;

/// Marker and state component for a toggle switch.
#[derive(Component, Clone, Debug, Reflect)]
pub struct LumaToggle {
    pub is_checked: bool,
    pub disabled: bool,
}

impl Default for LumaToggle {
    fn default() -> Self {
        Self {
            is_checked: false,
            disabled: false,
        }
    }
}

/// Component attached to the sliding knob inside a toggle.
#[derive(Component, Clone, Debug, Reflect, Default)]
pub struct LumaToggleKnob {
    pub current_offset: f32,
    pub target_offset: f32,
}

/// Event triggered when a toggle's state changes.
#[derive(Event, Clone, Copy, Debug)]
pub struct ToggleChanged {
    pub entity: Entity,
    pub is_checked: bool,
}

const TRACK_WIDTH: f32 = 44.0;
const TRACK_HEIGHT: f32 = 24.0;
const KNOB_SIZE: f32 = 20.0;
const TRAVEL_DISTANCE: f32 = TRACK_WIDTH - KNOB_SIZE - 4.0; // 20.0px

const CHECKED_BG: Color = Color::srgb(0.20, 0.45, 0.90);
const UNCHECKED_BG: Color = Color::srgb(0.18, 0.22, 0.28);
const CHECKED_BORDER: Color = Color::srgb(0.28, 0.55, 1.00);
const UNCHECKED_BORDER: Color = Color::srgb(0.28, 0.32, 0.40);

/// Creates an animated toggle switch scene.
pub fn luma_toggle(is_checked: bool) -> impl Scene {
    let initial_offset = if is_checked { TRAVEL_DISTANCE } else { 0.0 };
    let bg = if is_checked { CHECKED_BG } else { UNCHECKED_BG };
    let border = if is_checked { CHECKED_BORDER } else { UNCHECKED_BORDER };

    bsn! {
        LumaToggle {
            is_checked,
            disabled: false,
        }
        UNode {
            width: UVal::Px(TRACK_WIDTH),
            height: UVal::Px(TRACK_HEIGHT),
            background_color: bg,
            border_radius: UCornerRadius::all(12.0),
            padding: USides::all(2.0),
        }
        UBorder {
            color: border,
            width: 1.0,
            radius: UCornerRadius::all(12.0),
        }
        UInteraction::default()
        UFocusable::new()
        UFocusVisual::border(Color::srgb(0.35, 0.65, 1.0), 2.0)
        ULayout {
            display: UDisplay::Flex,
            flex_direction: UFlexDirection::Row,
            align_items: UAlignItems::Center,
        }
        Children [
            (
                LumaToggleKnob {
                    current_offset: initial_offset,
                    target_offset: initial_offset,
                }
                UNode {
                    width: UVal::Px(KNOB_SIZE),
                    height: UVal::Px(KNOB_SIZE),
                    background_color: Color::WHITE,
                    border_radius: UCornerRadius::all(10.0),
                }
                USelf {
                    position_type: UPositionType::Relative,
                    left: { UVal::Px(initial_offset) },
                }
            )
        ]
    }
}

/// Creates a toggle switch with an adjacent text label.
pub fn luma_toggle_with_label(
    label: impl Into<String>,
    is_checked: bool,
    font: Handle<Font>,
) -> impl Scene {
    let label = label.into();

    bsn! {
        UNode::default()
        ULayout {
            display: UDisplay::Flex,
            flex_direction: UFlexDirection::Row,
            align_items: UAlignItems::Center,
            gap: 12.0,
        }
        Children [
            (
                luma_toggle(is_checked)
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

fn on_toggle_click(
    trigger: On<Pointer<Click>>,
    mut commands: Commands,
    mut query: Query<(&mut LumaToggle, &mut UNode, &mut UBorder, Option<&Children>)>,
    mut knob_query: Query<&mut LumaToggleKnob>,
) {
    let entity = trigger.entity.entity();
    if let Ok((mut toggle, mut node, mut border, children)) = query.get_mut(entity) {
        if toggle.disabled {
            return;
        }

        toggle.is_checked = !toggle.is_checked;
        let is_checked = toggle.is_checked;

        node.background_color = if is_checked { CHECKED_BG } else { UNCHECKED_BG };
        border.color = if is_checked { CHECKED_BORDER } else { UNCHECKED_BORDER };

        let target_offset = if is_checked { TRAVEL_DISTANCE } else { 0.0 };

        if let Some(children) = children {
            for child in children.iter() {
                if let Ok(mut knob) = knob_query.get_mut(child) {
                    knob.target_offset = target_offset;
                }
            }
        }

        commands.trigger(ToggleChanged {
            entity,
            is_checked,
        });
    }
}

fn toggle_animation_system(
    time: Res<Time>,
    mut knob_query: Query<(&mut LumaToggleKnob, &mut USelf)>,
) {
    let dt = time.delta_secs();
    let speed = 24.0;

    for (mut knob, mut uself) in knob_query.iter_mut() {
        if (knob.current_offset - knob.target_offset).abs() > 0.01 {
            knob.current_offset += (knob.target_offset - knob.current_offset) * (speed * dt).min(1.0);
            uself.left = UVal::Px(knob.current_offset);
        }
    }
}

pub struct LumaTogglePlugin;

impl Plugin for LumaTogglePlugin {
    fn build(&self, app: &mut App) {
        app.register_type::<LumaToggle>()
            .register_type::<LumaToggleKnob>()
            .add_observer(on_toggle_click)
            .add_systems(Update, toggle_animation_system);
    }
}
