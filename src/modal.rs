//! Modal dialog overlay with backdrop and action buttons.

use bevy::prelude::*;
use bevy_luma::prelude::*;

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

/// Creates a complete centered modal dialog scene.
pub fn luma_modal_dialog(
    title: impl Into<String>,
    description: impl Into<String>,
    cancel_label: impl Into<String>,
    confirm_label: impl Into<String>,
    font: Handle<Font>,
) -> impl Scene {
    let title = title.into();
    let description = description.into();
    let cancel_label = cancel_label.into();
    let confirm_label = confirm_label.into();

    bsn! {
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
                    width: UVal::Px(440.0),
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
                        ULayout {
                            display: UDisplay::Flex,
                            flex_direction: UFlexDirection::Row,
                            justify_content: UJustifyContent::End,
                            gap: 10.0,
                        }
                        UNode {
                            width: UVal::Percent(1.0),
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
    }
}
