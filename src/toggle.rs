//! Animated toggle switch widget.

use bevy::picking::pointer::PointerButton;
use bevy::prelude::*;
use bevy::scene::{ResolveContext, ResolvedScene, ResolveSceneError};
use bevy_luma::prelude::*;

use crate::theme::ToggleStyle;

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

/// Declarative Toggle switch widget struct with all configurable properties.
#[derive(Clone, Debug, Reflect)]
pub struct Toggle {
    pub is_checked: bool,
    pub label: Option<String>,
    pub disabled: bool,
    pub font: Handle<Font>,
    pub style: Option<ToggleStyle>,
}

impl Default for Toggle {
    fn default() -> Self {
        Self {
            is_checked: false,
            label: None,
            disabled: false,
            font: Handle::default(),
            style: None,
        }
    }
}

impl Toggle {
    pub fn new(is_checked: bool) -> Self {
        Self {
            is_checked,
            ..default()
        }
    }

    pub fn label(mut self, label: impl Into<String>) -> Self {
        self.label = Some(label.into());
        self
    }

    pub fn checked(mut self, is_checked: bool) -> Self {
        self.is_checked = is_checked;
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

    pub fn style(mut self, style: ToggleStyle) -> Self {
        self.style = Some(style);
        self
    }
}

impl Scene for Toggle {
    fn resolve(
        self,
        context: &mut ResolveContext,
        scene: &mut ResolvedScene,
    ) -> Result<(), ResolveSceneError> {
        let (track_w, track_h, knob_sz, rad, chk_bg, unchk_bg, chk_bdr, unchk_bdr, knob_col) =
            if let Some(ref s) = self.style {
                (
                    s.track_width,
                    s.track_height,
                    s.knob_size,
                    s.radius,
                    s.checked_bg,
                    s.unchecked_bg,
                    s.checked_border,
                    s.unchecked_border,
                    s.knob_color,
                )
            } else {
                (
                    TRACK_WIDTH,
                    TRACK_HEIGHT,
                    KNOB_SIZE,
                    12.0,
                    CHECKED_BG,
                    UNCHECKED_BG,
                    CHECKED_BORDER,
                    UNCHECKED_BORDER,
                    Color::WHITE,
                )
            };

        let travel = track_w - knob_sz - 4.0;
        let initial_offset = if self.is_checked { travel } else { 0.0 };
        let bg = if self.is_checked { chk_bg } else { unchk_bg };
        let border = if self.is_checked { chk_bdr } else { unchk_bdr };
        let is_checked = self.is_checked;
        let disabled = self.disabled;
        let font = self.font;

        let switch_node = bsn! {
            LumaToggle {
                is_checked,
                disabled,
            }
            UNode {
                width: UVal::Px(track_w),
                height: UVal::Px(track_h),
                background_color: bg,
                border_radius: UCornerRadius::all(rad),
                padding: USides::all(2.0),
            }
            UBorder {
                color: border,
                width: 1.0,
                radius: UCornerRadius::all(rad),
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
                        width: UVal::Px(knob_sz),
                        height: UVal::Px(knob_sz),
                        background_color: knob_col,
                        border_radius: UCornerRadius::all(knob_sz * 0.5),
                    }
                    USelf {
                        position_type: UPositionType::Relative,
                        left: { UVal::Px(initial_offset) },
                    }
                )
            ]
        };

        if let Some(label_text) = self.label {
            let container = bsn! {
                UNode::default()
                ULayout {
                    display: UDisplay::Flex,
                    flex_direction: UFlexDirection::Row,
                    align_items: UAlignItems::Center,
                    gap: 12.0,
                }
                Children [
                    switch_node,
                    (UText {
                        text: label_text,
                        font_size: 14.0,
                        font,
                        color: Color::srgb(0.90, 0.92, 0.96),
                    })
                ]
            };
            container.resolve(context, scene)
        } else {
            switch_node.resolve(context, scene)
        }
    }
}

/// Creates an animated toggle switch scene.
pub fn luma_toggle(is_checked: bool) -> Toggle {
    Toggle::new(is_checked)
}

/// Creates a toggle switch with an adjacent text label.
pub fn luma_toggle_with_label(
    label: impl Into<String>,
    is_checked: bool,
    font: Handle<Font>,
) -> Toggle {
    Toggle::new(is_checked).label(label).font(font)
}

fn toggle_state(
    entity: Entity,
    commands: &mut Commands,
    query: &mut Query<(
        &mut LumaToggle,
        &mut UNode,
        &mut UBorder,
        Option<&Children>,
        Option<&mut UFocusVisual>,
    )>,
    knob_query: &mut Query<&mut LumaToggleKnob>,
) {
    if let Ok((mut toggle, mut node, mut border, children, mut visual_opt)) = query.get_mut(entity) {
        if toggle.disabled {
            return;
        }

        toggle.is_checked = !toggle.is_checked;
        let is_checked = toggle.is_checked;

        node.background_color = if is_checked { CHECKED_BG } else { UNCHECKED_BG };
        let new_border = if is_checked { CHECKED_BORDER } else { UNCHECKED_BORDER };

        if let Some(ref mut visual) = visual_opt {
            if visual.is_captured() {
                visual.set_original_border_color(new_border);
            } else {
                border.color = new_border;
            }
        } else {
            border.color = new_border;
        }

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

fn on_toggle_click(
    trigger: On<Pointer<Click>>,
    mut commands: Commands,
    mut query: Query<(
        &mut LumaToggle,
        &mut UNode,
        &mut UBorder,
        Option<&Children>,
        Option<&mut UFocusVisual>,
    )>,
    mut knob_query: Query<&mut LumaToggleKnob>,
) {
    if trigger.event().button != PointerButton::Primary {
        return;
    }
    toggle_state(trigger.entity.entity(), &mut commands, &mut query, &mut knob_query);
}

fn on_toggle_activate(
    trigger: On<UFocusActivate>,
    mut commands: Commands,
    mut query: Query<(
        &mut LumaToggle,
        &mut UNode,
        &mut UBorder,
        Option<&Children>,
        Option<&mut UFocusVisual>,
    )>,
    mut knob_query: Query<&mut LumaToggleKnob>,
) {
    toggle_state(trigger.entity, &mut commands, &mut query, &mut knob_query);
}

fn toggle_animation_system(
    time: Option<Res<Time>>,
    mut knob_query: Query<(&mut LumaToggleKnob, &mut USelf)>,
) {
    let Some(time) = time else {
        return;
    };
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
            .add_observer(on_toggle_activate)
            .add_systems(Update, toggle_animation_system);
    }
}
