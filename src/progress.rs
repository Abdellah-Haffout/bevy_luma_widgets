//! Progress widgets: linear ProgressBar and CircularProgress ring with semantic variants.

use bevy::prelude::*;
use bevy::scene::{ResolveContext, ResolveSceneError, ResolvedScene};
use bevy_luma::prelude::*;

use crate::helpers::ConditionalScene;
use crate::theme::{ProgressStyle, WidgetColors};

/// Styling variant for a progress indicator.
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
        let c = WidgetColors::default();
        match self {
            ProgressVariant::Default => c.primary,
            ProgressVariant::Success => c.success,
            ProgressVariant::Warning => c.warning,
            ProgressVariant::Danger => c.danger,
        }
    }
}

// ============================================================================
// Linear Progress Bar
// ============================================================================

/// Marker and state component for a linear progress bar.
#[derive(Component, Clone, Debug, Reflect)]
pub struct LumaProgressBar {
    pub value: f32, // 0.0 to 1.0
    pub variant: ProgressVariant,
    pub style: ProgressStyle,
    pub has_explicit_style: bool,
    pub has_explicit_height: bool,
    pub is_default_variant: bool,
}

impl Default for LumaProgressBar {
    fn default() -> Self {
        Self {
            value: 0.0,
            variant: ProgressVariant::Default,
            style: ProgressStyle::default(),
            has_explicit_style: false,
            has_explicit_height: false,
            is_default_variant: true,
        }
    }
}

/// Marker component for the internal fill element.
#[derive(Component, Clone, Debug, Default, Reflect)]
pub struct LumaProgressFill;

/// Declarative ProgressBar widget struct with all configurable properties.
#[derive(Clone, Debug, Reflect)]
pub struct ProgressBar {
    pub value: f32, // 0.0 to 1.0
    pub width: UVal,
    pub height: f32,
    pub variant: ProgressVariant,
    pub style: Option<ProgressStyle>,
}

impl Default for ProgressBar {
    fn default() -> Self {
        Self {
            value: 0.0,
            width: UVal::Px(200.0),
            height: 8.0,
            variant: ProgressVariant::Default,
            style: None,
        }
    }
}

impl ProgressBar {
    pub fn new(value: f32) -> Self {
        Self {
            value,
            ..default()
        }
    }

    pub fn value(mut self, value: f32) -> Self {
        self.value = value;
        self
    }

    pub fn width(mut self, width: UVal) -> Self {
        self.width = width;
        self
    }

    pub fn height(mut self, height: f32) -> Self {
        self.height = height;
        self
    }

    pub fn variant(mut self, variant: ProgressVariant) -> Self {
        self.variant = variant;
        self
    }

    pub fn style(mut self, style: ProgressStyle) -> Self {
        self.style = Some(style);
        self
    }

    /// Convenience constructor for a circular progress indicator.
    pub fn circle(value: f32) -> CircularProgress {
        CircularProgress::new(value)
    }
}

impl Scene for ProgressBar {
    fn resolve(
        self,
        context: &mut ResolveContext,
        scene: &mut ResolvedScene,
    ) -> Result<(), ResolveSceneError> {
        let clamped = self.value.clamp(0.0, 1.0);
        let variant = self.variant;
        let has_explicit_style = self.style.is_some();
        let has_explicit_height = (self.height - 8.0).abs() > f32::EPSILON;
        let is_default_variant = !has_explicit_style && variant == ProgressVariant::Default;
        let style = self.style.unwrap_or_default();

        let fill_color = if variant == ProgressVariant::Default {
            style.fill_color
        } else {
            variant.color()
        };
        let track_color = style.track_color;
        let border_color = style.border_color;
        let border_width = style.border_width;
        let width = self.width;
        let height = if has_explicit_height {
            self.height
        } else {
            style.height
        };
        let radius = if has_explicit_height || has_explicit_style {
            style.radius.max(height * 0.5).min(height * 0.5)
        } else {
            style.radius
        };
        let border_radius = if has_explicit_style { style.radius } else { radius };
        let luma_bar = LumaProgressBar {
            value: clamped,
            variant,
            style,
            has_explicit_style,
            has_explicit_height,
            is_default_variant,
        };

        let s = bsn! {
            luma_bar
            UNode {
                width,
                height: { UVal::Px(height) },
                background_color: track_color,
                border_radius: { UCornerRadius::all(border_radius) },
            }
            UBorder {
                color: border_color,
                width: border_width,
                radius: { UCornerRadius::all(border_radius) },
            }
            ULayout {
                display: UDisplay::Flex,
                align_items: UAlignItems::Center,
            }
            Children [
                LumaProgressFill
                UNode {
                    width: { UVal::Percent(clamped) },
                    height: { UVal::Percent(1.0) },
                    background_color: fill_color,
                    border_radius: { UCornerRadius::all(border_radius) },
                }
                Visibility::from(if clamped <= 0.001 { Visibility::Hidden } else { Visibility::Inherited })
            ]
        };
        s.resolve(context, scene)
    }
}

