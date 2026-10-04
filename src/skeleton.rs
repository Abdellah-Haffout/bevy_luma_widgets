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

/// Creates an animated loading skeleton placeholder scene.
pub fn luma_skeleton(width: UVal, height: UVal, radius: f32) -> impl Scene {
    bsn! {
        LumaSkeleton::default()
        UNode {
            width,
            height,
            background_color: SKELETON_BASE_COLOR,
            border_radius: UCornerRadius::all(radius),
        }
    }
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
