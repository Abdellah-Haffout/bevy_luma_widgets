//! Design tokens and theming for Luma widgets.

use bevy::prelude::*;

/// Cohesive color palette for widget styling.
#[derive(Clone, Debug, Reflect)]
pub struct WidgetColors {
    pub background: Color,
    pub surface: Color,
    pub surface_hover: Color,
    pub surface_border: Color,

    pub primary: Color,
    pub primary_hover: Color,
    pub primary_pressed: Color,

    pub secondary: Color,
    pub secondary_hover: Color,
    pub secondary_pressed: Color,

    pub danger: Color,
    pub danger_hover: Color,
    pub danger_pressed: Color,

    pub success: Color,
    pub warning: Color,
    pub info: Color,

    pub text_primary: Color,
    pub text_muted: Color,
    pub text_dim: Color,
}

impl Default for WidgetColors {
    fn default() -> Self {
        Self {
            background: Color::srgb(0.06, 0.07, 0.09),
            surface: Color::srgb(0.11, 0.13, 0.18),
            surface_hover: Color::srgb(0.15, 0.18, 0.24),
            surface_border: Color::srgb(0.20, 0.24, 0.32),

            primary: Color::srgb(0.20, 0.45, 0.90),
            primary_hover: Color::srgb(0.28, 0.55, 1.00),
            primary_pressed: Color::srgb(0.14, 0.35, 0.75),

            secondary: Color::srgb(0.18, 0.20, 0.27),
            secondary_hover: Color::srgb(0.24, 0.27, 0.36),
            secondary_pressed: Color::srgb(0.13, 0.15, 0.20),

            danger: Color::srgb(0.85, 0.22, 0.28),
            danger_hover: Color::srgb(0.95, 0.30, 0.36),
            danger_pressed: Color::srgb(0.70, 0.16, 0.22),

            success: Color::srgb(0.16, 0.75, 0.45),
            warning: Color::srgb(0.95, 0.65, 0.15),
            info: Color::srgb(0.20, 0.65, 0.95),

            text_primary: Color::srgb(0.95, 0.96, 0.98),
            text_muted: Color::srgb(0.60, 0.65, 0.75),
            text_dim: Color::srgb(0.40, 0.45, 0.55),
        }
    }
}

/// Standard corner radius tokens.
#[derive(Clone, Copy, Debug, Reflect)]
pub struct WidgetRadii {
    pub none: f32,
    pub sm: f32,
    pub md: f32,
    pub lg: f32,
    pub xl: f32,
    pub full: f32,
}

impl Default for WidgetRadii {
    fn default() -> Self {
        Self {
            none: 0.0,
            sm: 4.0,
            md: 8.0,
            lg: 12.0,
            xl: 16.0,
            full: 999.0,
        }
    }
}

/// Global widget theme resource.
#[derive(Resource, Clone, Debug, Default, Reflect)]
pub struct LumaWidgetTheme {
    pub colors: WidgetColors,
    pub radii: WidgetRadii,
}
