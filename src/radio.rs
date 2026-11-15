//! Radio button widget with grouped single-choice selection.

use bevy::prelude::*;
use bevy::scene::{ResolveContext, ResolvedScene, ResolveSceneError};
use bevy_luma::prelude::*;

use crate::helpers::{
    dim_color, find_target_with_children, interaction_colors, is_primary_click, shift_brightness,
    sync_focus_visual_border,
};
use crate::theme::{RadioStyle, WidgetColors};

/// Marker and state component for a radio button.
#[derive(Component, Clone, Debug, Reflect)]
pub struct LumaRadio {
    pub group: String,
    pub value: String,
    pub selected: bool,
    pub disabled: bool,
    pub style: RadioStyle,
    pub has_explicit_style: bool,
}

impl Default for LumaRadio {
    fn default() -> Self {
        Self {
            group: String::new(),
            value: String::new(),
            selected: false,
            disabled: false,
            style: RadioStyle::default(),
            has_explicit_style: false,
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

/// Declarative Radio button widget struct with all configurable properties.
#[derive(Clone, Debug, Reflect)]
pub struct Radio {
    pub group: String,
    pub value: String,
    pub label: Option<String>,
    pub selected: bool,
    pub disabled: bool,
    pub font: Handle<Font>,
    pub style: Option<RadioStyle>,
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
            style: None,
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

    pub fn style(mut self, style: RadioStyle) -> Self {
        self.style = Some(style);
        self
    }
}

pub(crate) fn radio_visual_state(
    style: &RadioStyle,
    selected: bool,
    disabled: bool,
) -> (Color, Color, Color, UInteractionColors) {
    let palette = WidgetColors::default();
    if disabled {
        let bg = palette.disabled_bg();
        let border = palette.disabled_border();
        let dot = if selected { dim_color(style.dot_color, 0.45) } else { Color::NONE };
        return (bg, border, dot, interaction_colors(bg, bg, bg, true));
    }

    let bg = if selected { style.selected_bg } else { style.unselected_bg };
    let border = if selected { style.selected_border } else { style.unselected_border };
    let dot = if selected { style.dot_color } else { Color::NONE };
    let (hovered, pressed) = if bg == palette.surface {
        (palette.surface_hover, palette.secondary_pressed)
    } else {
        (shift_brightness(bg, 0.06), shift_brightness(bg, -0.06))
    };
    (bg, border, dot, interaction_colors(bg, hovered, pressed, false))
}

impl Scene for Radio {
    fn resolve(
        self,
        context: &mut ResolveContext,
        scene: &mut ResolvedScene,
    ) -> Result<(), ResolveSceneError> {
        let has_explicit_style = self.style.is_some();
        let style = self.style.unwrap_or_default();
        let palette = WidgetColors::default();

        let group = self.group;
        let value = self.value;
        let selected = self.selected;
        let disabled = self.disabled;
        let (bg, border_color, dot_color, i_colors) = radio_visual_state(&style, selected, disabled);
        let label_color = if disabled { palette.disabled_text() } else { style.label_color };
        let focus_color = palette.focus_ring();
        let size = style.size;
        let radius = size * 0.5;
        let dot_size = style.dot_size;
        let dot_radius = dot_size * 0.5;
        let border_width = style.border_width;
        let label_font_size = style.font_size;

        let circle_node = bsn! {
            LumaRadio {
                group,
                value,
                selected,
                disabled,
                style,
                has_explicit_style,
            }
            UNode {
                width: UVal::Px(size),
                height: UVal::Px(size),
                background_color: bg,
                border_radius: UCornerRadius::all(radius),
            }
            UBorder {
                color: border_color,
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
                justify_content: UJustifyContent::Center,
                align_items: UAlignItems::Center,
            }
            Children [
                LumaRadioDot
                UNode {
                    width: UVal::Px(dot_size),
                    height: UVal::Px(dot_size),
                    background_color: dot_color,
                    border_radius: UCornerRadius::all(dot_radius),
                }
            ]
        };

        if let Some(label_str) = self.label {
            let font = self.font;
            let container = bsn! {
                UNode::default()
                UInteraction::default()
                ULayout {
                    display: UDisplay::Flex,
                    flex_direction: UFlexDirection::Row,
                    align_items: UAlignItems::Center,
                    gap: 10.0,
                }
                Children [
                    @{circle_node}
                    --
                    UText {
                        text: label_str,
                        font_size: label_font_size,
                        font,
                        color: label_color,
                    }
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
        &mut UNode,
        &mut UBorder,
        Option<&Children>,
        Option<&mut UFocusVisual>,
        Option<&mut UInteractionColors>,
    )>,
    dot_query: &mut Query<&mut UNode, (With<LumaRadioDot>, Without<LumaRadio>)>,
) {
    let Ok((_, target_radio, _, _, _, _, _)) = all_radios.get(target_entity) else {
        return;
    };

    if target_radio.disabled || target_radio.selected {
        return;
    }

    let target_group = target_radio.group.clone();
    let selected_value = target_radio.value.clone();

    // Synchronize the entire group: select target, deselect others in same group
    for (entity, mut radio, mut node, mut border, children, visual_opt, colors_opt) in all_radios.iter_mut() {
        if radio.group != target_group {
            continue;
        }
        let is_now_selected = entity == target_entity;
        radio.selected = is_now_selected;

        let (bg, target_border, target_dot, i_colors) =
            radio_visual_state(&radio.style, is_now_selected, radio.disabled);

        node.background_color = bg;
        if let Some(mut colors) = colors_opt {
            *colors = i_colors;
        }
        sync_focus_visual_border(visual_opt, &mut border, target_border);

        if let Some(children) = children {
            for child in children.iter() {
                if let Ok(mut dot) = dot_query.get_mut(child) {
                    dot.background_color = target_dot;
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
    trigger: On<PointerClick>,
    mut commands: Commands,
    mut all_radios: Query<(
        Entity,
        &mut LumaRadio,
        &mut UNode,
        &mut UBorder,
        Option<&Children>,
        Option<&mut UFocusVisual>,
        Option<&mut UInteractionColors>,
    )>,
    mut dot_query: Query<&mut UNode, (With<LumaRadioDot>, Without<LumaRadio>)>,
    parents_query: Query<&ChildOf>,
    children_query: Query<&Children>,
) {
    if !is_primary_click(&trigger) {
        return;
    }
    if let Some(entity) = find_target_with_children(trigger.entity, &all_radios, &parents_query, &children_query) {
        select_radio(entity, &mut commands, &mut all_radios, &mut dot_query);
    }
}

fn on_radio_activate(
    trigger: On<UFocusActivate>,
    mut commands: Commands,
    mut all_radios: Query<(
        Entity,
        &mut LumaRadio,
        &mut UNode,
        &mut UBorder,
        Option<&Children>,
        Option<&mut UFocusVisual>,
        Option<&mut UInteractionColors>,
    )>,
    mut dot_query: Query<&mut UNode, (With<LumaRadioDot>, Without<LumaRadio>)>,
    parents_query: Query<&ChildOf>,
    children_query: Query<&Children>,
) {
    if let Some(entity) = find_target_with_children(trigger.entity, &all_radios, &parents_query, &children_query) {
        select_radio(entity, &mut commands, &mut all_radios, &mut dot_query);
    }
}

fn keyboard_radio_system(
    keyboard: Option<Res<ButtonInput<KeyCode>>>,
    mut commands: Commands,
    mut focus_state: Option<ResMut<UFocusState>>,
    focused_query: Query<Entity, With<UFocused>>,
    mut all_radios: Query<(
        Entity,
        &mut LumaRadio,
        &mut UNode,
        &mut UBorder,
        Option<&Children>,
        Option<&mut UFocusVisual>,
        Option<&mut UInteractionColors>,
    )>,
    mut dot_query: Query<&mut UNode, (With<LumaRadioDot>, Without<LumaRadio>)>,
) {
    let Some(keyboard) = keyboard else {
        return;
    };

    let prev = keyboard.just_pressed(KeyCode::ArrowLeft) || keyboard.just_pressed(KeyCode::ArrowUp);
    let next = keyboard.just_pressed(KeyCode::ArrowRight) || keyboard.just_pressed(KeyCode::ArrowDown);
    if !prev && !next {
        return;
    }

    let Some(focused_entity) = focused_query.iter().find(|&e| all_radios.contains(e)) else {
        return;
    };

    let Ok((_, focused_radio, _, _, _, _, _)) = all_radios.get(focused_entity) else {
        return;
    };
    let target_group = focused_radio.group.clone();

    let group_entities: Vec<Entity> = all_radios
        .iter()
        .filter(|(_, r, _, _, _, _, _)| r.group == target_group && !r.disabled)
        .map(|(e, _, _, _, _, _, _)| e)
        .collect();

    if group_entities.is_empty() {
        return;
    }

    if let Some(pos) = group_entities.iter().position(|&e| e == focused_entity) {
        let next_idx = if prev {
            if pos == 0 { group_entities.len() - 1 } else { pos - 1 }
        } else {
            (pos + 1) % group_entities.len()
        };

        let target_entity = group_entities[next_idx];
        select_radio(target_entity, &mut commands, &mut all_radios, &mut dot_query);

        if let Some(ref mut state) = focus_state {
            set_focus(&mut commands, state, Some(target_entity), FocusModality::Keyboard);
        }
    }
}

pub struct LumaRadioPlugin;

impl Plugin for LumaRadioPlugin {
    fn build(&self, app: &mut App) {
        app.register_type::<LumaRadio>()
            .register_type::<LumaRadioDot>()
            .add_observer(on_radio_click)
            .add_observer(on_radio_activate)
            .add_systems(Update, keyboard_radio_system);
    }
}
