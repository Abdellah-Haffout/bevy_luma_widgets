//! Interactive DragValue numeric scrubber widget.
//!
//! Allows continuous horizontal scrubbing by clicking and dragging,
//! as well as single-step increments/decrements via arrow buttons.
//! Features Blender-style internal slider fill bar, 3D coordinate axis presets (X/Y/Z),
//! and split label/value layouts.

use bevy::prelude::*;
use bevy::scene::{ResolveContext, ResolvedScene, ResolveSceneError};
use bevy_luma::prelude::*;

use crate::helpers::{find_ancestor_or_self, interaction_colors, is_primary_click, snap_stepped};
use crate::theme::{DragValueStyle, WidgetColors};

/// Axis styling mode inspired by Blender 3D's Transform inputs.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Default, Reflect)]
pub enum DragValueAxis {
    #[default]
    None,
    /// Blender X Axis (Red accent stripe & glow)
    X,
    /// Blender Y Axis (Green accent stripe & glow)
    Y,
    /// Blender Z Axis (Blue accent stripe & glow)
    Z,
    /// Custom accent color
    Custom,
}

impl DragValueAxis {
    /// Returns the primary accent color associated with this axis (Blender 3D palette).
    pub fn color(&self) -> Option<Color> {
        match self {
            DragValueAxis::None => None,
            DragValueAxis::X => Some(Color::srgb(0.86, 0.24, 0.24)), // Blender Red
            DragValueAxis::Y => Some(Color::srgb(0.33, 0.78, 0.33)), // Blender Green
            DragValueAxis::Z => Some(Color::srgb(0.26, 0.53, 0.94)), // Blender Blue
            DragValueAxis::Custom => None,
        }
    }

    /// Returns the internal slider fill bar color associated with this axis.
    pub fn fill_color(&self) -> Color {
        match self {
            DragValueAxis::None => Color::srgba(0.28, 0.45, 0.70, 0.35), // Blender Slate Blue
            DragValueAxis::X => Color::srgba(0.86, 0.24, 0.24, 0.30),
            DragValueAxis::Y => Color::srgba(0.33, 0.78, 0.33, 0.30),
            DragValueAxis::Z => Color::srgba(0.26, 0.53, 0.94, 0.30),
            DragValueAxis::Custom => Color::srgba(0.28, 0.45, 0.70, 0.35),
        }
    }
}

/// Runtime state and parameters for a DragValue scrubber.
#[derive(Component, Clone, Debug, Reflect)]
pub struct LumaDragValue {
    pub value: f32,
    pub speed: f32,
    pub min: Option<f32>,
    pub max: Option<f32>,
    pub step: Option<f32>,
    pub prefix: String,
    pub suffix: String,
    pub decimals: usize,
    pub disabled: bool,
    pub axis: DragValueAxis,
    pub style: DragValueStyle,
    pub has_explicit_style: bool,
}

impl Default for LumaDragValue {
    fn default() -> Self {
        Self {
            value: 0.0,
            speed: 0.1,
            min: None,
            max: None,
            step: None,
            prefix: String::new(),
            suffix: String::new(),
            decimals: 1,
            disabled: false,
            axis: DragValueAxis::None,
            style: DragValueStyle::default(),
            has_explicit_style: false,
        }
    }
}

impl LumaDragValue {
    /// Formats the current value into a display string.
    pub fn formatted_text(&self) -> String {
        let val_str = format!("{:.*}", self.decimals, self.value);
        format!("{}{}{}", self.prefix, val_str, self.suffix)
    }

    /// Computes the normalized ratio (0.0 to 1.0) for the internal Blender slider fill bar.
    pub fn ratio(&self) -> f32 {
        if let (Some(min), Some(max)) = (self.min, self.max) {
            if (max - min).abs() > f32::EPSILON {
                ((self.value - min) / (max - min)).clamp(0.0, 1.0)
            } else {
                0.0
            }
        } else if let Some(min) = self.min {
            if min >= 0.0 && self.value >= min {
                let ceiling = 100.0f32.max(self.value);
                ((self.value - min) / (ceiling - min)).clamp(0.0, 1.0)
            } else {
                0.0
            }
        } else {
            0.0
        }
    }

