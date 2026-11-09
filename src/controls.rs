//! Specialized input and control widgets complementing the core primitives.

use bevy::prelude::*;
use bevy::scene::{ResolveContext, ResolvedScene, ResolveSceneError};
use bevy_luma::prelude::*;

use crate::prelude::*;
use crate::button::Button;

/// 1.2 Icon Button: square or circular button containing an icon only.
#[derive(Clone, Debug, Reflect)]
pub struct IconButton {
    pub icon: String,
    pub size: f32,
    pub is_circle: bool,
    pub font: Handle<Font>,
    pub icon_font: Handle<Font>,
    pub disabled: bool,
}

impl Default for IconButton {
    fn default() -> Self {
        Self {
            icon: Icon::STAR.to_string(),
            size: 38.0,
            is_circle: false,
            font: Handle::default(),
            icon_font: Handle::default(),
            disabled: false,
        }
    }
}

impl IconButton {
    pub fn new(icon: impl Into<String>) -> Self {
        Self {
            icon: icon.into(),
            ..default()
        }
    }

    pub fn circle(mut self) -> Self {
        self.is_circle = true;
        self
    }

    pub fn size(mut self, size: f32) -> Self {
        self.size = size;
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

    pub fn disabled(mut self, disabled: bool) -> Self {
        self.disabled = disabled;
        self
    }
}

impl Scene for IconButton {
    fn resolve(
        self,
        context: &mut ResolveContext,
        scene: &mut ResolvedScene,
    ) -> Result<(), ResolveSceneError> {
        let radius = if self.is_circle { self.size / 2.0 } else { 8.0 };
        let mut btn = Button::icon_button(self.icon)
            .icon_font(self.icon_font)
            .disabled(self.disabled);
        btn.style = Some(ButtonStyle {
            radius,
            min_height: self.size,
            ..ButtonStyle::icon()
        });
        btn.resolve(context, scene)
    }
}

/// 1.3 Floating Action Button (FAB): elevated prominent action button.
#[derive(Clone, Debug, Reflect)]
pub struct Fab {
    pub icon: String,
    pub label: Option<String>,
    pub icon_font: Handle<Font>,
    pub font: Handle<Font>,
}

impl Default for Fab {
    fn default() -> Self {
        Self {
            icon: Icon::PLUS.to_string(),
            label: None,
            icon_font: Handle::default(),
            font: Handle::default(),
        }
    }
}

impl Fab {
    pub fn new(icon: impl Into<String>) -> Self {
        Self {
            icon: icon.into(),
            ..default()
        }
    }

    pub fn extended(mut self, label: impl Into<String>) -> Self {
        self.label = Some(label.into());
        self
    }

    pub fn icon_font(mut self, icon_font: Handle<Font>) -> Self {
        self.icon_font = icon_font;
        self
    }

    pub fn font(mut self, font: Handle<Font>) -> Self {
        self.font = font;
        self
    }
}

impl Scene for Fab {
    fn resolve(
        self,
        context: &mut ResolveContext,
        scene: &mut ResolvedScene,
    ) -> Result<(), ResolveSceneError> {
        let mut btn = if let Some(ref label) = self.label {
            Button::new(label.clone())
                .icon(self.icon)
                .icon_font(self.icon_font)
                .font(self.font)
        } else {
            Button::fab_button(self.icon).icon_font(self.icon_font)
        };
        btn.style = Some(ButtonStyle::fab());
        btn.resolve(context, scene)
    }
}

/// 1.10 Search Bar: input field with search icon and clear button.
#[derive(Clone, Debug, Reflect)]
pub struct SearchBar {
    pub placeholder: String,
    pub font: Handle<Font>,
    pub icon_font: Handle<Font>,
}

impl Default for SearchBar {
    fn default() -> Self {
        Self {
            placeholder: "Search...".to_string(),
            font: Handle::default(),
            icon_font: Handle::default(),
        }
    }
}

impl SearchBar {
    pub fn new(placeholder: impl Into<String>) -> Self {
        Self {
            placeholder: placeholder.into(),
            ..default()
        }
    }

    pub fn font(mut self, font: Handle<Font>) -> Self {
        self.font = font;
        self
    }

