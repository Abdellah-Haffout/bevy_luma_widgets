//! Interactive slider / seekbar widget.

use bevy::picking::pointer::PointerButton;
use bevy::prelude::*;
use bevy::scene::{ResolveContext, ResolvedScene, ResolveSceneError};
use bevy_luma::prelude::*;

use crate::helpers::{dim_color, find_ancestor_or_self, interaction_colors, snap_stepped};
use crate::theme::{SliderStyle, WidgetColors};

/// Marker and state component for a slider.
#[derive(Component, Clone, Debug, Reflect)]
pub struct LumaSlider {
    pub value: f32,
    pub min: f32,
    pub max: f32,
    pub step: Option<f32>,
    pub width: f32,
    pub disabled: bool,
    pub style: SliderStyle,
    pub has_explicit_style: bool,
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
            style: SliderStyle::default(),
            has_explicit_style: false,
        }
    }
}

impl LumaSlider {
    pub fn ratio(&self) -> f32 {
        let span = self.max - self.min;
        if span.abs() < f32::EPSILON {
            0.0
        } else {
            ((self.value - self.min) / span).clamp(0.0, 1.0)
        }
    }

    pub fn set_ratio(&mut self, ratio: f32) {
        let clamped_ratio = ratio.clamp(0.0, 1.0);
        let raw = self.min + clamped_ratio * (self.max - self.min);
        let snapped = if let Some(step) = self.step {
            snap_stepped(raw, step, self.min)
        } else {
            raw
        };
        let lo = self.min.min(self.max);
        let hi = self.min.max(self.max);
        self.value = snapped.clamp(lo, hi);
    }
}

