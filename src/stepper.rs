//! Numeric stepper / counter input widget with increment and decrement buttons.

use bevy::prelude::*;
use bevy::scene::{ResolveContext, ResolvedScene, ResolveSceneError};
use bevy_luma::prelude::*;

use crate::helpers::{find_ancestor_or_self, interaction_colors, is_primary_click};
use crate::theme::{StepperStyle, WidgetColors};

/// Marker and state component for a number stepper widget.
#[derive(Component, Clone, Debug, Reflect)]
pub struct LumaStepper {
    pub value: f64,
    pub min: f64,
    pub max: f64,
    pub step: f64,
    pub disabled: bool,
    pub style: StepperStyle,
    pub has_explicit_style: bool,
}

impl Default for LumaStepper {
    fn default() -> Self {
        Self {
            value: 0.0,
            min: 0.0,
            max: 100.0,
            step: 1.0,
            disabled: false,
            style: StepperStyle::default(),
            has_explicit_style: false,
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

/// Marker component for icon glyphs inside stepper buttons.
#[derive(Component, Clone, Debug, Default, Reflect)]
pub struct LumaStepperIcon;

/// Event triggered when a stepper's value changes.
#[derive(Event, Clone, Copy, Debug)]
pub struct StepperChanged {
    pub entity: Entity,
    pub value: f64,
}

fn format_value(value: f64, step: f64) -> String {
    if step.fract().abs() < 1e-9 && value.fract().abs() < 1e-9 {
        format!("{:.0}", value.round())
    } else if (step * 10.0).fract().abs() < 1e-6 {
        format!("{:.1}", value)
    } else {
        format!("{:.2}", value)
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
    pub style: Option<StepperStyle>,
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
            style: None,
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

    pub fn style(mut self, style: StepperStyle) -> Self {
        self.style = Some(style);
        self
    }
}

impl Scene for Stepper {
    fn resolve(
        self,
        context: &mut ResolveContext,
        scene: &mut ResolvedScene,
    ) -> Result<(), ResolveSceneError> {
        let has_explicit_style = self.style.is_some();
        let style = self.style.unwrap_or_default();
        let colors = WidgetColors::default();
        let lo = self.min.min(self.max);
        let hi = self.min.max(self.max);
        let clamped_val = self.value.clamp(lo, hi);
        let min = self.min;
        let max = self.max;
        let step = self.step;
        let disabled = self.disabled;
        let icon_font = self.icon_font;
        let font = self.font;
        let display_str = format_value(clamped_val, step);

        let bg_color = if disabled {
            colors.disabled_bg()
        } else {
            style.bg_color
        };
        let border_color = if disabled {
            colors.disabled_border()
        } else {
            style.border_color
        };
        let btn_bg = if disabled {
            colors.disabled_bg()
        } else {
            style.button_bg
        };
        let btn_colors = interaction_colors(btn_bg, style.button_hover, style.button_pressed, disabled);
        let decr_colors = btn_colors.clone();
        let incr_colors = btn_colors;
        let text_color = if disabled {
            colors.disabled_text()
        } else {
            style.text_color
        };
        let icon_color = if disabled {
            colors.disabled_text()
        } else {
            Color::srgb(0.80, 0.85, 0.95)
        };
        let height = style.height;
        let radius = style.radius;
        let border_width = style.border_width;
        let font_size = style.font_size;
        let btn_h = (height - 8.0).max(20.0);
        let btn_w = btn_h.max(26.0);
        let btn_r = (radius - 2.0).max(4.0);
        let focus_color = colors.focus_ring();
        let luma_stepper = LumaStepper {
            value: clamped_val,
            min,
            max,
            step,
            disabled,
            style,
            has_explicit_style,
        };

        let s = bsn! {
            luma_stepper
            UNode {
                height: { UVal::Px(height) },
                background_color: bg_color,
                border_radius: { UCornerRadius::all(radius) },
                padding: USides::axes(4.0, 4.0),
            }
            UBorder {
                color: border_color,
                width: border_width,
                radius: { UCornerRadius::all(radius) },
            }
            ULayout {
                display: UDisplay::Flex,
                flex_direction: UFlexDirection::Row,
                align_items: UAlignItems::Center,
                gap: 6.0,
            }
            Children [
                // Decrement Button [-]
                LumaStepperButton { is_increment: false }
                UNode {
                    width: { UVal::Px(btn_w) },
                    height: { UVal::Px(btn_h) },
                    background_color: btn_bg,
                    border_radius: { UCornerRadius::all(btn_r) },
                }
                UInteraction::default()
                decr_colors
                UFocusable {
                    enabled: { !disabled },
                    ..UFocusable::default()
                }
                UFocusVisual::border(focus_color, 1.5)
                ULayout {
                    display: UDisplay::Flex,
                    justify_content: UJustifyContent::Center,
                    align_items: UAlignItems::Center,
                }
                Children [
                    LumaStepperIcon
                    UText {
                        text: { Icon::MINUS.to_string() },
                        font_size: 13.0,
                        font: { icon_font.clone() },
                        color: icon_color,
                    }
                ]
                --
                // Value Text Display
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
                    UText {
                        text: display_str,
                        font_size,
                        font,
                        color: text_color,
                    }
                ]
                --
                // Increment Button [+]
                LumaStepperButton { is_increment: true }
                UNode {
                    width: { UVal::Px(btn_w) },
                    height: { UVal::Px(btn_h) },
                    background_color: btn_bg,
                    border_radius: { UCornerRadius::all(btn_r) },
                }
                UInteraction::default()
                incr_colors
                UFocusable {
                    enabled: { !disabled },
                    ..UFocusable::default()
                }
                UFocusVisual::border(focus_color, 1.5)
                ULayout {
                    display: UDisplay::Flex,
                    justify_content: UJustifyContent::Center,
                    align_items: UAlignItems::Center,
                }
                Children [
                    LumaStepperIcon
                    UText {
                        text: { Icon::PLUS.to_string() },
                        font_size: 13.0,
                        font: icon_font,
                        color: icon_color,
                    }
                ]
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

fn apply_stepper_step(
    stepper_entity: Entity,
    stepper: &mut LumaStepper,
    children: &Children,
    is_increment: bool,
    commands: &mut Commands,
    text_container_query: &Query<&Children, With<LumaStepperValueText>>,
    text_query: &mut Query<&mut UText>,
) {
    if stepper.disabled {
        return;
    }

    let raw = if is_increment {
        stepper.value + stepper.step
    } else {
        stepper.value - stepper.step
    };
    let stepped = if stepper.step > 0.0 {
        stepper.min + ((raw - stepper.min) / stepper.step).round() * stepper.step
    } else {
        raw
    };
    let lo = stepper.min.min(stepper.max);
    let hi = stepper.min.max(stepper.max);
    let new_val = stepped.clamp(lo, hi);

    if (new_val - stepper.value).abs() <= f64::EPSILON {
        return;
    }

    stepper.value = new_val;
    let new_str = format_value(stepper.value, stepper.step);

    for child in children.iter() {
        if let Ok(value_text_children) = text_container_query.get(child) {
            for inner_child in value_text_children.iter() {
                if let Ok(mut text) = text_query.get_mut(inner_child) {
                    text.text = new_str.clone();
                }
            }
        }
    }

    commands.trigger(StepperChanged {
        entity: stepper_entity,
        value: stepper.value,
    });
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
    let Some(button_entity) = find_ancestor_or_self(target_entity, button_query, parents_query) else {
        return;
    };
    let Ok(btn) = button_query.get(button_entity) else {
        return;
    };
    let is_increment = btn.is_increment;

    let Some(stepper_entity) = find_ancestor_or_self(button_entity, stepper_query, parents_query) else {
        return;
    };
    let Ok((entity, mut stepper, children)) = stepper_query.get_mut(stepper_entity) else {
        return;
    };

    apply_stepper_step(
        entity,
        &mut stepper,
        children,
        is_increment,
        commands,
        text_container_query,
        text_query,
    );
}

fn on_stepper_click(
    trigger: On<PointerClick>,
    mut commands: Commands,
    parents_query: Query<&ChildOf>,
    button_query: Query<&LumaStepperButton>,
    mut stepper_query: Query<(Entity, &mut LumaStepper, &Children)>,
    text_container_query: Query<&Children, With<LumaStepperValueText>>,
    mut text_query: Query<&mut UText>,
) {
    if !is_primary_click(&trigger) {
        return;
    }
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

fn keyboard_stepper_system(
    keyboard: Option<Res<ButtonInput<KeyCode>>>,
    mut commands: Commands,
    focused_buttons: Query<Entity, (With<LumaStepperButton>, With<UFocused>)>,
    parents_query: Query<&ChildOf>,
    mut stepper_query: Query<(Entity, &mut LumaStepper, &Children)>,
    text_container_query: Query<&Children, With<LumaStepperValueText>>,
    mut text_query: Query<&mut UText>,
) {
    let Some(keyboard) = keyboard else {
        return;
    };

    let decr = keyboard.just_pressed(KeyCode::ArrowLeft) || keyboard.just_pressed(KeyCode::ArrowDown);
    let incr = keyboard.just_pressed(KeyCode::ArrowRight) || keyboard.just_pressed(KeyCode::ArrowUp);
    if !decr && !incr {
        return;
    }

    for btn_entity in focused_buttons.iter() {
        let Some(stepper_entity) = find_ancestor_or_self(btn_entity, &stepper_query, &parents_query) else {
            continue;
        };
        if let Ok((entity, mut stepper, children)) = stepper_query.get_mut(stepper_entity) {
            apply_stepper_step(
                entity,
                &mut stepper,
                children,
                incr,
                &mut commands,
                &text_container_query,
                &mut text_query,
            );
        }
    }
}

pub struct LumaStepperPlugin;

impl Plugin for LumaStepperPlugin {
    fn build(&self, app: &mut App) {
        app.register_type::<LumaStepper>()
            .register_type::<LumaStepperButton>()
            .register_type::<LumaStepperValueText>()
            .register_type::<LumaStepperIcon>()
            .add_observer(on_stepper_click)
            .add_observer(on_stepper_activate)
            .add_systems(Update, keyboard_stepper_system);
    }
}
