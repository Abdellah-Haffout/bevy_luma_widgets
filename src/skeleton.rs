//! Loading skeleton placeholder widget with smooth pulsing animation.

use bevy::prelude::*;
use bevy_luma::prelude::*;

/// Marker and state component for an animated loading skeleton shape.
#[derive(Component, Clone, Debug, Reflect)]
pub struct LumaSkeleton {
    pub pulse_speed: f32,
}

impl Default for LumaSkeleton {
    fn default() -> Self {
        Self { pulse_speed: 2.5 }
    }
}

const SKELETON_BASE_COLOR: Color = Color::srgb(0.12, 0.15, 0.20);
const SKELETON_PEAK_COLOR: Color = Color::srgb(0.20, 0.25, 0.34);

use bevy::scene::{ResolveContext, ResolvedScene, ResolveSceneError};

/// Declarative Skeleton placeholder widget struct with all configurable properties.
#[derive(Clone, Debug, Reflect)]
pub struct Skeleton {
    pub width: UVal,
    pub height: UVal,
    pub radius: f32,
    pub pulse_speed: f32,
    pub base_color: Color,
}

impl Default for Skeleton {
    fn default() -> Self {
        Self {
            width: UVal::Percent(1.0),
            height: UVal::Px(16.0),
            radius: 4.0,
            pulse_speed: 2.5,
            base_color: SKELETON_BASE_COLOR,
        }
    }
}

impl Skeleton {
    pub fn new(width: UVal, height: UVal) -> Self {
        Self {
            width,
            height,
            ..default()
        }
    }

    pub fn radius(mut self, radius: f32) -> Self {
        self.radius = radius;
        self
    }

    pub fn pulse_speed(mut self, pulse_speed: f32) -> Self {
        self.pulse_speed = pulse_speed;
        self
    }

    pub fn base_color(mut self, base_color: Color) -> Self {
        self.base_color = base_color;
        self
    }
}

impl Scene for Skeleton {
    fn resolve(
        self,
        context: &mut ResolveContext,
        scene: &mut ResolvedScene,
    ) -> Result<(), ResolveSceneError> {
        let width = self.width;
        let height = self.height;
        let radius = self.radius;
        let pulse_speed = self.pulse_speed;
        let base_color = self.base_color;

        let s = bsn! {
            LumaSkeleton {
                pulse_speed,
            }
            UNode {
                width,
                height,
                background_color: base_color,
                border_radius: { UCornerRadius::all(radius) },
            }
        };
        s.resolve(context, scene)
    }
}

/// Creates an animated loading skeleton placeholder scene.
pub fn luma_skeleton(width: UVal, height: UVal, radius: f32) -> Skeleton {
    Skeleton::new(width, height).radius(radius)
}

fn skeleton_pulse_system(
    time: Res<Time>,
    mut query: Query<(&LumaSkeleton, &mut UNode)>,
) {
    let t = time.elapsed_secs();
    for (skeleton, mut node) in query.iter_mut() {
        let factor = (t * skeleton.pulse_speed).sin() * 0.5 + 0.5; // [0.0, 1.0]
        let base_linear = SKELETON_BASE_COLOR.to_linear();
        let peak_linear = SKELETON_PEAK_COLOR.to_linear();

        let r = base_linear.red + (peak_linear.red - base_linear.red) * factor;
        let g = base_linear.green + (peak_linear.green - base_linear.green) * factor;
        let b = base_linear.blue + (peak_linear.blue - base_linear.blue) * factor;

        node.background_color = Color::LinearRgba(LinearRgba {
            red: r,
            green: g,
            blue: b,
            alpha: 1.0,
        });
    }
}

pub struct LumaSkeletonPlugin;

impl Plugin for LumaSkeletonPlugin {
    fn build(&self, app: &mut App) {
        app.register_type::<LumaSkeleton>()
            .add_systems(Update, skeleton_pulse_system);
    }
}
