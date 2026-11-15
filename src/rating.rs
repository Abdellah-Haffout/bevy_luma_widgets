//! Star rating widget with display and interactive modes.

use bevy::prelude::*;
use bevy::scene::{ResolveContext, ResolvedScene, ResolveSceneError};
use bevy_luma::prelude::*;

use crate::helpers::{find_ancestor_or_self, is_primary_click, visit_descendants, DescendantVisit};
use crate::navigation::SceneList;
use crate::theme::WidgetColors;

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

pub const STAR_ACTIVE_COLOR: Color = Color::srgb(0.98, 0.78, 0.12);
pub const STAR_INACTIVE_COLOR: Color = Color::srgb(0.30, 0.30, 0.33);

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
            UText {
                text: { Icon::STAR.to_string() },
                font_size: 15.0,
                font: icon_font,
                color,
            }
        ]
    }
}

fn interactive_star_scene(index: u8, active: bool, icon_font: Handle<Font>) -> impl Scene {
    let color = if active { STAR_ACTIVE_COLOR } else { STAR_INACTIVE_COLOR };
    let focus_ring = WidgetColors::default().focus_ring();

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
        UFocusVisual::border(focus_ring, 1.5)
        ULayout {
            display: UDisplay::Flex,
            justify_content: UJustifyContent::Center,
            align_items: UAlignItems::Center,
        }
        Children [
            UText {
                text: { Icon::STAR.to_string() },
                font_size: 16.0,
                font: icon_font,
                color,
            }
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

impl Scene for Rating {
    fn resolve(
        self,
        context: &mut ResolveContext,
        scene: &mut ResolvedScene,
    ) -> Result<(), ResolveSceneError> {
        let max_stars = self.max_stars.max(1);
        let value = self.value.min(max_stars);

        if self.interactive {
            let stars: Vec<_> = (1..=max_stars)
                .map(|i| interactive_star_scene(i, value >= i, self.icon_font.clone()))
                .collect();
            let stars_scene = SceneList::new(stars);
            let s = bsn! {
                LumaRating {
                    value,
                    max_stars,
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
                    {stars_scene}
                ]
            };
            s.resolve(context, scene)
        } else {
            let stars: Vec<_> = (1..=max_stars)
                .map(|i| static_star_scene(i, value >= i, self.icon_font.clone()))
                .collect();
            let stars_scene = SceneList::new(stars);
            let s = bsn! {
                LumaRating {
                    value,
                    max_stars,
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
                    {stars_scene}
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
    let Some(star_entity) = find_ancestor_or_self(target_entity, star_query, parents_query) else {
        return;
    };
    let Ok(star) = star_query.get(star_entity) else {
        return;
    };
    let selected_star_index = star.index;

    let Some(rating_entity) = find_ancestor_or_self(star_entity, rating_query, parents_query) else {
        return;
    };

    let Ok((entity, mut rating, children)) = rating_query.get_mut(rating_entity) else {
        return;
    };

    if rating.read_only || rating.value == selected_star_index {
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
            visit_descendants(child, children_query, |e| {
                if let Ok(mut text) = text_query.get_mut(e) {
                    text.color = color;
                }
                DescendantVisit::Continue
            });
        }
    }

    commands.trigger(RatingChanged {
        entity,
        value: selected_star_index,
    });
}

fn on_star_click(
    trigger: On<PointerClick>,
    mut commands: Commands,
    parents_query: Query<&ChildOf>,
    star_query: Query<&LumaRatingStar>,
    mut rating_query: Query<(Entity, &mut LumaRating, &Children)>,
    mut text_query: Query<&mut UText>,
    children_query: Query<&Children>,
) {
    if !is_primary_click(&trigger) {
        return;
    }
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
