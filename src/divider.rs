//! Clean divider and separator lines.

use bevy::prelude::*;
use bevy_luma::prelude::*;

/// Orientation for a divider line.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Default, Reflect)]
pub enum DividerOrientation {
    #[default]
    Horizontal,
    Vertical,
}

/// Marker component for a divider widget.
#[derive(Component, Clone, Debug, Reflect)]
pub struct LumaDivider {
    pub orientation: DividerOrientation,
    pub thickness: f32,
}

impl Default for LumaDivider {
    fn default() -> Self {
        Self {
            orientation: DividerOrientation::Horizontal,
            thickness: 1.0,
        }
    }
}

const DIVIDER_COLOR: Color = Color::srgb(0.20, 0.24, 0.32);

/// Creates a horizontal divider scene spanning the available width.
pub fn luma_divider() -> impl Scene {
    bsn! {
        LumaDivider {
            orientation: DividerOrientation::Horizontal,
            thickness: 1.0,
        }
        UNode {
            width: UVal::Percent(1.0),
            height: UVal::Px(1.0),
            background_color: DIVIDER_COLOR,
        }
    }
}

/// Creates a vertical divider scene of the given height.
pub fn luma_divider_vertical(height: f32) -> impl Scene {
    bsn! {
        LumaDivider {
            orientation: DividerOrientation::Vertical,
            thickness: 1.0,
        }
        UNode {
            width: UVal::Px(1.0),
            height: UVal::Px(height),
            background_color: DIVIDER_COLOR,
        }
    }
}

/// Creates a horizontal divider with a centered label (e.g. "OR").
pub fn luma_divider_with_label(label: impl Into<String>, font: Handle<Font>) -> impl Scene {
    let label = label.into();

    bsn! {
        UNode {
            width: UVal::Percent(1.0),
        }
        ULayout {
            display: UDisplay::Flex,
            flex_direction: UFlexDirection::Row,
            align_items: UAlignItems::Center,
            gap: 12.0,
        }
        Children [
            (
                UNode {
                    height: UVal::Px(1.0),
                    background_color: DIVIDER_COLOR,
                }
                USelf {
                    flex_grow: { Some(1.0) },
                }
            ),
            (
                UText {
                    text: label,
                    font_size: 12.0,
                    font,
                    color: Color::srgb(0.50, 0.55, 0.65),
                }
            ),
            (
                UNode {
                    height: UVal::Px(1.0),
                    background_color: DIVIDER_COLOR,
                }
                USelf {
                    flex_grow: { Some(1.0) },
                }
            )
        ]
    }
}