    /// Applies a direct delta in value units (used by arrow buttons and keyboard stepping).
    pub fn apply_value_delta(&mut self, value_delta: f32) -> bool {
        if self.disabled || value_delta.abs() < f32::EPSILON {
            return false;
        }

        let raw = self.value + value_delta;

        let stepped = if let Some(step) = self.step {
            snap_stepped(raw, step, self.min.unwrap_or(0.0))
        } else {
            raw
        };

        let mut clamped = stepped;
        if let Some(min) = self.min {
            clamped = clamped.max(min);
        }
        if let Some(max) = self.max {
            clamped = clamped.min(max);
        }

        let factor = 10f32.powi(self.decimals.min(9) as i32);
        let rounded = (clamped * factor).round() / factor;

        if (rounded - self.value).abs() > f32::EPSILON {
            self.value = rounded;
            true
        } else {
            false
        }
    }

    /// Applies a horizontal delta movement to the value.
    ///
    /// Returns `true` if the value actually changed.
    pub fn apply_delta(&mut self, delta_x: f32) -> bool {
        if self.disabled || delta_x.abs() < f32::EPSILON {
            return false;
        }
        self.apply_value_delta(delta_x * self.speed)
    }
}

/// Marker component for the internal slider fill bar of a DragValue (Blender-style).
#[derive(Component, Clone, Debug, Default, Reflect)]
pub struct LumaDragValueFill;

/// Marker component for the optional label text node of a DragValue.
#[derive(Component, Clone, Debug, Default, Reflect)]
pub struct LumaDragValueLabel;

/// Marker component for the internal text node of a DragValue.
#[derive(Component, Clone, Debug, Default, Reflect)]
pub struct LumaDragValueText;

/// Marker component for the decrement button inside a DragValue.
#[derive(Component, Clone, Debug, Default, Reflect)]
pub struct LumaDragValueDecr;

/// Marker component for the increment button inside a DragValue.
#[derive(Component, Clone, Debug, Default, Reflect)]
pub struct LumaDragValueIncr;

/// Event triggered when a DragValue value changes.
#[derive(Event, Clone, Copy, Debug)]
pub struct DragValueChanged {
    pub entity: Entity,
    pub value: f32,
}

/// Declarative DragValue numeric scrubber widget.
#[derive(Clone, Debug, Reflect)]
pub struct DragValue {
    pub value: f32,
    pub speed: f32,
    pub min: Option<f32>,
    pub max: Option<f32>,
    pub step: Option<f32>,
    pub prefix: String,
    pub suffix: String,
    pub decimals: usize,
    pub width: UVal,
    pub disabled: bool,
    pub font: Handle<Font>,
    pub icon_font: Handle<Font>,
    pub style: Option<DragValueStyle>,

    // Blender-specific visual enhancements:
    pub label: Option<String>,
    pub axis: DragValueAxis,
    pub fill_color: Option<Color>,
    pub show_slider: bool,
    pub show_arrows: bool,
}

impl Default for DragValue {
    fn default() -> Self {
        Self {
            value: 0.0,
            speed: 0.1,
            min: None,
            max: None,
            step: None,
            prefix: String::new(),
            suffix: String::new(),
            decimals: 1,
            width: UVal::Px(130.0),
            disabled: false,
            font: Handle::default(),
            icon_font: Handle::default(),
            style: None,
            label: None,
            axis: DragValueAxis::None,
            fill_color: None,
            show_slider: true,
            show_arrows: true,
        }
    }
}

impl DragValue {
    pub fn new(value: f32) -> Self {
        Self {
            value,
            ..default()
        }
    }

    /// Creates a Blender-style X axis coordinate scrubber (Red theme).
    pub fn blender_x(value: f32) -> Self {
        Self::new(value)
            .axis(DragValueAxis::X)
            .label("X")
    }

    /// Creates a Blender-style Y axis coordinate scrubber (Green theme).
    pub fn blender_y(value: f32) -> Self {
        Self::new(value)
            .axis(DragValueAxis::Y)
            .label("Y")
    }

    /// Creates a Blender-style Z axis coordinate scrubber (Blue theme).
    pub fn blender_z(value: f32) -> Self {
        Self::new(value)
            .axis(DragValueAxis::Z)
            .label("Z")
    }

    /// Creates a Blender-style property scrubber with a label and range.
    pub fn blender_slider(label: impl Into<String>, value: f32, min: f32, max: f32) -> Self {
        Self::new(value)
            .label(label)
            .range(min, max)
            .show_slider(true)
    }

