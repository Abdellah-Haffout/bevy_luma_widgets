//! Interactive slider / seekbar widget.

use bevy::prelude::*;
use bevy_luma::prelude::*;

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

/// Marker for the active progress fill inside a slider.
#[derive(Component, Clone, Debug, Default, Reflect)]
pub struct LumaSliderFill;

/// Marker for the draggable thumb knob inside a slider.
#[derive(Component, Clone, Debug, Default, Reflect)]
pub struct LumaSliderThumb;

/// Event triggered when a slider value changes.
#[derive(Event, Clone, Copy, Debug)]
pub struct SliderChanged {
    pub entity: Entity,
    pub value: f32,
}

const THUMB_SIZE: f32 = 16.0;

/// Creates a horizontal slider scene.
pub fn luma_slider(value: f32, min: f32, max: f32, width: f32) -> impl Scene {
    let range = (max - min).max(0.001);
    let ratio = ((value - min) / range).clamp(0.0, 1.0);
    let thumb_travel = (width - THUMB_SIZE).max(0.0);
    let thumb_offset = ratio * thumb_travel;

    bsn! {
        LumaSlider {
            value,
            min,
            max,
            step: None,
            width,
            disabled: false,
        }
        UNode {
            width: UVal::Px(width),
            height: UVal::Px(24.0),
            background_color: Color::NONE,
        }
        UInteraction::default()
        UFocusable::new()
        UFocusVisual::border(Color::srgb(0.35, 0.65, 1.0), 2.0)
        ULayout {
            display: UDisplay::Flex,
            align_items: UAlignItems::Center,
        }
        Children [
            // Track bar
            (
                UNode {
                    width: UVal::Percent(1.0),
                    height: UVal::Px(6.0),
                    background_color: Color::srgb(0.18, 0.22, 0.28),
                    border_radius: UCornerRadius::all(999.0),
                }
                UBorder {
                    color: Color::srgb(0.25, 0.30, 0.38),
                    width: 1.0,
                    radius: UCornerRadius::all(999.0),
                }
                ULayout {
                    display: UDisplay::Flex,
                    align_items: UAlignItems::Center,
                }
                Children [
                    // Active Fill
                    (
                        LumaSliderFill
                        UNode {
                            width: UVal::Percent(ratio),
                            height: UVal::Percent(1.0),
                            background_color: Color::srgb(0.20, 0.45, 0.90),
                            border_radius: UCornerRadius::all(999.0),
                        }
                    )
                ]
            ),
            // Draggable Thumb Knob
            (
                LumaSliderThumb
                UNode {
                    width: UVal::Px(THUMB_SIZE),
                    height: UVal::Px(THUMB_SIZE),
                    background_color: Color::WHITE,
                    border_radius: UCornerRadius::all(999.0),
                }
                UBorder {
                    color: Color::srgb(0.20, 0.45, 0.90),
                    width: 2.0,
                    radius: UCornerRadius::all(999.0),
                }
                USelf {
                    position_type: UPositionType::Absolute,
                    left: { UVal::Px(thumb_offset) },
                }
            )
        ]
    }
}

/// Observer handling pointer dragging across the slider.
fn on_slider_drag(
    trigger: On<Pointer<Drag>>,
    mut commands: Commands,
    mut query: Query<&mut LumaSlider>,
) {
    let entity = trigger.entity.entity();
    if let Ok(mut slider) = query.get_mut(entity) {
        if slider.disabled || slider.width <= 0.0 {
            return;
        }

        let delta_ratio = trigger.delta.x / slider.width;
        let current_ratio = slider.ratio();
        slider.set_ratio(current_ratio + delta_ratio);

        commands.trigger(SliderChanged {
            entity,
            value: slider.value,
        });
    }
}

/// Observer handling direct click on the slider track.
fn on_slider_click(
    trigger: On<Pointer<Click>>,
    mut commands: Commands,
    cameras: Query<(&Camera, &GlobalTransform)>,
    mut query: Query<(&mut LumaSlider, &GlobalTransform)>,
) {
    let entity = trigger.entity.entity();
    if let Ok((mut slider, gt)) = query.get_mut(entity) {
        if slider.disabled || slider.width <= 0.0 {
            return;
        }

        let cursor_world_2d = cameras.iter().find_map(|(camera, cam_gt)| {
            camera
                .viewport_to_world_2d(cam_gt, trigger.pointer_location.position)
                .ok()
        });

        if let Some(pos) = cursor_world_2d {
            let left_edge = gt.translation().x - slider.width * 0.5;
            let ratio = ((pos.x - left_edge) / slider.width).clamp(0.0, 1.0);
            slider.set_ratio(ratio);

            commands.trigger(SliderChanged {
                entity,
                value: slider.value,
            });
        }
    }
}

/// System synchronizing slider visual state (fill width and thumb position) with component value.
fn slider_sync_system(
    slider_query: Query<(Entity, &LumaSlider), Changed<LumaSlider>>,
    children_query: Query<&Children>,
    mut fill_query: Query<&mut UNode, With<LumaSliderFill>>,
    mut thumb_query: Query<&mut USelf, With<LumaSliderThumb>>,
) {
    for (slider_entity, slider) in slider_query.iter() {
        let ratio = slider.ratio();
        let thumb_travel = (slider.width - THUMB_SIZE).max(0.0);
        let thumb_offset = ratio * thumb_travel;

        // Traverse descendants to update fill and thumb
        if let Ok(children) = children_query.get(slider_entity) {
            for child in children.iter() {
                // Check if this child has the thumb
                if let Ok(mut uself) = thumb_query.get_mut(child) {
                    uself.left = UVal::Px(thumb_offset);
                }

                // Check grand-children for fill
                if let Ok(sub_children) = children_query.get(child) {
                    for sub_child in sub_children.iter() {
                        if let Ok(mut fill_node) = fill_query.get_mut(sub_child) {
                            fill_node.width = UVal::Percent(ratio);
                        }
                    }
                }
            }
        }
    }
}

pub struct LumaSliderPlugin;

impl Plugin for LumaSliderPlugin {
    fn build(&self, app: &mut App) {
        app.register_type::<LumaSlider>()
            .register_type::<LumaSliderFill>()
            .register_type::<LumaSliderThumb>()
            .add_observer(on_slider_drag)
            .add_observer(on_slider_click)
            .add_systems(Update, slider_sync_system);
    }
}