    pub fn icon_font(mut self, icon_font: Handle<Font>) -> Self {
        self.icon_font = icon_font;
        self
    }
}

impl Scene for SearchBar {
    fn resolve(
        self,
        context: &mut ResolveContext,
        scene: &mut ResolvedScene,
    ) -> Result<(), ResolveSceneError> {
        let input = TextInput::search(self.placeholder)
            .font(self.font)
            .icon_font(self.icon_font)
            .trailing_icon(Icon::X);
        input.resolve(context, scene)
    }
}

/// 1.11 Text Area: multi-line text input with character counter.
#[derive(Clone, Debug, Reflect)]
pub struct TextArea {
    pub placeholder: String,
    pub max_chars: Option<usize>,
    pub font: Handle<Font>,
}

impl Default for TextArea {
    fn default() -> Self {
        Self {
            placeholder: "Write description...".to_string(),
            max_chars: Some(500),
            font: Handle::default(),
        }
    }
}

impl TextArea {
    pub fn new(placeholder: impl Into<String>) -> Self {
        Self {
            placeholder: placeholder.into(),
            ..default()
        }
    }

    pub fn max_chars(mut self, max: usize) -> Self {
        self.max_chars = Some(max);
        self
    }

    pub fn font(mut self, font: Handle<Font>) -> Self {
        self.font = font;
        self
    }
}

impl Scene for TextArea {
    fn resolve(
        self,
        context: &mut ResolveContext,
        scene: &mut ResolvedScene,
    ) -> Result<(), ResolveSceneError> {
        let input = TextInput::multiline(self.placeholder).font(self.font);
        input.resolve(context, scene)
    }
}

/// 1.14 Multi-Select Dropdown: select multiple items displayed as tags.
#[derive(Clone, Debug, Reflect)]
pub struct MultiSelect {
    pub placeholder: String,
    pub selected_tags: Vec<String>,
    pub font: Handle<Font>,
    pub icon_font: Handle<Font>,
}

impl Default for MultiSelect {
    fn default() -> Self {
        Self {
            placeholder: "Select tags...".to_string(),
            selected_tags: Vec::new(),
            font: Handle::default(),
            icon_font: Handle::default(),
        }
    }
}

impl MultiSelect {
    pub fn new(placeholder: impl Into<String>) -> Self {
        Self {
            placeholder: placeholder.into(),
            ..default()
        }
    }

    pub fn tags(mut self, tags: Vec<String>) -> Self {
        self.selected_tags = tags;
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
}

impl Scene for MultiSelect {
    fn resolve(
        self,
        context: &mut ResolveContext,
        scene: &mut ResolvedScene,
    ) -> Result<(), ResolveSceneError> {
        let font = self.font;
        let icon_font = self.icon_font;
        let ifont_chip = icon_font.clone();
        let ifont_text = icon_font;
        let placeholder = self.placeholder;
        let has_tags = !self.selected_tags.is_empty();

        let s = bsn! {
            UNode {
                width: UVal::Percent(1.0),
                min_height: 42.0,
                background_color: Color::srgb(0.10, 0.12, 0.16),
                border_radius: UCornerRadius::all(8.0),
                padding: USides::axes(12.0, 6.0),
            }
            UBorder {
                color: Color::srgb(0.20, 0.24, 0.32),
                width: 1.0,
                radius: UCornerRadius::all(8.0),
            }
            ULayout {
                display: UDisplay::Flex,
                flex_direction: UFlexDirection::Row,
                align_items: UAlignItems::Center,
                justify_content: UJustifyContent::SpaceBetween,
            }
            Children [
                UNode::default()
                ULayout {
                    display: UDisplay::Flex,
                    flex_direction: UFlexDirection::Row,
                    align_items: UAlignItems::Center,
                    gap: 6.0,
                }
                Children [
                    UNode::default()
                    ULayout {
                        display: { if has_tags { UDisplay::None } else { UDisplay::Flex } },
                    }
                    Children [
                        UText {
                            text: placeholder,
                            font_size: 14.0,
                            font: { font.clone() },
                            color: Color::srgb(0.50, 0.55, 0.65),
                        }
                    ]
                    --
                    @{Chip::new("Rust").removable(true).font(font).icon_font(ifont_chip)}
                ]
                --
                UText {
                    text: { Icon::CHEVRON_DOWN.to_string() },
                    font_size: 14.0,
                    font: ifont_text,
                    color: Color::srgb(0.60, 0.65, 0.75),
                }
            ]
        };
        s.resolve(context, scene)
    }
}

/// 1.15 Combobox / Autocomplete: input with instant filtered suggestions.
#[derive(Clone, Debug, Reflect)]
pub struct Combobox {
    pub placeholder: String,
    pub font: Handle<Font>,
    pub icon_font: Handle<Font>,
}

impl Default for Combobox {
    fn default() -> Self {
        Self {
            placeholder: "Search or choose...".to_string(),
            font: Handle::default(),
            icon_font: Handle::default(),
        }
    }
}

impl Combobox {
    pub fn new(placeholder: impl Into<String>) -> Self {
        Self {
            placeholder: placeholder.into(),
            ..default()
        }
    }
}

impl Scene for Combobox {
    fn resolve(
        self,
        context: &mut ResolveContext,
        scene: &mut ResolvedScene,
    ) -> Result<(), ResolveSceneError> {
        let input = TextInput::new(self.placeholder)
            .leading_icon(Icon::SEARCH)
            .trailing_icon(Icon::CHEVRON_DOWN)
            .font(self.font)
            .icon_font(self.icon_font);
        input.resolve(context, scene)
    }
}

/// 1.17 Range Slider: dual-thumb slider for min and max bounds.
#[derive(Clone, Debug, Reflect)]
pub struct RangeSlider {
    pub min_value: f32,
    pub max_value: f32,
    pub min: f32,
    pub max: f32,
    pub width: f32,
}

impl Default for RangeSlider {
    fn default() -> Self {
        Self {
            min_value: 20.0,
            max_value: 80.0,
            min: 0.0,
            max: 100.0,
            width: 240.0,
        }
    }
}

impl RangeSlider {
    pub fn new(min_value: f32, max_value: f32, min: f32, max: f32) -> Self {
        Self {
            min_value,
            max_value,
            min,
            max,
            ..default()
        }
    }

