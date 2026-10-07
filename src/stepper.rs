//! Numeric stepper / counter input widget with increment and decrement buttons.

use bevy::ecs::relationship::Relationship;
use bevy::picking::pointer::PointerButton;
use bevy::prelude::*;
use bevy_luma::prelude::*;

/// Marker and state component for a number stepper widget.
#[derive(Component, Clone, Debug, Reflect)]
pub struct LumaStepper {
    pub value: f64,
    pub min: f64,
    pub max: f64,
    pub step: f64,
    pub disabled: bool,
}

impl Default for LumaStepper {
    fn default() -> Self {
        Self {
            value: 0.0,
            min: 0.0,
            max: 100.0,
            step: 1.0,
            disabled: false,
        }
    }
}

/// Marker component for the increment / decrement button on a stepper.
#[derive(Component, Clone, Debug, Default, Reflect)]
pub struct LumaStepperButton {
    pub is_increment: bool,
}

/// Marker component for the text display showing the stepper's current numeric value.
#[derive(Component, Clone, Debug, Default, Reflect)]
pub struct LumaStepperValueText;

/// Event triggered when a stepper's value changes.
#[derive(Event, Clone, Copy, Debug)]
pub struct StepperChanged {
    pub entity: Entity,
    pub value: f64,
}

const STEPPER_BG: Color = Color::srgb(0.12, 0.15, 0.20);
const STEPPER_BORDER: Color = Color::srgb(0.22, 0.26, 0.36);
const BTN_BG_NORMAL: Color = Color::srgba(1.0, 1.0, 1.0, 0.05);
const BTN_BG_HOVER: Color = Color::srgba(1.0, 1.0, 1.0, 0.12);
const BTN_BG_PRESSED: Color = Color::srgba(1.0, 1.0, 1.0, 0.20);

fn format_value(value: f64, step: f64) -> String {
    if step.fract() == 0.0 && value.fract() == 0.0 {
        format!("{:.0}", value)
    } else {
        format!("{:.1}", value)
    }
}

/// Declarative Stepper widget struct with all configurable properties.
#[derive(Clone, Debug, Reflect)]
pub struct Stepper {
    pub value: f64,
    pub min: f64,
    pub max: f64,
    pub step: f64,
    pub disabled: bool,
    pub font: Handle<Font>,
    pub icon_font: Handle<Font>,
}

impl Default for Stepper {
    fn default() -> Self {
        Self {
            value: 0.0,
            min: 0.0,
            max: 100.0,
            step: 1.0,
            disabled: false,
            font: Handle::default(),
            icon_font: Handle::default(),
        }
    }
}

impl Stepper {
    pub fn new(value: f64, min: f64, max: f64, step: f64) -> Self {
        Self {
            value,
            min,
            max,
            step,
            ..default()
        }
    }

