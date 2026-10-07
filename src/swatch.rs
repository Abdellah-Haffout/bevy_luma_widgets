//! Color swatch widget for palette selection and color previews.

use bevy::picking::pointer::PointerButton;
use bevy::prelude::*;
use bevy::scene::{ResolveContext, ResolvedScene, ResolveSceneError};
use bevy_luma::prelude::*;

/// Marker and state component for a color swatch tile.
#[derive(Component, Clone, Debug, Reflect)]
pub struct LumaSwatch {
    pub color: Color,
    pub selected: bool,
}

impl Default for LumaSwatch {
    fn default() -> Self {
        Self {
            color: Color::WHITE,
            selected: false,
        }
    }
}

/// Event triggered when a color swatch is clicked/selected.
#[derive(Event, Clone, Copy, Debug)]
pub struct SwatchSelected {
    pub entity: Entity,
    pub color: Color,
}

const SWATCH_UNSELECTED_BORDER: Color = Color::srgba(1.0, 1.0, 1.0, 0.20);
const SWATCH_SELECTED_BORDER: Color = Color::WHITE;

/// Declarative Swatch widget struct with all configurable properties.
#[derive(Clone, Debug, Reflect)]
pub struct Swatch {
    pub color: Color,
    pub selected: bool,
    pub label: Option<String>,
    pub font: Handle<Font>,
}

impl Default for Swatch {
    fn default() -> Self {
        Self {
            color: Color::WHITE,
            selected: false,
            label: None,
            font: Handle::default(),
        }
    }
}

impl Swatch {
    pub fn new(color: Color) -> Self {
        Self {
            color,
            ..default()
        }
    }

    pub fn labeled(color: Color, label: impl Into<String>) -> Self {
        Self {
            color,
            label: Some(label.into()),
            ..default()
        }
    }

    pub fn selected(mut self, selected: bool) -> Self {
        self.selected = selected;
        self
    }

    pub fn font(mut self, font: Handle<Font>) -> Self {
        self.font = font;
        self
    }
}

impl Scene for Swatch {
    fn resolve(
        self,
        context: &mut ResolveContext,
        scene: &mut ResolvedScene,
    ) -> Result<(), ResolveSceneError> {
        let border_color = if self.selected {
            SWATCH_SELECTED_BORDER
        } else {
            SWATCH_UNSELECTED_BORDER
        };
        let border_width = if self.selected { 2.5 } else { 1.5 };
        let color = self.color;
        let selected = self.selected;
        let font = self.font;

        let tile = bsn! {
            LumaSwatch {
                color,
                selected,
            }
            UNode {
                width: UVal::Px(28.0),
                height: UVal::Px(28.0),
                background_color: color,
                border_radius: UCornerRadius::all(8.0),
            }
            UBorder {
                color: border_color,
                width: border_width,
                radius: UCornerRadius::all(8.0),
            }
            UInteraction::default()
            UFocusable::new()
            UFocusVisual::border(Color::srgb(0.35, 0.65, 1.0), 2.0)
        };

        if let Some(label_str) = self.label {
            let container = bsn! {
                UNode::default()
                ULayout {
                    display: UDisplay::Flex,
                    flex_direction: UFlexDirection::Row,
                    align_items: UAlignItems::Center,
                    gap: 10.0,
                }
                Children [
                    tile,
                    (UText {
                        text: label_str,
                        font_size: 13.0,
                        font,
                        color: Color::srgb(0.85, 0.88, 0.95),
                    })
                ]
            };
            container.resolve(context, scene)
        } else {
            tile.resolve(context, scene)
        }
    }
}

/// Creates a compact square/rounded color swatch scene.
pub fn luma_swatch(color: Color, selected: bool) -> Swatch {
    Swatch::new(color).selected(selected)
}

/// Creates a color swatch with an adjacent name or hex code label.
pub fn luma_swatch_with_label(
    color: Color,
    label: impl Into<String>,
    selected: bool,
    font: Handle<Font>,
) -> Swatch {
    Swatch::labeled(color, label).selected(selected).font(font)
}

fn toggle_swatch(
    target_entity: Entity,
    commands: &mut Commands,
    mut query: Query<(Entity, &mut LumaSwatch, &mut UBorder)>,
) {
    if let Ok((entity, mut swatch, mut border)) = query.get_mut(target_entity) {
        swatch.selected = !swatch.selected;
        let selected = swatch.selected;
        border.color = if selected {
            SWATCH_SELECTED_BORDER
        } else {
            SWATCH_UNSELECTED_BORDER
        };
        border.width = if selected { 2.5 } else { 1.5 };

        commands.trigger(SwatchSelected {
            entity,
            color: swatch.color,
        });
    }
}

fn on_swatch_click(
    trigger: On<Pointer<Click>>,
    mut commands: Commands,
    query: Query<(Entity, &mut LumaSwatch, &mut UBorder)>,
) {
    if trigger.event().button != PointerButton::Primary {
        return;
    }
    toggle_swatch(trigger.entity.entity(), &mut commands, query);
}

fn on_swatch_activate(
    trigger: On<UFocusActivate>,
    mut commands: Commands,
    query: Query<(Entity, &mut LumaSwatch, &mut UBorder)>,
) {
    toggle_swatch(trigger.entity, &mut commands, query);
}

pub struct LumaSwatchPlugin;

impl Plugin for LumaSwatchPlugin {
    fn build(&self, app: &mut App) {
        app.register_type::<LumaSwatch>()
            .add_observer(on_swatch_click)
            .add_observer(on_swatch_activate);
    }
}