    pub fn width(mut self, width: f32) -> Self {
        self.width = width;
        self
    }
}

impl Scene for RangeSlider {
    fn resolve(
        self,
        context: &mut ResolveContext,
        scene: &mut ResolvedScene,
    ) -> Result<(), ResolveSceneError> {
        let width = self.width;
        let s = bsn! {
            UNode {
                width: UVal::Px(width),
                height: UVal::Px(24.0),
            }
            ULayout {
                display: UDisplay::Flex,
                align_items: UAlignItems::Center,
                justify_content: UJustifyContent::SpaceBetween,
            }
            Children [
                @{Slider::new(self.min_value, self.min, self.max, width / 2.0 - 4.0)}
                --
                @{Slider::new(self.max_value, self.min, self.max, width / 2.0 - 4.0)}
            ]
        };
        s.resolve(context, scene)
    }
}

/// 1.18 Date Picker: calendar popup button for selecting date.
#[derive(Clone, Debug, Reflect)]
pub struct DatePicker {
    pub date: String,
    pub font: Handle<Font>,
    pub icon_font: Handle<Font>,
}

impl Default for DatePicker {
    fn default() -> Self {
        Self {
            date: "2026-10-15".to_string(),
            font: Handle::default(),
            icon_font: Handle::default(),
        }
    }
}

impl DatePicker {
    pub fn new(date: impl Into<String>) -> Self {
        Self {
            date: date.into(),
            ..default()
        }
    }
}

impl Scene for DatePicker {
    fn resolve(
        self,
        context: &mut ResolveContext,
        scene: &mut ResolvedScene,
    ) -> Result<(), ResolveSceneError> {
        let btn = Button::new(self.date)
            .icon(Icon::CALENDAR)
            .variant(ButtonVariant::Outline)
            .font(self.font)
            .icon_font(self.icon_font);
        btn.resolve(context, scene)
    }
}

/// 1.19 Date Range Picker: pick start and end dates.
#[derive(Clone, Debug, Reflect)]
pub struct DateRangePicker {
    pub start: String,
    pub end: String,
    pub font: Handle<Font>,
    pub icon_font: Handle<Font>,
}

impl Default for DateRangePicker {
    fn default() -> Self {
        Self {
            start: "Oct 01, 2026".to_string(),
            end: "Oct 15, 2026".to_string(),
            font: Handle::default(),
            icon_font: Handle::default(),
        }
    }
}

impl DateRangePicker {
    pub fn new(start: impl Into<String>, end: impl Into<String>) -> Self {
        Self {
            start: start.into(),
            end: end.into(),
            ..default()
        }
    }
}

impl Scene for DateRangePicker {
    fn resolve(
        self,
        context: &mut ResolveContext,
        scene: &mut ResolvedScene,
    ) -> Result<(), ResolveSceneError> {
        let label = format!("{}  →  {}", self.start, self.end);
        let btn = Button::new(label)
            .icon(Icon::CALENDAR)
            .variant(ButtonVariant::Outline)
            .font(self.font)
            .icon_font(self.icon_font);
        btn.resolve(context, scene)
    }
}

/// 1.20 Time Picker: select hours, minutes, AM/PM.
#[derive(Clone, Debug, Reflect)]
pub struct TimePicker {
    pub time: String,
    pub font: Handle<Font>,
    pub icon_font: Handle<Font>,
}

impl Default for TimePicker {
    fn default() -> Self {
        Self {
            time: "09:30 AM".to_string(),
            font: Handle::default(),
            icon_font: Handle::default(),
        }
    }
}

impl TimePicker {
    pub fn new(time: impl Into<String>) -> Self {
        Self {
            time: time.into(),
            ..default()
        }
    }
}

impl Scene for TimePicker {
    fn resolve(
        self,
        context: &mut ResolveContext,
        scene: &mut ResolvedScene,
    ) -> Result<(), ResolveSceneError> {
        let btn = Button::new(self.time)
            .icon(Icon::CLOCK_4)
            .variant(ButtonVariant::Outline)
            .font(self.font)
            .icon_font(self.icon_font);
        btn.resolve(context, scene)
    }
}

/// Marker component for an interactive ColorPicker container.
#[derive(Component, Clone, Debug, Default, Reflect)]
pub struct LumaColorPicker {
    pub current_color: Color,
    pub current_hex: String,
}

/// Marker component for the active color preview box inside a ColorPicker.
#[derive(Component, Clone, Debug, Default, Reflect)]
pub struct LumaColorPickerPreview;

/// Marker component for the hex text label inside a ColorPicker.
#[derive(Component, Clone, Debug, Default, Reflect)]
pub struct LumaColorPickerHex;

/// Component attached to palette swatch items in ColorPicker.
#[derive(Component, Clone, Debug, Default, Reflect)]
pub struct LumaColorPickerSwatch {
    pub color: Color,
    pub hex: String,
}

/// Event triggered when a color is picked in a ColorPicker.
#[derive(Event, Clone, Debug)]
pub struct ColorPickerChanged {
    pub entity: Entity,
    pub color: Color,
    pub hex: String,
}

/// 1.21 Color Picker: color preview tile + hex code + quick palette swatches.
#[derive(Clone, Debug, Reflect)]
pub struct ColorPicker {
    pub color: Color,
    pub hex: String,
    pub palette: Vec<(Color, String)>,
    pub font: Handle<Font>,
}

impl Default for ColorPicker {
    fn default() -> Self {
        Self {
            color: Color::srgb(0.20, 0.45, 0.90),
            hex: "#3373E6".to_string(),
            palette: vec![
                (Color::srgb(0.90, 0.25, 0.25), "#E64040".to_string()),
                (Color::srgb(0.95, 0.60, 0.15), "#F29926".to_string()),
                (Color::srgb(0.20, 0.75, 0.40), "#33BF66".to_string()),
                (Color::srgb(0.20, 0.45, 0.90), "#3373E6".to_string()),
                (Color::srgb(0.65, 0.30, 0.85), "#A64DD9".to_string()),
                (Color::srgb(0.95, 0.35, 0.65), "#F259A6".to_string()),
            ],
            font: Handle::default(),
        }
    }
}

impl ColorPicker {
    pub fn new(color: Color, hex: impl Into<String>) -> Self {
        Self {
            color,
            hex: hex.into(),
            ..default()
        }
    }

