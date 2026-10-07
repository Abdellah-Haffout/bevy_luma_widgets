//! Breadcrumb navigation trail widget.

use bevy::prelude::*;
use bevy::scene::{ResolveContext, ResolvedScene, ResolveSceneError};
use bevy_luma::prelude::*;

/// Marker component for a breadcrumb navigation trail container.
#[derive(Component, Clone, Debug, Default, Reflect)]
pub struct LumaBreadcrumb;

/// An individual navigation item inside a breadcrumb trail.
#[derive(Component, Clone, Debug, Default, Reflect)]
pub struct LumaBreadcrumbItem {
    pub index: usize,
    pub value: String,
    pub is_current: bool,
}

/// Event triggered when a non-current breadcrumb item is clicked.
#[derive(Event, Clone, Debug)]
pub struct BreadcrumbClicked {
    pub entity: Entity,
    pub index: usize,
    pub value: String,
}

const CRUMB_COLOR_NORMAL: Color = Color::srgb(0.55, 0.60, 0.72);
const CRUMB_COLOR_CURRENT: Color = Color::srgb(0.96, 0.97, 0.99);
const SEPARATOR_COLOR: Color = Color::srgb(0.35, 0.40, 0.50);

/// Declarative BreadcrumbItem widget struct.
#[derive(Clone, Debug, Reflect)]
pub struct BreadcrumbItem {
    pub index: usize,
    pub value: String,
    pub label: String,
    pub is_current: bool,
    pub font: Handle<Font>,
}

impl Default for BreadcrumbItem {
    fn default() -> Self {
        Self {
            index: 0,
            value: String::new(),
            label: String::new(),
            is_current: false,
            font: Handle::default(),
        }
    }
}

impl BreadcrumbItem {
    pub fn new(value: impl Into<String>, label: impl Into<String>) -> Self {
        Self {
            value: value.into(),
            label: label.into(),
            ..default()
        }
    }

    pub fn index(mut self, index: usize) -> Self {
        self.index = index;
        self
    }

    pub fn current(mut self, is_current: bool) -> Self {
        self.is_current = is_current;
        self
    }

    pub fn font(mut self, font: Handle<Font>) -> Self {
        self.font = font;
        self
    }
}

impl Scene for BreadcrumbItem {
    fn resolve(
        self,
        context: &mut ResolveContext,
        scene: &mut ResolvedScene,
    ) -> Result<(), ResolveSceneError> {
        let index = self.index;
        let value = self.value;
        let label = self.label;
        let is_current = self.is_current;
        let font = self.font;

        let text_color = if is_current { CRUMB_COLOR_CURRENT } else { CRUMB_COLOR_NORMAL };
        let hover_color = if is_current { Color::NONE } else { Color::srgba(1.0, 1.0, 1.0, 0.05) };
        let pressed_color = if is_current { Color::NONE } else { Color::srgba(1.0, 1.0, 1.0, 0.10) };

        let s = bsn! {
            LumaBreadcrumbItem {
                index,
                value,
                is_current,
            }
            UNode {
                padding: USides::axes(4.0, 2.0),
                border_radius: UCornerRadius::all(4.0),
            }
            UInteraction::default()
            UInteractionColors {
                normal: Color::NONE,
                hovered: hover_color,
                pressed: pressed_color,
            }
            UFocusable::new()
            UFocusVisual::border(Color::srgb(0.35, 0.65, 1.0), 1.5)
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
        };
        s.resolve(context, scene)
    }
}

/// Declarative BreadcrumbSeparator widget struct.
#[derive(Clone, Debug, Default, Reflect)]
pub struct BreadcrumbSeparator {
    pub icon_font: Handle<Font>,
}

impl BreadcrumbSeparator {
    pub fn new(icon_font: Handle<Font>) -> Self {
        Self { icon_font }
    }

    pub fn font(mut self, icon_font: Handle<Font>) -> Self {
        self.icon_font = icon_font;
        self
    }
}

impl Scene for BreadcrumbSeparator {
    fn resolve(
        self,
        context: &mut ResolveContext,
        scene: &mut ResolvedScene,
    ) -> Result<(), ResolveSceneError> {
        let icon_font = self.icon_font;
        let s = bsn! {
            UNode::default()
            ULayout {
                display: UDisplay::Flex,
                justify_content: UJustifyContent::Center,
                align_items: UAlignItems::Center,
            }
            Children [
                (UText {
                    text: { Icon::CHEVRON_RIGHT.to_string() },
                    font_size: 12.0,
                    font: icon_font,
                    color: SEPARATOR_COLOR,
                })
            ]
        };
        s.resolve(context, scene)
    }
}

/// Declarative BreadcrumbTrail container widget struct.
#[derive(Clone, Debug, Default, Reflect)]
pub struct BreadcrumbTrail;

impl Scene for BreadcrumbTrail {
    fn resolve(
        self,
        context: &mut ResolveContext,
        scene: &mut ResolvedScene,
    ) -> Result<(), ResolveSceneError> {
        let s = bsn! {
            LumaBreadcrumb
            UNode::default()
            ULayout {
                display: UDisplay::Flex,
                flex_direction: UFlexDirection::Row,
                align_items: UAlignItems::Center,
                gap: 6.0,
            }
        };
        s.resolve(context, scene)
    }
}

/// Creates an individual breadcrumb navigation item scene.
pub fn luma_breadcrumb_item(
    index: usize,
    value: impl Into<String>,
    label: impl Into<String>,
    is_current: bool,
    font: Handle<Font>,
) -> BreadcrumbItem {
    BreadcrumbItem::new(value, label).index(index).current(is_current).font(font)
}

/// Creates a breadcrumb separator icon scene (chevron right).
pub fn luma_breadcrumb_separator(icon_font: Handle<Font>) -> BreadcrumbSeparator {
    BreadcrumbSeparator::new(icon_font)
}

/// Creates a horizontal container for grouping breadcrumb items and separators.
pub fn luma_breadcrumb_trail(items: impl Scene) -> impl Scene {
    bsn! {
        LumaBreadcrumb
        UNode::default()
        ULayout {
            display: UDisplay::Flex,
            flex_direction: UFlexDirection::Row,
            align_items: UAlignItems::Center,
            gap: 6.0,
        }
        Children [
            items
        ]
    }
}

fn on_breadcrumb_click(
    trigger: On<Pointer<Click>>,
    mut commands: Commands,
    item_query: Query<&LumaBreadcrumbItem>,
) {
    let entity = trigger.entity.entity();
    if let Ok(item) = item_query.get(entity) {
        if !item.is_current {
            commands.trigger(BreadcrumbClicked {
                entity,
                index: item.index,
                value: item.value.clone(),
            });
        }
    }
}

fn on_breadcrumb_activate(
    trigger: On<UFocusActivate>,
    mut commands: Commands,
    item_query: Query<&LumaBreadcrumbItem>,
) {
    let entity = trigger.entity;
    if let Ok(item) = item_query.get(entity) {
        if !item.is_current {
            commands.trigger(BreadcrumbClicked {
                entity,
                index: item.index,
                value: item.value.clone(),
            });
        }
    }
}

pub struct LumaBreadcrumbPlugin;

impl Plugin for LumaBreadcrumbPlugin {
    fn build(&self, app: &mut App) {
        app.register_type::<LumaBreadcrumb>()
            .register_type::<LumaBreadcrumbItem>()
            .add_observer(on_breadcrumb_click)
            .add_observer(on_breadcrumb_activate);
    }
}