/// Creates a linear progress bar scene.
pub fn luma_progress_bar(
    value: f32,
    width: UVal,
    height: f32,
    variant: ProgressVariant,
) -> ProgressBar {
    ProgressBar::new(value).width(width).height(height).variant(variant)
}

fn progress_bar_sync_system(
    bar_query: Query<(Entity, &LumaProgressBar), Changed<LumaProgressBar>>,
    children_query: Query<&Children>,
    mut fill_query: Query<(&mut UNode, &mut Visibility), With<LumaProgressFill>>,
) {
    for (bar_entity, bar) in bar_query.iter() {
        let clamped = bar.value.clamp(0.0, 1.0);
        let color = if bar.variant == ProgressVariant::Default {
            bar.style.fill_color
        } else {
            bar.variant.color()
        };

        if let Ok(children) = children_query.get(bar_entity) {
            for child in children.iter() {
                if let Ok((mut node, mut vis)) = fill_query.get_mut(child) {
                    node.width = UVal::Percent(clamped);
                    node.background_color = color;
                    let target_vis = if clamped <= 0.001 {
                        Visibility::Hidden
                    } else {
                        Visibility::Inherited
                    };
                    if *vis != target_vis {
                        *vis = target_vis;
                    }
                }
            }
        }
    }
}

// ============================================================================
// Circular Progress (Ring Indicator)
// ============================================================================

/// Marker and state component for a circular progress ring.
#[derive(Component, Clone, Debug, Reflect)]
pub struct LumaCircularProgress {
    pub value: f32, // 0.0 to 1.0
    pub variant: ProgressVariant,
    pub size: f32,
    pub thickness: f32,
    pub track_color: Color,
    pub show_label: bool,
}

impl Default for LumaCircularProgress {
    fn default() -> Self {
        Self {
            value: 0.0,
            variant: ProgressVariant::Default,
            size: 64.0,
            thickness: 6.0,
            track_color: Color::srgba(0.20, 0.25, 0.35, 0.40),
            show_label: true,
        }
    }
}

/// Marker component for the internal circular progress percentage text.
#[derive(Component, Clone, Debug, Default, Reflect)]
pub struct LumaCircularProgressText;

/// Declarative CircularProgress widget struct with configurable properties.
#[derive(Clone, Debug, Reflect)]
pub struct CircularProgress {
    pub value: f32, // 0.0 to 1.0
    pub size: f32,
    pub thickness: f32,
    pub variant: ProgressVariant,
    pub track_color: Color,
    pub show_label: bool,
    pub font: Handle<Font>,
}

impl Default for CircularProgress {
    fn default() -> Self {
        Self {
            value: 0.0,
            size: 64.0,
            thickness: 6.0,
            variant: ProgressVariant::Default,
            track_color: Color::srgba(0.20, 0.25, 0.35, 0.40),
            show_label: true,
            font: Handle::default(),
        }
    }
}

impl CircularProgress {
    pub fn new(value: f32) -> Self {
        Self {
            value,
            ..default()
        }
    }

    pub fn value(mut self, value: f32) -> Self {
        self.value = value;
        self
    }

    pub fn size(mut self, size: f32) -> Self {
        self.size = size;
        self
    }

    pub fn thickness(mut self, thickness: f32) -> Self {
        self.thickness = thickness;
        self
    }

    pub fn variant(mut self, variant: ProgressVariant) -> Self {
        self.variant = variant;
        self
    }

    pub fn track_color(mut self, track_color: Color) -> Self {
        self.track_color = track_color;
        self
    }

    pub fn show_label(mut self, show_label: bool) -> Self {
        self.show_label = show_label;
        self
    }

    pub fn font(mut self, font: Handle<Font>) -> Self {
        self.font = font;
        self
    }

    /// Compact small circular progress (36px diameter, 4px track).
    pub fn sm(value: f32) -> Self {
        Self::new(value).size(36.0).thickness(4.0)
    }

    /// Standard medium circular progress (64px diameter, 6px track).
    pub fn md(value: f32) -> Self {
        Self::new(value).size(64.0).thickness(6.0)
    }

    /// Large circular progress (96px diameter, 8px track).
    pub fn lg(value: f32) -> Self {
        Self::new(value).size(96.0).thickness(8.0)
    }

    /// Extra-large prominent circular progress (128px diameter, 10px track).
    pub fn xl(value: f32) -> Self {
        Self::new(value).size(128.0).thickness(10.0)
    }
}