    pub fn palette(mut self, palette: Vec<(Color, String)>) -> Self {
        self.palette = palette;
        self
    }
}

impl Scene for ColorPicker {
    fn resolve(
        self,
        context: &mut ResolveContext,
        scene: &mut ResolvedScene,
    ) -> Result<(), ResolveSceneError> {
        let color = self.color;
        let hex = self.hex;
        let font = self.font;
        let palette = self.palette;

        let swatch_nodes: Vec<_> = palette
            .into_iter()
            .map(|(sw_color, sw_hex)| {
                bsn! {
                    UNode {
                        width: UVal::Px(16.0),
                        height: UVal::Px(16.0),
                        background_color: sw_color,
                        border_radius: UCornerRadius::all(8.0),
                    }
                    LumaColorPickerSwatch {
                        color: sw_color,
                        hex: sw_hex,
                    }
                    UInteraction::default()
                    UInteractionColors {
                        normal: sw_color,
                        hovered: Color::WHITE,
                        pressed: sw_color,
                    }
                    UBorder {
                        color: Color::srgba(1.0, 1.0, 1.0, 0.25),
                        width: 1.0,
                        radius: UCornerRadius::all(8.0),
                    }
                }
            })
            .collect();

        let current_hex = hex.clone();
        let swatches_scene = crate::navigation::SceneList::new(swatch_nodes);

        let s = bsn! {
            LumaColorPicker {
                current_color: color,
                current_hex,
            }
            UNode {
                background_color: Color::srgb(0.10, 0.12, 0.16),
                border_radius: UCornerRadius::all(8.0),
                padding: USides::axes(10.0, 6.0),
            }
            UBorder {
                color: Color::srgb(0.20, 0.24, 0.32),
                width: 1.0,
                radius: UCornerRadius::all(8.0),
            }
            ULayout {
                display: UDisplay::Flex,
                flex_direction: UFlexDirection::Row,
                align_items: UAlignItems::Center,
                gap: 8.0,
            }
            Children [
                UNode {
                    width: UVal::Px(20.0),
                    height: UVal::Px(20.0),
                    background_color: color,
                    border_radius: UCornerRadius::all(4.0),
                }
                LumaColorPickerPreview
                UBorder {
                    color: Color::srgba(1.0, 1.0, 1.0, 0.2),
                    width: 1.0,
                    radius: UCornerRadius::all(4.0),
                }
                --
                UNode::default()
                Children [
                    LumaColorPickerHex
                    UText {
                        text: hex,
                        font_size: 13.0,
                        font,
                        color: Color::WHITE,
                    }
                ]
                --
                // Divider
                UNode {
                    width: UVal::Px(1.0),
                    height: UVal::Px(16.0),
                    background_color: Color::srgb(0.22, 0.26, 0.36),
                }
                --
                // Palette row
                UNode::default()
                ULayout {
                    display: UDisplay::Flex,
                    flex_direction: UFlexDirection::Row,
                    align_items: UAlignItems::Center,
                    gap: 6.0,
                }
                Children [
                    @{swatches_scene}
                ]
            ]
        };
        s.resolve(context, scene)
    }
}

fn on_color_picker_swatch_click(
    trigger: On<PointerClick>,
    mut commands: Commands,
    swatch_query: Query<&LumaColorPickerSwatch>,
    mut picker_query: Query<(Entity, &mut LumaColorPicker, &Children)>,
    mut preview_query: Query<&mut UNode, With<LumaColorPickerPreview>>,
    mut hex_text_query: Query<&mut UText, With<LumaColorPickerHex>>,
    parents_query: Query<&ChildOf>,
    children_query: Query<&Children>,
) {
    if trigger.event().button != PointerButton::Primary {
        return;
    }

    let clicked = trigger.entity;
    let swatch_target = if swatch_query.contains(clicked) {
        Some(clicked)
    } else {
        parents_query
            .iter_ancestors(clicked)
            .find(|&e| swatch_query.contains(e))
    };

    let Some(swatch_entity) = swatch_target else {
        return;
    };
    let Ok(swatch) = swatch_query.get(swatch_entity) else {
        return;
    };

    let picked_color = swatch.color;
    let picked_hex = swatch.hex.clone();

    // Find parent LumaColorPicker
    let picker_target = parents_query
        .iter_ancestors(swatch_entity)
        .find(|&e| picker_query.contains(e));
    let Some(picker_entity) = picker_target else {
        return;
    };

    let Ok((entity, mut picker, children)) = picker_query.get_mut(picker_entity) else {
        return;
    };

    picker.current_color = picked_color;
    picker.current_hex = picked_hex.clone();

    // Update preview tile and hex text
    for child in children.iter() {
        if let Ok(mut node) = preview_query.get_mut(child) {
            node.background_color = picked_color;
        }

        if let Ok(sub_children) = children_query.get(child) {
            for sub in sub_children.iter() {
                if let Ok(mut node) = preview_query.get_mut(sub) {
                    node.background_color = picked_color;
                }
                if let Ok(mut text) = hex_text_query.get_mut(sub) {
                    text.text = picked_hex.clone();
                }
            }
        }
    }

    commands.trigger(ColorPickerChanged {
        entity,
        color: picked_color,
        hex: picked_hex,
    });
}

/// Plugin registering interactive control widget systems and observers.
pub struct LumaControlsPlugin;

impl Plugin for LumaControlsPlugin {
    fn build(&self, app: &mut App) {
        app.register_type::<LumaColorPicker>()
            .register_type::<LumaColorPickerPreview>()
            .register_type::<LumaColorPickerHex>()
            .register_type::<LumaColorPickerSwatch>()
            .add_observer(on_color_picker_swatch_click);
    }
}

/// 1.22 File Uploader / Dropzone: drag & drop file area with progress.
#[derive(Clone, Debug, Reflect)]
pub struct FileUploader {
    pub message: String,
    pub progress: Option<f32>,
    pub font: Handle<Font>,
    pub icon_font: Handle<Font>,
}

impl Default for FileUploader {
    fn default() -> Self {
        Self {
            message: "Drag & drop files here, or browse".to_string(),
            progress: None,
            font: Handle::default(),
            icon_font: Handle::default(),
        }
    }
}

impl FileUploader {
    pub fn new(message: impl Into<String>) -> Self {
        Self {
            message: message.into(),
            ..default()
        }
    }

