//! Star rating widget with display and interactive modes.

use bevy::ecs::relationship::Relationship;
use bevy::picking::pointer::PointerButton;
use bevy::prelude::*;
use bevy_luma::prelude::*;

/// Marker and state component for a star rating widget.
#[derive(Component, Clone, Debug, Reflect)]
pub struct LumaRating {
    pub value: u8,
    pub max_stars: u8,
    pub read_only: bool,
}

impl Default for LumaRating {
    fn default() -> Self {
        Self {
            value: 5,
            max_stars: 5,
            read_only: true,
        }
    }
}

/// Marker component for an individual star inside a rating widget.
#[derive(Component, Clone, Debug, Default, Reflect)]
pub struct LumaRatingStar {
    pub index: u8,
}

/// Event triggered when a user selects a star rating.
#[derive(Event, Clone, Copy, Debug)]
pub struct RatingChanged {
    pub entity: Entity,
    pub value: u8,
}

pub const STAR_ACTIVE_COLOR: Color = Color::srgb(1.0, 0.78, 0.18);
pub const STAR_INACTIVE_COLOR: Color = Color::srgb(0.28, 0.32, 0.42);

fn static_star_scene(index: u8, active: bool, icon_font: Handle<Font>) -> impl Scene {
    let color = if active { STAR_ACTIVE_COLOR } else { STAR_INACTIVE_COLOR };

    bsn! {
        LumaRatingStar { index }
        UNode {
            width: UVal::Px(20.0),
            height: UVal::Px(20.0),
        }
        ULayout {
            display: UDisplay::Flex,
            justify_content: UJustifyContent::Center,
            align_items: UAlignItems::Center,
        }
        Children [
            (UText {
                text: { Icon::STAR.to_string() },
                font_size: 15.0,
                font: icon_font,
                color,
            })
        ]
    }
}

fn interactive_star_scene(index: u8, active: bool, icon_font: Handle<Font>) -> impl Scene {
    let color = if active { STAR_ACTIVE_COLOR } else { STAR_INACTIVE_COLOR };

    bsn! {
        LumaRatingStar { index }
        UNode {
            width: UVal::Px(24.0),
            height: UVal::Px(24.0),
            border_radius: UCornerRadius::all(4.0),
        }
        UInteraction::default()
        UInteractionColors {
            normal: Color::NONE,
            hovered: Color::srgba(1.0, 1.0, 1.0, 0.08),
            pressed: Color::srgba(1.0, 1.0, 1.0, 0.15),
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
                text: { Icon::STAR.to_string() },
                font_size: 16.0,
                font: icon_font,
                color,
            })
        ]
    }
}

/// Declarative Rating widget struct with all configurable properties.
#[derive(Clone, Debug, Reflect)]
pub struct Rating {
    pub value: u8,
    pub max_stars: u8,
    pub interactive: bool,
    pub icon_font: Handle<Font>,
}

impl Default for Rating {
    fn default() -> Self {
        Self {
            value: 5,
            max_stars: 5,
            interactive: false,
            icon_font: Handle::default(),
        }
    }
}

impl Rating {
    pub fn new(value: u8) -> Self {
        Self {
            value,
            ..default()
        }
    }

    pub fn interactive_new(value: u8) -> Self {
        Self {
            value,
            interactive: true,
            ..default()
        }
    }

    pub fn max_stars(mut self, max: u8) -> Self {
        self.max_stars = max;
        self
    }

    pub fn interactive(mut self, interactive: bool) -> Self {
        self.interactive = interactive;
        self
    }

    pub fn icon_font(mut self, icon_font: Handle<Font>) -> Self {
        self.icon_font = icon_font;
        self
    }
}

use bevy::scene::{ResolveContext, ResolvedScene, ResolveSceneError};

