//! Circular spinner loader widget with smooth continuous rotation.

use bevy::prelude::*;
use bevy::scene::{ResolveContext, ResolvedScene, ResolveSceneError};
use bevy_luma::prelude::*;

use crate::theme::{SpinnerStyle, WidgetColors};

/// Component marking an active rotating spinner.
#[derive(Component, Clone, Debug, Reflect)]
pub struct LumaSpinner {
    pub speed: f32,
    pub angle: f32,
    pub has_explicit_style: bool,
}

impl Default for LumaSpinner {
    fn default() -> Self {
        Self {
            speed: 6.0,
            angle: 0.0,
            has_explicit_style: false,
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
        let base = SpinnerStyle::default();
        Self {
            size: base.size,
            color: WidgetColors::default().primary,
            track_color: base.track_color,
            thickness: base.thickness,
            speed: base.speed,
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
        let has_explicit_style = self.style.is_some();
        let (size, color, track_color, thickness, speed) = if let Some(ref s) = self.style {
            (s.size, s.color, s.track_color, s.thickness, s.speed)
        } else {
            (self.size, self.color, self.track_color, self.thickness, self.speed)
        };

        let radius = size / 2.0;
        let grad = UGradient::angular_stops(vec![
            UGradientStop::new(0.0, color),
            UGradientStop::new(0.30, track_color),
            UGradientStop::new(1.0, track_color),
        ])
        .stepped();

        let s = bsn! {
            LumaSpinner {
                speed,
                angle: 0.0,
                has_explicit_style,
            }
            UNode {
                width: UVal::Px(size),
                height: UVal::Px(size),
                background_color: Color::NONE,
                border_radius: UCornerRadius::all(radius),
            }
            UBorder {
                color: Color::NONE,
                width: thickness,
                radius: UCornerRadius::all(radius),
            }
            grad
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
