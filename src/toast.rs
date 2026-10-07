//! Toast notification card widget with variant styles, dismiss button, and auto-dismiss timer.

use bevy::ecs::relationship::Relationship;
use bevy::prelude::*;
use bevy::scene::{ResolveContext, ResolvedScene, ResolveSceneError};
use bevy_luma::prelude::*;

/// Semantic styling variant for toast notifications.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Default, Reflect)]
pub enum ToastVariant {
    #[default]
    Info,
    Success,
    Warning,
    Danger,
}

impl ToastVariant {
    pub fn colors(&self) -> (Color, Color, Color, &'static str) {
        // (background, border, icon_color, icon)
        match self {
            ToastVariant::Info => (
                Color::srgb(0.10, 0.14, 0.22),
                Color::srgb(0.20, 0.35, 0.65),
                Color::srgb(0.40, 0.70, 1.00),
                Icon::INFO,
            ),
            ToastVariant::Success => (
                Color::srgb(0.08, 0.16, 0.13),
                Color::srgb(0.18, 0.45, 0.32),
                Color::srgb(0.35, 0.85, 0.55),
                Icon::CHECK,
            ),
            ToastVariant::Warning => (
                Color::srgb(0.18, 0.14, 0.08),
                Color::srgb(0.50, 0.38, 0.18),
                Color::srgb(1.00, 0.75, 0.30),
                Icon::TRIANGLE,
            ),
            ToastVariant::Danger => (
                Color::srgb(0.18, 0.10, 0.12),
                Color::srgb(0.55, 0.22, 0.26),
                Color::srgb(1.00, 0.45, 0.50),
                Icon::CIRCLE_X,
            ),
        }
    }
}

/// Marker component for a toast notification card.
#[derive(Component, Clone, Debug, Default, Reflect)]
pub struct LumaToast {
    pub variant: ToastVariant,
}

/// Component specifying an auto-dismiss countdown timer for a toast.
#[derive(Component, Clone, Debug, Reflect)]
pub struct LumaToastTimer {
    pub timer: Timer,
}

impl Default for LumaToastTimer {
    fn default() -> Self {
        Self {
            timer: Timer::from_seconds(5.0, TimerMode::Once),
        }
    }
}

/// Marker component for the close / dismiss button inside a toast.
#[derive(Component, Clone, Debug, Default, Reflect)]
pub struct LumaToastClose;

/// Event triggered when a toast notification is dismissed.
#[derive(Event, Clone, Copy, Debug)]
pub struct ToastDismissed(pub Entity);

const TOAST_WIDTH: f32 = 360.0;
const TOAST_TITLE_COLOR: Color = Color::srgb(0.96, 0.97, 0.99);
const TOAST_DESC_COLOR: Color = Color::srgb(0.68, 0.72, 0.82);

/// Declarative Toast widget struct with all configurable properties.
#[derive(Clone, Debug, Reflect)]
pub struct Toast {
    pub title: String,
    pub description: String,
    pub variant: ToastVariant,
    pub duration_secs: Option<f32>,
    pub font: Handle<Font>,
    pub icon_font: Handle<Font>,
}

impl Default for Toast {
    fn default() -> Self {
        Self {
            title: String::new(),
            description: String::new(),
            variant: ToastVariant::Info,
            duration_secs: None,
            font: Handle::default(),
            icon_font: Handle::default(),
        }
    }
}

impl Toast {
    pub fn new(title: impl Into<String>, description: impl Into<String>) -> Self {
        Self {
            title: title.into(),
            description: description.into(),
            ..default()
        }
    }

    pub fn variant(mut self, variant: ToastVariant) -> Self {
        self.variant = variant;
        self
    }

    pub fn info(title: impl Into<String>, description: impl Into<String>) -> Self {
        Self::new(title, description).variant(ToastVariant::Info)
    }

    pub fn success(title: impl Into<String>, description: impl Into<String>) -> Self {
        Self::new(title, description).variant(ToastVariant::Success)
    }

    pub fn warning(title: impl Into<String>, description: impl Into<String>) -> Self {
        Self::new(title, description).variant(ToastVariant::Warning)
    }

    pub fn danger(title: impl Into<String>, description: impl Into<String>) -> Self {
        Self::new(title, description).variant(ToastVariant::Danger)
    }