impl Scene for Rating {
    fn resolve(
        self,
        context: &mut ResolveContext,
        scene: &mut ResolvedScene,
    ) -> Result<(), ResolveSceneError> {
        let value = self.value.min(5);

        if self.interactive {
            let s = bsn! {
                LumaRating {
                    value,
                    max_stars: 5,
                    read_only: false,
                }
                UNode::default()
                ULayout {
                    display: UDisplay::Flex,
                    flex_direction: UFlexDirection::Row,
                    align_items: UAlignItems::Center,
                    gap: 4.0,
                }
                Children [
                    (interactive_star_scene(1, value >= 1, self.icon_font.clone())),
                    (interactive_star_scene(2, value >= 2, self.icon_font.clone())),
                    (interactive_star_scene(3, value >= 3, self.icon_font.clone())),
                    (interactive_star_scene(4, value >= 4, self.icon_font.clone())),
                    (interactive_star_scene(5, value >= 5, self.icon_font))
                ]
            };
            s.resolve(context, scene)
        } else {
            let s = bsn! {
                LumaRating {
                    value,
                    max_stars: 5,
                    read_only: true,
                }
                UNode::default()
                ULayout {
                    display: UDisplay::Flex,
                    flex_direction: UFlexDirection::Row,
                    align_items: UAlignItems::Center,
                    gap: 2.0,
                }
                Children [
                    (static_star_scene(1, value >= 1, self.icon_font.clone())),
                    (static_star_scene(2, value >= 2, self.icon_font.clone())),
                    (static_star_scene(3, value >= 3, self.icon_font.clone())),
                    (static_star_scene(4, value >= 4, self.icon_font.clone())),
                    (static_star_scene(5, value >= 5, self.icon_font))
                ]
            };
            s.resolve(context, scene)
        }
    }
}

/// Creates a read-only 5-star rating display scene.
pub fn luma_rating(value: u8, icon_font: Handle<Font>) -> Rating {
    Rating::new(value).icon_font(icon_font)
}

/// Creates an interactive 5-star rating selector scene.
pub fn luma_interactive_rating(value: u8, icon_font: Handle<Font>) -> Rating {
    Rating::interactive_new(value).icon_font(icon_font)
}

fn set_rating(
    target_entity: Entity,
    commands: &mut Commands,
    parents_query: &Query<&ChildOf>,
    star_query: &Query<&LumaRatingStar>,
    rating_query: &mut Query<(Entity, &mut LumaRating, &Children)>,
    text_query: &mut Query<&mut UText>,
    children_query: &Query<&Children>,
) {
    let (selected_star_index, star_entity) = if let Ok(star) = star_query.get(target_entity) {
        (star.index, target_entity)
    } else if let Ok(parent) = parents_query.get(target_entity) {
        if let Ok(star) = star_query.get(parent.get()) {
            (star.index, parent.get())
        } else {
            return;
        }
    } else {
        return;
    };

    let mut current = star_entity;
    let mut rating_entity_opt = None;
    while let Ok(parent) = parents_query.get(current) {
        current = parent.get();
        if rating_query.contains(current) {
            rating_entity_opt = Some(current);
            break;
        }
    }

    let Some(rating_entity) = rating_entity_opt else {
        return;
    };

    let Ok((entity, mut rating, children)) = rating_query.get_mut(rating_entity) else {
        return;
    };

    if rating.read_only {
        return;
    }

    rating.value = selected_star_index;

    for child in children.iter() {
        if let Ok(star) = star_query.get(child) {
            let color = if star.index <= selected_star_index {
                STAR_ACTIVE_COLOR
            } else {
                STAR_INACTIVE_COLOR
            };

            if let Ok(inner_children) = children_query.get(child) {
                for inner_child in inner_children.iter() {
                    if let Ok(mut text) = text_query.get_mut(inner_child) {
                        text.color = color;
                    }
                }
            } else if let Ok(mut text) = text_query.get_mut(child) {
                text.color = color;
            }
        }
    }

    commands.trigger(RatingChanged {
        entity,
        value: selected_star_index,
    });
}

fn on_star_click(
    trigger: On<Pointer<Click>>,
    mut commands: Commands,
    parents_query: Query<&ChildOf>,
    star_query: Query<&LumaRatingStar>,
    mut rating_query: Query<(Entity, &mut LumaRating, &Children)>,
    mut text_query: Query<&mut UText>,
    children_query: Query<&Children>,
) {
    if trigger.event().button != PointerButton::Primary {
        return;
    }
    set_rating(
        trigger.entity.entity(),
        &mut commands,
        &parents_query,
        &star_query,
        &mut rating_query,
        &mut text_query,
        &children_query,
    );
}

fn on_star_activate(
    trigger: On<UFocusActivate>,
    mut commands: Commands,
    parents_query: Query<&ChildOf>,
    star_query: Query<&LumaRatingStar>,
    mut rating_query: Query<(Entity, &mut LumaRating, &Children)>,
    mut text_query: Query<&mut UText>,
    children_query: Query<&Children>,
) {
    set_rating(
        trigger.entity,
        &mut commands,
        &parents_query,
        &star_query,
        &mut rating_query,
        &mut text_query,
        &children_query,
    );
}

pub struct LumaRatingPlugin;

impl Plugin for LumaRatingPlugin {
    fn build(&self, app: &mut App) {
        app.register_type::<LumaRating>()
            .register_type::<LumaRatingStar>()
            .add_observer(on_star_click)
            .add_observer(on_star_activate);
    }
}
