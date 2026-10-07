//! Contextual tooltip bubble widget for showing helpful descriptions on hover and focus.

use bevy::prelude::*;
use bevy_luma::prelude::*;

/// Marker component identifying a tooltip bubble.
#[derive(Component, Clone, Debug, Default, Reflect)]
pub struct LumaTooltip;

/// Marker for an element that triggers a tooltip on hover/focus.
#[derive(Component, Clone, Debug, Reflect)]
pub struct LumaTooltipTrigger {
    pub tooltip_entity: Option<Entity>,
}

use bevy::scene::{ResolveContext, ResolvedScene, ResolveSceneError};

/// Declarative Tooltip widget struct with all configurable properties.
#[derive(Clone, Debug, Reflect)]
pub struct Tooltip {
    pub text: String,
    pub font: Handle<Font>,
    pub font_size: f32,
    pub bg_color: Color,
    pub border_color: Color,
}

impl Default for Tooltip {
    fn default() -> Self {
        Self {
            text: String::new(),
            font: Handle::default(),
            font_size: 12.0,
            bg_color: Color::srgb(0.08, 0.10, 0.14),
            border_color: Color::srgb(0.25, 0.30, 0.42),
        }
    }
}

impl Tooltip {
    pub fn new(text: impl Into<String>) -> Self {
        Self {
            text: text.into(),
            ..default()
        }
    }

    pub fn font(mut self, font: Handle<Font>) -> Self {
        self.font = font;
        self
    }

    pub fn font_size(mut self, font_size: f32) -> Self {
        self.font_size = font_size;
        self
    }

    pub fn bg_color(mut self, bg_color: Color) -> Self {
        self.bg_color = bg_color;
        self
    }

    pub fn border_color(mut self, border_color: Color) -> Self {
        self.border_color = border_color;
        self
    }
}

impl Scene for Tooltip {
    fn resolve(
        self,
        context: &mut ResolveContext,
        scene: &mut ResolvedScene,
    ) -> Result<(), ResolveSceneError> {
        let text = self.text;
        let font = self.font;
        let font_size = self.font_size;
        let bg_color = self.bg_color;
        let border_color = self.border_color;

        let s = bsn! {
            LumaTooltip
            UNode {
                padding: USides::axes(10.0, 6.0),
                background_color: bg_color,
                border_radius: UCornerRadius::all(6.0),
            }
            UBorder {
                color: border_color,
                width: 1.0,
                radius: UCornerRadius::all(6.0),
            }
            ULayout {
                display: UDisplay::Flex,
                justify_content: UJustifyContent::Center,
                align_items: UAlignItems::Center,
            }
            Children [
                (UText {
                    text,
                    font_size,
                    font,
                    color: Color::srgb(0.92, 0.95, 0.98),
                })
            ]
        };
        s.resolve(context, scene)
    }
}

/// Creates a styled floating tooltip bubble scene.
pub fn luma_tooltip(
    text: impl Into<String>,
    font: Handle<Font>,
) -> Tooltip {
    Tooltip::new(text).font(font)
}