    pub fn progress(mut self, progress: f32) -> Self {
        self.progress = Some(progress);
        self
    }
}

impl Scene for FileUploader {
    fn resolve(
        self,
        context: &mut ResolveContext,
        scene: &mut ResolvedScene,
    ) -> Result<(), ResolveSceneError> {
        let message = self.message;
        let font = self.font;
        let icon_font = self.icon_font;

        let s = bsn! {
            UNode {
                width: UVal::Percent(1.0),
                min_height: 100.0,
                background_color: Color::srgb(0.08, 0.10, 0.14),
                border_radius: UCornerRadius::all(10.0),
                padding: USides::all(16.0),
            }
            UBorder {
                color: Color::srgb(0.25, 0.35, 0.55),
                width: 1.5,
                radius: UCornerRadius::all(10.0),
            }
            ULayout {
                display: UDisplay::Flex,
                flex_direction: UFlexDirection::Column,
                align_items: UAlignItems::Center,
                justify_content: UJustifyContent::Center,
                gap: 8.0,
            }
            Children [
                UText {
                    text: { Icon::FOLDER.to_string() },
                    font_size: 24.0,
                    font: icon_font,
                    color: Color::srgb(0.40, 0.70, 1.0),
                }
                --
                UText {
                    text: message,
                    font_size: 13.0,
                    font,
                    color: Color::srgb(0.70, 0.75, 0.85),
                }
            ]
        };
        s.resolve(context, scene)
    }
}

/// 1.23 OTP / PIN Input: segmented digits boxes.
#[derive(Clone, Debug, Reflect)]
pub struct OtpInput {
    pub code: String,
    pub length: usize,
    pub font: Handle<Font>,
}

impl Default for OtpInput {
    fn default() -> Self {
        Self {
            code: "4829".to_string(),
            length: 4,
            font: Handle::default(),
        }
    }
}

impl OtpInput {
    pub fn new(length: usize) -> Self {
        Self {
            length,
            ..default()
        }
    }

