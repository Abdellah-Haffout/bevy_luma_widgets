use bevy::ecs::relationship::Relationship;
use bevy::picking::pointer::PointerButton;
use bevy::prelude::*;
use bevy::scene::{ResolveContext, ResolvedScene, ResolveSceneError};
use bevy_luma::prelude::*;

/// Marker and state component for a segmented control container.
#[derive(Component, Clone, Debug, Default, Reflect)]
pub struct LumaSegmentedControl {
    pub group: String,
    pub active: String,
}

/// An individual clickable option within a segmented control.
#[derive(Component, Clone, Debug, Default, Reflect)]
pub struct LumaSegmentOption {
    pub group: String,
    pub value: String,
    pub is_active: bool,
}

/// Event triggered when a segment option is chosen.
#[derive(Event, Clone, Debug)]
pub struct SegmentChanged {
    pub entity: Entity,
    pub group: String,
    pub value: String,
}

const SEGMENT_ACTIVE_BG: Color = Color::srgb_u8(0x1F, 0xD6, 0x9A);
const SEGMENT_INACTIVE_BG: Color = Color::NONE;
const SEGMENT_ACTIVE_TEXT: Color = Color::srgb_u8(0x06, 0x22, 0x18);
const SEGMENT_INACTIVE_TEXT: Color = Color::srgb_u8(0xEC, 0xEC, 0xEE);
const CONTAINER_BG: Color = Color::srgb_u8(0x18, 0x18, 0x1B);
const CONTAINER_BORDER: Color = Color::srgb_u8(0x2B, 0x2B, 0x30);

/// Declarative SegmentOption widget struct.
#[derive(Clone, Debug, Reflect)]
pub struct SegmentOption {
    pub group: String,
    pub value: String,
    pub label: String,
    pub is_active: bool,
    pub font: Handle<Font>,
}

impl Default for SegmentOption {
    fn default() -> Self {
        Self {
            group: String::new(),
            value: String::new(),
            label: String::new(),
            is_active: false,
            font: Handle::default(),
        }
    }
}

impl SegmentOption {
    pub fn new(
        group: impl Into<String>,
        value: impl Into<String>,
        label: impl Into<String>,
    ) -> Self {
        Self {
            group: group.into(),
            value: value.into(),
            label: label.into(),
            ..default()
        }
    }

    pub fn active(mut self, is_active: bool) -> Self {
        self.is_active = is_active;
        self
    }

    pub fn font(mut self, font: Handle<Font>) -> Self {
        self.font = font;
        self
    }
}

impl Scene for SegmentOption {
    fn resolve(
        self,
        context: &mut ResolveContext,
        scene: &mut ResolvedScene,
    ) -> Result<(), ResolveSceneError> {
        let group = self.group;
        let value = self.value;
        let label = self.label;
        let is_active = self.is_active;
        let font = self.font;

        let bg = if is_active { SEGMENT_ACTIVE_BG } else { SEGMENT_INACTIVE_BG };
        let text_color = if is_active { SEGMENT_ACTIVE_TEXT } else { SEGMENT_INACTIVE_TEXT };

        let s = bsn! {
            LumaSegmentOption {
                group,
                value,
                is_active,
            }
            UNode {
                height: UVal::Px(30.0),
                padding: USides::axes(14.0, 4.0),
                background_color: bg,
                border_radius: UCornerRadius::all(6.0),
            }
            UInteraction::default()
            UInteractionColors {
                normal: bg,
                hovered: { if is_active { SEGMENT_ACTIVE_BG } else { Color::srgba(1.0, 1.0, 1.0, 0.05) } },
                pressed: { if is_active { SEGMENT_ACTIVE_BG } else { Color::srgba(1.0, 1.0, 1.0, 0.10) } },
            }
            UFocusable::new()
            UFocusVisual::border(Color::srgb(0.35, 0.65, 1.0), 1.5)
            ULayout {
                display: UDisplay::Flex,
                justify_content: UJustifyContent::Center,
                align_items: UAlignItems::Center,
            }
            Children [
                UText {
                    text: label,
                    font_size: 13.0,
                    font,
                    color: text_color,
                }
            ]
        };
        s.resolve(context, scene)
    }
}

