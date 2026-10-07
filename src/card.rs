//! Elevated card and panel containers.

use bevy::prelude::*;
use bevy::scene::{ResolveContext, ResolvedScene, ResolveSceneError};
use bevy_luma::prelude::*;

use crate::theme::CardStyle;

/// Marker component for an elevated card container.
#[derive(Component, Clone, Debug, Default, Reflect)]
pub struct LumaCard;

const CARD_BG: Color = Color::srgb(0.11, 0.13, 0.18);
const CARD_BORDER: Color = Color::srgb(0.20, 0.24, 0.32);

/// Declarative Card container widget struct with all configurable properties.
#[derive(Clone, Debug, Reflect)]
pub struct Card {
    pub width: UVal,
    pub padding: f32,
    pub gap: f32,
    pub style: Option<CardStyle>,
}

impl Default for Card {
    fn default() -> Self {
        Self {
            width: UVal::Percent(1.0),
            padding: 16.0,
            gap: 16.0,
            style: None,
        }
    }
}

impl Card {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn width(mut self, width: UVal) -> Self {
        self.width = width;
        self
    }

    pub fn padding(mut self, padding: f32) -> Self {
        self.padding = padding;
        self
    }

    pub fn gap(mut self, gap: f32) -> Self {
        self.gap = gap;
        self
    }

    pub fn style(mut self, style: CardStyle) -> Self {
        self.style = Some(style);
        self
    }
}

impl Scene for Card {
    fn resolve(
        self,
        context: &mut ResolveContext,
        scene: &mut ResolvedScene,
    ) -> Result<(), ResolveSceneError> {
        let (bg, border_color, border_width, radius, padding) = if let Some(ref s) = self.style {
            (s.bg_color, s.border_color, s.border_width, s.radius, s.padding)
        } else {
            (CARD_BG, CARD_BORDER, 1.0, 12.0, self.padding)
        };

        let width = self.width;
        let gap = self.gap;

        let s = bsn! {
            LumaCard
            UNode {
                width,
                background_color: bg,
                border_radius: UCornerRadius::all(radius),
                padding: USides::all(padding),
            }
            UBorder {
                color: border_color,
                width: border_width,
                radius: UCornerRadius::all(radius),
            }
            ULayout {
                display: UDisplay::Flex,
                flex_direction: UFlexDirection::Column,
                gap,
            }
        };

        s.resolve(context, scene)
    }
}

/// Creates an elevated card container scene.
pub fn luma_card(width: UVal, padding: f32) -> Card {
    Card::new().width(width).padding(padding)
}
