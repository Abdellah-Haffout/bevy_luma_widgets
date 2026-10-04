//! Progress bar widget with variants and smooth fill sync.

use bevy::prelude::*;
use bevy_luma::prelude::*;

/// Styling variant for a progress bar.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Default, Reflect)]
pub enum ProgressVariant {
    #[default]
    Default,
    Success,
    Warning,
    Danger,
}

impl ProgressVariant {
    pub fn color(&self) -> Color {
        match self {
            ProgressVariant::Default => Color::srgb(0.20, 0.45, 0.90),
            ProgressVariant::Success => Color::srgb(0.16, 0.75, 0.45),
            ProgressVariant::Warning => Color::srgb(0.95, 0.65, 0.15),
            ProgressVariant::Danger => Color::srgb(0.85, 0.22, 0.28),
        }
    }
}

/// Marker and state component for a progress bar.
#[derive(Component, Clone, Debug, Reflect)]
pub struct LumaProgressBar {
    pub value: f32, // 0.0 to 1.0
    pub variant: ProgressVariant,
}

impl Default for LumaProgressBar {
    fn default() -> Self {
        Self {
            value: 0.0,
            variant: ProgressVariant::Default,
        }
    }
}

/// Marker component for the internal fill element.
#[derive(Component, Clone, Debug, Default, Reflect)]
pub struct LumaProgressFill;

/// Creates a progress bar scene.
pub fn luma_progress_bar(
    value: f32,
    width: UVal,
    height: f32,
    variant: ProgressVariant,
) -> impl Scene {
    let clamped = value.clamp(0.0, 1.0);
    let fill_color = variant.color();

    bsn! {
        LumaProgressBar {
            value: clamped,
            variant,
        }
        UNode {
            width,
            height: UVal::Px(height),
            background_color: Color::srgb(0.15, 0.18, 0.24),
            border_radius: UCornerRadius::all(999.0),
        }
        UBorder {
            color: Color::srgb(0.22, 0.26, 0.34),
            width: 1.0,
            radius: UCornerRadius::all(999.0),
        }
        ULayout {
            display: UDisplay::Flex,
            align_items: UAlignItems::Center,
        }
        Children [
            (
                LumaProgressFill
                UNode {
                    width: UVal::Percent(clamped),
                    height: UVal::Percent(1.0),
                    background_color: fill_color,
                    border_radius: UCornerRadius::all(999.0),
                }
            )
        ]
    }
}

fn progress_bar_sync_system(
    bar_query: Query<(Entity, &LumaProgressBar), Changed<LumaProgressBar>>,
    children_query: Query<&Children>,
    mut fill_query: Query<(&mut UNode, &mut LumaProgressFill)>,
) {
    for (bar_entity, bar) in bar_query.iter() {
        let clamped = bar.value.clamp(0.0, 1.0);
        let color = bar.variant.color();

        if let Ok(children) = children_query.get(bar_entity) {
            for child in children.iter() {
                if let Ok((mut node, _)) = fill_query.get_mut(child) {
                    node.width = UVal::Percent(clamped);
                    node.background_color = color;
                }
            }
        }
    }
}

pub struct LumaProgressPlugin;

impl Plugin for LumaProgressPlugin {
    fn build(&self, app: &mut App) {
        app.register_type::<LumaProgressBar>()
            .register_type::<LumaProgressFill>()
            .register_type::<ProgressVariant>()
            .add_systems(Update, progress_bar_sync_system);
    }
}