/// Marker component for the background track bar on a slider.
#[derive(Component, Clone, Debug, Default, Reflect)]
pub struct LumaSliderTrack;

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
        let has_explicit_style = self.style.is_some();
        let style = self.style.unwrap_or_default();
        let colors = WidgetColors::default();
        let disabled = self.disabled;

        let trk_bg = if disabled {
            dim_color(style.track_bg, 0.55)
        } else {
            style.track_bg
        };
        let fill_col = if disabled {
            dim_color(style.fill_color, 0.45)
        } else {
            style.fill_color
        };
        let thmb_col = if disabled {
            colors.disabled_text()
        } else {
            style.thumb_color
        };
        let thmb_bdr = if disabled {
            colors.disabled_border()
        } else {
            style.thumb_border
        };
        let trk_bdr = if disabled {
            colors.disabled_border()
        } else {
            colors.surface_border
        };
        let trk_h = style.track_height;
        let thmb_sz = style.thumb_size;
        let container_h = SLIDER_HEIGHT.max(thmb_sz + 4.0);

        let lo = self.min.min(self.max);
        let hi = self.min.max(self.max);
        let initial_value = self.value.clamp(lo, hi);

        let luma_slider = LumaSlider {
            value: initial_value,
            min: self.min,
            max: self.max,
            step: self.step,
            width: self.width,
            disabled,
            style,
            has_explicit_style,
        };
        let ratio = luma_slider.ratio();
        let thumb_travel = (self.width - thmb_sz).max(0.0);
        let thumb_offset = ratio * thumb_travel;
        let track_top = (container_h - trk_h) * 0.5;
        let thumb_top = (container_h - thmb_sz) * 0.5;
        let fill_width = (ratio * self.width).clamp(0.0, self.width);
        let width = self.width;
        let thumb_colors = interaction_colors(
            thmb_col,
            Color::WHITE,
            Color::srgb(0.85, 0.90, 1.0),
            disabled,
        );

        let s = bsn! {
            luma_slider
            UNode {
                width: { UVal::Px(width) },
                height: { UVal::Px(container_h) },
                background_color: Color::NONE,
            }
            UInteraction::default()
            UFocusable {
                enabled: { !disabled },
                ..UFocusable::default()
            }
            UFocusVisual::border(colors.focus_ring(), 2.0)
            Children [
                // 1. Inactive Track background bar
                LumaSliderTrack
                UNode {
                    width: { UVal::Px(width) },
                    height: UVal::Px(trk_h),
                    background_color: trk_bg,
                    border_radius: UCornerRadius::all(trk_h * 0.5),
                }
                UBorder {
                    color: trk_bdr,
                    width: 1.0,
                    radius: UCornerRadius::all(trk_h * 0.5),
                }
                USelf {
                    position_type: UPositionType::Absolute,
                    left: { UVal::Px(0.0) },
                    top: { UVal::Px(track_top) },
                }
                UZIndex::Local(0)
                --
                // 2. Active Fill bar
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
                UZIndex::Local(1)
                Visibility::from(if fill_width <= 0.001 { Visibility::Hidden } else { Visibility::Inherited })
                --
                // 3. Draggable Thumb Knob
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
                UInteraction::default()
                thumb_colors
                USelf {
                    position_type: UPositionType::Absolute,
                    left: { UVal::Px(thumb_offset) },
                    top: { UVal::Px(thumb_top) },
                }
                UZIndex::Local(2)
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
    fill_query: &mut Query<(&mut UNode, &mut Visibility), With<LumaSliderFill>>,
    thumb_query: &mut Query<&mut USelf, With<LumaSliderThumb>>,
) {
    let ratio = slider.ratio();
    let thumb_travel = (slider.width - slider.style.thumb_size).max(0.0);
    let thumb_offset = ratio * thumb_travel;
    let fill_width = (ratio * slider.width).clamp(0.0, slider.width);

    if let Some(children) = children {
        for child in children.iter() {
            if let Ok((mut fill_node, mut visibility)) = fill_query.get_mut(child) {
                let target_width = UVal::Px(fill_width);
                if fill_node.width != target_width {
                    fill_node.width = target_width;
                }
                let target_vis = if fill_width <= 0.001 {
                    Visibility::Hidden
                } else {
                    Visibility::Inherited
                };
                if *visibility != target_vis {
                    *visibility = target_vis;
                }
            }
            if let Ok(mut thumb_self) = thumb_query.get_mut(child) {
                let target_left = UVal::Px(thumb_offset);
                if thumb_self.left != target_left {
                    thumb_self.left = target_left;
                }
            }
        }
    }
}

fn pointer_slider_ratio(
    pointer_pos: Vec2,
    hit_pos: Option<Vec3>,
    global_tf: &GlobalTransform,
    computed_size: &ComputedSize,
    slider_width: f32,
    cameras: &Query<(&Camera, &GlobalTransform)>,
) -> Option<f32> {
    let hit_world = hit_pos.or_else(|| {
        cameras.iter().find_map(|(cam, cam_gt)| {
            cam.viewport_to_world_2d(cam_gt, pointer_pos)
                .ok()
                .map(|p| Vec3::new(p.x, p.y, global_tf.translation().z))
        })
    })?;

    let local_hit = global_tf
        .to_matrix()
        .inverse()
        .transform_point3(hit_world);

    let track_width = if slider_width > 0.0 {
        slider_width
    } else {
        computed_size.size().x
    };
    if track_width <= 0.0 {
        return Some(0.0);
    }
    let relative_x = (local_hit.x + track_width * 0.5).clamp(0.0, track_width);
    Some(relative_x / track_width)
}

fn apply_pointer_ratio(
    entity: Entity,
    slider: &mut LumaSlider,
    children: Option<&Children>,
    ratio: f32,
    commands: &mut Commands,
    fill_query: &mut Query<(&mut UNode, &mut Visibility), With<LumaSliderFill>>,
    thumb_query: &mut Query<&mut USelf, With<LumaSliderThumb>>,
) {
    let prev = slider.value;
    slider.set_ratio(ratio);
    if (slider.value - prev).abs() > f32::EPSILON {
        update_slider_visuals(slider, children, fill_query, thumb_query);
        commands.trigger(SliderChanged {
            entity,
            value: slider.value,
        });
    }
}

fn on_slider_pointer_down(
    mut trigger: On<PointerPress>,
    mut commands: Commands,
    mut slider_query: Query<(Entity, &mut LumaSlider, &GlobalTransform, &ComputedSize, Option<&Children>)>,
    mut fill_query: Query<(&mut UNode, &mut Visibility), With<LumaSliderFill>>,
    mut thumb_query: Query<&mut USelf, With<LumaSliderThumb>>,
    parents_query: Query<&ChildOf>,
    cameras: Query<(&Camera, &GlobalTransform)>,
) {
    if trigger.event().button != PointerButton::Primary {
        return;
    }
    let Some(entity) = find_ancestor_or_self(trigger.entity, &slider_query, &parents_query) else {
        return;
    };
    trigger.propagate(false);

    // Clicking directly on the thumb grabs it for dragging without jumping.
    if thumb_query.contains(trigger.entity) {
        return;
    }

    if let Ok((entity, mut slider, global_tf, computed_size, children)) = slider_query.get_mut(entity) {
        if slider.disabled {
            return;
        }
        if let Some(ratio) = pointer_slider_ratio(
            trigger.pointer.position,
            trigger.hit.position,
            global_tf,
            computed_size,
            slider.width,
            &cameras,
        ) {
            apply_pointer_ratio(
                entity,
                &mut slider,
                children,
                ratio,
                &mut commands,
                &mut fill_query,
                &mut thumb_query,
            );
        }
    }
}

fn on_slider_drag(
    mut trigger: On<PointerDrag>,
    mut commands: Commands,
    mut slider_query: Query<(Entity, &mut LumaSlider, &GlobalTransform, &ComputedSize, Option<&Children>)>,
    mut fill_query: Query<(&mut UNode, &mut Visibility), With<LumaSliderFill>>,
    mut thumb_query: Query<&mut USelf, With<LumaSliderThumb>>,
    parents_query: Query<&ChildOf>,
    cameras: Query<(&Camera, &GlobalTransform)>,
) {
    if trigger.event().button != PointerButton::Primary {
        return;
    }
    let Some(entity) = find_ancestor_or_self(trigger.entity, &slider_query, &parents_query) else {
        return;
    };
    trigger.propagate(false);

    if let Ok((entity, mut slider, global_tf, computed_size, children)) = slider_query.get_mut(entity) {
        if slider.disabled {
            return;
        }
        if let Some(ratio) = pointer_slider_ratio(
            trigger.pointer.position,
            None,
            global_tf,
            computed_size,
            slider.width,
            &cameras,
        ) {
            apply_pointer_ratio(
                entity,
                &mut slider,
                children,
                ratio,
                &mut commands,
                &mut fill_query,
                &mut thumb_query,
            );
        }
    }
}

fn keyboard_slider_system(
    keyboard: Option<Res<ButtonInput<KeyCode>>>,
    mut commands: Commands,
    mut slider_query: Query<(Entity, &mut LumaSlider, Option<&Children>), With<UFocused>>,
    mut fill_query: Query<(&mut UNode, &mut Visibility), With<LumaSliderFill>>,
    mut thumb_query: Query<&mut USelf, With<LumaSliderThumb>>,
) {
    let Some(keyboard) = keyboard else {
        return;
    };

    let left = keyboard.just_pressed(KeyCode::ArrowLeft) || keyboard.just_pressed(KeyCode::ArrowDown);
    let right = keyboard.just_pressed(KeyCode::ArrowRight) || keyboard.just_pressed(KeyCode::ArrowUp);

    if !left && !right {
        return;
    }

    for (entity, mut slider, children) in slider_query.iter_mut() {
        if slider.disabled {
            continue;
        }

        let delta = if let Some(step) = slider.step {
            step
        } else {
            let range = (slider.max - slider.min).abs();
            if range <= 2.0 {
                range * 0.05
            } else {
                (range * 0.05).max(1.0)
            }
        };

        let raw = if right {
            slider.value + delta
        } else {
            slider.value - delta
        };
        let snapped = if let Some(step) = slider.step {
            snap_stepped(raw, step, slider.min)
        } else {
            raw
        };
        let lo = slider.min.min(slider.max);
        let hi = slider.min.max(slider.max);
        let new_val = snapped.clamp(lo, hi);

        if (new_val - slider.value).abs() > f32::EPSILON {
            slider.value = new_val;
            update_slider_visuals(&slider, children, &mut fill_query, &mut thumb_query);
            commands.trigger(SliderChanged {
                entity,
                value: new_val,
            });
        }
    }
}

fn sync_slider_on_change(
    slider_query: Query<(&LumaSlider, Option<&Children>), Changed<LumaSlider>>,
    mut fill_query: Query<(&mut UNode, &mut Visibility), With<LumaSliderFill>>,
    mut thumb_query: Query<&mut USelf, With<LumaSliderThumb>>,
) {
    for (slider, children) in slider_query.iter() {
        update_slider_visuals(slider, children, &mut fill_query, &mut thumb_query);
    }
}

pub struct LumaSliderPlugin;

impl Plugin for LumaSliderPlugin {
    fn build(&self, app: &mut App) {
        app.register_type::<LumaSlider>()
            .register_type::<LumaSliderTrack>()
            .register_type::<LumaSliderFill>()
            .register_type::<LumaSliderThumb>()
            .add_observer(on_slider_pointer_down)
            .add_observer(on_slider_drag)
            .add_systems(Update, (keyboard_slider_system, sync_slider_on_change));
    }
}