    /// Creates a Blender-style property scrubber with a label.
    pub fn blender(label: impl Into<String>, value: f32) -> Self {
        Self::new(value).label(label)
    }

    pub fn axis(mut self, axis: DragValueAxis) -> Self {
        self.axis = axis;
        self
    }

    pub fn x_axis(self) -> Self {
        self.axis(DragValueAxis::X)
    }

    pub fn y_axis(self) -> Self {
        self.axis(DragValueAxis::Y)
    }

    pub fn z_axis(self) -> Self {
        self.axis(DragValueAxis::Z)
    }

    pub fn label(mut self, label: impl Into<String>) -> Self {
        self.label = Some(label.into());
        self
    }

    pub fn fill_color(mut self, color: Color) -> Self {
        self.fill_color = Some(color);
        self
    }

    pub fn show_slider(mut self, show: bool) -> Self {
        self.show_slider = show;
        self
    }

    pub fn show_arrows(mut self, show: bool) -> Self {
        self.show_arrows = show;
        self
    }

    pub fn speed(mut self, speed: f32) -> Self {
        self.speed = speed;
        self
    }

    pub fn range(mut self, min: f32, max: f32) -> Self {
        self.min = Some(min);
        self.max = Some(max);
        self
    }

    pub fn min(mut self, min: f32) -> Self {
        self.min = Some(min);
        self
    }

    pub fn max(mut self, max: f32) -> Self {
        self.max = Some(max);
        self
    }

    pub fn step(mut self, step: f32) -> Self {
        self.step = Some(step);
        self
    }

    pub fn prefix(mut self, prefix: impl Into<String>) -> Self {
        self.prefix = prefix.into();
        self
    }

    pub fn suffix(mut self, suffix: impl Into<String>) -> Self {
        self.suffix = suffix.into();
        self
    }

    pub fn decimals(mut self, decimals: usize) -> Self {
        self.decimals = decimals;
        self
    }

    pub fn width(mut self, width: impl Into<UVal>) -> Self {
        self.width = width.into();
        self
    }

    pub fn disabled(mut self, disabled: bool) -> Self {
        self.disabled = disabled;
        self
    }

    pub fn font(mut self, font: Handle<Font>) -> Self {
        self.font = font;
        self
    }

    pub fn icon_font(mut self, icon_font: Handle<Font>) -> Self {
        self.icon_font = icon_font;
        self
    }

    pub fn style(mut self, style: DragValueStyle) -> Self {
        self.style = Some(style);
        self
    }
}