/// Declarative SegmentedControl container widget struct.
#[derive(Clone, Debug, Reflect)]
pub struct SegmentedControl {
    pub group: String,
    pub active: String,
}

impl Default for SegmentedControl {
    fn default() -> Self {
        Self {
            group: String::new(),
            active: String::new(),
        }
    }
}

impl SegmentedControl {
    pub fn new(group: impl Into<String>, active: impl Into<String>) -> Self {
        Self {
            group: group.into(),
            active: active.into(),
        }
    }
}

impl Scene for SegmentedControl {
    fn resolve(
        self,
        context: &mut ResolveContext,
        scene: &mut ResolvedScene,
    ) -> Result<(), ResolveSceneError> {
        let group = self.group;
        let active = self.active;

        let s = bsn! {
            LumaSegmentedControl {
                group,
                active,
            }
            UNode {
                background_color: CONTAINER_BG,
                border_radius: UCornerRadius::all(8.0),
                padding: USides::all(3.0),
            }
            UBorder {
                color: CONTAINER_BORDER,
                width: 1.0,
                radius: UCornerRadius::all(8.0),
            }
            ULayout {
                display: UDisplay::Flex,
                flex_direction: UFlexDirection::Row,
                align_items: UAlignItems::Center,
                gap: 2.0,
            }
        };
        s.resolve(context, scene)
    }
}

/// Creates an individual clickable segment option scene.
pub fn luma_segment_option(
    group: impl Into<String>,
    value: impl Into<String>,
    label: impl Into<String>,
    is_active: bool,
    font: Handle<Font>,
) -> SegmentOption {
    SegmentOption::new(group, value, label).active(is_active).font(font)
}

/// Creates a segmented control container enclosing multiple segment options.
pub fn luma_segmented_control(
    group: impl Into<String>,
    active: impl Into<String>,
    options: impl Scene,
) -> impl Scene {
    let group = group.into();
    let active = active.into();

    bsn! {
        LumaSegmentedControl {
            group,
            active,
        }
        UNode {
            background_color: CONTAINER_BG,
            border_radius: UCornerRadius::all(8.0),
            padding: USides::all(3.0),
        }
        UBorder {
            color: CONTAINER_BORDER,
            width: 1.0,
            radius: UCornerRadius::all(8.0),
        }
        ULayout {
            display: UDisplay::Flex,
            flex_direction: UFlexDirection::Row,
            align_items: UAlignItems::Center,
            gap: 2.0,
        }
        Children [
            @{options}
        ]
    }
}

fn switch_segment(
    target_entity: Entity,
    commands: &mut Commands,
    parents_query: &Query<&ChildOf>,
    option_query: &mut Query<(Entity, &mut LumaSegmentOption, &mut UNode, &Children)>,
    control_query: &mut Query<&mut LumaSegmentedControl>,
    text_query: &mut Query<&mut UText>,
) {
    let (group, target_value) = if let Ok((_, opt, _, _)) = option_query.get(target_entity) {
        (opt.group.clone(), opt.value.clone())
    } else {
        return;
    };

    // Find parent control container
    let mut current = target_entity;
    while let Ok(parent) = parents_query.get(current) {
        current = parent.get();
        if let Ok(mut control) = control_query.get_mut(current) {
            if control.group == group {
                control.active = target_value.clone();
                break;
            }
        }
    }

    // Update sibling options
    for (entity, mut option, mut node, children) in option_query.iter_mut() {
        if option.group == group {
            let is_active = entity == target_entity;
            option.is_active = is_active;
            node.background_color = if is_active { SEGMENT_ACTIVE_BG } else { SEGMENT_INACTIVE_BG };

            let text_color = if is_active { SEGMENT_ACTIVE_TEXT } else { SEGMENT_INACTIVE_TEXT };
            for child in children.iter() {
                if let Ok(mut text) = text_query.get_mut(child) {
                    text.color = text_color;
                }
            }
        }
    }

    commands.trigger(SegmentChanged {
        entity: target_entity,
        group,
        value: target_value,
    });
}

