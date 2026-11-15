//! Modal dialog overlay with backdrop and action buttons.

use bevy::prelude::*;
use bevy::scene::{ResolveContext, ResolvedScene, ResolveSceneError};
use bevy_luma::prelude::*;

use crate::button::{luma_button, ButtonSize, ButtonVariant};
use crate::helpers::{find_ancestor_or_self, is_primary_click};

/// Marker component for the modal backdrop overlay.
#[derive(Component, Clone, Debug, Default, Reflect)]
pub struct LumaModal;

/// Marker component for the inner dialog container box of a modal.
#[derive(Component, Clone, Debug, Default, Reflect)]
pub struct LumaModalDialog;

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
            UInteraction::default()
            ULayout {
                display: UDisplay::Flex,
                justify_content: UJustifyContent::Center,
                align_items: UAlignItems::Center,
            }
            Children [
                UNode {
                    width: { UVal::Px(width) },
                    background_color: Color::srgb(0.11, 0.13, 0.18),
                    border_radius: UCornerRadius::all(14.0),
                    padding: USides::all(24.0),
                }
                LumaModalDialog
                UInteraction::default()
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
                    UText {
                        text: title,
                        font_size: 18.0,
                        font: { font.clone() },
                        color: Color::srgb(0.96, 0.97, 0.99),
                    }
                    --
                    // Description
                    UText {
                        text: description,
                        font_size: 14.0,
                        font: { font.clone() },
                        color: Color::srgb(0.65, 0.70, 0.80),
                    }
                    --
                    // Actions row
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
                        UNode::default()
                        LumaModalCancel
                        Children [
                            @{ luma_button(cancel_label, ButtonVariant::Secondary, ButtonSize::Medium, font.clone()) }
                        ]
                        --
                        UNode::default()
                        LumaModalConfirm
                        Children [
                            @{ luma_button(confirm_label, ButtonVariant::Primary, ButtonSize::Medium, font) }
                        ]
                    ]
                ]
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

fn resolve_modal_interaction(
    target: Entity,
    allow_backdrop: bool,
    commands: &mut Commands,
    modal_query: &Query<Entity, With<LumaModal>>,
    confirm_query: &Query<Entity, With<LumaModalConfirm>>,
    cancel_query: &Query<Entity, With<LumaModalCancel>>,
    parents_query: &Query<&ChildOf>,
) {
    let Some(modal_entity) = find_ancestor_or_self(target, modal_query, parents_query) else {
        return;
    };

    if find_ancestor_or_self(target, confirm_query, parents_query).is_some() {
        commands.trigger(ModalConfirmed(modal_entity));
        if let Ok(mut entity_cmd) = commands.get_entity(modal_entity) {
            entity_cmd.try_despawn();
        }
    } else if find_ancestor_or_self(target, cancel_query, parents_query).is_some() {
        commands.trigger(ModalCancelled(modal_entity));
        if let Ok(mut entity_cmd) = commands.get_entity(modal_entity) {
            entity_cmd.try_despawn();
        }
    } else if allow_backdrop && modal_query.contains(target) {
        commands.trigger(ModalCancelled(modal_entity));
        if let Ok(mut entity_cmd) = commands.get_entity(modal_entity) {
            entity_cmd.try_despawn();
        }
    }
}

fn on_modal_click(
    trigger: On<PointerClick>,
    mut commands: Commands,
    modal_query: Query<Entity, With<LumaModal>>,
    confirm_query: Query<Entity, With<LumaModalConfirm>>,
    cancel_query: Query<Entity, With<LumaModalCancel>>,
    parents_query: Query<&ChildOf>,
) {
    if !is_primary_click(&trigger) {
        return;
    }

    resolve_modal_interaction(
        trigger.entity,
        true,
        &mut commands,
        &modal_query,
        &confirm_query,
        &cancel_query,
        &parents_query,
    );
}

fn on_modal_activate(
    trigger: On<UFocusActivate>,
    mut commands: Commands,
    modal_query: Query<Entity, With<LumaModal>>,
    confirm_query: Query<Entity, With<LumaModalConfirm>>,
    cancel_query: Query<Entity, With<LumaModalCancel>>,
    parents_query: Query<&ChildOf>,
) {
    resolve_modal_interaction(
        trigger.entity,
        false,
        &mut commands,
        &modal_query,
        &confirm_query,
        &cancel_query,
        &parents_query,
    );
}

fn close_modal_on_escape(
    mut commands: Commands,
    keyboard: Option<Res<ButtonInput<KeyCode>>>,
    modal_query: Query<Entity, With<LumaModal>>,
) {
    let Some(keyboard) = keyboard else {
        return;
    };
    if keyboard.just_pressed(KeyCode::Escape) {
        for modal_entity in modal_query.iter() {
            commands.trigger(ModalCancelled(modal_entity));
            if let Ok(mut entity_cmd) = commands.get_entity(modal_entity) {
                entity_cmd.try_despawn();
            }
        }
    }
}

/// Plugin registering modal dialog systems and observers.
pub struct LumaModalPlugin;

impl Plugin for LumaModalPlugin {
    fn build(&self, app: &mut App) {
        app.register_type::<LumaModal>()
            .register_type::<LumaModalDialog>()
            .register_type::<LumaModalConfirm>()
            .register_type::<LumaModalCancel>()
            .add_observer(on_modal_click)
            .add_observer(on_modal_activate)
            .add_systems(Update, close_modal_on_escape);
    }
}