impl Scene for DragValue {
    fn resolve(
        self,
        context: &mut ResolveContext,
        scene: &mut ResolvedScene,
    ) -> Result<(), ResolveSceneError> {
        let has_explicit_style = self.style.is_some();
        let style = self.style.unwrap_or_default();
        let colors = WidgetColors::default();
        let bg = if self.disabled {
            colors.disabled_bg()
        } else {
            style.bg_normal
        };

        let axis = self.axis;
        let axis_color = axis.color();
        let effective_fill_color = self.fill_color.unwrap_or_else(|| {
            if axis != DragValueAxis::None {
                axis.fill_color()
            } else {
                style.fill_color
            }
        });

        let border_color = if self.disabled {
            colors.disabled_border()
        } else if let Some(ac) = axis_color {
            Color::srgba(ac.to_srgba().red, ac.to_srgba().green, ac.to_srgba().blue, 0.40)
        } else {
            style.border_color
        };

        let active_border_color = if let Some(ac) = axis_color {
            ac
        } else {
            style.border_active
        };

        let border_width = style.border_width;
        let radius = style.radius;
        let height = style.height;
        let font_size = style.font_size;
        let label_font_size = font_size * 0.95;
        let root_colors = interaction_colors(bg, style.bg_hover, style.bg_active, self.disabled);
        let text_color = if self.disabled {
            colors.disabled_text()
        } else {
            style.text_color
        };

        let label_color = if self.disabled {
            dim_color_fallback(&colors)
        } else if let Some(ac) = axis_color {
            ac
        } else {
            Color::srgb(0.68, 0.72, 0.80)
        };
        let arrow_color = if self.disabled {
            colors.disabled_text()
        } else {
            Color::srgb(0.50, 0.54, 0.62)
        };

        let formatted = {
            let val_str = format!("{:.*}", self.decimals, self.value);
            format!("{}{}{}", self.prefix, val_str, self.suffix)
        };

        let value = self.value;
        let speed = self.speed;
        let min = self.min;
        let max = self.max;
        let step = self.step;
        let prefix = self.prefix;
        let suffix = self.suffix;
        let decimals = self.decimals;
        let disabled = self.disabled;
        let width = self.width;
        let font = self.font;
        let icon_font = self.icon_font;
        let show_slider = self.show_slider;
        let show_arrows = self.show_arrows;
        let label_str = self.label.clone().unwrap_or_default();
        let has_label = self.label.is_some();

        let luma_drag = LumaDragValue {
            value,
            speed,
            min,
            max,
            step,
            prefix,
            suffix,
            decimals,
            disabled,
            axis,
            style,
            has_explicit_style,
        };
        let ratio = luma_drag.ratio();

        let s = bsn! {
            luma_drag
            UNode {
                width,
                height: UVal::Px(height),
                background_color: bg,
                border_radius: UCornerRadius::all(radius),
            }
            UBorder {
                color: border_color,
                width: border_width,
                radius: UCornerRadius::all(radius),
            }
            ULayout {
                display: UDisplay::Flex,
                flex_direction: UFlexDirection::Row,
                align_items: UAlignItems::Center,
            }
            UInteraction::default()
            root_colors
            UFocusable {
                enabled: { !disabled },
                ..UFocusable::default()
            }
            UFocusVisual::border(active_border_color, 1.5)
            Children [
                // 1. Blender Internal Slider Fill Track (Absolute behind text)
                LumaDragValueFill
                UNode {
                    width: {
                        if show_slider && ratio > 0.0 {
                            UVal::Percent(ratio.min(1.0))
                        } else {
                            UVal::Px(0.0)
                        }
                    },
                    height: UVal::Px(height),
                    background_color: effective_fill_color,
                    border_radius: UCornerRadius::all(radius),
                }
                USelf {
                    position_type: UPositionType::Absolute,
                    left: UVal::Px(0.0),
                    top: UVal::Px(0.0),
                }
                --
                // 2. Blender Axis Accent Left Stripe (3.5px width)
                UNode {
                    width: {
                        if axis_color.is_some() {
                            UVal::Px(3.5)
                        } else {
                            UVal::Px(0.0)
                        }
                    },
                    height: UVal::Px(height),
                    background_color: { axis_color.unwrap_or(Color::NONE) },
                    border_radius: UCornerRadius {
                        top_left: radius,
                        bottom_left: radius,
                        top_right: 0.0,
                        bottom_right: 0.0,
                    },
                }
                USelf {
                    position_type: UPositionType::Absolute,
                    left: UVal::Px(0.0),
                    top: UVal::Px(0.0),
                }
                --
                // 3. Foreground Interactive Row Layer (Arrows, Label, Value Text)
                UNode {
                    width: UVal::Percent(1.0),
                    height: UVal::Px(height),
                    padding: USides::axes(6.0, 2.0),
                }
                ULayout {
                    display: UDisplay::Flex,
                    flex_direction: UFlexDirection::Row,
                    justify_content: UJustifyContent::SpaceBetween,
                    align_items: UAlignItems::Center,
                }
                USelf {
                    position_type: UPositionType::Absolute,
                    left: UVal::Px(0.0),
                    top: UVal::Px(0.0),
                }
                Children [
                    // Left Decrement arrow `<`
                    LumaDragValueDecr
                    UNode {
                        padding: USides::axes(4.0, 2.0),
                    }
                    UInteraction::default()
                    ULayout {
                        display: { if show_arrows { UDisplay::Flex } else { UDisplay::None } },
                        align_items: UAlignItems::Center,
                        justify_content: UJustifyContent::Center,
                    }
                    Children [
                        UText {
                            text: { Icon::CHEVRON_LEFT.to_string() },
                            font_size: 10.0,
                            font: { icon_font.clone() },
                            color: arrow_color,
                        }
                    ]
                    --
                    // Center Content (Split Label & Value or Centered Value)
                    UNode::default()
                    USelf {
                        flex_grow: { Some(1.0) },
                    }
                    ULayout {
                        display: UDisplay::Flex,
                        flex_direction: UFlexDirection::Row,
                        justify_content: { if has_label { UJustifyContent::SpaceBetween } else { UJustifyContent::Center } },
                        align_items: UAlignItems::Center,
                        gap: 6.0,
                    }
                    Children [
                        // Optional Left Label (e.g. "X", "Y", "Z", or "Roughness")
                        UNode {
                            margin: USides::axes(2.0, 0.0),
                        }
                        ULayout {
                            display: { if has_label { UDisplay::Flex } else { UDisplay::None } },
                            align_items: UAlignItems::Center,
                        }
                        Children [
                            LumaDragValueLabel
                            UText {
                                text: label_str,
                                font_size: label_font_size,
                                font: { font.clone() },
                                color: label_color,
                            }
                        ]
                        --
                        // Formatted Value Text
                        UNode::default()
                        ULayout {
                            display: UDisplay::Flex,
                            align_items: UAlignItems::Center,
                            justify_content: UJustifyContent::Center,
                        }
                        Children [
                            LumaDragValueText
                            UText {
                                text: formatted,
                                font_size,
                                font: { font.clone() },
                                color: text_color,
                            }
                        ]
                    ]
                    --
                    // Right Increment arrow `>`
                    LumaDragValueIncr
                    UNode {
                        padding: USides::axes(4.0, 2.0),
                    }
                    UInteraction::default()
                    ULayout {
                        display: { if show_arrows { UDisplay::Flex } else { UDisplay::None } },
                        align_items: UAlignItems::Center,
                        justify_content: UJustifyContent::Center,
                    }
                    Children [
                        UText {
                            text: { Icon::CHEVRON_RIGHT.to_string() },
                            font_size: 10.0,
                            font: icon_font,
                            color: arrow_color,
                        }
                    ]
                ]
            ]
        };

        s.resolve(context, scene)
    }
}

