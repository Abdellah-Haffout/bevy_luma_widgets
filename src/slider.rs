//! Interactive slider / seekbar widget.

use bevy::prelude::*;
use bevy::scene::{ResolveContext, ResolvedScene, ResolveSceneError};
use bevy_luma::prelude::*;

use crate::theme::SliderStyle;

/// Marker and state component for a slider.
#[derive(Component, Clone, Debug, Reflect)]
pub struct LumaSlider {
    pub value: f32,
    pub min: f32,
    pub max: f32,
    pub step: Option<f32>,
    pub width: f32,
    pub disabled: bool,
}

impl Default for LumaSlider {
    fn default() -> Self {
        Self {
            value: 0.0,
            min: 0.0,
            max: 100.0,
            step: None,
            width: 200.0,
            disabled: false,
        }
    }
}

impl LumaSlider {
    pub fn ratio(&self) -> f32 {
        if (self.max - self.min).abs() < f32::EPSILON {
            0.0
        } else {
            ((self.value - self.min) / (self.max - self.min)).clamp(0.0, 1.0)
        }
    }

    pub fn set_ratio(&mut self, ratio: f32) {
        let raw = self.min + ratio.clamp(0.0, 1.0) * (self.max - self.min);
        self.value = if let Some(step) = self.step {
            if step > 0.0 {
                (raw / step).round() * step
            } else {
                raw
            }
        } else {
            raw
        }
        .clamp(self.min, self.max);
    }
}

/// Marker component for the active fill progress track on a slider.
#[derive(Component, Clone, Debug, Default, Reflect)]
pub struct LumaSliderFill;

/// Marker component for the draggable thumb knob on a slider.
#[derive(Component, Clone, Debug, Default, Reflect)]
pub struct LumaSliderThumb;

/// Event triggered when a slider value changes.
#[derive(Event, Clone, Copy, Debug)]
pub struct SliderChanged {
    pub entity: Entity,
    pub value: f32,
}

const SLIDER_HEIGHT: f32 = 24.0;
const TRACK_HEIGHT: f32 = 6.0;
const THUMB_SIZE: f32 = 18.0;

/// Declarative Slider widget struct with all configurable properties.
#[derive(Clone, Debug, Reflect)]
pub struct Slider {
    pub value: f32,
    pub min: f32,
    pub max: f32,
    pub step: Option<f32>,
    pub width: f32,
    pub disabled: bool,
    pub style: Option<SliderStyle>,
}

impl Default for Slider {
    fn default() -> Self {
        Self {
            value: 0.0,
            min: 0.0,
            max: 100.0,
            step: None,
            width: 200.0,
            disabled: false,
            style: None,
        }
    }
}

impl Slider {
    pub fn new(value: f32, min: f32, max: f32, width: f32) -> Self {
        Self {
            value,
            min,
            max,
            width,
            ..default()
        }
    }

    pub fn stepped(value: f32, min: f32, max: f32, step: f32, width: f32) -> Self {
        Self {
            value,
            min,
            max,
            step: Some(step),
            width,
            ..default()
        }
    }

    pub fn value(mut self, value: f32) -> Self {
        self.value = value;
        self
    }

    pub fn min(mut self, min: f32) -> Self {
        self.min = min;
        self
    }

    pub fn max(mut self, max: f32) -> Self {
        self.max = max;
        self
    }

    pub fn width(mut self, width: f32) -> Self {
        self.width = width;
        self
    }

    pub fn step(mut self, step: f32) -> Self {
        self.step = Some(step);
        self
    }

    pub fn disabled(mut self, disabled: bool) -> Self {
        self.disabled = disabled;
        self
    }

    pub fn style(mut self, style: SliderStyle) -> Self {
        self.style = Some(style);
        self
    }
}