    pub fn code(mut self, code: impl Into<String>) -> Self {
        self.code = code.into();
        self
    }
}

impl Scene for OtpInput {
    fn resolve(
        self,
        context: &mut ResolveContext,
        scene: &mut ResolvedScene,
    ) -> Result<(), ResolveSceneError> {
        let s = bsn! {
            UNode::default()
            ULayout {
                display: UDisplay::Flex,
                flex_direction: UFlexDirection::Row,
                gap: 8.0,
            }
            Children [
                @{TextInput::otp().value("4").font(self.font.clone())}
                --
                @{TextInput::otp().value("8").font(self.font.clone())}
                --
                @{TextInput::otp().value("2").font(self.font.clone())}
                --
                @{TextInput::otp().value("9").font(self.font)}
            ]
        };
        s.resolve(context, scene)
    }
}

/// 1.24 Phone Number Input: country code prefix with phone field.
#[derive(Clone, Debug, Reflect)]
pub struct PhoneInput {
    pub country_code: String,
    pub number: String,
    pub font: Handle<Font>,
}

impl Default for PhoneInput {
    fn default() -> Self {
        Self {
            country_code: "+1".to_string(),
            number: String::new(),
            font: Handle::default(),
        }
    }
}

impl PhoneInput {
    pub fn new(country_code: impl Into<String>) -> Self {
        Self {
            country_code: country_code.into(),
            ..default()
        }
    }
}

impl Scene for PhoneInput {
    fn resolve(
        self,
        context: &mut ResolveContext,
        scene: &mut ResolvedScene,
    ) -> Result<(), ResolveSceneError> {
        let input = TextInput::new("Phone number...")
            .leading_icon(self.country_code)
            .value(self.number)
            .font(self.font);
        input.resolve(context, scene)
    }
}

/// 1.26 Emoji / Reaction Picker: reaction pills row.
#[derive(Clone, Debug, Reflect)]
pub struct ReactionPicker {
    pub reactions: Vec<String>,
    pub icon_font: Handle<Font>,
}

impl Default for ReactionPicker {
    fn default() -> Self {
        Self {
            reactions: vec![
                Icon::THUMBS_UP.into(),
                Icon::HEART.into(),
                Icon::FLAME.into(),
                Icon::STAR.into(),
            ],
            icon_font: Handle::default(),
        }
    }
}

impl ReactionPicker {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn icon_font(mut self, icon_font: Handle<Font>) -> Self {
        self.icon_font = icon_font;
        self
    }
}

impl Scene for ReactionPicker {
    fn resolve(
        self,
        context: &mut ResolveContext,
        scene: &mut ResolvedScene,
    ) -> Result<(), ResolveSceneError> {
        let icon_font = self.icon_font;
        let s = bsn! {
            UNode {
                background_color: Color::srgb(0.12, 0.14, 0.20),
                border_radius: UCornerRadius::all(20.0),
                padding: USides::axes(8.0, 4.0),
            }
            UBorder {
                color: Color::srgb(0.24, 0.28, 0.38),
                width: 1.0,
                radius: UCornerRadius::all(20.0),
            }
            ULayout {
                display: UDisplay::Flex,
                flex_direction: UFlexDirection::Row,
                align_items: UAlignItems::Center,
                gap: 4.0,
            }
            Children [
                @{Button::new("").icon(Icon::THUMBS_UP).ghost().icon_font(icon_font.clone()).size(ButtonSize::Small)}
                --
                @{Button::new("").icon(Icon::HEART).ghost().icon_font(icon_font.clone()).size(ButtonSize::Small)}
                --
                @{Button::new("").icon(Icon::FLAME).ghost().icon_font(icon_font.clone()).size(ButtonSize::Small)}
                --
                @{Button::new("").icon(Icon::STAR).ghost().icon_font(icon_font).size(ButtonSize::Small)}
            ]
        };
        s.resolve(context, scene)
    }
}

/// 1.27 Rich Text Toolbar: bold, italic, list, link actions.
#[derive(Clone, Debug, Reflect)]
pub struct RichTextToolbar {
    pub icon_font: Handle<Font>,
}

impl Default for RichTextToolbar {
    fn default() -> Self {
        Self {
            icon_font: Handle::default(),
        }
    }
}

impl RichTextToolbar {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn icon_font(mut self, icon_font: Handle<Font>) -> Self {
        self.icon_font = icon_font;
        self
    }
}

impl Scene for RichTextToolbar {
    fn resolve(
        self,
        context: &mut ResolveContext,
        scene: &mut ResolvedScene,
    ) -> Result<(), ResolveSceneError> {
        let s = bsn! {
            UNode {
                background_color: Color::srgb(0.10, 0.12, 0.16),
                border_radius: UCornerRadius::all(8.0),
                padding: USides::all(4.0),
            }
            UBorder {
                color: Color::srgb(0.20, 0.24, 0.32),
                width: 1.0,
                radius: UCornerRadius::all(8.0),
            }
            ULayout {
                display: UDisplay::Flex,
                flex_direction: UFlexDirection::Row,
                gap: 4.0,
            }
            Children [
                @{Button::icon_button("B").ghost()}
                --
                @{Button::icon_button("I").ghost()}
                --
                @{Button::icon_button("U").ghost()}
                --
                @{Button::icon_button(Icon::LINK).ghost().icon_font(self.icon_font)}
            ]
        };
        s.resolve(context, scene)
    }
}

/// 1.28 Tag Input: turns entered terms into chips.
#[derive(Clone, Debug, Reflect)]
pub struct TagInput {
    pub placeholder: String,
    pub font: Handle<Font>,
    pub icon_font: Handle<Font>,
}

impl Default for TagInput {
    fn default() -> Self {
        Self {
            placeholder: "Add tag...".to_string(),
            font: Handle::default(),
            icon_font: Handle::default(),
        }
    }
}

impl TagInput {
    pub fn new(placeholder: impl Into<String>) -> Self {
        Self {
            placeholder: placeholder.into(),
            ..default()
        }
    }
}

impl Scene for TagInput {
    fn resolve(
        self,
        context: &mut ResolveContext,
        scene: &mut ResolvedScene,
    ) -> Result<(), ResolveSceneError> {
        let ms = MultiSelect::new(self.placeholder)
            .font(self.font)
            .icon_font(self.icon_font);
        ms.resolve(context, scene)
    }
}

/// 1.29 Credit Card Input: card number, expiry, CVV row.
#[derive(Clone, Debug, Reflect)]
pub struct CreditCardInput {
    pub font: Handle<Font>,
    pub icon_font: Handle<Font>,
}

impl Default for CreditCardInput {
    fn default() -> Self {
        Self {
            font: Handle::default(),
            icon_font: Handle::default(),
        }
    }
}

impl CreditCardInput {
    pub fn new() -> Self {
        Self::default()
    }
}

impl Scene for CreditCardInput {
    fn resolve(
        self,
        context: &mut ResolveContext,
        scene: &mut ResolvedScene,
    ) -> Result<(), ResolveSceneError> {
        let font = self.font;
        let s = bsn! {
            UNode::default()
            ULayout {
                display: UDisplay::Flex,
                flex_direction: UFlexDirection::Row,
                gap: 10.0,
            }
            Children [
                UNode { width: UVal::Px(240.0) }
                Children [
                    @{TextInput::new("Card number...").leading_icon(Icon::CREDIT_CARD).font(font.clone())}
                ]
                --
                UNode { width: UVal::Px(80.0) }
                Children [
                    @{TextInput::new("MM/YY").font(font.clone())}
                ]
                --
                UNode { width: UVal::Px(70.0) }
                Children [
                    @{TextInput::new("CVV").font(font)}
                ]
            ]
        };
        s.resolve(context, scene)
    }
}

/// 1.30 Voice Input Button: mic button with recording wave style.
#[derive(Clone, Debug, Reflect)]
pub struct VoiceInputButton {
    pub is_recording: bool,
    pub icon_font: Handle<Font>,
}

impl Default for VoiceInputButton {
    fn default() -> Self {
        Self {
            is_recording: false,
            icon_font: Handle::default(),
        }
    }
}

impl VoiceInputButton {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn recording(mut self, is_recording: bool) -> Self {
        self.is_recording = is_recording;
        self
    }
}

impl Scene for VoiceInputButton {
    fn resolve(
        self,
        context: &mut ResolveContext,
        scene: &mut ResolvedScene,
    ) -> Result<(), ResolveSceneError> {
        let variant = if self.is_recording {
            ButtonVariant::Danger
        } else {
            ButtonVariant::Secondary
        };
        let btn = Button::fab_button(Icon::MIC)
            .variant(variant)
            .icon_font(self.icon_font);
        btn.resolve(context, scene)
    }
}

/// 1.9 Password Input: masked input with toggle visibility eye icon and strength meter.
#[derive(Clone, Debug, Reflect)]
pub struct PasswordInput {
    pub value: String,
    pub placeholder: String,
    pub show_password: bool,
    pub show_strength: bool,
    pub strength_score: f32, // 0.0 to 1.0
    pub font: Handle<Font>,
    pub icon_font: Handle<Font>,
    pub disabled: bool,
}

impl Default for PasswordInput {
    fn default() -> Self {
        Self {
            value: String::new(),
            placeholder: "Enter password...".into(),
            show_password: false,
            show_strength: false,
            strength_score: 0.65,
            font: Handle::default(),
            icon_font: Handle::default(),
            disabled: false,
        }
    }
}

impl PasswordInput {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn value(mut self, value: impl Into<String>) -> Self {
        self.value = value.into();
        self
    }

