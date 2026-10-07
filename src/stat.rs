//! Dashboard metric / KPI stat card widget.

use bevy::prelude::*;
use bevy::scene::{ResolveContext, ResolvedScene, ResolveSceneError};
use bevy_luma::prelude::*;

/// Directional trend indicator for metric statistics.
#[derive(Clone, Debug, PartialEq, Eq, Reflect)]
pub enum StatTrend {
    Up(String),
    Down(String),
    Neutral(String),
}

impl StatTrend {
    pub fn colors(&self) -> (Color, Color, &'static str) {
        // (background_tint, text_and_icon_color, icon)
        match self {
            StatTrend::Up(_) => (
                Color::srgba(0.16, 0.75, 0.45, 0.16),
                Color::srgb(0.35, 0.90, 0.55),
                Icon::ARROW_UP,
            ),
            StatTrend::Down(_) => (
                Color::srgba(0.85, 0.22, 0.28, 0.16),
                Color::srgb(1.00, 0.45, 0.50),
                Icon::ARROW_DOWN,
            ),
            StatTrend::Neutral(_) => (
                Color::srgba(0.50, 0.55, 0.68, 0.16),
                Color::srgb(0.70, 0.75, 0.85),
                "",
            ),
        }
    }

    pub fn text(&self) -> &str {
        match self {
            StatTrend::Up(t) => t.as_str(),
            StatTrend::Down(t) => t.as_str(),
            StatTrend::Neutral(t) => t.as_str(),
        }
    }
}

/// Marker component for a stat / KPI metric card.
#[derive(Component, Clone, Debug, Default, Reflect)]
pub struct LumaStatCard;

const STAT_CARD_BG: Color = Color::srgb(0.11, 0.13, 0.18);
const STAT_CARD_BORDER: Color = Color::srgb(0.20, 0.24, 0.32);
const STAT_TITLE_COLOR: Color = Color::srgb(0.55, 0.60, 0.72);
const STAT_VALUE_COLOR: Color = Color::srgb(0.96, 0.97, 0.99);

/// Declarative StatCard widget struct with all configurable properties.
#[derive(Clone, Debug, Reflect)]
pub struct StatCard {
    pub title: String,
    pub value: String,
    pub trend: Option<StatTrend>,
    pub description: Option<String>,
    pub icon: Option<String>,
    pub font: Handle<Font>,
    pub icon_font: Handle<Font>,
}

impl Default for StatCard {
    fn default() -> Self {
        Self {
            title: String::new(),
            value: String::new(),
            trend: None,
            description: None,
            icon: None,
            font: Handle::default(),
            icon_font: Handle::default(),
        }
    }
}

impl StatCard {
    pub fn new(title: impl Into<String>, value: impl Into<String>) -> Self {
        Self {
            title: title.into(),
            value: value.into(),
            ..default()
        }
    }

    pub fn trend(mut self, trend: StatTrend) -> Self {
        self.trend = Some(trend);
        self
    }