    pub fn disabled(mut self, disabled: bool) -> Self {
        self.disabled = disabled;
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

use bevy::scene::{ResolveContext, ResolvedScene, ResolveSceneError};

impl Scene for Stepper {
    fn resolve(
        self,
        context: &mut ResolveContext,
        scene: &mut ResolvedScene,
    ) -> Result<(), ResolveSceneError> {
        let clamped_val = self.value.clamp(self.min, self.max);
        let min = self.min;
        let max = self.max;
        let step = self.step;
        let disabled = self.disabled;
        let icon_font = self.icon_font;
        let font = self.font;
        let display_str = format_value(clamped_val, step);

        let s = bsn! {
            LumaStepper {
                value: clamped_val,
                min,
                max,
                step,
                disabled,
            }
            UNode {
                height: UVal::Px(34.0),
                background_color: STEPPER_BG,
                border_radius: UCornerRadius::all(8.0),
                padding: USides::axes(4.0, 4.0),
            }
            UBorder {
                color: STEPPER_BORDER,
                width: 1.0,
                radius: UCornerRadius::all(8.0),
            }
            ULayout {
                display: UDisplay::Flex,
                flex_direction: UFlexDirection::Row,
                align_items: UAlignItems::Center,
                gap: 6.0,
            }
            Children [
                // Decrement Button [-]
                (
                    LumaStepperButton { is_increment: false }
                    UNode {
                        width: UVal::Px(28.0),
                        height: UVal::Px(26.0),
                        background_color: BTN_BG_NORMAL,
                        border_radius: UCornerRadius::all(6.0),
                    }
                    UInteraction::default()
                    UInteractionColors {
                        normal: BTN_BG_NORMAL,
                        hovered: BTN_BG_HOVER,
                        pressed: BTN_BG_PRESSED,
                    }
                    UFocusable::new()
                    UFocusVisual::border(Color::srgb(0.35, 0.65, 1.0), 1.5)
                    ULayout {
                        display: UDisplay::Flex,
                        justify_content: UJustifyContent::Center,
                        align_items: UAlignItems::Center,
                    }
                    Children [
                        (UText {
                            text: { Icon::MINUS.to_string() },
                            font_size: 13.0,
                            font: { icon_font.clone() },
                            color: Color::srgb(0.80, 0.85, 0.95),
                        })
                    ]
                ),
                // Value Text Display
                (
                    LumaStepperValueText
                    UNode {
                        min_width: 48.0,
                    }
                    ULayout {
                        display: UDisplay::Flex,
                        justify_content: UJustifyContent::Center,
                        align_items: UAlignItems::Center,
                    }
                    Children [
                        (UText {
                            text: display_str,
                            font_size: 14.0,
                            font,
                            color: Color::srgb(0.95, 0.96, 0.98),
                        })
                    ]
                ),
                // Increment Button [+]
                (
                    LumaStepperButton { is_increment: true }
                    UNode {
                        width: UVal::Px(28.0),
                        height: UVal::Px(26.0),
                        background_color: BTN_BG_NORMAL,
                        border_radius: UCornerRadius::all(6.0),
                    }
                    UInteraction::default()
                    UInteractionColors {
                        normal: BTN_BG_NORMAL,
                        hovered: BTN_BG_HOVER,
                        pressed: BTN_BG_PRESSED,
                    }
                    UFocusable::new()
                    UFocusVisual::border(Color::srgb(0.35, 0.65, 1.0), 1.5)
                    ULayout {
                        display: UDisplay::Flex,
                        justify_content: UJustifyContent::Center,
                        align_items: UAlignItems::Center,
                    }
                    Children [
                        (UText {
                            text: { Icon::PLUS.to_string() },
                            font_size: 13.0,
                            font: icon_font,
                            color: Color::srgb(0.80, 0.85, 0.95),
                        })
                    ]
                )
            ]
        };

        s.resolve(context, scene)
    }
}

/// Creates a numeric stepper widget scene.
pub fn luma_stepper(
    value: f64,
    min: f64,
    max: f64,
    step: f64,
    font: Handle<Font>,
    icon_font: Handle<Font>,
) -> Stepper {
    Stepper::new(value, min, max, step).font(font).icon_font(icon_font)
}

fn step_stepper(
    target_entity: Entity,
    commands: &mut Commands,
    parents_query: &Query<&ChildOf>,
    button_query: &Query<&LumaStepperButton>,
    stepper_query: &mut Query<(Entity, &mut LumaStepper, &Children)>,
    text_container_query: &Query<&Children, With<LumaStepperValueText>>,
    text_query: &mut Query<&mut UText>,
) {
    let (is_increment, button_entity) = if let Ok(btn) = button_query.get(target_entity) {
        (btn.is_increment, target_entity)
    } else if let Ok(parent) = parents_query.get(target_entity) {
        if let Ok(btn) = button_query.get(parent.get()) {
            (btn.is_increment, parent.get())
        } else {
            return;
        }
    } else {
        return;
    };

    let mut current = button_entity;
    let mut stepper_entity_opt = None;
    while let Ok(parent) = parents_query.get(current) {
        current = parent.get();
        if stepper_query.contains(current) {
            stepper_entity_opt = Some(current);
            break;
        }
    }

    let Some(stepper_entity) = stepper_entity_opt else {
        return;
    };

    let Ok((entity, mut stepper, children)) = stepper_query.get_mut(stepper_entity) else {
        return;
    };

    if stepper.disabled {
        return;
    }

    if is_increment {
        stepper.value = (stepper.value + stepper.step).min(stepper.max);
    } else {
        stepper.value = (stepper.value - stepper.step).max(stepper.min);
    }

    let new_str = format_value(stepper.value, stepper.step);

    for child in children.iter() {
        if let Ok(value_text_children) = text_container_query.get(child) {
            for inner_child in value_text_children.iter() {
                if let Ok(mut text) = text_query.get_mut(inner_child) {
                    text.text = new_str.clone();
                }
            }
        } else if let Ok(mut text) = text_query.get_mut(child) {
            text.text = new_str.clone();
        }
    }

    commands.trigger(StepperChanged {
        entity,
        value: stepper.value,
    });
}

fn on_stepper_click(
    trigger: On<Pointer<Click>>,
    mut commands: Commands,
    parents_query: Query<&ChildOf>,
    button_query: Query<&LumaStepperButton>,
    mut stepper_query: Query<(Entity, &mut LumaStepper, &Children)>,
    text_container_query: Query<&Children, With<LumaStepperValueText>>,
    mut text_query: Query<&mut UText>,
) {
    if trigger.event().button != PointerButton::Primary {
        return;
    }
    step_stepper(
        trigger.entity.entity(),
        &mut commands,
        &parents_query,
        &button_query,
        &mut stepper_query,
        &text_container_query,
        &mut text_query,
    );
}

fn on_stepper_activate(
    trigger: On<UFocusActivate>,
    mut commands: Commands,
    parents_query: Query<&ChildOf>,
    button_query: Query<&LumaStepperButton>,
    mut stepper_query: Query<(Entity, &mut LumaStepper, &Children)>,
    text_container_query: Query<&Children, With<LumaStepperValueText>>,
    mut text_query: Query<&mut UText>,
) {
    step_stepper(
        trigger.entity,
        &mut commands,
        &parents_query,
        &button_query,
        &mut stepper_query,
        &text_container_query,
        &mut text_query,
    );
}

pub struct LumaStepperPlugin;

impl Plugin for LumaStepperPlugin {
    fn build(&self, app: &mut App) {
        app.register_type::<LumaStepper>()
            .register_type::<LumaStepperButton>()
            .register_type::<LumaStepperValueText>()
            .add_observer(on_stepper_click)
            .add_observer(on_stepper_activate);
    }
}
