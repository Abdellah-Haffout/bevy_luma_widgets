//! User avatar widget with fallback initials and presence status indicators.

use bevy::prelude::*;
use bevy_luma::prelude::*;

/// Predefined sizes for avatar circles.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Default, Reflect)]
pub enum AvatarSize {
    Small,
    #[default]
    Medium,
    Large,
}

impl AvatarSize {
    pub fn size_px(&self) -> f32 {
        match self {
            AvatarSize::Small => 32.0,
            AvatarSize::Medium => 40.0,
            AvatarSize::Large => 48.0,
        }
    }

    pub fn font_size(&self) -> f32 {
        match self {
            AvatarSize::Small => 12.0,
            AvatarSize::Medium => 14.0,
            AvatarSize::Large => 18.0,
        }
    }

    pub fn status_size(&self) -> f32 {
        match self {
            AvatarSize::Small => 8.0,
            AvatarSize::Medium => 10.0,
            AvatarSize::Large => 12.0,
        }
    }
}

/// User presence status indicator.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Reflect)]
pub enum AvatarStatus {
    Online,
    Offline,
    Busy,
    Away,
}

impl AvatarStatus {
    pub fn color(&self) -> Color {
        match self {
            AvatarStatus::Online => Color::srgb(0.15, 0.75, 0.40),
            AvatarStatus::Offline => Color::srgb(0.40, 0.45, 0.55),
            AvatarStatus::Busy => Color::srgb(0.92, 0.25, 0.30),
            AvatarStatus::Away => Color::srgb(0.95, 0.65, 0.15),
        }
    }
}

/// Marker component for an avatar.
#[derive(Component, Clone, Debug, Default, Reflect)]
pub struct LumaAvatar;

use bevy::scene::{ResolveContext, ResolvedScene, ResolveSceneError};

/// Declarative Avatar widget struct with all configurable properties.
#[derive(Clone, Debug, Reflect)]
pub struct Avatar {
    pub initials: String,
    pub size: AvatarSize,
    pub status: Option<AvatarStatus>,
    pub font: Handle<Font>,
}

impl Default for Avatar {
    fn default() -> Self {
        Self {
            initials: String::new(),
            size: AvatarSize::Medium,
            status: None,
            font: Handle::default(),
        }
    }
}

impl Avatar {
    pub fn new(initials: impl Into<String>) -> Self {
        Self {
            initials: initials.into(),
            ..default()
        }
    }

    pub fn size(mut self, size: AvatarSize) -> Self {
        self.size = size;
        self
    }

    pub fn status(mut self, status: AvatarStatus) -> Self {
        self.status = Some(status);
        self
    }

    pub fn font(mut self, font: Handle<Font>) -> Self {
        self.font = font;
        self
    }
}

impl Scene for Avatar {
    fn resolve(
        self,
        context: &mut ResolveContext,
        scene: &mut ResolvedScene,
    ) -> Result<(), ResolveSceneError> {
        let initials = self.initials;
        let dim = self.size.size_px();
        let radius = dim * 0.5;
        let font_size = self.size.font_size();
        let font = self.font;

        if let Some(status) = self.status {
            let status_dim = self.size.status_size();
            let status_radius = status_dim * 0.5;
            let status_color = status.color();

            let s = bsn! {
                LumaAvatar
                UNode {
                    width: { UVal::Px(dim) },
                    height: { UVal::Px(dim) },
                    background_color: Color::srgb(0.18, 0.24, 0.36),
                    border_radius: { UCornerRadius::all(radius) },
                }
                UBorder {
                    color: Color::srgb(0.30, 0.40, 0.58),
                    width: 1.5,
                    radius: { UCornerRadius::all(radius) },
                }
                ULayout {
                    display: UDisplay::Flex,
                    justify_content: UJustifyContent::Center,
                    align_items: UAlignItems::Center,
                }
                Children [
                    (UText {
                        text: initials,
                        font_size,
                        font,
                        color: Color::srgb(0.92, 0.95, 1.00),
                    }),
                    // Status Presence Dot
                    (
                        UNode {
                            width: { UVal::Px(status_dim) },
                            height: { UVal::Px(status_dim) },
                            background_color: status_color,
                            border_radius: { UCornerRadius::all(status_radius) },
                        }
                        UBorder {
                            color: Color::srgb(0.08, 0.10, 0.14),
                            width: 1.5,
                            radius: { UCornerRadius::all(status_radius) },
                        }
                        USelf {
                            position_type: UPositionType::Absolute,
                            right: { UVal::Px(0.0) },
                            bottom: { UVal::Px(0.0) },
                        }
                    )
                ]
            };
            s.resolve(context, scene)
        } else {
            let s = bsn! {
                LumaAvatar
                UNode {
                    width: { UVal::Px(dim) },
                    height: { UVal::Px(dim) },
                    background_color: Color::srgb(0.18, 0.24, 0.36),
                    border_radius: { UCornerRadius::all(radius) },
                }
                UBorder {
                    color: Color::srgb(0.30, 0.40, 0.58),
                    width: 1.5,
                    radius: { UCornerRadius::all(radius) },
                }
                ULayout {
                    display: UDisplay::Flex,
                    justify_content: UJustifyContent::Center,
                    align_items: UAlignItems::Center,
                }
                Children [
                    (UText {
                        text: initials,
                        font_size,
                        font,
                        color: Color::srgb(0.92, 0.95, 1.00),
                    })
                ]
            };
            s.resolve(context, scene)
        }
    }
}

/// Creates a user avatar scene with initials fallback.
pub fn luma_avatar(
    initials: impl Into<String>,
    size: AvatarSize,
    font: Handle<Font>,
) -> Avatar {
    Avatar::new(initials).size(size).font(font)
}

/// Creates a user avatar scene with an active presence status badge dot.
pub fn luma_avatar_with_status(
    initials: impl Into<String>,
    size: AvatarSize,
    status: AvatarStatus,
    font: Handle<Font>,
) -> Avatar {
    Avatar::new(initials).size(size).status(status).font(font)
}
