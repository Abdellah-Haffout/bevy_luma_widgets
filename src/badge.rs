//! Badge and tag indicators with semantic variant styling.

use bevy::prelude::*;
use bevy_luma::prelude::*;

/// Semantic variant for badges.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Default, Reflect)]
pub enum BadgeVariant {
    #[default]
    Info,
    Success,
    Warning,
    Danger,
    Neutral,
}

impl BadgeVariant {
    pub fn colors(&self) -> (Color, Color, Color) {
        // (background, text, border)
        match self {
            BadgeVariant::Info => (
                Color::srgba(0.20, 0.50, 0.95, 0.16),
                Color::srgb(0.45, 0.75, 1.00),
                Color::srgba(0.20, 0.50, 0.95, 0.35),
            ),
            BadgeVariant::Success => (
                Color::srgba(0.16, 0.75, 0.45, 0.16),
                Color::srgb(0.35, 0.90, 0.55),
                Color::srgba(0.16, 0.75, 0.45, 0.35),
            ),
            BadgeVariant::Warning => (
                Color::srgba(0.95, 0.65, 0.15, 0.16),
                Color::srgb(1.00, 0.80, 0.35),
                Color::srgba(0.95, 0.65, 0.15, 0.35),
            ),
            BadgeVariant::Danger => (
                Color::srgba(0.85, 0.22, 0.28, 0.16),
                Color::srgb(1.00, 0.45, 0.50),
                Color::srgba(0.85, 0.22, 0.28, 0.35),
            ),
            BadgeVariant::Neutral => (
                Color::srgba(0.50, 0.55, 0.68, 0.16),
                Color::srgb(0.80, 0.85, 0.92),
                Color::srgba(0.50, 0.55, 0.68, 0.35),
            ),
        }
    }
}

/// Marker component for a badge.
#[derive(Component, Clone, Debug, Default, Reflect)]
pub struct LumaBadge {
    pub variant: BadgeVariant,
}

/// Creates a standard text badge scene.
pub fn luma_badge(
    label: impl Into<String>,
    variant: BadgeVariant,
    font: Handle<Font>,
) -> impl Scene {
    let label = label.into();
    let (bg, text_color, border_color) = variant.colors();

    bsn! {
        LumaBadge { variant }
        UNode {
            background_color: bg,
            border_radius: UCornerRadius::all(8.0),
            padding: USides::axes(8.0, 3.0),
        }
        UBorder {
            color: border_color,
            width: 1.0,
            radius: UCornerRadius::all(8.0),
        }
        ULayout {
            display: UDisplay::Flex,
            justify_content: UJustifyContent::Center,
            align_items: UAlignItems::Center,
        }
        Children [
            (UText {
                text: label,
                font_size: 11.0,
                font,
                color: text_color,
            })
        ]
    }
}

/// Creates a badge scene with an icon.
pub fn luma_badge_with_icon(
    icon: &str,
    label: impl Into<String>,
    variant: BadgeVariant,
    font: Handle<Font>,
    icon_font: Handle<Font>,
) -> impl Scene {
    let label = label.into();
    let icon = icon.to_string();
    let (bg, text_color, border_color) = variant.colors();

    bsn! {
        LumaBadge { variant }
        UNode {
            background_color: bg,
            border_radius: UCornerRadius::all(8.0),
            padding: USides::axes(8.0, 3.0),
        }
        UBorder {
            color: border_color,
            width: 1.0,
            radius: UCornerRadius::all(8.0),
        }
        ULayout {
            display: UDisplay::Flex,
            flex_direction: UFlexDirection::Row,
            justify_content: UJustifyContent::Center,
            align_items: UAlignItems::Center,
            gap: 4.0,
        }
        Children [
            (UText {
                text: icon,
                font_size: 12.0,
                font: icon_font,
                color: text_color,
            }),
            (UText {
                text: label,
                font_size: 11.0,
                font,
                color: text_color,
            })
        ]
    }
}
