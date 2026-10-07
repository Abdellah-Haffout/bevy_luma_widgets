//! Collapsible accordion component for expandable disclosure sections.

use bevy::prelude::*;
use bevy_luma::prelude::*;

/// Marker and state component for an accordion item.
#[derive(Component, Clone, Debug, Default, Reflect)]
pub struct LumaAccordionItem {
    pub is_expanded: bool,
}

/// Marker for the clickable header of an accordion.
#[derive(Component, Clone, Debug, Default, Reflect)]
pub struct LumaAccordionHeader;

/// Marker for the collapsible content container of an accordion.
#[derive(Component, Clone, Debug, Default, Reflect)]
pub struct LumaAccordionContent;

/// Marker for the directional chevron icon inside an accordion header.
#[derive(Component, Clone, Debug, Default, Reflect)]
pub struct LumaAccordionChevron;

/// Event fired when an accordion section expands or collapses.
#[derive(Event, Clone, Debug)]
pub struct AccordionToggled {
    pub entity: Entity,
    pub is_expanded: bool,
}

use bevy::scene::{ResolveContext, ResolvedScene, ResolveSceneError};

/// Declarative AccordionItem widget struct.
#[derive(Clone, Debug, Reflect)]
pub struct AccordionItem<C = ()> {
    pub title: String,
    pub is_expanded: bool,
    pub font: Handle<Font>,
    pub icon_font: Handle<Font>,
    pub content: C,
}

pub type Accordion<C = ()> = AccordionItem<C>;

impl Default for AccordionItem<()> {
    fn default() -> Self {
        Self {
            title: String::new(),
            is_expanded: false,
            font: Handle::default(),
            icon_font: Handle::default(),
            content: (),
        }
    }
}

impl AccordionItem<()> {
    pub fn new(title: impl Into<String>) -> Self {
        Self {
            title: title.into(),
            ..default()
        }
    }
}

