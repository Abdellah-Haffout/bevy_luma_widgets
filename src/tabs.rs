//! Tabbed navigation widget for organizing content into multiple switchable views.

use bevy::prelude::*;
use bevy_luma::prelude::*;

/// Root container for a tabs component.
#[derive(Component, Clone, Debug, Default, Reflect)]
pub struct LumaTabs {
    pub group: String,
    pub active: String,
}

/// A clickable tab trigger button inside a tab list.
#[derive(Component, Clone, Debug, Default, Reflect)]
pub struct LumaTabTrigger {
    pub group: String,
    pub value: String,
    pub active: bool,
}

/// A content panel associated with a specific tab value.
#[derive(Component, Clone, Debug, Default, Reflect)]
pub struct LumaTabContent {
    pub group: String,
    pub value: String,
}

/// Event fired when an active tab changes.
#[derive(Event, Clone, Debug)]
pub struct TabSelected {
    pub group: String,
    pub value: String,
    pub entity: Entity,
}

const TAB_ACTIVE_BG: Color = Color::srgb(0.20, 0.45, 0.90);
const TAB_INACTIVE_BG: Color = Color::NONE;
const TAB_ACTIVE_TEXT: Color = Color::WHITE;
const TAB_INACTIVE_TEXT: Color = Color::srgb(0.65, 0.70, 0.80);

use bevy::scene::{ResolveContext, ResolvedScene, ResolveSceneError};

/// Declarative TabTrigger widget struct.
#[derive(Clone, Debug, Reflect)]
pub struct TabTrigger {
    pub group: String,
    pub value: String,
    pub label: String,
    pub is_active: bool,
    pub font: Handle<Font>,
}

impl Default for TabTrigger {
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

impl TabTrigger {
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

impl Scene for TabTrigger {
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

        let bg_normal = if is_active { TAB_ACTIVE_BG } else { TAB_INACTIVE_BG };
        let bg_hover = if is_active { Color::srgb(0.28, 0.55, 1.00) } else { Color::srgb(0.16, 0.20, 0.28) };
        let bg_pressed = if is_active { Color::srgb(0.15, 0.38, 0.80) } else { Color::srgb(0.12, 0.15, 0.22) };
        let text_color = if is_active { TAB_ACTIVE_TEXT } else { TAB_INACTIVE_TEXT };