    pub fn description(mut self, desc: impl Into<String>) -> Self {
        self.description = Some(desc.into());
        self
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
}

impl Scene for StatCard {
    fn resolve(
        self,
        context: &mut ResolveContext,
        scene: &mut ResolvedScene,
    ) -> Result<(), ResolveSceneError> {
        let (trend_bg, trend_color, trend_icon, trend_text, has_trend) = if let Some(ref tr) = self.trend {
            let (bg, tc, ic) = tr.colors();
            (bg, tc, ic, tr.text().to_string(), true)
        } else {
            (Color::NONE, Color::NONE, "", String::new(), false)
        };

        let has_trend_icon = !trend_icon.is_empty();
        let header_icon = self.icon.unwrap_or_default();
        let has_header_icon = !header_icon.is_empty();
        let has_desc = self.description.is_some();
        let desc_text = self.description.unwrap_or_default();
        let title = self.title;
        let value = self.value;
        let font = self.font;
        let icon_font = self.icon_font;

        let s = bsn! {
            LumaStatCard
            UNode {
                min_width: 220.0,
                background_color: STAT_CARD_BG,
                border_radius: UCornerRadius::all(14.0),
                padding: USides::all(18.0),
            }
            UBorder {
                color: STAT_CARD_BORDER,
                width: 1.0,
                radius: UCornerRadius::all(14.0),
            }
            ULayout {
                display: UDisplay::Flex,
                flex_direction: UFlexDirection::Column,
                gap: 10.0,
            }
            Children [
                // Header: Title & Optional Icon
                (
                    UNode::default()
                    ULayout {
                        display: UDisplay::Flex,
                        flex_direction: UFlexDirection::Row,
                        align_items: UAlignItems::Center,
                        justify_content: UJustifyContent::SpaceBetween,
                    }
                    Children [
                        (UText {
                            text: title,
                            font_size: 13.0,
                            font: { font.clone() },
                            color: STAT_TITLE_COLOR,
                        }),
                        (
                            UNode {
                                width: UVal::Px(28.0),
                                height: UVal::Px(28.0),
                                background_color: Color::srgba(0.20, 0.45, 0.90, 0.15),
                                border_radius: UCornerRadius::all(6.0),
                            }
                            ULayout {
                                display: { if has_header_icon { UDisplay::Flex } else { UDisplay::None } },
                                justify_content: UJustifyContent::Center,
                                align_items: UAlignItems::Center,
                            }
                            Children [
                                (UText {
                                    text: header_icon,
                                    font_size: 14.0,
                                    font: { icon_font.clone() },
                                    color: Color::srgb(0.40, 0.65, 1.00),
                                })
                            ]
                        )
                    ]
                ),
                // Value & Optional Trend Pill
                (
                    UNode::default()
                    ULayout {
                        display: UDisplay::Flex,
                        flex_direction: UFlexDirection::Row,
                        align_items: UAlignItems::Baseline,
                        justify_content: UJustifyContent::SpaceBetween,
                    }
                    Children [
                        (UText {
                            text: value,
                            font_size: 26.0,
                            font: { font.clone() },
                            color: STAT_VALUE_COLOR,
                        }),
                        (
                            UNode {
                                background_color: trend_bg,
                                border_radius: UCornerRadius::all(6.0),
                                padding: USides::axes(6.0, 3.0),
                            }
                            ULayout {
                                display: { if has_trend { UDisplay::Flex } else { UDisplay::None } },
                                flex_direction: UFlexDirection::Row,
                                align_items: UAlignItems::Center,
                                gap: 3.0,
                            }
                            Children [
                                (
                                    UText {
                                        text: { if has_trend_icon { trend_icon.to_string() } else { String::new() } },
                                        font_size: 11.0,
                                        font: { icon_font.clone() },
                                        color: trend_color,
                                    }
                                ),
                                (
                                    UText {
                                        text: trend_text,
                                        font_size: 11.0,
                                        font: { font.clone() },
                                        color: trend_color,
                                    }
                                )
                            ]
                        )
                    ]
                ),
                // Optional Footer Description
                (
                    UNode::default()
                    ULayout {
                        display: { if has_desc { UDisplay::Flex } else { UDisplay::None } },
                    }
                    Children [
                        (UText {
                            text: desc_text,
                            font_size: 12.0,
                            font,
                            color: Color::srgb(0.45, 0.50, 0.60),
                        })
                    ]
                )
            ]
        };

        s.resolve(context, scene)
    }
}

/// Creates a minimal stat card with a title and primary value.
pub fn luma_stat_card(
    title: impl Into<String>,
    value: impl Into<String>,
    font: Handle<Font>,
) -> StatCard {
    StatCard::new(title, value).font(font)
}

/// Creates a stat card with a title, primary value, and trend change indicator.
pub fn luma_stat_card_with_trend(
    title: impl Into<String>,
    value: impl Into<String>,
    trend: StatTrend,
    font: Handle<Font>,
    icon_font: Handle<Font>,
) -> StatCard {
    StatCard::new(title, value)
        .trend(trend)
        .font(font)
        .icon_font(icon_font)
}

/// Creates a rich stat card with title, icon, value, trend badge, and helper description.
pub fn luma_stat_card_full(
    title: impl Into<String>,
    value: impl Into<String>,
    trend: StatTrend,
    description: impl Into<String>,
    icon: Option<&str>,
    font: Handle<Font>,
    icon_font: Handle<Font>,
) -> StatCard {
    let mut card = StatCard::new(title, value)
        .trend(trend)
        .description(description)
        .font(font)
        .icon_font(icon_font);
    if let Some(ic) = icon {
        card = card.icon(ic);
    }
    card
}

pub struct LumaStatPlugin;

impl Plugin for LumaStatPlugin {
    fn build(&self, app: &mut App) {
        app.register_type::<LumaStatCard>()
            .register_type::<StatTrend>();
    }
}