impl<C> AccordionItem<C> {
    pub fn expanded(mut self, is_expanded: bool) -> Self {
        self.is_expanded = is_expanded;
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

    pub fn content<NewC: Scene>(self, content: NewC) -> AccordionItem<NewC> {
        AccordionItem {
            title: self.title,
            is_expanded: self.is_expanded,
            font: self.font,
            icon_font: self.icon_font,
            content,
        }
    }
}

impl<C: Scene> Scene for AccordionItem<C> {
    fn resolve(
        self,
        context: &mut ResolveContext,
        scene: &mut ResolvedScene,
    ) -> Result<(), ResolveSceneError> {
        let title = self.title;
        let is_expanded = self.is_expanded;
        let font = self.font;
        let icon_font = self.icon_font;
        let content_scene = self.content;

        let chevron_char = if is_expanded { Icon::CHEVRON_UP } else { Icon::CHEVRON_DOWN };
        let content_display = if is_expanded { UDisplay::Flex } else { UDisplay::None };

        let s = bsn! {
            LumaAccordionItem {
                is_expanded,
            }
            UNode {
                width: UVal::Percent(1.0),
                background_color: Color::srgb(0.10, 0.12, 0.17),
                border_radius: UCornerRadius::all(8.0),
            }
            UBorder {
                color: Color::srgb(0.20, 0.25, 0.35),
                width: 1.0,
                radius: UCornerRadius::all(8.0),
            }
            ULayout {
                display: UDisplay::Flex,
                flex_direction: UFlexDirection::Column,
            }
            Children [
                // Accordion Header Bar
                (
                    LumaAccordionHeader
                    UNode {
                        width: UVal::Percent(1.0),
                        padding: USides::axes(16.0, 12.0),
                        background_color: Color::NONE,
                    }
                    UInteraction::default()
                    UInteractionColors {
                        normal: Color::NONE,
                        hovered: Color::srgba(1.0, 1.0, 1.0, 0.04),
                        pressed: Color::srgba(1.0, 1.0, 1.0, 0.08),
                    }
                    UFocusable::new()
                    UFocusVisual::border(Color::srgb(0.35, 0.65, 1.0), 2.0)
                    ULayout {
                        display: UDisplay::Flex,
                        flex_direction: UFlexDirection::Row,
                        justify_content: UJustifyContent::SpaceBetween,
                        align_items: UAlignItems::Center,
                    }
                    Children [
                        (UText {
                            text: title,
                            font_size: 14.0,
                            font,
                            color: Color::srgb(0.92, 0.95, 0.98),
                        }),
                        (
                            LumaAccordionChevron
                            UText {
                                text: { chevron_char.to_string() },
                                font_size: 16.0,
                                font: icon_font,
                                color: Color::srgb(0.60, 0.65, 0.75),
                            }
                        )
                    ]
                ),
                // Accordion Content Body
                (
                    LumaAccordionContent
                    UNode {
                        width: UVal::Percent(1.0),
                        padding: USides::axes(16.0, 12.0),
                        background_color: Color::srgba(0.06, 0.08, 0.12, 0.5),
                    }
                    ULayout {
                        display: content_display,
                        flex_direction: UFlexDirection::Column,
                    }
                    Children [
                        content_scene
                    ]
                )
            ]
        };
        s.resolve(context, scene)
    }
}

/// Creates a collapsible accordion panel scene.
pub fn luma_accordion_item<C: Scene>(
    title: impl Into<String>,
    content_scene: C,
    is_expanded: bool,
    font: Handle<Font>,
    icon_font: Handle<Font>,
) -> AccordionItem<C> {
    AccordionItem::new(title)
        .content(content_scene)
        .expanded(is_expanded)
        .font(font)
        .icon_font(icon_font)
}

fn toggle_accordion(
    header_entity: Entity,
    commands: &mut Commands,
    header_query: Query<&ChildOf, With<LumaAccordionHeader>>,
    mut item_query: Query<(Entity, &mut LumaAccordionItem, &Children)>,
    mut content_query: Query<&mut ULayout, With<LumaAccordionContent>>,
    mut chevron_query: Query<&mut UText, With<LumaAccordionChevron>>,
    children_query: Query<&Children>,
) {
    let Ok(child_of) = header_query.get(header_entity) else {
        return;
    };
    let item_entity = child_of.parent();

    let Ok((entity, mut item, children)) = item_query.get_mut(item_entity) else {
        return;
    };

    item.is_expanded = !item.is_expanded;
    let is_expanded = item.is_expanded;

    // Traverse children to find Content and Chevron
    for child in children.iter() {
        if let Ok(mut layout) = content_query.get_mut(child) {
            layout.display = if is_expanded { UDisplay::Flex } else { UDisplay::None };
        }

        // Header has children containing Chevron
        if let Ok(sub_children) = children_query.get(child) {
            for sub_child in sub_children.iter() {
                if let Ok(mut text) = chevron_query.get_mut(sub_child) {
                    text.text = if is_expanded {
                        Icon::CHEVRON_UP.to_string()
                    } else {
                        Icon::CHEVRON_DOWN.to_string()
                    };
                }
            }
        }
    }

    commands.trigger(AccordionToggled {
        entity,
        is_expanded,
    });
}

fn on_accordion_header_click(
    trigger: On<Pointer<Click>>,
    mut commands: Commands,
    header_query: Query<&ChildOf, With<LumaAccordionHeader>>,
    item_query: Query<(Entity, &mut LumaAccordionItem, &Children)>,
    content_query: Query<&mut ULayout, With<LumaAccordionContent>>,
    chevron_query: Query<&mut UText, With<LumaAccordionChevron>>,
    children_query: Query<&Children>,
) {
    toggle_accordion(
        trigger.entity.entity(),
        &mut commands,
        header_query,
        item_query,
        content_query,
        chevron_query,
        children_query,
    );
}

fn on_accordion_header_activate(
    trigger: On<UFocusActivate>,
    mut commands: Commands,
    header_query: Query<&ChildOf, With<LumaAccordionHeader>>,
    item_query: Query<(Entity, &mut LumaAccordionItem, &Children)>,
    content_query: Query<&mut ULayout, With<LumaAccordionContent>>,
    chevron_query: Query<&mut UText, With<LumaAccordionChevron>>,
    children_query: Query<&Children>,
) {
    toggle_accordion(
        trigger.entity,
        &mut commands,
        header_query,
        item_query,
        content_query,
        chevron_query,
        children_query,
    );
}

pub struct LumaAccordionPlugin;

impl Plugin for LumaAccordionPlugin {
    fn build(&self, app: &mut App) {
        app.register_type::<LumaAccordionItem>()
            .register_type::<LumaAccordionHeader>()
            .register_type::<LumaAccordionContent>()
            .register_type::<LumaAccordionChevron>()
            .add_observer(on_accordion_header_click)
            .add_observer(on_accordion_header_activate);
    }
}