impl Scene for Slider {
    fn resolve(
        self,
        context: &mut ResolveContext,
        scene: &mut ResolvedScene,
    ) -> Result<(), ResolveSceneError> {
        let (trk_bg, fill_col, thmb_col, thmb_bdr, trk_h, thmb_sz) = if let Some(ref s) = self.style {
            (
                s.track_bg,
                s.fill_color,
                s.thumb_color,
                s.thumb_border,
                s.track_height,
                s.thumb_size,
            )
        } else {
            (
                Color::srgb(0.18, 0.22, 0.28),
                Color::srgb(0.20, 0.45, 0.90),
                Color::WHITE,
                Color::srgb(0.20, 0.45, 0.90),
                TRACK_HEIGHT,
                THUMB_SIZE,
            )
        };

        let temp_slider = LumaSlider {
            value: self.value,
            min: self.min,
            max: self.max,
            step: self.step,
            width: self.width,
            disabled: self.disabled,
        };
        let ratio = temp_slider.ratio();
        let thumb_travel = (self.width - thmb_sz).max(0.0);
        let thumb_offset = ratio * thumb_travel;
        let track_top = (SLIDER_HEIGHT - trk_h) * 0.5;
        let thumb_top = (SLIDER_HEIGHT - thmb_sz) * 0.5;
        let fill_width = if ratio <= 0.001 {
            0.0
        } else if ratio >= 0.999 {
            self.width
        } else {
            thumb_offset + thmb_sz * 0.5
        };

        let value = self.value;
        let min = self.min;
        let max = self.max;
        let step = self.step;
        let width = self.width;
        let disabled = self.disabled;

        let s = bsn! {
            LumaSlider {
                value,
                min,
                max,
                step,
                width,
                disabled,
            }
            UNode {
                width: { UVal::Px(width) },
                height: UVal::Px(SLIDER_HEIGHT),
                background_color: Color::NONE,
            }
            UInteraction::default()
            UFocusable::new()
            UFocusVisual::border(Color::srgb(0.35, 0.65, 1.0), 2.0)
            Children [
                // 1. Inactive Track background bar
                (
                    UNode {
                        width: { UVal::Px(width) },
                        height: UVal::Px(trk_h),
                        background_color: trk_bg,
                        border_radius: UCornerRadius::all(trk_h * 0.5),
                    }
                    UBorder {
                        color: Color::srgb(0.25, 0.30, 0.38),
                        width: 1.0,
                        radius: UCornerRadius::all(trk_h * 0.5),
                    }
                    USelf {
                        position_type: UPositionType::Absolute,
                        left: { UVal::Px(0.0) },
                        top: { UVal::Px(track_top) },
                    }
                ),
                // 2. Active Fill bar
                (
                    LumaSliderFill
                    UNode {
                        width: { UVal::Px(fill_width) },
                        height: UVal::Px(trk_h),
                        background_color: fill_col,
                        border_radius: UCornerRadius::all(trk_h * 0.5),
                    }
                    USelf {
                        position_type: UPositionType::Absolute,
                        left: { UVal::Px(0.0) },
                        top: { UVal::Px(track_top) },
                    }
                ),
                // 3. Draggable Thumb Knob
                (
                    LumaSliderThumb
                    UNode {
                        width: UVal::Px(thmb_sz),
                        height: UVal::Px(thmb_sz),
                        background_color: thmb_col,
                        border_radius: UCornerRadius::all(thmb_sz * 0.5),
                    }
                    UBorder {
                        color: thmb_bdr,
                        width: 2.0,
                        radius: UCornerRadius::all(thmb_sz * 0.5),
                    }
                    USelf {
                        position_type: UPositionType::Absolute,
                        left: { UVal::Px(thumb_offset) },
                        top: { UVal::Px(thumb_top) },
                    }
                )
            ]
        };

        s.resolve(context, scene)
    }
}

/// Creates a continuous slider scene.
pub fn luma_slider(value: f32, min: f32, max: f32, width: f32) -> Slider {
    Slider::new(value, min, max, width)
}

/// Creates a stepped slider scene.
pub fn luma_slider_stepped(value: f32, min: f32, max: f32, step: f32, width: f32) -> Slider {
    Slider::stepped(value, min, max, step, width)
}