#[inline]
fn dim_color_fallback(colors: &WidgetColors) -> Color {
    colors.disabled_text()
}

/// Helper constructor to create a continuous numeric scrubber.
pub fn luma_drag_value(value: f32) -> DragValue {
    DragValue::new(value)
}

/// Helper constructor for a Blender-style X coordinate scrubber.
pub fn luma_drag_value_x(value: f32) -> DragValue {
    DragValue::blender_x(value)
}

/// Helper constructor for a Blender-style Y coordinate scrubber.
pub fn luma_drag_value_y(value: f32) -> DragValue {
    DragValue::blender_y(value)
}

/// Helper constructor for a Blender-style Z coordinate scrubber.
pub fn luma_drag_value_z(value: f32) -> DragValue {
    DragValue::blender_z(value)
}

/// Helper constructor for a Blender-style property scrubber with a label.
pub fn luma_drag_value_blender(label: impl Into<String>, value: f32) -> DragValue {
    DragValue::blender(label, value)
}

fn sync_drag_value_visuals(
    root: Entity,
    new_text: &str,
    ratio: f32,
    children_query: &Query<&Children>,
    text_query: &mut Query<&mut UText, With<LumaDragValueText>>,
    fill_query: &mut Query<&mut UNode, With<LumaDragValueFill>>,
) {
    let fill_w = if ratio > 0.0 {
        UVal::Percent(ratio.min(1.0))
    } else {
        UVal::Px(0.0)
    };

    let mut stack = vec![root];
    while let Some(current) = stack.pop() {
        if let Ok(mut text) = text_query.get_mut(current) {
            text.text = new_text.to_string();
        }
        if let Ok(mut fill_node) = fill_query.get_mut(current) {
            fill_node.width = fill_w;
        }
        if let Ok(children) = children_query.get(current) {
            for child in children.iter() {
                stack.push(child);
            }
        }
    }
}

fn apply_drag_delta_and_emit(
    entity: Entity,
    drag_val: &mut LumaDragValue,
    value_delta: f32,
    commands: &mut Commands,
    children_query: &Query<&Children>,
    text_query: &mut Query<&mut UText, With<LumaDragValueText>>,
    fill_query: &mut Query<&mut UNode, With<LumaDragValueFill>>,
) {
    if drag_val.apply_value_delta(value_delta) {
        let new_text = drag_val.formatted_text();
        let new_val = drag_val.value;
        let ratio = drag_val.ratio();

        sync_drag_value_visuals(entity, &new_text, ratio, children_query, text_query, fill_query);

        commands.trigger(DragValueChanged {
            entity,
            value: new_val,
        });
    }
}

fn drag_value_default_border(drag_val: &LumaDragValue) -> Color {
    if drag_val.disabled {
        WidgetColors::default().disabled_border()
    } else if let Some(ac) = drag_val.axis.color() {
        let rgba = ac.to_srgba();
        Color::srgba(rgba.red, rgba.green, rgba.blue, 0.40)
    } else {
        drag_val.style.border_color
    }
}