    pub fn placeholder(mut self, placeholder: impl Into<String>) -> Self {
        self.placeholder = placeholder.into();
        self
    }

    pub fn show_password(mut self, show: bool) -> Self {
        self.show_password = show;
        self
    }

    pub fn strength(mut self, show: bool, score: f32) -> Self {
        self.show_strength = show;
        self.strength_score = score.clamp(0.0, 1.0);
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

    pub fn disabled(mut self, disabled: bool) -> Self {
        self.disabled = disabled;
        self
    }
}

impl Scene for PasswordInput {
    fn resolve(
        self,
        context: &mut ResolveContext,
        scene: &mut ResolvedScene,
    ) -> Result<(), ResolveSceneError> {
        let eye_icon = if self.show_password { Icon::EYE_OFF } else { Icon::EYE };
        let strength_color = if self.strength_score < 0.4 {
            Color::srgb(0.94, 0.27, 0.27)
        } else if self.strength_score < 0.75 {
            Color::srgb(0.95, 0.65, 0.15)
        } else {
            Color::srgb(0.18, 0.80, 0.44)
        };
        let strength_width = UVal::Percent(self.strength_score);

        let state = if self.disabled { InputState::Disabled } else { InputState::Normal };
        let input_elem = TextInput::new(self.placeholder)
            .value(self.value)
            .trailing_icon(eye_icon)
            .font(self.font)
            .icon_font(self.icon_font)
            .state(state);

        if self.show_strength {
            let s = bsn! {
                UNode {
                    width: UVal::Percent(1.0),
                }
                ULayout {
                    display: UDisplay::Flex,
                    flex_direction: UFlexDirection::Column,
                    gap: 4.0,
                }
                Children [
                    @{input_elem}
                    --
                    UNode {
                        width: UVal::Percent(1.0),
                        height: UVal::Px(4.0),
                        background_color: Color::srgba(0.2, 0.2, 0.25, 0.5),
                        border_radius: UCornerRadius::all(2.0),
                    }
                    ULayout {
                        display: UDisplay::Flex,
                    }
                    Children [
                        UNode {
                            width: strength_width,
                            height: UVal::Px(4.0),
                            background_color: strength_color,
                            border_radius: UCornerRadius::all(2.0),
                        }
                    ]
                ]
            };
            s.resolve(context, scene)
        } else {
            input_elem.resolve(context, scene)
        }
    }
}

