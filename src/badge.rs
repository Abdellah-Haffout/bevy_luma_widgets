//! Badge and tag indicators with semantic variant styling and customizable styles.

use bevy::prelude::*;
use bevy::scene::{ResolveContext, ResolvedScene, ResolveSceneError};
use bevy_luma::prelude::*;

use crate::theme::BadgeStyle;

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

/// Declarative Badge widget struct with all configurable properties.
#[derive(Clone, Debug, Reflect)]
pub struct Badge {
    pub label: String,
    pub variant: BadgeVariant,
    pub icon: Option<String>,
    pub font: Handle<Font>,
    pub icon_font: Handle<Font>,
    pub style: Option<BadgeStyle>,
}

impl Default for Badge {
    fn default() -> Self {
        Self {
            label: String::new(),
            variant: BadgeVariant::Info,
            icon: None,
            font: Handle::default(),
            icon_font: Handle::default(),
            style: None,
        }
    }
}

impl Badge {
    pub fn new(label: impl Into<String>) -> Self {
        Self {
            label: label.into(),
            ..default()
        }
    }

    pub fn variant(mut self, variant: BadgeVariant) -> Self {
        self.variant = variant;
        self
    }

    pub fn info(label: impl Into<String>) -> Self {
        Self::new(label).variant(BadgeVariant::Info)
    }

    pub fn success(label: impl Into<String>) -> Self {
        Self::new(label).variant(BadgeVariant::Success)
    }

    pub fn warning(label: impl Into<String>) -> Self {
        Self::new(label).variant(BadgeVariant::Warning)
    }

    pub fn danger(label: impl Into<String>) -> Self {
        Self::new(label).variant(BadgeVariant::Danger)
    }

    pub fn neutral(label: impl Into<String>) -> Self {
        Self::new(label).variant(BadgeVariant::Neutral)
    }

    pub fn icon(mut self, icon: impl Into<String>) -> Self {
        self.icon = Some(icon.into());
        self
    }

    pub fn font(mut self, font: Handle<Font>) -> Self {
        self.font = font;
        self
    }

    pub fn icon_font(mut self, icon_font: Handle<Font>) -> Self {
        self.icon_font = icon_font;
        self
    }

    pub fn style(mut self, style: BadgeStyle) -> Self {
        self.style = Some(style);
        self
    }
}

impl Scene for Badge {
    fn resolve(
        self,
        context: &mut ResolveContext,
        scene: &mut ResolvedScene,
    ) -> Result<(), ResolveSceneError> {
        let (bg, text_color, border_color, radius, padding, font_size) = if let Some(ref s) = self.style {
            (s.bg_color, s.text_color, s.border_color, s.radius, s.padding, s.font_size)
        } else {
            let (bg, tc, bc) = self.variant.colors();
            (bg, tc, bc, 8.0, USides::axes(8.0, 3.0), 11.0)
        };

        let has_icon = self.icon.is_some();
        let icon_str = self.icon.unwrap_or_default();
        let variant = self.variant;
        let icon_font = self.icon_font;
        let label = self.label;
        let font = self.font;

        let s = bsn! {
            LumaBadge { variant }
            UNode {
                background_color: bg,
                border_radius: UCornerRadius::all(radius),
                padding,
            }
            UBorder {
                color: border_color,
                width: 1.0,
                radius: UCornerRadius::all(radius),
            }
            ULayout {
                display: UDisplay::Flex,
                flex_direction: UFlexDirection::Row,
                justify_content: UJustifyContent::Center,
                align_items: UAlignItems::Center,
                gap: 4.0,
            }
            Children [
                (
                    UNode::default()
                    ULayout {
                        display: { if has_icon { UDisplay::Flex } else { UDisplay::None } },
                    }
                    Children [
                        (UText {
                            text: icon_str,
                            font_size: 12.0,
                            font: icon_font,
                            color: text_color,
                        })
                    ]
                ),
                (UText {
                    text: label,
                    font_size,
                    font,
                    color: text_color,
                })
            ]
        };

        s.resolve(context, scene)
    }
}

/// Creates a standard text badge scene.
pub fn luma_badge(
    label: impl Into<String>,
    variant: BadgeVariant,
    font: Handle<Font>,
) -> Badge {
    Badge::new(label).variant(variant).font(font)
}

/// Creates a badge scene with an icon.
pub fn luma_badge_with_icon(
    icon: &str,
    label: impl Into<String>,
    variant: BadgeVariant,
    font: Handle<Font>,
    icon_font: Handle<Font>,
) -> Badge {
    Badge::new(label)
        .icon(icon)
        .variant(variant)
        .font(font)
        .icon_font(icon_font)
}