fn on_drag_value_drag(
    trigger: On<PointerDrag>,
    mut commands: Commands,
    mut query: Query<(Entity, &mut LumaDragValue)>,
    children_query: Query<&Children>,
    parents_query: Query<&ChildOf>,
    mut text_query: Query<&mut UText, With<LumaDragValueText>>,
    mut fill_query: Query<&mut UNode, With<LumaDragValueFill>>,
) {
    if trigger.event().button != PointerButton::Primary {
        return;
    }
    let Some(entity) = find_ancestor_or_self(trigger.entity, &query, &parents_query) else {
        return;
    };

    if let Ok((entity, mut drag_val)) = query.get_mut(entity) {
        if drag_val.disabled {
            return;
        }

        let delta_x = trigger.delta.x;
        let speed = drag_val.speed;
        apply_drag_delta_and_emit(
            entity,
            &mut drag_val,
            delta_x * speed,
            &mut commands,
            &children_query,
            &mut text_query,
            &mut fill_query,
        );
    }
}

fn on_drag_value_drag_start(
    trigger: On<PointerDragStart>,
    query: Query<&LumaDragValue>,
    parents_query: Query<&ChildOf>,
    mut border_query: Query<&mut UBorder>,
) {
    let Some(entity) = find_ancestor_or_self(trigger.entity, &query, &parents_query) else {
        return;
    };
    if let Ok(drag_val) = query.get(entity) {
        if !drag_val.disabled {
            if let Ok(mut border) = border_query.get_mut(entity) {
                let active_color = drag_val.axis.color().unwrap_or(drag_val.style.border_active);
                border.color = active_color;
                border.width = (drag_val.style.border_width + 0.5).max(1.5);
            }
        }
    }
}

fn on_drag_value_drag_end(
    trigger: On<PointerDragEnd>,
    query: Query<&LumaDragValue>,
    parents_query: Query<&ChildOf>,
    mut border_query: Query<&mut UBorder>,
) {
    let Some(entity) = find_ancestor_or_self(trigger.entity, &query, &parents_query) else {
        return;
    };
    if let Ok(drag_val) = query.get(entity) {
        if let Ok(mut border) = border_query.get_mut(entity) {
            border.color = drag_value_default_border(drag_val);
            border.width = drag_val.style.border_width;
        }
    }
}

fn on_drag_value_hover(
    trigger: On<PointerOver>,
    query: Query<&LumaDragValue>,
    parents_query: Query<&ChildOf>,
    mut border_query: Query<&mut UBorder>,
) {
    let Some(entity) = find_ancestor_or_self(trigger.entity, &query, &parents_query) else {
        return;
    };
    if let Ok(drag_val) = query.get(entity) {
        if !drag_val.disabled {
            if let Ok(mut border) = border_query.get_mut(entity) {
                let hover_color = if let Some(ac) = drag_val.axis.color() {
                    let rgba = ac.to_srgba();
                    Color::srgba(rgba.red, rgba.green, rgba.blue, 0.70)
                } else {
                    drag_val.style.border_hover
                };
                border.color = hover_color;
            }
        }
    }
}

fn on_drag_value_out(
    trigger: On<PointerOut>,
    query: Query<&LumaDragValue>,
    parents_query: Query<&ChildOf>,
    mut border_query: Query<&mut UBorder>,
) {
    let Some(entity) = find_ancestor_or_self(trigger.entity, &query, &parents_query) else {
        return;
    };
    if let Ok(drag_val) = query.get(entity) {
        if let Ok(mut border) = border_query.get_mut(entity) {
            border.color = drag_value_default_border(drag_val);
        }
    }
}

