//! Animated toggle switch widget.

use bevy::prelude::*;
use bevy::scene::{ResolveContext, ResolvedScene, ResolveSceneError};
use bevy_luma::prelude::*;

use crate::helpers::{
    dim_color, find_target_with_children, interaction_colors, is_primary_click, shift_brightness,
    sync_focus_visual_border,
};
use crate::theme::{ToggleStyle, WidgetColors};

/// Marker and state component for a toggle switch.
#[derive(Component, Clone, Debug, Reflect)]
pub struct LumaToggle {
    pub is_checked: bool,
    pub disabled: bool,
    pub style: ToggleStyle,
    pub has_explicit_style: bool,
}

impl Default for LumaToggle {
    fn default() -> Self {
        Self {
            is_checked: false,
            disabled: false,
            style: ToggleStyle::default(),
            has_explicit_style: false,
        }
    }
}

/// Component attached to the sliding knob inside a toggle.
#[derive(Component, Clone, Debug, Reflect)]
pub struct LumaToggleKnob {
    pub current_offset: f32,
    pub target_offset: f32,
    pub travel_distance: f32,
}

impl Default for LumaToggleKnob {
    fn default() -> Self {
        let s = ToggleStyle::default();
        Self {
            current_offset: 0.0,
            target_offset: 0.0,
            travel_distance: (s.track_width - s.knob_size - 4.0).max(0.0),
        }
    }
}

/// Event triggered when a toggle's state changes.
#[derive(Event, Clone, Copy, Debug)]
pub struct ToggleChanged {
    pub entity: Entity,
    pub is_checked: bool,
}

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

pub(crate) fn toggle_interaction_colors(style: &ToggleStyle, is_checked: bool, disabled: bool) -> (Color, Color, UInteractionColors) {
    let palette = WidgetColors::default();
    if disabled {
        let bg = palette.disabled_bg();
        let border = palette.disabled_border();
        return (bg, border, interaction_colors(bg, bg, bg, true));
    }

    let bg = if is_checked { style.checked_bg } else { style.unchecked_bg };
    let border = if is_checked { style.checked_border } else { style.unchecked_border };
    let (hovered, pressed) = if is_checked {
        if style.checked_bg == palette.primary {
            (palette.primary_hover, palette.primary_pressed)
        } else {
            (shift_brightness(style.checked_bg, 0.08), shift_brightness(style.checked_bg, -0.08))
        }
    } else if style.unchecked_bg == palette.surface {
        (palette.surface_hover, palette.secondary_pressed)
    } else {
        (shift_brightness(style.unchecked_bg, 0.06), shift_brightness(style.unchecked_bg, -0.06))
    };
    (bg, border, interaction_colors(bg, hovered, pressed, false))
}

impl Scene for Toggle {
    fn resolve(
        self,
        context: &mut ResolveContext,
        scene: &mut ResolvedScene,
    ) -> Result<(), ResolveSceneError> {
        let has_explicit_style = self.style.is_some();
        let style = self.style.unwrap_or_default();
        let track_w = style.track_width;
        let track_h = style.track_height;
        let knob_sz = style.knob_size;
        let rad = style.radius;
        let travel = (track_w - knob_sz - 4.0).max(0.0);
        let initial_offset = if self.is_checked { travel } else { 0.0 };

        let palette = WidgetColors::default();
        let is_checked = self.is_checked;
        let disabled = self.disabled;
        let (bg, border, i_colors) = toggle_interaction_colors(&style, is_checked, disabled);
        let knob_col = if disabled {
            dim_color(style.knob_color, 0.45)
        } else {
            style.knob_color
        };
        let label_color = if disabled {
            palette.disabled_text()
        } else {
            palette.text_primary
        };
        let focus_color = palette.focus_ring();
        let font = self.font;

        let switch_node = bsn! {
            LumaToggle {
                is_checked,
                disabled,
                style,
                has_explicit_style,
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
            i_colors
            UFocusable {
                enabled: { !disabled },
                ..UFocusable::new()
            }
            UFocusVisual::border(focus_color, 2.0)
            ULayout {
                display: UDisplay::Flex,
                flex_direction: UFlexDirection::Row,
                align_items: UAlignItems::Center,
            }
            Children [
                LumaToggleKnob {
                    current_offset: initial_offset,
                    target_offset: initial_offset,
                    travel_distance: travel,
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
            ]
        };

        if let Some(label_text) = self.label {
            let container = bsn! {
                UNode::default()
                UInteraction::default()
                ULayout {
                    display: UDisplay::Flex,
                    flex_direction: UFlexDirection::Row,
                    align_items: UAlignItems::Center,
                    gap: 12.0,
                }
                Children [
                    @{switch_node}
                    --
                    UText {
                        text: label_text,
                        font_size: 14.0,
                        font,
                        color: label_color,
                    }
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
        Option<&mut UInteractionColors>,
    )>,
    knob_query: &mut Query<&mut LumaToggleKnob>,
) {
    if let Ok((mut toggle, mut node, mut border, children, visual_opt, colors_opt)) = query.get_mut(entity) {
        if toggle.disabled {
            return;
        }

        toggle.is_checked = !toggle.is_checked;
        let is_checked = toggle.is_checked;

        let (new_bg, new_border, new_i_colors) =
            toggle_interaction_colors(&toggle.style, is_checked, false);
        node.background_color = new_bg;

        if let Some(mut colors) = colors_opt {
            *colors = new_i_colors;
        }
        sync_focus_visual_border(visual_opt, &mut border, new_border);

        if let Some(children) = children {
            for child in children.iter() {
                if let Ok(mut knob) = knob_query.get_mut(child) {
                    knob.target_offset = if is_checked { knob.travel_distance } else { 0.0 };
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
    trigger: On<PointerClick>,
    mut commands: Commands,
    mut query: Query<(
        &mut LumaToggle,
        &mut UNode,
        &mut UBorder,
        Option<&Children>,
        Option<&mut UFocusVisual>,
        Option<&mut UInteractionColors>,
    )>,
    mut knob_query: Query<&mut LumaToggleKnob>,
    parents_query: Query<&ChildOf>,
    children_query: Query<&Children>,
) {
    if !is_primary_click(&trigger) {
        return;
    }
    if let Some(entity) = find_target_with_children(trigger.entity, &query, &parents_query, &children_query) {
        toggle_state(entity, &mut commands, &mut query, &mut knob_query);
    }
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
        Option<&mut UInteractionColors>,
    )>,
    mut knob_query: Query<&mut LumaToggleKnob>,
    parents_query: Query<&ChildOf>,
    children_query: Query<&Children>,
) {
    if let Some(entity) = find_target_with_children(trigger.entity, &query, &parents_query, &children_query) {
        toggle_state(entity, &mut commands, &mut query, &mut knob_query);
    }
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