    pub fn timed(mut self, duration_secs: f32) -> Self {
        self.duration_secs = Some(duration_secs);
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

impl Scene for Toast {
    fn resolve(
        self,
        context: &mut ResolveContext,
        scene: &mut ResolvedScene,
    ) -> Result<(), ResolveSceneError> {
        let (bg, border, icon_color, icon) = self.variant.colors();
        let variant = self.variant;
        let title = self.title;
        let description = self.description;
        let font = self.font;
        let icon_font = self.icon_font;

        if let Some(duration_secs) = self.duration_secs {
            let s = bsn! {
                LumaToast { variant }
                LumaToastTimer {
                    timer: { Timer::from_seconds(duration_secs, TimerMode::Once) },
                }
                UNode {
                    width: UVal::Px(TOAST_WIDTH),
                    background_color: bg,
                    border_radius: UCornerRadius::all(10.0),
                    padding: USides::all(14.0),
                }
                UBorder {
                    color: border,
                    width: 1.0,
                    radius: UCornerRadius::all(10.0),
                }
                ULayout {
                    display: UDisplay::Flex,
                    flex_direction: UFlexDirection::Row,
                    align_items: UAlignItems::Start,
                    gap: 12.0,
                }
                Children [
                    // Status Icon Box
                    (
                        UNode {
                            width: UVal::Px(24.0),
                            height: UVal::Px(24.0),
                        }
                        ULayout {
                            display: UDisplay::Flex,
                            justify_content: UJustifyContent::Center,
                            align_items: UAlignItems::Center,
                        }
                        Children [
                            (UText {
                                text: { icon.to_string() },
                                font_size: 16.0,
                                font: { icon_font.clone() },
                                color: icon_color,
                            })
                        ]
                    ),
                    // Message Body (Title & Description)
                    (
                        UNode::default()
                        USelf {
                            flex_grow: { Some(1.0) },
                        }
                        ULayout {
                            display: UDisplay::Flex,
                            flex_direction: UFlexDirection::Column,
                            gap: 3.0,
                        }
                        Children [
                            (UText {
                                text: title,
                                font_size: 14.0,
                                font: { font.clone() },
                                color: TOAST_TITLE_COLOR,
                            }),
                            (UText {
                                text: description,
                                font_size: 12.0,
                                font: { font.clone() },
                                color: TOAST_DESC_COLOR,
                            })
                        ]
                    ),
                    // Close Button
                    (
                        LumaToastClose
                        UNode {
                            width: UVal::Px(20.0),
                            height: UVal::Px(20.0),
                            border_radius: UCornerRadius::all(4.0),
                        }
                        UInteraction::default()
                        UInteractionColors {
                            normal: Color::NONE,
                            hovered: Color::srgba(1.0, 1.0, 1.0, 0.10),
                            pressed: Color::srgba(1.0, 1.0, 1.0, 0.20),
                        }
                        UFocusable::new()
                        UFocusVisual::border(Color::srgb(0.35, 0.65, 1.0), 1.0)
                        ULayout {
                            display: UDisplay::Flex,
                            justify_content: UJustifyContent::Center,
                            align_items: UAlignItems::Center,
                        }
                        Children [
                            (UText {
                                text: { Icon::X.to_string() },
                                font_size: 13.0,
                                font: icon_font,
                                color: Color::srgb(0.60, 0.65, 0.75),
                            })
                        ]
                    )
                ]
            };
            s.resolve(context, scene)
        } else {
            let s = bsn! {
                LumaToast { variant }
                UNode {
                    width: UVal::Px(TOAST_WIDTH),
                    background_color: bg,
                    border_radius: UCornerRadius::all(10.0),
                    padding: USides::all(14.0),
                }
                UBorder {
                    color: border,
                    width: 1.0,
                    radius: UCornerRadius::all(10.0),
                }
                ULayout {
                    display: UDisplay::Flex,
                    flex_direction: UFlexDirection::Row,
                    align_items: UAlignItems::Start,
                    gap: 12.0,
                }
                Children [
                    // Status Icon Box
                    (
                        UNode {
                            width: UVal::Px(24.0),
                            height: UVal::Px(24.0),
                        }
                        ULayout {
                            display: UDisplay::Flex,
                            justify_content: UJustifyContent::Center,
                            align_items: UAlignItems::Center,
                        }
                        Children [
                            (UText {
                                text: { icon.to_string() },
                                font_size: 16.0,
                                font: { icon_font.clone() },
                                color: icon_color,
                            })
                        ]
                    ),
                    // Message Body (Title & Description)
                    (
                        UNode::default()
                        USelf {
                            flex_grow: { Some(1.0) },
                        }
                        ULayout {
                            display: UDisplay::Flex,
                            flex_direction: UFlexDirection::Column,
                            gap: 3.0,
                        }
                        Children [
                            (UText {
                                text: title,
                                font_size: 14.0,
                                font: { font.clone() },
                                color: TOAST_TITLE_COLOR,
                            }),
                            (UText {
                                text: description,
                                font_size: 12.0,
                                font: { font.clone() },
                                color: TOAST_DESC_COLOR,
                            })
                        ]
                    ),
                    // Close Button
                    (
                        LumaToastClose
                        UNode {
                            width: UVal::Px(20.0),
                            height: UVal::Px(20.0),
                            border_radius: UCornerRadius::all(4.0),
                        }
                        UInteraction::default()
                        UInteractionColors {
                            normal: Color::NONE,
                            hovered: Color::srgba(1.0, 1.0, 1.0, 0.10),
                            pressed: Color::srgba(1.0, 1.0, 1.0, 0.20),
                        }
                        UFocusable::new()
                        UFocusVisual::border(Color::srgb(0.35, 0.65, 1.0), 1.0)
                        ULayout {
                            display: UDisplay::Flex,
                            justify_content: UJustifyContent::Center,
                            align_items: UAlignItems::Center,
                        }
                        Children [
                            (UText {
                                text: { Icon::X.to_string() },
                                font_size: 13.0,
                                font: icon_font,
                                color: Color::srgb(0.60, 0.65, 0.75),
                            })
                        ]
                    )
                ]
            };
            s.resolve(context, scene)
        }
    }
}

/// Creates an interactive toast notification card scene.
pub fn luma_toast(
    title: impl Into<String>,
    description: impl Into<String>,
    variant: ToastVariant,
    font: Handle<Font>,
    icon_font: Handle<Font>,
) -> Toast {
    Toast::new(title, description)
        .variant(variant)
        .font(font)
        .icon_font(icon_font)
}

/// Creates a toast notification card scene that automatically dismisses after a duration.
pub fn luma_toast_timed(
    title: impl Into<String>,
    description: impl Into<String>,
    variant: ToastVariant,
    duration_secs: f32,
    font: Handle<Font>,
    icon_font: Handle<Font>,
) -> Toast {
    Toast::new(title, description)
        .variant(variant)
        .timed(duration_secs)
        .font(font)
        .icon_font(icon_font)
}

fn dismiss_toast(
    target_entity: Entity,
    commands: &mut Commands,
    parents_query: &Query<&ChildOf>,
    close_query: &Query<&LumaToastClose>,
    toast_query: &Query<&LumaToast>,
) {
    let close_entity = if close_query.contains(target_entity) {
        target_entity
    } else if let Ok(parent) = parents_query.get(target_entity) {
        if close_query.contains(parent.get()) {
            parent.get()
        } else {
            return;
        }
    } else {
        return;
    };

    let mut current = close_entity;
    while let Ok(parent) = parents_query.get(current) {
        current = parent.get();
        if toast_query.contains(current) {
            commands.trigger(ToastDismissed(current));
            if let Ok(mut cmd) = commands.get_entity(current) {
                cmd.despawn();
            }
            break;
        }
    }
}

fn on_toast_close_click(
    trigger: On<Pointer<Click>>,
    mut commands: Commands,
    parents_query: Query<&ChildOf>,
    close_query: Query<&LumaToastClose>,
    toast_query: Query<&LumaToast>,
) {
    dismiss_toast(
        trigger.entity.entity(),
        &mut commands,
        &parents_query,
        &close_query,
        &toast_query,
    );
}

fn on_toast_close_activate(
    trigger: On<UFocusActivate>,
    mut commands: Commands,
    parents_query: Query<&ChildOf>,
    close_query: Query<&LumaToastClose>,
    toast_query: Query<&LumaToast>,
) {
    dismiss_toast(
        trigger.entity,
        &mut commands,
        &parents_query,
        &close_query,
        &toast_query,
    );
}

fn toast_timer_system(
    time: Res<Time>,
    mut commands: Commands,
    mut query: Query<(Entity, &mut LumaToastTimer), With<LumaToast>>,
) {
    for (entity, mut toast_timer) in query.iter_mut() {
        toast_timer.timer.tick(time.delta());
        if toast_timer.timer.just_finished() {
            commands.trigger(ToastDismissed(entity));
            if let Ok(mut cmd) = commands.get_entity(entity) {
                cmd.despawn();
            }
        }
    }
}

pub struct LumaToastPlugin;

impl Plugin for LumaToastPlugin {
    fn build(&self, app: &mut App) {
        app.register_type::<LumaToast>()
            .register_type::<ToastVariant>()
            .register_type::<LumaToastTimer>()
            .add_observer(on_toast_close_click)
            .add_observer(on_toast_close_activate)
            .add_systems(Update, toast_timer_system);
    }
}
