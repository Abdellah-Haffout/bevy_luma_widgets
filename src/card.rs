//! Elevated card and panel containers.

use bevy::prelude::*;
use bevy::scene::{ResolveContext, ResolvedScene, ResolveSceneError};
use bevy_luma::prelude::*;

use crate::theme::CardStyle;

/// Marker and metadata component for an elevated card container.
#[derive(Component, Clone, Debug, Default, Reflect)]
pub struct LumaCard {
    pub has_explicit_style: bool,
    pub has_explicit_padding: bool,
    pub has_explicit_bg: bool,
    pub has_explicit_border: bool,
}

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
        let defaults = CardStyle::default();
        let has_explicit_style = self.style.is_some();
        let has_explicit_padding = (self.padding - defaults.padding).abs() > f32::EPSILON;
        let (bg, border_color, border_width, radius, padding) = if let Some(ref s) = self.style {
            (s.bg_color, s.border_color, s.border_width, s.radius, s.padding)
        } else {
            (defaults.bg_color, defaults.border_color, defaults.border_width, defaults.radius, self.padding)
        };

        let width = self.width;
        let gap = self.gap;

        let s = bsn! {
            LumaCard {
                has_explicit_style,
                has_explicit_padding,
                has_explicit_bg: false,
                has_explicit_border: false,
            }
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
