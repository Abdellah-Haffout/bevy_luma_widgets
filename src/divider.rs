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

use bevy::scene::{ResolveContext, ResolvedScene, ResolveSceneError};

/// Declarative Divider widget struct with all configurable properties.
#[derive(Clone, Debug, Reflect)]
pub struct Divider {
    pub orientation: DividerOrientation,
    pub thickness: f32,
    pub length: Option<f32>,
    pub color: Color,
    pub label: Option<String>,
    pub font: Handle<Font>,
}

impl Default for Divider {
    fn default() -> Self {
        Self {
            orientation: DividerOrientation::Horizontal,
            thickness: 1.0,
            length: None,
            color: DIVIDER_COLOR,
            label: None,
            font: Handle::default(),
        }
    }
}

impl Divider {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn horizontal() -> Self {
        Self {
            orientation: DividerOrientation::Horizontal,
            ..default()
        }
    }

    pub fn vertical(height: f32) -> Self {
        Self {
            orientation: DividerOrientation::Vertical,
            length: Some(height),
            ..default()
        }
    }

    pub fn thickness(mut self, thickness: f32) -> Self {
        self.thickness = thickness;
        self
    }

    pub fn color(mut self, color: Color) -> Self {
        self.color = color;
        self
    }

    pub fn label(mut self, label: impl Into<String>) -> Self {
        self.label = Some(label.into());
        self
    }

    pub fn font(mut self, font: Handle<Font>) -> Self {
        self.font = font;
        self
    }
}

impl Scene for Divider {
    fn resolve(
        self,
        context: &mut ResolveContext,
        scene: &mut ResolvedScene,
    ) -> Result<(), ResolveSceneError> {
        let orientation = self.orientation;
        let thickness = self.thickness;
        let color = self.color;

        if let Some(label_str) = self.label {
            let font = self.font;
            let s = bsn! {
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
                            height: { UVal::Px(thickness) },
                            background_color: color,
                        }
                        USelf {
                            flex_grow: { Some(1.0) },
                        }
                    ),
                    (
                        UText {
                            text: label_str,
                            font_size: 12.0,
                            font,
                            color: Color::srgb(0.50, 0.55, 0.65),
                        }
                    ),
                    (
                        UNode {
                            height: { UVal::Px(thickness) },
                            background_color: color,
                        }
                        USelf {
                            flex_grow: { Some(1.0) },
                        }
                    )
                ]
            };
            s.resolve(context, scene)
        } else {
            match orientation {
                DividerOrientation::Horizontal => {
                    let s = bsn! {
                        LumaDivider {
                            orientation,
                            thickness,
                        }
                        UNode {
                            width: UVal::Percent(1.0),
                            height: { UVal::Px(thickness) },
                            background_color: color,
                        }
                    };
                    s.resolve(context, scene)
                }
                DividerOrientation::Vertical => {
                    let h = self.length.unwrap_or(24.0);
                    let s = bsn! {
                        LumaDivider {
                            orientation,
                            thickness,
                        }
                        UNode {
                            width: { UVal::Px(thickness) },
                            height: { UVal::Px(h) },
                            background_color: color,
                        }
                    };
                    s.resolve(context, scene)
                }
            }
        }
    }
}

/// Creates a horizontal divider scene spanning the available width.
pub fn luma_divider() -> Divider {
    Divider::horizontal()
}

/// Creates a vertical divider scene of the given height.
pub fn luma_divider_vertical(height: f32) -> Divider {
    Divider::vertical(height)
}

/// Creates a horizontal divider with a centered label (e.g. "OR").
pub fn luma_divider_with_label(label: impl Into<String>, font: Handle<Font>) -> Divider {
    Divider::horizontal().label(label).font(font)
}
