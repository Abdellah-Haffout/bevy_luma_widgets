//! Modal dialog overlay with backdrop and action buttons.

use bevy::prelude::*;
use bevy_luma::prelude::*;

use bevy::scene::{ResolveContext, ResolvedScene, ResolveSceneError};

use crate::button::{luma_button, ButtonSize, ButtonVariant};

/// Marker component for the modal backdrop overlay.
#[derive(Component, Clone, Debug, Default, Reflect)]
pub struct LumaModal;

/// Marker component for the modal confirm action button.
#[derive(Component, Clone, Debug, Default, Reflect)]
pub struct LumaModalConfirm;

/// Marker component for the modal cancel action button.
#[derive(Component, Clone, Debug, Default, Reflect)]
pub struct LumaModalCancel;

/// Event triggered when the modal confirm button is clicked.
#[derive(Event, Clone, Copy, Debug)]
pub struct ModalConfirmed(pub Entity);

/// Event triggered when the modal cancel button or backdrop is clicked.
#[derive(Event, Clone, Copy, Debug)]
pub struct ModalCancelled(pub Entity);

/// Declarative Modal dialog widget struct with all configurable properties.
#[derive(Clone, Debug, Reflect)]
pub struct Modal {
    pub title: String,
    pub description: String,
    pub cancel_label: String,
    pub confirm_label: String,
    pub font: Handle<Font>,
    pub width: f32,
}

impl Default for Modal {
    fn default() -> Self {
        Self {
            title: String::new(),
            description: String::new(),
            cancel_label: "Cancel".to_string(),
            confirm_label: "Confirm".to_string(),
            font: Handle::default(),
            width: 440.0,
        }
    }
}

impl Modal {
    pub fn new(title: impl Into<String>, description: impl Into<String>) -> Self {
        Self {
            title: title.into(),
            description: description.into(),
            ..default()
        }
    }

    pub fn cancel_label(mut self, label: impl Into<String>) -> Self {
        self.cancel_label = label.into();
        self
    }

    pub fn confirm_label(mut self, label: impl Into<String>) -> Self {
        self.confirm_label = label.into();
        self
    }

    pub fn font(mut self, font: Handle<Font>) -> Self {
        self.font = font;
        self
    }

    pub fn width(mut self, width: f32) -> Self {
        self.width = width;
        self
    }
}

impl Scene for Modal {
    fn resolve(
        self,
        context: &mut ResolveContext,
        scene: &mut ResolvedScene,
    ) -> Result<(), ResolveSceneError> {
        let title = self.title;
        let description = self.description;
        let cancel_label = self.cancel_label;
        let confirm_label = self.confirm_label;
        let font = self.font;
        let width = self.width;

        let s = bsn! {
            LumaModal
            URootUi::screen()
            UNode {
                width: UVal::Percent(1.0),
                height: UVal::Percent(1.0),
                background_color: Color::srgba(0.0, 0.0, 0.0, 0.65),
            }
            ULayout {
                display: UDisplay::Flex,
                justify_content: UJustifyContent::Center,
                align_items: UAlignItems::Center,
            }
            Children [
                (
                    UNode {
                        width: { UVal::Px(width) },
                        background_color: Color::srgb(0.11, 0.13, 0.18),
                        border_radius: UCornerRadius::all(14.0),
                        padding: USides::all(24.0),
                    }
                    UBorder {
                        color: Color::srgb(0.22, 0.26, 0.36),
                        width: 1.0,
                        radius: UCornerRadius::all(14.0),
                    }
                    ULayout {
                        display: UDisplay::Flex,
                        flex_direction: UFlexDirection::Column,
                        gap: 16.0,
                    }
                    Children [
                        // Title
                        (UText {
                            text: title,
                            font_size: 18.0,
                            font: { font.clone() },
                            color: Color::srgb(0.96, 0.97, 0.99),
                        }),
                        // Description
                        (UText {
                            text: description,
                            font_size: 14.0,
                            font: { font.clone() },
                            color: Color::srgb(0.65, 0.70, 0.80),
                        }),
                        // Actions row
                        (
                            UNode {
                                width: UVal::Percent(1.0),
                            }
                            ULayout {
                                display: UDisplay::Flex,
                                flex_direction: UFlexDirection::Row,
                                justify_content: UJustifyContent::End,
                                gap: 10.0,
                            }
                            Children [
                                (
                                    LumaModalCancel
                                    luma_button(cancel_label, ButtonVariant::Secondary, ButtonSize::Medium, font.clone())
                                ),
                                (
                                    LumaModalConfirm
                                    luma_button(confirm_label, ButtonVariant::Primary, ButtonSize::Medium, font)
                                )
                            ]
                        )
                    ]
                )
            ]
        };
        s.resolve(context, scene)
    }
}

/// Creates a complete centered modal dialog scene.
pub fn luma_modal_dialog(
    title: impl Into<String>,
    description: impl Into<String>,
    cancel_label: impl Into<String>,
    confirm_label: impl Into<String>,
    font: Handle<Font>,
) -> Modal {
    Modal::new(title, description)
        .cancel_label(cancel_label)
        .confirm_label(confirm_label)
        .font(font)
}