fn update_slider_visuals(
    slider: &LumaSlider,
    children: Option<&Children>,
    fill_query: &mut Query<&mut UNode, With<LumaSliderFill>>,
    thumb_query: &mut Query<&mut USelf, With<LumaSliderThumb>>,
) {
    let ratio = slider.ratio();
    let thumb_travel = (slider.width - THUMB_SIZE).max(0.0);
    let thumb_offset = ratio * thumb_travel;

    let fill_width = if ratio <= 0.001 {
        0.0
    } else if ratio >= 0.999 {
        slider.width
    } else {
        thumb_offset + THUMB_SIZE * 0.5
    };

    if let Some(children) = children {
        for child in children.iter() {
            if let Ok(mut fill_node) = fill_query.get_mut(child) {
                fill_node.width = UVal::Px(fill_width);
            }
            if let Ok(mut thumb_self) = thumb_query.get_mut(child) {
                thumb_self.left = UVal::Px(thumb_offset);
            }
        }
    }
}

fn on_slider_pointer_down(
    trigger: On<Pointer<Press>>,
    mut commands: Commands,
    mut slider_query: Query<(Entity, &mut LumaSlider, &GlobalTransform, &ComputedSize, Option<&Children>)>,
    mut fill_query: Query<&mut UNode, With<LumaSliderFill>>,
    mut thumb_query: Query<&mut USelf, With<LumaSliderThumb>>,
) {
    let entity = trigger.entity.entity();
    if let Ok((entity, mut slider, global_tf, computed_size, children)) = slider_query.get_mut(entity) {
        if slider.disabled {
            return;
        }

        let click_pos = trigger.pointer_location.position;
        let slider_world_pos = global_tf.translation().truncate();
        let half_width = computed_size.size().x * 0.5;
        let left_edge = slider_world_pos.x - half_width;

        let relative_x = (click_pos.x - left_edge).clamp(0.0, slider.width);
        let ratio = if slider.width > 0.0 { relative_x / slider.width } else { 0.0 };

        slider.set_ratio(ratio);
        let new_value = slider.value;

        update_slider_visuals(&slider, children, &mut fill_query, &mut thumb_query);

        commands.trigger(SliderChanged {
            entity,
            value: new_value,
        });
    }
}

fn on_slider_drag(
    trigger: On<Pointer<Drag>>,
    mut commands: Commands,
    mut slider_query: Query<(Entity, &mut LumaSlider, &GlobalTransform, &ComputedSize, Option<&Children>)>,
    mut fill_query: Query<&mut UNode, With<LumaSliderFill>>,
    mut thumb_query: Query<&mut USelf, With<LumaSliderThumb>>,
) {
    let entity = trigger.entity.entity();
    if let Ok((entity, mut slider, global_tf, computed_size, children)) = slider_query.get_mut(entity) {
        if slider.disabled {
            return;
        }

        let drag_pos = trigger.pointer_location.position;
        let slider_world_pos = global_tf.translation().truncate();
        let half_width = computed_size.size().x * 0.5;
        let left_edge = slider_world_pos.x - half_width;

        let relative_x = (drag_pos.x - left_edge).clamp(0.0, slider.width);
        let ratio = if slider.width > 0.0 { relative_x / slider.width } else { 0.0 };

        slider.set_ratio(ratio);
        let new_value = slider.value;

        update_slider_visuals(&slider, children, &mut fill_query, &mut thumb_query);

        commands.trigger(SliderChanged {
            entity,
            value: new_value,
        });
    }
}

pub struct LumaSliderPlugin;

impl Plugin for LumaSliderPlugin {
    fn build(&self, app: &mut App) {
        app.register_type::<LumaSlider>()
            .register_type::<LumaSliderFill>()
            .register_type::<LumaSliderThumb>()
            .add_observer(on_slider_pointer_down)
            .add_observer(on_slider_drag);
    }
}
