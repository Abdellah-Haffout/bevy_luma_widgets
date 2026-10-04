//! Alert and callout notification widget for displaying contextual status messages.

use bevy::prelude::*;
use bevy_luma::prelude::*;

/// Semantic severity variant for an alert box.
#[derive(Component, Clone, Copy, Debug, PartialEq, Eq, Default, Reflect)]
pub enum AlertVariant {
    #[default]
    Info,
    Success,
    Warning,
    Danger,
}

impl AlertVariant {
    pub fn colors(&self) -> (Color, Color, Color, &'static str) {
        match self {
            AlertVariant::Info => (
                Color::srgba(0.10, 0.25, 0.50, 0.25),
                Color::srgb(0.20, 0.45, 0.90),
                Color::srgb(0.55, 0.75, 1.00),
                Icon::INFO,
            ),
            AlertVariant::Success => (
                Color::srgba(0.08, 0.38, 0.22, 0.25),
                Color::srgb(0.12, 0.65, 0.36),
                Color::srgb(0.40, 0.90, 0.60),
                Icon::CIRCLE_CHECK_BIG,
            ),
            AlertVariant::Warning => (
                Color::srgba(0.45, 0.30, 0.08, 0.25),
                Color::srgb(0.85, 0.55, 0.12),
                Color::srgb(1.00, 0.80, 0.40),
                Icon::TRIANGLE_ALERT,
            ),
            AlertVariant::Danger => (
                Color::srgba(0.48, 0.12, 0.15, 0.25),
                Color::srgb(0.90, 0.22, 0.28),
                Color::srgb(1.00, 0.60, 0.65),
                Icon::CIRCLE_ALERT,
            ),
        }
    }
}

/// Marker component for an alert banner.
#[derive(Component, Clone, Copy, Debug, Default, Reflect)]
pub struct LumaAlert {
    pub variant: AlertVariant,
}

/// Creates a rich semantic alert box scene.
pub fn luma_alert(
    title: impl Into<String>,
    description: impl Into<String>,
    variant: AlertVariant,
    font: Handle<Font>,
    icon_font: Handle<Font>,
) -> impl Scene {
    let title = title.into();
    let description = description.into();
    let (bg_color, border_color, accent_color, icon_char) = variant.colors();

    bsn! {
        LumaAlert {
            variant,
        }
        UNode {
            width: UVal::Percent(1.0),
            padding: USides::all(16.0),
            background_color: bg_color,
            border_radius: UCornerRadius::all(8.0),
        }
        UBorder {
            color: border_color,
            width: 1.0,
            radius: UCornerRadius::all(8.0),
        }
        ULayout {
            display: UDisplay::Flex,
            flex_direction: UFlexDirection::Row,
            gap: 14.0,
            align_items: UAlignItems::FlexStart,
        }
        Children [
            // Status Icon
            (UText {
                text: { icon_char.to_string() },
                font_size: 20.0,
                font: icon_font,
                color: accent_color,
            }),
            // Text Column (Title + Description)
            (
                UNode::default()
                ULayout {
                    display: UDisplay::Flex,
                    flex_direction: UFlexDirection::Column,
                    gap: 4.0,
                }
                Children [
                    (UText {
                        text: title,
                        font_size: 14.0,
                        font: { font.clone() },
                        color: Color::WHITE,
                    }),
                    (UText {
                        text: description,
                        font_size: 13.0,
                        font,
                        color: Color::srgb(0.75, 0.80, 0.88),
                    })
                ]
            )
        ]
    }
}
