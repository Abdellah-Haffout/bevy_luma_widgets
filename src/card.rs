//! Elevated card and panel containers.

use bevy::prelude::*;
use bevy_luma::prelude::*;

/// Marker component for an elevated card container.
#[derive(Component, Clone, Debug, Default, Reflect)]
pub struct LumaCard;

const CARD_BG: Color = Color::srgb(0.11, 0.13, 0.18);
const CARD_BORDER: Color = Color::srgb(0.20, 0.24, 0.32);

/// Creates an elevated card container scene.
pub fn luma_card(width: UVal, padding: f32) -> impl Scene {
    bsn! {
        LumaCard
        UNode {
            width,
            background_color: CARD_BG,
            border_radius: UCornerRadius::all(12.0),
            padding: USides::all(padding),
        }
        UBorder {
            color: CARD_BORDER,
            width: 1.0,
            radius: UCornerRadius::all(12.0),
        }
        ULayout {
            display: UDisplay::Flex,
            flex_direction: UFlexDirection::Column,
            gap: 16.0,
        }
    }
}