fn step_drag_value_arrow<F: bevy::ecs::query::QueryFilter>(
    trigger: &On<PointerClick>,
    sign: f32,
    arrow_query: &Query<Entity, F>,
    drag_val_query: &mut Query<(Entity, &mut LumaDragValue)>,
    children_query: &Query<&Children>,
    parents_query: &Query<&ChildOf>,
    text_query: &mut Query<&mut UText, With<LumaDragValueText>>,
    fill_query: &mut Query<&mut UNode, With<LumaDragValueFill>>,
    commands: &mut Commands,
) {
    if !is_primary_click(trigger) {
        return;
    }
    let Some(arrow_entity) = find_ancestor_or_self(trigger.entity, arrow_query, parents_query) else {
        return;
    };
    let Some(root_entity) = find_ancestor_or_self(arrow_entity, drag_val_query, parents_query) else {
        return;
    };

    if let Ok((entity, mut drag_val)) = drag_val_query.get_mut(root_entity) {
        if drag_val.disabled {
            return;
        }
        let step = drag_val.step.unwrap_or_else(|| (drag_val.speed * 5.0).max(0.1));
        apply_drag_delta_and_emit(
            entity,
            &mut drag_val,
            sign * step,
            commands,
            children_query,
            text_query,
            fill_query,
        );
    }
}

fn on_drag_value_decr_click(
    trigger: On<PointerClick>,
    decr_query: Query<Entity, With<LumaDragValueDecr>>,
    mut drag_val_query: Query<(Entity, &mut LumaDragValue)>,
    children_query: Query<&Children>,
    parents_query: Query<&ChildOf>,
    mut text_query: Query<&mut UText, With<LumaDragValueText>>,
    mut fill_query: Query<&mut UNode, With<LumaDragValueFill>>,
    mut commands: Commands,
) {
    step_drag_value_arrow(
        &trigger,
        -1.0,
        &decr_query,
        &mut drag_val_query,
        &children_query,
        &parents_query,
        &mut text_query,
        &mut fill_query,
        &mut commands,
    );
}

fn on_drag_value_incr_click(
    trigger: On<PointerClick>,
    incr_query: Query<Entity, With<LumaDragValueIncr>>,
    mut drag_val_query: Query<(Entity, &mut LumaDragValue)>,
    children_query: Query<&Children>,
    parents_query: Query<&ChildOf>,
    mut text_query: Query<&mut UText, With<LumaDragValueText>>,
    mut fill_query: Query<&mut UNode, With<LumaDragValueFill>>,
    mut commands: Commands,
) {
    step_drag_value_arrow(
        &trigger,
        1.0,
        &incr_query,
        &mut drag_val_query,
        &children_query,
        &parents_query,
        &mut text_query,
        &mut fill_query,
        &mut commands,
    );
}

fn keyboard_drag_value_system(
    keyboard: Option<Res<ButtonInput<KeyCode>>>,
    mut commands: Commands,
    mut query: Query<(Entity, &mut LumaDragValue), With<UFocused>>,
    children_query: Query<&Children>,
    mut text_query: Query<&mut UText, With<LumaDragValueText>>,
    mut fill_query: Query<&mut UNode, With<LumaDragValueFill>>,
) {
    let Some(keyboard) = keyboard else {
        return;
    };
    let decr = keyboard.just_pressed(KeyCode::ArrowLeft) || keyboard.just_pressed(KeyCode::ArrowDown);
    let incr = keyboard.just_pressed(KeyCode::ArrowRight) || keyboard.just_pressed(KeyCode::ArrowUp);
    if !decr && !incr {
        return;
    }

    let sign = if incr { 1.0 } else { -1.0 };
    for (entity, mut drag_val) in query.iter_mut() {
        if drag_val.disabled {
            continue;
        }
        let step = drag_val.step.unwrap_or_else(|| (drag_val.speed * 5.0).max(0.1));
        apply_drag_delta_and_emit(
            entity,
            &mut drag_val,
            sign * step,
            &mut commands,
            &children_query,
            &mut text_query,
            &mut fill_query,
        );
    }
}

/// Plugin registering DragValue types and observers.
pub struct LumaDragValuePlugin;

impl Plugin for LumaDragValuePlugin {
    fn build(&self, app: &mut App) {
        app.register_type::<DragValueAxis>()
            .register_type::<LumaDragValue>()
            .register_type::<LumaDragValueFill>()
            .register_type::<LumaDragValueLabel>()
            .register_type::<LumaDragValueText>()
            .register_type::<LumaDragValueDecr>()
            .register_type::<LumaDragValueIncr>()
            .add_observer(on_drag_value_drag)
            .add_observer(on_drag_value_drag_start)
            .add_observer(on_drag_value_drag_end)
            .add_observer(on_drag_value_hover)
            .add_observer(on_drag_value_out)
            .add_observer(on_drag_value_decr_click)
            .add_observer(on_drag_value_incr_click)
            .add_systems(Update, keyboard_drag_value_system);
    }
}
