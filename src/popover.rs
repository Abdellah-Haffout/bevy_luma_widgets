//! Floating popover overlay card widget.

use bevy::prelude::*;
use bevy::scene::{ResolveContext, ResolvedScene, ResolveSceneError};
use bevy_luma::prelude::*;

use crate::helpers::is_primary_click;
use crate::theme::PopoverStyle;

/// Marker component for an open popover card.
#[derive(Component, Clone, Debug, Default, Reflect)]
pub struct LumaPopover {
    pub is_open: bool,
    pub style: PopoverStyle,
    pub has_explicit_style: bool,
}

/// Event triggered to close an open popover.
#[derive(Event, Clone, Copy, Debug)]
pub struct ClosePopover(pub Entity);

/// Declarative Popover overlay widget struct.
#[derive(Clone, Debug, Reflect)]
pub struct Popover {
    pub is_open: bool,
    pub width: UVal,
    pub height: UVal,
    pub style: Option<PopoverStyle>,
}

impl Default for Popover {
    fn default() -> Self {
        Self {
            is_open: true,
            width: UVal::Auto,
            height: UVal::Auto,
            style: None,
        }
    }
}

impl Popover {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn open(mut self, is_open: bool) -> Self {
        self.is_open = is_open;
        self
    }

    pub fn width(mut self, width: UVal) -> Self {
        self.width = width;
        self
    }

    pub fn height(mut self, height: UVal) -> Self {
        self.height = height;
        self
    }

    pub fn style(mut self, style: PopoverStyle) -> Self {
        self.style = Some(style);
        self
    }
}

impl Scene for Popover {
    fn resolve(
        self,
        context: &mut ResolveContext,
        scene: &mut ResolvedScene,
    ) -> Result<(), ResolveSceneError> {
        let has_explicit_style = self.style.is_some();
        let style = self.style.unwrap_or_default();
        let is_open = self.is_open;
        let width = self.width;
        let height = self.height;
        let bg_color = style.bg_color;
        let border_color = style.border_color;
        let border_width = style.border_width;
        let radius = style.radius;
        let padding = style.padding;
        let popover_style = style.clone();

        let s = bsn! {
            LumaPopover {
                is_open,
                style: popover_style,
                has_explicit_style,
            }
            UNode {
                width,
                height,
                background_color: bg_color,
                border_radius: UCornerRadius::all(radius),
                padding,
            }
            UZIndex::Global(100)
            UShadow::drop(Color::srgba(0.0, 0.0, 0.0, 0.6), Vec2::new(0.0, 8.0), 20.0)
            UBorder {
                color: border_color,
                width: border_width,
                radius: UCornerRadius::all(radius),
            }
            UInteraction::default()
            ULayout {
                display: { if is_open { UDisplay::Flex } else { UDisplay::None } },
                flex_direction: UFlexDirection::Column,
                gap: 8.0,
            }
        };

        s.resolve(context, scene)
    }
}

/// Helper constructor to spawn a popover scene.
pub fn luma_popover() -> Popover {
    Popover::new()
}

fn on_popover_outside_click(
    trigger: On<PointerClick>,
    mut commands: Commands,
    popover_query: Query<(Entity, &LumaPopover)>,
    parents_query: Query<&ChildOf>,
) {
    if !is_primary_click(&trigger) {
        return;
    }

    let clicked = trigger.entity;

    for (popover_entity, popover) in popover_query.iter() {
        if !popover.is_open {
            continue;
        }

        let is_inside = clicked == popover_entity
            || parents_query
                .iter_ancestors(clicked)
                .any(|e| e == popover_entity);

        if !is_inside {
            commands.trigger(ClosePopover(popover_entity));
        }
    }
}

fn close_popover_on_escape(
    mut commands: Commands,
    keyboard: Option<Res<ButtonInput<KeyCode>>>,
    popover_query: Query<(Entity, &LumaPopover)>,
) {
    let Some(keyboard) = keyboard else {
        return;
    };
    if keyboard.just_pressed(KeyCode::Escape) {
        for (entity, popover) in popover_query.iter() {
            if popover.is_open {
                commands.trigger(ClosePopover(entity));
            }
        }
    }
}

fn on_close_popover_event(
    trigger: On<ClosePopover>,
    mut popover_query: Query<(&mut LumaPopover, &mut ULayout)>,
) {
    let entity = trigger.event().0;
    if let Ok((mut popover, mut layout)) = popover_query.get_mut(entity) {
        popover.is_open = false;
        layout.display = UDisplay::None;
    }
}

/// Plugin registering popover systems and observers.
pub struct LumaPopoverPlugin;

impl Plugin for LumaPopoverPlugin {
    fn build(&self, app: &mut App) {
        app.register_type::<LumaPopover>()
            .add_observer(on_popover_outside_click)
            .add_observer(on_close_popover_event)
            .add_systems(Update, close_popover_on_escape);
    }
}
