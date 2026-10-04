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

/// Creates an active or inactive tab trigger button.
pub fn luma_tab_trigger(
    group: impl Into<String>,
    value: impl Into<String>,
    label: impl Into<String>,
    is_active: bool,
    font: Handle<Font>,
) -> impl Scene {
    let group = group.into();
    let value = value.into();
    let label = label.into();

    let bg = if is_active { TAB_ACTIVE_BG } else { TAB_INACTIVE_BG };
    let text_color = if is_active { TAB_ACTIVE_TEXT } else { TAB_INACTIVE_TEXT };

    bsn! {
        LumaTabTrigger {
            group,
            value,
            active: is_active,
        }
        UNode {
            height: UVal::Px(32.0),
            padding: USides::axes(14.0, 6.0),
            background_color: bg,
            border_radius: UCornerRadius::all(6.0),
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
            (UText {
                text: label,
                font_size: 13.0,
                font,
                color: text_color,
            })
        ]
    }
}

/// Creates a horizontal tab list container for grouping tab triggers.
pub fn luma_tab_list(triggers: impl Scene) -> impl Scene {
    bsn! {
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
        Children [
            triggers
        ]
    }
}

fn switch_tab(
    target_entity: Entity,
    commands: &mut Commands,
    mut triggers: Query<(Entity, &mut LumaTabTrigger, &mut UNode, &Children)>,
    mut contents: Query<(&LumaTabContent, &mut ULayout)>,
    mut text_query: Query<&mut UText>,
) {
    let Ok((_, target_trigger, _, _)) = triggers.get(target_entity) else {
        return;
    };

    let group = target_trigger.group.clone();
    let target_value = target_trigger.value.clone();

    // 1. Update triggers
    for (entity, mut trigger, mut node, children) in triggers.iter_mut() {
        if trigger.group == group {
            let is_active = entity == target_entity;
            trigger.active = is_active;
            node.background_color = if is_active { TAB_ACTIVE_BG } else { TAB_INACTIVE_BG };

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
    trigger: On<Pointer<Click>>,
    mut commands: Commands,
    triggers: Query<(Entity, &mut LumaTabTrigger, &mut UNode, &Children)>,
    contents: Query<(&LumaTabContent, &mut ULayout)>,
    text_query: Query<&mut UText>,
) {
    switch_tab(
        trigger.entity.entity(),
        &mut commands,
        triggers,
        contents,
        text_query,
    );
}

fn on_tab_activate(
    trigger: On<UFocusActivate>,
    mut commands: Commands,
    triggers: Query<(Entity, &mut LumaTabTrigger, &mut UNode, &Children)>,
    contents: Query<(&LumaTabContent, &mut ULayout)>,
    text_query: Query<&mut UText>,
) {
    switch_tab(
        trigger.entity,
        &mut commands,
        triggers,
        contents,
        text_query,
    );
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
