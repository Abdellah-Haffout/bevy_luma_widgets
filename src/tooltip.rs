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

/// Creates a styled floating tooltip bubble scene.
pub fn luma_tooltip(
    text: impl Into<String>,
    font: Handle<Font>,
) -> impl Scene {
    let text = text.into();

    bsn! {
        LumaTooltip
        UNode {
            padding: USides::axes(10.0, 6.0),
            background_color: Color::srgb(0.08, 0.10, 0.14),
            border_radius: UCornerRadius::all(6.0),
        }
        UBorder {
            color: Color::srgb(0.25, 0.30, 0.42),
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
                font_size: 12.0,
                font,
                color: Color::srgb(0.92, 0.95, 0.98),
            })
        ]
    }
}