        let s = bsn! {
            LumaTabTrigger {
                group,
                value,
                active: is_active,
            }
            UNode {
                height: UVal::Px(34.0),
                padding: USides::axes(16.0, 7.0),
                background_color: bg_normal,
                border_radius: UCornerRadius::all(6.0),
            }
            UInteraction::default()
            UInteractionColors {
                normal: bg_normal,
                hovered: bg_hover,
                pressed: bg_pressed,
            }
            UFocusable::new()
            UFocusVisual::border(Color::srgb(0.35, 0.65, 1.0), 2.0)
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

/// Declarative TabList container widget struct.
#[derive(Clone, Debug, Default, Reflect)]
pub struct TabList;

impl Scene for TabList {
    fn resolve(
        self,
        context: &mut ResolveContext,
        scene: &mut ResolvedScene,
    ) -> Result<(), ResolveSceneError> {
        let s = bsn! {
            UNode {
                background_color: Color::srgb(0.12, 0.15, 0.20),
                border_radius: UCornerRadius::all(8.0),
                padding: USides::all(4.0),
            }
            UBorder {
                color: Color::srgb(0.18, 0.22, 0.30),
                width: 1.0,
                radius: UCornerRadius::all(8.0),
            }
            ULayout {
                display: UDisplay::Flex,
                flex_direction: UFlexDirection::Row,
                align_items: UAlignItems::Center,
                gap: 4.0,
            }
        };
        s.resolve(context, scene)
    }
}

/// Creates an active or inactive tab trigger button.
pub fn luma_tab_trigger(
    group: impl Into<String>,
    value: impl Into<String>,
    label: impl Into<String>,
    is_active: bool,
    font: Handle<Font>,
) -> TabTrigger {
    TabTrigger::new(group, value, label).active(is_active).font(font)
}

/// Creates a horizontal tab list container for grouping tab triggers.
pub fn luma_tab_list(triggers: impl Scene) -> impl Scene {
    bsn! {
        UNode {
            background_color: Color::srgb(0.09, 0.11, 0.15),
            border_radius: UCornerRadius::all(8.0),
            padding: USides::all(4.0),
        }
        UBorder {
            color: Color::srgb(0.18, 0.22, 0.30),
            width: 1.0,
            radius: UCornerRadius::all(8.0),
        }
        ULayout {
            display: UDisplay::Flex,
            flex_direction: UFlexDirection::Row,
            align_items: UAlignItems::Center,
            gap: 6.0,
        }
        Children [
            @{triggers}
        ]
    }
}

fn switch_tab(
    target_entity: Entity,
    commands: &mut Commands,
    mut triggers: Query<(Entity, &mut LumaTabTrigger, &mut UNode, &Children, Option<&mut UInteractionColors>)>,
    mut contents: Query<(&LumaTabContent, &mut ULayout)>,
    mut text_query: Query<&mut UText>,
) {
    let Ok((_, target_trigger, _, _, _)) = triggers.get(target_entity) else {
        return;
    };

    let group = target_trigger.group.clone();
    let target_value = target_trigger.value.clone();

    // 1. Update triggers
    for (entity, mut trigger, mut node, children, mut colors_opt) in triggers.iter_mut() {
        if trigger.group == group {
            let is_active = entity == target_entity;
            trigger.active = is_active;
            node.background_color = if is_active { TAB_ACTIVE_BG } else { TAB_INACTIVE_BG };

            if let Some(ref mut colors) = colors_opt {
                if is_active {
                    colors.normal = TAB_ACTIVE_BG;
                    colors.hovered = Color::srgb(0.28, 0.55, 1.00);
                    colors.pressed = Color::srgb(0.15, 0.38, 0.80);
                } else {
                    colors.normal = TAB_INACTIVE_BG;
                    colors.hovered = Color::srgb(0.18, 0.22, 0.30);
                    colors.pressed = Color::srgb(0.14, 0.17, 0.24);
                }
            }

            let text_color = if is_active { TAB_ACTIVE_TEXT } else { TAB_INACTIVE_TEXT };
            for child in children.iter() {
                if let Ok(mut text) = text_query.get_mut(child) {
                    text.color = text_color;
                }
            }
        }
    }

    // 2. Update content visibility
    for (content, mut layout) in contents.iter_mut() {
        if content.group == group {
            layout.display = if content.value == target_value {
                UDisplay::Flex
            } else {
                UDisplay::None
            };
        }
    }

    commands.trigger(TabSelected {
        group,
        value: target_value,
        entity: target_entity,
    });
}

fn on_tab_click(
    trigger: On<PointerClick>,
    mut commands: Commands,
    triggers: Query<(Entity, &mut LumaTabTrigger, &mut UNode, &Children, Option<&mut UInteractionColors>)>,
    contents: Query<(&LumaTabContent, &mut ULayout)>,
    text_query: Query<&mut UText>,
    parents_query: Query<&ChildOf>,
) {
    let target = if triggers.contains(trigger.entity) {
        Some(trigger.entity)
    } else {
        parents_query.iter_ancestors(trigger.entity).find(|&e| triggers.contains(e))
    };
    if let Some(target_entity) = target {
        switch_tab(
            target_entity,
            &mut commands,
            triggers,
            contents,
            text_query,
        );
    }
}

fn on_tab_activate(
    trigger: On<UFocusActivate>,
    mut commands: Commands,
    triggers: Query<(Entity, &mut LumaTabTrigger, &mut UNode, &Children, Option<&mut UInteractionColors>)>,
    contents: Query<(&LumaTabContent, &mut ULayout)>,
    text_query: Query<&mut UText>,
    parents_query: Query<&ChildOf>,
) {
    let target = if triggers.contains(trigger.entity) {
        Some(trigger.entity)
    } else {
        parents_query.iter_ancestors(trigger.entity).find(|&e| triggers.contains(e))
    };
    if let Some(target_entity) = target {
        switch_tab(
            target_entity,
            &mut commands,
            triggers,
            contents,
            text_query,
        );
    }
}

pub struct LumaTabsPlugin;

impl Plugin for LumaTabsPlugin {
    fn build(&self, app: &mut App) {
        app.register_type::<LumaTabs>()
            .register_type::<LumaTabTrigger>()
            .register_type::<LumaTabContent>()
            .add_observer(on_tab_click)
            .add_observer(on_tab_activate);
    }
}