/// Builds the stepped conic gradient for circular progress ring rendering.
pub fn build_conic_gradient(value: f32, fill_color: Color, track_color: Color) -> UGradient {
    let clamped = value.clamp(0.0, 1.0);
    let stops = if clamped <= 0.001 {
        vec![
            UGradientStop::new(0.0, track_color),
            UGradientStop::new(1.0, track_color),
        ]
    } else if clamped >= 0.999 {
        vec![
            UGradientStop::new(0.0, fill_color),
            UGradientStop::new(1.0, fill_color),
        ]
    } else {
        vec![
            UGradientStop::new(0.0, fill_color),
            UGradientStop::new(clamped, track_color),
            UGradientStop::new(1.0, track_color),
        ]
    };
    UGradient::angular_stops(stops).stepped()
}

impl Scene for CircularProgress {
    fn resolve(
        self,
        context: &mut ResolveContext,
        scene: &mut ResolvedScene,
    ) -> Result<(), ResolveSceneError> {
        let clamped = self.value.clamp(0.0, 1.0);
        let variant = self.variant;
        let fill_color = variant.color();
        let size = self.size;
        let thickness = self.thickness;
        let radius = size * 0.5;
        let track_color = self.track_color;
        let show_label = self.show_label;
        let font = self.font;
        let font_size = (size * 0.22).clamp(10.0, 32.0);
        let label_text = format!("{:.0}%", clamped * 100.0);

        let grad = build_conic_gradient(clamped, fill_color, track_color);

        let s = bsn! {
            LumaCircularProgress {
                value: clamped,
                variant,
                size,
                thickness,
                track_color,
                show_label,
            }
            UNode {
                width: { UVal::Px(size) },
                height: { UVal::Px(size) },
                background_color: Color::NONE,
                border_radius: { UCornerRadius::all(radius) },
            }
            UBorder {
                color: Color::NONE,
                width: thickness,
                radius: { UCornerRadius::all(radius) },
            }
            grad
            ULayout {
                display: UDisplay::Flex,
                justify_content: UJustifyContent::Center,
                align_items: UAlignItems::Center,
            }
            Children [
                @{ConditionalScene(show_label.then(|| bsn! {
                    LumaCircularProgressText
                    UText {
                        text: label_text,
                        font_size,
                        font,
                        color: Color::srgb(0.95, 0.96, 0.98),
                    }
                }))}
            ]
        };
        s.resolve(context, scene)
    }
}

/// Creates a circular progress ring scene.
pub fn luma_circular_progress(
    value: f32,
    size: f32,
    variant: ProgressVariant,
) -> CircularProgress {
    CircularProgress::new(value).size(size).variant(variant)
}

fn circular_progress_sync_system(
    progress_query: Query<(Entity, &LumaCircularProgress), Changed<LumaCircularProgress>>,
    mut grad_query: Query<&mut UGradient>,
    mut node_query: Query<&mut UNode>,
    mut border_query: Query<&mut UBorder>,
    children_query: Query<&Children>,
    mut text_query: Query<&mut UText, With<LumaCircularProgressText>>,
) {
    for (entity, prog) in progress_query.iter() {
        let clamped = prog.value.clamp(0.0, 1.0);
        let fill_color = prog.variant.color();
        let grad = build_conic_gradient(clamped, fill_color, prog.track_color);

        if let Ok(mut ugrad) = grad_query.get_mut(entity) {
            *ugrad = grad;
        }

        if let Ok(mut node) = node_query.get_mut(entity) {
            let radius = prog.size * 0.5;
            node.width = UVal::Px(prog.size);
            node.height = UVal::Px(prog.size);
            node.border_radius = UCornerRadius::all(radius);
        }

        if let Ok(mut border) = border_query.get_mut(entity) {
            let radius = prog.size * 0.5;
            border.width = prog.thickness;
            border.radius = UCornerRadius::all(radius);
        }

        if prog.show_label {
            let percent_str = format!("{:.0}%", clamped * 100.0);
            let font_size = (prog.size * 0.22).clamp(10.0, 32.0);
            if let Ok(children) = children_query.get(entity) {
                for child in children.iter() {
                    if let Ok(mut utext) = text_query.get_mut(child) {
                        utext.text = percent_str.clone();
                        utext.font_size = font_size;
                    }
                }
            }
        }
    }
}

// ============================================================================
// Plugin
// ============================================================================

pub struct LumaProgressPlugin;

impl Plugin for LumaProgressPlugin {
    fn build(&self, app: &mut App) {
        app.register_type::<LumaProgressBar>()
            .register_type::<LumaProgressFill>()
            .register_type::<LumaCircularProgress>()
            .register_type::<LumaCircularProgressText>()
            .register_type::<ProgressVariant>()
            .add_systems(
                Update,
                (progress_bar_sync_system, circular_progress_sync_system),
            );
    }
}
