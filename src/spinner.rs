//! Circular spinner loader widget with smooth continuous rotation.

use bevy::prelude::*;
use bevy::scene::{ResolveContext, ResolvedScene, ResolveSceneError};
use bevy_luma::prelude::*;

use crate::theme::SpinnerStyle;

/// Component marking an active rotating spinner.
#[derive(Component, Clone, Debug, Reflect)]
pub struct LumaSpinner {
    pub speed: f32,
    pub angle: f32,
}

impl Default for LumaSpinner {
    fn default() -> Self {
        Self {
            speed: 6.0,
            angle: 0.0,
        }
    }
}

/// Declarative Spinner widget struct.
#[derive(Clone, Debug, Reflect)]
pub struct Spinner {
    pub size: f32,
    pub color: Color,
    pub track_color: Color,
    pub thickness: f32,
    pub speed: f32,
    pub style: Option<SpinnerStyle>,
}

impl Default for Spinner {
    fn default() -> Self {
        Self {
            size: 24.0,
            color: Color::srgb(0.35, 0.65, 1.00),
            track_color: Color::srgba(1.0, 1.0, 1.0, 0.15),
            thickness: 3.0,
            speed: 6.0,
            style: None,
        }
    }
}

impl Spinner {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn size(mut self, size: f32) -> Self {
        self.size = size;
        self
    }

    pub fn color(mut self, color: Color) -> Self {
        self.color = color;
        self
    }

    pub fn track_color(mut self, track_color: Color) -> Self {
        self.track_color = track_color;
        self
    }

    pub fn thickness(mut self, thickness: f32) -> Self {
        self.thickness = thickness;
        self
    }

    pub fn speed(mut self, speed: f32) -> Self {
        self.speed = speed;
        self
    }

    pub fn style(mut self, style: SpinnerStyle) -> Self {
        self.style = Some(style);
        self
    }

    pub fn sm() -> Self {
        Self::default().style(SpinnerStyle::sm())
    }

    pub fn md() -> Self {
        Self::default().style(SpinnerStyle::md())
    }

    pub fn lg() -> Self {
        Self::default().style(SpinnerStyle::lg())
    }
}

impl Scene for Spinner {
    fn resolve(
        self,
        context: &mut ResolveContext,
        scene: &mut ResolvedScene,
    ) -> Result<(), ResolveSceneError> {
        let (size, color, thickness, speed) = if let Some(ref s) = self.style {
            (s.size, s.color, s.thickness, s.speed)
        } else {
            (self.size, self.color, self.thickness, self.speed)
        };

        let radius = size / 2.0;

        let s = bsn! {
            LumaSpinner {
                speed,
                angle: 0.0,
            }
            UNode {
                width: UVal::Px(size),
                height: UVal::Px(size),
                background_color: Color::NONE,
                border_radius: UCornerRadius::all(radius),
            }
            UBorder {
                color,
                width: thickness,
                radius: UCornerRadius::all(radius),
            }
            ULayout {
                display: UDisplay::Flex,
                justify_content: UJustifyContent::Center,
                align_items: UAlignItems::Center,
            }
        };

        s.resolve(context, scene)
    }
}

/// Helper constructor to spawn a standard spinner.
pub fn luma_spinner(size: f32, color: Color) -> Spinner {
    Spinner::new().size(size).color(color)
}

fn spinner_rotation_system(
    time: Option<Res<Time>>,
    mut query: Query<(&mut LumaSpinner, &mut Transform)>,
) {
    let Some(time) = time else {
        return;
    };
    let dt = time.delta_secs();

    for (mut spinner, mut transform) in query.iter_mut() {
        spinner.angle += spinner.speed * dt;
        if spinner.angle > std::f32::consts::TAU {
            spinner.angle -= std::f32::consts::TAU;
        }
        transform.rotation = Quat::from_rotation_z(spinner.angle);
    }
}

/// Plugin registering spinner systems and components.
pub struct LumaSpinnerPlugin;

impl Plugin for LumaSpinnerPlugin {
    fn build(&self, app: &mut App) {
        app.register_type::<LumaSpinner>()
            .add_systems(Update, spinner_rotation_system);
    }
}