fn on_segment_click(
    trigger: On<PointerClick>,
    mut commands: Commands,
    parents_query: Query<&ChildOf>,
    mut option_query: Query<(Entity, &mut LumaSegmentOption, &mut UNode, &Children)>,
    mut control_query: Query<&mut LumaSegmentedControl>,
    mut text_query: Query<&mut UText>,
) {
    if trigger.event().button != PointerButton::Primary {
        return;
    }
    switch_segment(
        trigger.entity,
        &mut commands,
        &parents_query,
        &mut option_query,
        &mut control_query,
        &mut text_query,
    );
}

fn on_segment_activate(
    trigger: On<UFocusActivate>,
    mut commands: Commands,
    parents_query: Query<&ChildOf>,
    mut option_query: Query<(Entity, &mut LumaSegmentOption, &mut UNode, &Children)>,
    mut control_query: Query<&mut LumaSegmentedControl>,
    mut text_query: Query<&mut UText>,
) {
    switch_segment(
        trigger.entity,
        &mut commands,
        &parents_query,
        &mut option_query,
        &mut control_query,
        &mut text_query,
    );
}

fn keyboard_segmented_control_system(
    keyboard: Option<Res<ButtonInput<KeyCode>>>,
    mut commands: Commands,
    mut focus_state: Option<ResMut<UFocusState>>,
    focused_query: Query<Entity, With<UFocused>>,
    parents_query: Query<&ChildOf>,
    mut option_query: Query<(Entity, &mut LumaSegmentOption, &mut UNode, &Children)>,
    mut control_query: Query<&mut LumaSegmentedControl>,
    mut text_query: Query<&mut UText>,
) {
    let Some(keyboard) = keyboard else {
        return;
    };

    let left = keyboard.just_pressed(KeyCode::ArrowLeft) || keyboard.just_pressed(KeyCode::ArrowUp);
    let right = keyboard.just_pressed(KeyCode::ArrowRight) || keyboard.just_pressed(KeyCode::ArrowDown);
    if !left && !right {
        return;
    }

    let Some(focused_entity) = focused_query.iter().find(|&e| option_query.contains(e)) else {
        return;
    };

    let Ok((_, focused_opt, _, _)) = option_query.get(focused_entity) else {
        return;
    };
    let target_group = focused_opt.group.clone();

    let group_entities: Vec<Entity> = option_query
        .iter()
        .filter(|(_, opt, _, _)| opt.group == target_group)
        .map(|(e, _, _, _)| e)
        .collect();

    if group_entities.is_empty() {
        return;
    }

    if let Some(pos) = group_entities.iter().position(|&e| e == focused_entity) {
        let next_idx = if left {
            if pos == 0 { group_entities.len() - 1 } else { pos - 1 }
        } else {
            (pos + 1) % group_entities.len()
        };

        let target_entity = group_entities[next_idx];
        switch_segment(
            target_entity,
            &mut commands,
            &parents_query,
            &mut option_query,
            &mut control_query,
            &mut text_query,
        );

        if let Some(ref mut state) = focus_state {
            set_focus(&mut commands, state, Some(target_entity), FocusModality::Keyboard);
        }
    }
}

pub struct LumaSegmentedControlPlugin;

impl Plugin for LumaSegmentedControlPlugin {
    fn build(&self, app: &mut App) {
        app.register_type::<LumaSegmentedControl>()
            .register_type::<LumaSegmentOption>()
            .add_observer(on_segment_click)
            .add_observer(on_segment_activate)
            .add_systems(Update, keyboard_segmented_control_system);
    }
}
