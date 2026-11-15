//! Specialized input and control widgets complementing the core primitives.

use bevy::prelude::*;
use bevy::scene::{ResolveContext, ResolvedScene, ResolveSceneError};
use bevy_luma::prelude::*;

use crate::button::{
    apply_button_variant_visuals, Button, LumaButton, LumaButtonIcon, LumaButtonLabel,
};
use crate::chip::{Chip, ChipDismissed, LumaChip};
use crate::helpers::{
    compute_overlay_anchor, find_ancestor, find_ancestor_or_self, interaction_colors,
    is_primary_click, visit_descendants, ConditionalScene, DescendantVisit,
};
use crate::input::LumaInputText;
use crate::navigation::SceneList;
use crate::prelude::*;
use crate::slider::{LumaSlider, SliderChanged};
use crate::theme::WidgetColors;

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
        let (mut btn, fab_style) = if let Some(ref label) = self.label {
            (
                Button::new(label.clone())
                    .icon(self.icon)
                    .icon_font(self.icon_font)
                    .font(self.font),
                ButtonStyle {
                    padding: USides::axes(20.0, 12.0),
                    min_height: 48.0,
                    font_size: 15.0,
                    ..ButtonStyle::fab()
                },
            )
        } else {
            (
                Button::fab_button(self.icon).icon_font(self.icon_font),
                ButtonStyle::fab(),
            )
        };
        btn.style = Some(fab_style);
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
        let mut input = TextInput::multiline(self.placeholder).font(self.font);
        if let Some(max) = self.max_chars {
            input = input.max_length(max);
        }
        input.resolve(context, scene)
    }
}

/// Event triggered when selected tags in a [`MultiSelect`] change.
#[derive(Event, Clone, Debug, Reflect)]
pub struct MultiSelectChanged {
    pub entity: Entity,
    pub selected_tags: Vec<String>,
}

/// Runtime state component for an interactive [`MultiSelect`] dropdown.
#[derive(Component, Clone, Debug, Default, Reflect)]
pub struct LumaMultiSelect {
    pub selected_tags: Vec<String>,
    pub options: Vec<String>,
    pub is_open: bool,
    pub overlay_entity: Option<Entity>,
}

#[derive(Component, Clone, Debug, Default, Reflect)]
pub struct LumaMultiSelectTagsRow;

#[derive(Component, Clone, Debug, Default, Reflect)]
pub struct LumaMultiSelectPlaceholder;

#[derive(Component, Clone, Debug, Default, Reflect)]
pub struct LumaMultiSelectChevron;

/// 1.14 Multi-Select Dropdown: select multiple items displayed as tags.
#[derive(Clone, Debug, Reflect)]
pub struct MultiSelect {
    pub placeholder: String,
    pub selected_tags: Vec<String>,
    pub options: Vec<String>,
    pub font: Handle<Font>,
    pub icon_font: Handle<Font>,
}

impl Default for MultiSelect {
    fn default() -> Self {
        Self {
            placeholder: "Select tags...".to_string(),
            selected_tags: Vec::new(),
            options: Vec::new(),
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

    pub fn options(mut self, options: Vec<impl Into<String>>) -> Self {
        self.options = options.into_iter().map(Into::into).collect();
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
        let mut pool = self.options;
        for tag in &self.selected_tags {
            if !pool.contains(tag) {
                pool.push(tag.clone());
            }
        }
        let luma_multi = LumaMultiSelect {
            selected_tags: self.selected_tags.clone(),
            options: pool,
            is_open: false,
            overlay_entity: None,
        };
        let colors = WidgetColors::default();
        let trigger_colors = interaction_colors(
            Color::srgb(0.10, 0.12, 0.16),
            colors.surface_hover,
            colors.surface,
            false,
        );
        let focus_color = colors.focus_ring();

        let tag_chips: Vec<_> = self
            .selected_tags
            .into_iter()
            .map(|tag| {
                Chip::new(tag)
                    .removable(true)
                    .font(font.clone())
                    .icon_font(ifont_chip.clone())
            })
            .collect();
        let tags_scene = SceneList::new(tag_chips);

        let s = bsn! {
            luma_multi
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
            UInteraction::default()
            trigger_colors
            UFocusable::default()
            UFocusVisual::border(focus_color, 2.0)
            ULayout {
                display: UDisplay::Flex,
                flex_direction: UFlexDirection::Row,
                align_items: UAlignItems::Center,
                justify_content: UJustifyContent::SpaceBetween,
                gap: 8.0,
            }
            Children [
                UNode::default()
                LumaMultiSelectTagsRow
                USelf {
                    flex_grow: { Some(1.0) },
                }
                ULayout {
                    display: UDisplay::Flex,
                    flex_direction: UFlexDirection::Row,
                    flex_wrap: UFlexWrap::Wrap,
                    align_items: UAlignItems::Center,
                    gap: 6.0,
                }
                Children [
                    UNode::default()
                    LumaMultiSelectPlaceholder
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
                    {tags_scene}
                ]
                --
                LumaMultiSelectChevron
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

/// Event triggered when an option is chosen from a [`Combobox`].
#[derive(Event, Clone, Debug, Reflect)]
pub struct ComboboxSelected {
    pub entity: Entity,
    pub value: String,
}

/// Runtime state component for an interactive [`Combobox`].
#[derive(Component, Clone, Debug, Default, Reflect)]
pub struct LumaCombobox {
    pub options: Vec<String>,
    pub is_open: bool,
    pub overlay_entity: Option<Entity>,
}

/// 1.15 Combobox / Autocomplete: input with instant filtered suggestions.
#[derive(Clone, Debug, Reflect)]
pub struct Combobox {
    pub placeholder: String,
    pub options: Vec<String>,
    pub font: Handle<Font>,
    pub icon_font: Handle<Font>,
}

impl Default for Combobox {
    fn default() -> Self {
        Self {
            placeholder: "Search or choose...".to_string(),
            options: Vec::new(),
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

    pub fn options(mut self, options: Vec<impl Into<String>>) -> Self {
        self.options = options.into_iter().map(Into::into).collect();
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

impl Scene for Combobox {
    fn resolve(
        self,
        context: &mut ResolveContext,
        scene: &mut ResolvedScene,
    ) -> Result<(), ResolveSceneError> {
        let luma_combo = LumaCombobox {
            options: self.options,
            is_open: false,
            overlay_entity: None,
        };
        let input = TextInput::new(self.placeholder)
            .leading_icon(Icon::SEARCH)
            .trailing_icon(Icon::CHEVRON_DOWN)
            .font(self.font)
            .icon_font(self.icon_font);
        let s = bsn! {
            luma_combo
            UNode {
                width: UVal::Percent(1.0),
            }
            ULayout {
                display: UDisplay::Flex,
            }
            Children [
                @{input}
            ]
        };
        s.resolve(context, scene)
    }
}

/// Event triggered when either bound of a [`RangeSlider`] changes.
#[derive(Event, Clone, Copy, Debug, Reflect)]
pub struct RangeSliderChanged {
    pub entity: Entity,
    pub min_value: f32,
    pub max_value: f32,
}

/// Runtime state component for an interactive [`RangeSlider`].
#[derive(Component, Clone, Copy, Debug, Default, Reflect)]
pub struct LumaRangeSlider {
    pub min_value: f32,
    pub max_value: f32,
    pub min: f32,
    pub max: f32,
}

#[derive(Component, Clone, Copy, Debug, Default, Reflect)]
pub struct LumaRangeMinSlider;

#[derive(Component, Clone, Copy, Debug, Default, Reflect)]
pub struct LumaRangeMaxSlider;

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
        let lo = self.min.min(self.max);
        let hi = self.max.max(lo);
        let min_val = self.min_value.min(self.max_value).clamp(lo, hi);
        let max_val = self.max_value.max(min_val).clamp(lo, hi);
        let luma_range = LumaRangeSlider {
            min_value: min_val,
            max_value: max_val,
            min: lo,
            max: hi,
        };
        let half_w = (width / 2.0 - 4.0).max(24.0);
        let s = bsn! {
            luma_range
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
                UNode::default()
                LumaRangeMinSlider
                Children [
                    @{Slider::new(min_val, lo, hi, half_w)}
                ]
                --
                UNode::default()
                LumaRangeMaxSlider
                Children [
                    @{Slider::new(max_val, lo, hi, half_w)}
                ]
            ]
        };
        s.resolve(context, scene)
    }
}

/// Event triggered when a date is chosen in a [`DatePicker`].
#[derive(Event, Clone, Debug, Reflect)]
pub struct DateChanged {
    pub entity: Entity,
    pub date: String,
}

/// Runtime state component for an interactive [`DatePicker`].
#[derive(Component, Clone, Debug, Default, Reflect)]
pub struct LumaDatePicker {
    pub date: String,
    pub year: i32,
    pub month: u32,
    pub day: u32,
    pub is_open: bool,
    pub overlay_entity: Option<Entity>,
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

    pub fn font(mut self, font: Handle<Font>) -> Self {
        self.font = font;
        self
    }

    pub fn icon_font(mut self, icon_font: Handle<Font>) -> Self {
        self.icon_font = icon_font;
        self
    }
}

impl Scene for DatePicker {
    fn resolve(
        self,
        context: &mut ResolveContext,
        scene: &mut ResolvedScene,
    ) -> Result<(), ResolveSceneError> {
        let (year, month, day) = parse_ymd_date(&self.date);
        let luma_dp = LumaDatePicker {
            date: self.date.clone(),
            year,
            month,
            day,
            is_open: false,
            overlay_entity: None,
        };
        let btn = Button::new(self.date)
            .icon(Icon::CALENDAR)
            .variant(ButtonVariant::Outline)
            .font(self.font)
            .icon_font(self.icon_font);
        let s = bsn! {
            luma_dp
            UNode::default()
            ULayout { display: UDisplay::Flex }
            Children [
                @{btn}
            ]
        };
        s.resolve(context, scene)
    }
}

/// Event triggered when a range is chosen in a [`DateRangePicker`].
#[derive(Event, Clone, Debug, Reflect)]
pub struct DateRangeChanged {
    pub entity: Entity,
    pub start: String,
    pub end: String,
}

/// Runtime state component for an interactive [`DateRangePicker`].
#[derive(Component, Clone, Debug, Default, Reflect)]
pub struct LumaDateRangePicker {
    pub start: String,
    pub end: String,
    pub is_open: bool,
    pub overlay_entity: Option<Entity>,
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

    pub fn font(mut self, font: Handle<Font>) -> Self {
        self.font = font;
        self
    }

    pub fn icon_font(mut self, icon_font: Handle<Font>) -> Self {
        self.icon_font = icon_font;
        self
    }
}

impl Scene for DateRangePicker {
    fn resolve(
        self,
        context: &mut ResolveContext,
        scene: &mut ResolvedScene,
    ) -> Result<(), ResolveSceneError> {
        let label = format!("{}  →  {}", self.start, self.end);
        let luma_drp = LumaDateRangePicker {
            start: self.start,
            end: self.end,
            is_open: false,
            overlay_entity: None,
        };
        let btn = Button::new(label)
            .icon(Icon::CALENDAR)
            .variant(ButtonVariant::Outline)
            .font(self.font)
            .icon_font(self.icon_font);
        let s = bsn! {
            luma_drp
            UNode::default()
            ULayout { display: UDisplay::Flex }
            Children [
                @{btn}
            ]
        };
        s.resolve(context, scene)
    }
}

/// Event triggered when time is updated in a [`TimePicker`].
#[derive(Event, Clone, Debug, Reflect)]
pub struct TimeChanged {
    pub entity: Entity,
    pub time: String,
}

/// Runtime state component for an interactive [`TimePicker`].
#[derive(Component, Clone, Debug, Default, Reflect)]
pub struct LumaTimePicker {
    pub time: String,
    pub hour: u8,
    pub minute: u8,
    pub suffix: String,
    pub is_open: bool,
    pub overlay_entity: Option<Entity>,
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

    pub fn font(mut self, font: Handle<Font>) -> Self {
        self.font = font;
        self
    }

    pub fn icon_font(mut self, icon_font: Handle<Font>) -> Self {
        self.icon_font = icon_font;
        self
    }
}

impl Scene for TimePicker {
    fn resolve(
        self,
        context: &mut ResolveContext,
        scene: &mut ResolvedScene,
    ) -> Result<(), ResolveSceneError> {
        let (hour, minute, suffix) = parse_hm_time(&self.time);
        let luma_tp = LumaTimePicker {
            time: self.time.clone(),
            hour,
            minute,
            suffix,
            is_open: false,
            overlay_entity: None,
        };
        let btn = Button::new(self.time)
            .icon(Icon::CLOCK_4)
            .variant(ButtonVariant::Outline)
            .font(self.font)
            .icon_font(self.icon_font);
        let s = bsn! {
            luma_tp
            UNode::default()
            ULayout { display: UDisplay::Flex }
            Children [
                @{btn}
            ]
        };
        s.resolve(context, scene)
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
                    {swatches_scene}
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
    if !is_primary_click(&trigger) {
        return;
    }

    let Some(swatch_entity) = find_ancestor_or_self(trigger.entity, &swatch_query, &parents_query)
    else {
        return;
    };
    let Ok(swatch) = swatch_query.get(swatch_entity) else {
        return;
    };

    let picked_color = swatch.color;
    let picked_hex = swatch.hex.clone();

    // Find parent LumaColorPicker
    let Some(picker_entity) = find_ancestor_or_self(swatch_entity, &picker_query, &parents_query)
    else {
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

/// Event triggered when an upload area ([`FileUploader`]) is clicked or activated.
#[derive(Event, Clone, Copy, Debug, Reflect)]
pub struct FileUploadRequested {
    pub entity: Entity,
}

/// Runtime marker component for an interactive [`FileUploader`] dropzone.
#[derive(Component, Clone, Debug, Default, Reflect)]
pub struct LumaFileUploader {
    pub has_progress: bool,
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

    pub fn font(mut self, font: Handle<Font>) -> Self {
        self.font = font;
        self
    }

    pub fn icon_font(mut self, icon_font: Handle<Font>) -> Self {
        self.icon_font = icon_font;
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
        let has_progress = self.progress.is_some();
        let progress_scene = ConditionalScene(
            self.progress
                .map(|p| ProgressBar::new(p).width(UVal::Percent(0.8))),
        );
        let colors = WidgetColors::default();
        let zone_colors = interaction_colors(
            Color::srgb(0.08, 0.10, 0.14),
            Color::srgb(0.10, 0.13, 0.19),
            Color::srgb(0.07, 0.09, 0.13),
            false,
        );
        let focus_color = colors.focus_ring();
        let luma_uploader = LumaFileUploader { has_progress };

        let s = bsn! {
            luma_uploader
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
            UInteraction::default()
            zone_colors
            UFocusable::default()
            UFocusVisual::border(focus_color, 2.0)
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
                --
                @{progress_scene}
            ]
        };
        s.resolve(context, scene)
    }
}

/// Event triggered whenever digits in an [`OtpInput`] change.
#[derive(Event, Clone, Debug, Reflect)]
pub struct OtpChanged {
    pub entity: Entity,
    pub code: String,
}

/// Event triggered when all digits of an [`OtpInput`] have been filled.
#[derive(Event, Clone, Debug, Reflect)]
pub struct OtpCompleted {
    pub entity: Entity,
    pub code: String,
}

/// Runtime state component for an interactive [`OtpInput`].
#[derive(Component, Clone, Debug, Default, Reflect)]
pub struct LumaOtpInput {
    pub length: usize,
    pub code: String,
}

#[derive(Component, Clone, Copy, Debug, Default, Reflect)]
pub struct LumaOtpBox {
    pub index: usize,
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

    pub fn font(mut self, font: Handle<Font>) -> Self {
        self.font = font;
        self
    }
}

impl Scene for OtpInput {
    fn resolve(
        self,
        context: &mut ResolveContext,
        scene: &mut ResolvedScene,
    ) -> Result<(), ResolveSceneError> {
        let chars: Vec<String> = self.code.chars().map(|c| c.to_string()).collect();
        let luma_otp = LumaOtpInput {
            length: self.length,
            code: self.code,
        };
        let boxes: Vec<_> = (0..self.length)
            .map(|i| {
                let digit = chars.get(i).cloned().unwrap_or_default();
                let otp_box = TextInput::otp()
                    .value(digit)
                    .max_length(1)
                    .font(self.font.clone());
                bsn! {
                    LumaOtpBox { index: i }
                    UNode { width: UVal::Px(42.0) }
                    ULayout { display: UDisplay::Flex }
                    Children [
                        @{otp_box}
                    ]
                }
            })
            .collect();
        let boxes_scene = SceneList::new(boxes);

        let s = bsn! {
            luma_otp
            UNode::default()
            ULayout {
                display: UDisplay::Flex,
                flex_direction: UFlexDirection::Row,
                gap: 8.0,
            }
            Children [
                {boxes_scene}
            ]
        };
        s.resolve(context, scene)
    }
}

/// Event triggered when a [`PhoneInput`] number or dial code changes.
#[derive(Event, Clone, Debug, Reflect)]
pub struct PhoneNumberChanged {
    pub entity: Entity,
    pub country_code: String,
    pub number: String,
}

/// Runtime state component for an interactive [`PhoneInput`].
#[derive(Component, Clone, Debug, Default, Reflect)]
pub struct LumaPhoneInput {
    pub country_code: String,
    pub number: String,
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

    pub fn number(mut self, number: impl Into<String>) -> Self {
        self.number = number.into();
        self
    }

    pub fn font(mut self, font: Handle<Font>) -> Self {
        self.font = font;
        self
    }
}

impl Scene for PhoneInput {
    fn resolve(
        self,
        context: &mut ResolveContext,
        scene: &mut ResolvedScene,
    ) -> Result<(), ResolveSceneError> {
        let luma_phone = LumaPhoneInput {
            country_code: self.country_code.clone(),
            number: self.number.clone(),
        };
        let input = TextInput::new("Phone number...")
            .leading_icon(self.country_code)
            .value(self.number)
            .font(self.font);
        let s = bsn! {
            luma_phone
            UNode { width: UVal::Percent(1.0) }
            ULayout { display: UDisplay::Flex }
            Children [
                @{input}
            ]
        };
        s.resolve(context, scene)
    }
}

/// Event triggered when an emoji/reaction is selected in a [`ReactionPicker`].
#[derive(Event, Clone, Debug, Reflect)]
pub struct ReactionSelected {
    pub entity: Entity,
    pub index: usize,
    pub reaction: String,
    pub selected: bool,
}

/// Runtime state component for an interactive [`ReactionPicker`].
#[derive(Component, Clone, Debug, Default, Reflect)]
pub struct LumaReactionPicker {
    pub selected_index: Option<usize>,
    pub selected_reaction: Option<String>,
}

#[derive(Component, Clone, Debug, Default, Reflect)]
pub struct LumaReactionItem {
    pub index: usize,
    pub reaction: String,
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

    pub fn reactions(mut self, reactions: Vec<impl Into<String>>) -> Self {
        self.reactions = reactions.into_iter().map(Into::into).collect();
        self
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
        let reaction_buttons: Vec<_> = self
            .reactions
            .into_iter()
            .enumerate()
            .map(|(index, reaction)| {
                let item = LumaReactionItem {
                    index,
                    reaction: reaction.clone(),
                };
                let btn = Button::icon_button(reaction)
                    .ghost()
                    .icon_font(icon_font.clone())
                    .size(ButtonSize::Small);
                bsn! {
                    item
                    UNode::default()
                    ULayout { display: UDisplay::Flex }
                    Children [
                        @{btn}
                    ]
                }
            })
            .collect();
        let reactions_scene = SceneList::new(reaction_buttons);
        let s = bsn! {
            LumaReactionPicker::default()
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
                {reactions_scene}
            ]
        };
        s.resolve(context, scene)
    }
}

/// Formatting actions available on a [`RichTextToolbar`].
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, Default, Reflect)]
pub enum RichTextFormat {
    #[default]
    Bold,
    Italic,
    Underline,
    Link,
}

/// Event triggered when a formatting toggle or action is pressed on [`RichTextToolbar`].
#[derive(Event, Clone, Copy, Debug, Reflect)]
pub struct RichTextFormatToggled {
    pub entity: Entity,
    pub format: RichTextFormat,
    pub active: bool,
}

/// Runtime state component tracking active toggles on [`RichTextToolbar`].
#[derive(Component, Clone, Copy, Debug, Default, Reflect)]
pub struct LumaRichTextToolbar {
    pub bold: bool,
    pub italic: bool,
    pub underline: bool,
}

#[derive(Component, Clone, Copy, Debug, Default, Reflect)]
pub struct LumaRichTextAction {
    pub format: RichTextFormat,
}

/// 1.27 Rich Text Toolbar: bold, italic, underline, link actions.
#[derive(Clone, Debug, Reflect)]
pub struct RichTextToolbar {
    pub font: Handle<Font>,
    pub icon_font: Handle<Font>,
}

impl Default for RichTextToolbar {
    fn default() -> Self {
        Self {
            font: Handle::default(),
            icon_font: Handle::default(),
        }
    }
}

impl RichTextToolbar {
    pub fn new() -> Self {
        Self::default()
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

impl Scene for RichTextToolbar {
    fn resolve(
        self,
        context: &mut ResolveContext,
        scene: &mut ResolvedScene,
    ) -> Result<(), ResolveSceneError> {
        let font = self.font;
        let icon_font = self.icon_font;
        let s = bsn! {
            LumaRichTextToolbar::default()
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
                LumaRichTextAction { format: RichTextFormat::Bold }
                UNode::default()
                Children [
                    @{Button::new("B").ghost().size(ButtonSize::Small).font(font.clone())}
                ]
                --
                LumaRichTextAction { format: RichTextFormat::Italic }
                UNode::default()
                Children [
                    @{Button::new("I").ghost().size(ButtonSize::Small).font(font.clone())}
                ]
                --
                LumaRichTextAction { format: RichTextFormat::Underline }
                UNode::default()
                Children [
                    @{Button::new("U").ghost().size(ButtonSize::Small).font(font)}
                ]
                --
                LumaRichTextAction { format: RichTextFormat::Link }
                UNode::default()
                Children [
                    @{Button::icon_button(Icon::LINK).ghost().size(ButtonSize::Small).icon_font(icon_font)}
                ]
            ]
        };
        s.resolve(context, scene)
    }
}

/// Event triggered when tags in a [`TagInput`] are added or removed.
#[derive(Event, Clone, Debug, Reflect)]
pub struct TagsChanged {
    pub entity: Entity,
    pub tags: Vec<String>,
}

/// Runtime state component for an interactive [`TagInput`].
#[derive(Component, Clone, Debug, Default, Reflect)]
pub struct LumaTagInput {
    pub tags: Vec<String>,
}

#[derive(Component, Clone, Copy, Debug, Default, Reflect)]
pub struct LumaTagInputChips;

#[derive(Component, Clone, Copy, Debug, Default, Reflect)]
pub struct LumaTagInputIconFont;

#[derive(Component, Clone, Copy, Debug, Default, Reflect)]
pub struct LumaTagInputField;

/// 1.28 Tag Input: turns entered terms into chips.
#[derive(Clone, Debug, Reflect)]
pub struct TagInput {
    pub placeholder: String,
    pub tags: Vec<String>,
    pub font: Handle<Font>,
    pub icon_font: Handle<Font>,
}

impl Default for TagInput {
    fn default() -> Self {
        Self {
            placeholder: "Add tag...".to_string(),
            tags: Vec::new(),
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

    pub fn tags(mut self, tags: Vec<impl Into<String>>) -> Self {
        self.tags = tags.into_iter().map(Into::into).collect();
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

impl Scene for TagInput {
    fn resolve(
        self,
        context: &mut ResolveContext,
        scene: &mut ResolvedScene,
    ) -> Result<(), ResolveSceneError> {
        let font = self.font;
        let icon_font = self.icon_font;
        let luma_tag = LumaTagInput {
            tags: self.tags.clone(),
        };
        let initial_chips: Vec<_> = self
            .tags
            .into_iter()
            .map(|t| {
                Chip::new(t)
                    .removable(true)
                    .font(font.clone())
                    .icon_font(icon_font.clone())
            })
            .collect();
        let chips_scene = SceneList::new(initial_chips);
        let hidden_ifont = icon_font.clone();
        let field_style = InputStyle {
            bg_normal: Color::srgba(1.0, 1.0, 1.0, 0.03),
            bg_focus: Color::srgba(1.0, 1.0, 1.0, 0.06),
            border_normal: Color::srgba(1.0, 1.0, 1.0, 0.08),
            border_width: 1.0,
            min_height: 30.0,
            padding: USides::axes(10.0, 4.0),
            ..default()
        };
        let input = TextInput::new(self.placeholder)
            .style(field_style)
            .font(font)
            .icon_font(icon_font);

        let s = bsn! {
            luma_tag
            UNode {
                width: UVal::Percent(1.0),
                min_height: 44.0,
                background_color: Color::srgb(0.10, 0.12, 0.16),
                border_radius: UCornerRadius::all(8.0),
                padding: USides::all(8.0),
            }
            UBorder {
                color: Color::srgb(0.20, 0.24, 0.32),
                width: 1.0,
                radius: UCornerRadius::all(8.0),
            }
            ULayout {
                display: UDisplay::Flex,
                flex_direction: UFlexDirection::Column,
                gap: 6.0,
            }
            Children [
                UNode::default()
                LumaTagInputChips
                ULayout {
                    display: UDisplay::Flex,
                    flex_direction: UFlexDirection::Row,
                    flex_wrap: UFlexWrap::Wrap,
                    align_items: UAlignItems::Center,
                    gap: 6.0,
                }
                Children [
                    LumaTagInputIconFont
                    UText {
                        text: { String::new() },
                        font_size: 1.0,
                        font: hidden_ifont,
                        color: Color::NONE,
                    }
                    ULayout { display: UDisplay::None }
                    --
                    {chips_scene}
                ]
                --
                UNode { width: UVal::Percent(1.0) }
                LumaTagInputField
                Children [
                    @{input}
                ]
            ]
        };
        s.resolve(context, scene)
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

    pub fn font(mut self, font: Handle<Font>) -> Self {
        self.font = font;
        self
    }

    pub fn icon_font(mut self, icon_font: Handle<Font>) -> Self {
        self.icon_font = icon_font;
        self
    }
}

impl Scene for CreditCardInput {
    fn resolve(
        self,
        context: &mut ResolveContext,
        scene: &mut ResolvedScene,
    ) -> Result<(), ResolveSceneError> {
        let font = self.font;
        let icon_font = self.icon_font;
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
                    @{TextInput::new("Card number...").leading_icon(Icon::CREDIT_CARD).font(font.clone()).icon_font(icon_font)}
                ]
                --
                UNode { width: UVal::Px(80.0) }
                Children [
                    @{TextInput::new("MM/YY").max_length(5).font(font.clone())}
                ]
                --
                UNode { width: UVal::Px(70.0) }
                Children [
                    @{TextInput::new("CVV").max_length(4).font(font)}
                ]
            ]
        };
        s.resolve(context, scene)
    }
}

/// Event triggered when [`VoiceInputButton`] toggles recording state.
#[derive(Event, Clone, Copy, Debug, Reflect)]
pub struct VoiceRecordingToggled {
    pub entity: Entity,
    pub is_recording: bool,
}

/// Runtime state component for an interactive [`VoiceInputButton`].
#[derive(Component, Clone, Copy, Debug, Default, Reflect)]
pub struct LumaVoiceInput {
    pub is_recording: bool,
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

    pub fn icon_font(mut self, icon_font: Handle<Font>) -> Self {
        self.icon_font = icon_font;
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
        let luma_voice = LumaVoiceInput {
            is_recording: self.is_recording,
        };
        let s = bsn! {
            luma_voice
            UNode::default()
            ULayout { display: UDisplay::Flex }
            Children [
                @{btn}
            ]
        };
        s.resolve(context, scene)
    }
}

/// Runtime marker for [`PasswordInput`] live strength bar synchronization.
#[derive(Component, Clone, Copy, Debug, Default, Reflect)]
pub struct LumaPasswordInput;

#[derive(Component, Clone, Copy, Debug, Default, Reflect)]
pub struct LumaPasswordStrengthBar;

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

fn strength_color_for_score(score: f32) -> Color {
    if score < 0.4 {
        Color::srgb(0.94, 0.27, 0.27)
    } else if score < 0.75 {
        Color::srgb(0.95, 0.65, 0.15)
    } else {
        Color::srgb(0.18, 0.80, 0.44)
    }
}

fn compute_password_strength(pw: &str) -> f32 {
    if pw.is_empty() {
        return 0.05;
    }
    let len = pw.chars().count();
    let mut score = (len as f32 / 14.0).min(0.5);
    if pw.chars().any(|c| c.is_ascii_uppercase()) {
        score += 0.15;
    }
    if pw.chars().any(|c| c.is_ascii_digit()) {
        score += 0.15;
    }
    if pw.chars().any(|c| !c.is_alphanumeric()) {
        score += 0.20;
    }
    if score >= 0.999 {
        1.0
    } else {
        score.clamp(0.08, 1.0)
    }
}

impl Scene for PasswordInput {
    fn resolve(
        self,
        context: &mut ResolveContext,
        scene: &mut ResolvedScene,
    ) -> Result<(), ResolveSceneError> {
        let strength_score = if self.value.is_empty() {
            self.strength_score.clamp(0.0, 1.0)
        } else {
            compute_password_strength(&self.value)
        };
        let eye_icon = if self.show_password { Icon::EYE_OFF } else { Icon::EYE };
        let strength_color = strength_color_for_score(strength_score);
        let strength_width = UVal::Percent(strength_score);

        let state = if self.disabled { InputState::Disabled } else { InputState::Normal };
        let mode = if self.show_password {
            TextInputMode::Normal
        } else {
            TextInputMode::Password
        };
        let input_elem = TextInput::new(self.placeholder)
            .value(self.value)
            .mode(mode)
            .trailing_icon(eye_icon)
            .font(self.font)
            .icon_font(self.icon_font)
            .state(state);

        if self.show_strength {
            let s = bsn! {
                LumaPasswordInput
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
                        LumaPasswordStrengthBar
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

// ============================================================================
// Runtime State, Overlays, Observers, and Plugin
// ============================================================================

#[derive(Clone, Copy, Debug, PartialEq, Eq, Default, Reflect)]
enum ControlPopupKind {
    #[default]
    MultiSelect,
    Combobox,
    DatePicker,
    DateRangePicker,
    TimePicker,
}

#[derive(Component, Clone, Copy, Debug, Reflect)]
struct LumaControlOverlay {
    owner: Entity,
    kind: ControlPopupKind,
}

impl Default for LumaControlOverlay {
    fn default() -> Self {
        Self {
            owner: Entity::PLACEHOLDER,
            kind: ControlPopupKind::default(),
        }
    }
}

#[derive(Component, Clone, Copy, Debug, Reflect)]
struct LumaControlBackdrop {
    owner: Entity,
    kind: ControlPopupKind,
}

impl Default for LumaControlBackdrop {
    fn default() -> Self {
        Self {
            owner: Entity::PLACEHOLDER,
            kind: ControlPopupKind::default(),
        }
    }
}

#[derive(Component, Clone, Copy, Debug, Default, Reflect)]
struct LumaControlPopupCard;

#[derive(Component, Clone, Debug, Reflect)]
struct LumaControlMenuItem {
    owner: Entity,
    kind: ControlPopupKind,
    value: String,
    index: usize,
}

impl Default for LumaControlMenuItem {
    fn default() -> Self {
        Self {
            owner: Entity::PLACEHOLDER,
            kind: ControlPopupKind::default(),
            value: String::new(),
            index: 0,
        }
    }
}

#[derive(Component, Clone, Copy, Debug, Default, Reflect)]
struct LumaControlMenuCheck;

#[derive(Clone, Debug, Default, Reflect)]
enum ControlPopupActionKind {
    #[default]
    DatePrevMonth,
    DateNextMonth,
    DateSelectDay(u32),
    DateSelectToday,
    RangePreset(String, String),
    TimeAdjustHour(i32),
    TimeAdjustMinute(i32),
    TimePreset(String),
}

#[derive(Component, Clone, Debug, Reflect)]
struct LumaControlPopupAction {
    owner: Entity,
    kind: ControlPopupKind,
    action: ControlPopupActionKind,
}

impl Default for LumaControlPopupAction {
    fn default() -> Self {
        Self {
            owner: Entity::PLACEHOLDER,
            kind: ControlPopupKind::default(),
            action: ControlPopupActionKind::default(),
        }
    }
}

#[derive(Component, Clone, Copy, Debug, Default, Reflect)]
struct LumaTimeDisplayText;

fn parse_ymd_date(s: &str) -> (i32, u32, u32) {
    let parts: Vec<&str> = s.trim().split('-').collect();
    if parts.len() == 3 {
        if let (Ok(y), Ok(m), Ok(d)) = (
            parts[0].parse::<i32>(),
            parts[1].parse::<u32>(),
            parts[2].parse::<u32>(),
        ) {
            let month = m.clamp(1, 12);
            let max_d = days_in_month(y, month);
            return (y, month, d.clamp(1, max_d));
        }
    }
    (2026, 10, 15)
}

fn days_in_month(year: i32, month: u32) -> u32 {
    match month {
        1 | 3 | 5 | 7 | 8 | 10 | 12 => 31,
        4 | 6 | 9 | 11 => 30,
        2 => {
            let leap = (year % 4 == 0 && year % 100 != 0) || (year % 400 == 0);
            if leap { 29 } else { 28 }
        }
        _ => 30,
    }
}

fn parse_hm_time(s: &str) -> (u8, u8, String) {
    let trimmed = s.trim();
    let mut parts = trimmed.split_whitespace();
    let hm = parts.next().unwrap_or("09:30");
    let suffix = parts.collect::<Vec<_>>().join(" ");
    let mut hm_parts = hm.split(':');
    let hour = hm_parts
        .next()
        .and_then(|v| v.parse::<u8>().ok())
        .unwrap_or(9)
        .min(23);
    let minute = hm_parts
        .next()
        .and_then(|v| v.parse::<u8>().ok())
        .unwrap_or(30)
        .min(59);
    (hour, minute, suffix)
}

fn format_hm_time(hour: u8, minute: u8, suffix: &str) -> String {
    if suffix.is_empty() {
        format!("{hour:02}:{minute:02}")
    } else {
        format!("{hour:02}:{minute:02} {suffix}")
    }
}

trait OverlayState {
    fn close_overlay(&mut self, commands: &mut Commands);
}

fn close_overlay_fields(
    commands: &mut Commands,
    is_open: &mut bool,
    overlay_entity: &mut Option<Entity>,
) {
    *is_open = false;
    if let Some(overlay) = overlay_entity.take() {
        if let Ok(mut cmd) = commands.get_entity(overlay) {
            cmd.try_despawn();
        }
    }
}

macro_rules! impl_overlay_state {
    ($($ty:ty),* $(,)?) => {
        $(
            impl OverlayState for $ty {
                fn close_overlay(&mut self, commands: &mut Commands) {
                    close_overlay_fields(commands, &mut self.is_open, &mut self.overlay_entity);
                }
            }
        )*
    };
}

impl_overlay_state!(
    LumaMultiSelect,
    LumaCombobox,
    LumaDatePicker,
    LumaDateRangePicker,
    LumaTimePicker,
);

trait TextFontAccess {
    fn font_of(&self, e: Entity) -> Option<Handle<Font>>;
}

impl TextFontAccess for Query<'_, '_, &UText> {
    fn font_of(&self, e: Entity) -> Option<Handle<Font>> {
        self.get(e).ok().map(|t| t.font.clone())
    }
}

impl TextFontAccess for Query<'_, '_, &mut UText> {
    fn font_of(&self, e: Entity) -> Option<Handle<Font>> {
        self.get(e).ok().map(|t| t.font.clone())
    }
}

fn find_descendant_font<M: Component, T: TextFontAccess>(
    root: Entity,
    children_query: &Query<&Children>,
    marker_query: &Query<(), With<M>>,
    text_query: &T,
) -> Handle<Font> {
    let mut found = Handle::default();
    visit_descendants(root, children_query, |e| {
        if marker_query.contains(e) {
            if let Some(hf) = text_query.font_of(e) {
                found = hf;
                return DescendantVisit::Stop;
            }
        }
        DescendantVisit::Continue
    });
    found
}

fn find_multi_select_fonts<T: TextFontAccess>(
    root: Entity,
    children_query: &Query<&Children>,
    is_placeholder: impl Fn(Entity) -> bool,
    chevron_query: &Query<(), With<LumaMultiSelectChevron>>,
    text_query: &T,
) -> (Handle<Font>, Handle<Font>) {
    let mut placeholder_ent = None;
    let mut icon_font = Handle::default();
    visit_descendants(root, children_query, |e| {
        if placeholder_ent.is_none() && is_placeholder(e) {
            placeholder_ent = Some(e);
        } else if chevron_query.contains(e) {
            if let Some(hf) = text_query.font_of(e) {
                icon_font = hf;
            }
        }
        DescendantVisit::Continue
    });
    let mut font = Handle::default();
    if let Some(pe) = placeholder_ent {
        visit_descendants(pe, children_query, |e| {
            if let Some(hf) = text_query.font_of(e) {
                font = hf;
                return DescendantVisit::Stop;
            }
            DescendantVisit::Continue
        });
    }
    (font, icon_font)
}

fn set_descendant_button_label(
    root: Entity,
    new_label: &str,
    children_query: &Query<&Children>,
    text_query: &mut Query<&mut UText>,
    label_query: &Query<(), With<LumaButtonLabel>>,
) {
    visit_descendants(root, children_query, |e| {
        if label_query.contains(e) {
            if let Ok(mut txt) = text_query.get_mut(e) {
                txt.text = new_label.to_string();
                return DescendantVisit::Stop;
            }
        }
        DescendantVisit::Continue
    });
}

fn spawn_menu_overlay(
    commands: &mut Commands,
    owner: Entity,
    kind: ControlPopupKind,
    menu_left: f32,
    menu_top: f32,
    menu_width: f32,
    options: Vec<String>,
    selected_tags: &[String],
    font: Handle<Font>,
    icon_font: Handle<Font>,
) -> Entity {
    let colors = WidgetColors::default();
    let item_h = 34.0;
    let menu_h = (8.0 + options.len().max(1) as f32 * item_h + options.len().saturating_sub(1) as f32 * 2.0)
        .min(240.0);

    let is_empty = options.is_empty();
    let empty_font = font.clone();
    let empty_row = ConditionalScene(is_empty.then(move || {
        bsn! {
            UNode {
                width: UVal::Percent(1.0),
                height: UVal::Px(32.0),
                padding: USides::axes(12.0, 6.0),
            }
            ULayout {
                display: UDisplay::Flex,
                align_items: UAlignItems::Center,
            }
            Children [
                UText {
                    text: { "No options available".to_string() },
                    font_size: 13.0,
                    font: empty_font,
                    color: { colors.text_dim },
                }
            ]
        }
    }));

    let rows: Vec<_> = options
        .into_iter()
        .enumerate()
        .map(|(index, opt)| {
            let is_selected = selected_tags.contains(&opt);
            let item_bg = if is_selected {
                Color::srgba(0.12, 0.84, 0.60, 0.16)
            } else {
                Color::NONE
            };
            let check_text = if is_selected {
                Icon::CHECK.to_string()
            } else {
                String::new()
            };
            let f = font.clone();
            let ifnt = icon_font.clone();
            let val_clone = opt.clone();
            bsn! {
                UNode {
                    width: UVal::Percent(1.0),
                    height: UVal::Px(item_h),
                    padding: USides::axes(12.0, 6.0),
                    border_radius: UCornerRadius::all(6.0),
                    background_color: item_bg,
                }
                LumaControlMenuItem {
                    owner,
                    kind,
                    value: val_clone,
                    index,
                }
                UInteraction::default()
                UInteractionColors {
                    normal: item_bg,
                    hovered: Color::srgba(1.0, 1.0, 1.0, 0.08),
                    pressed: Color::srgba(1.0, 1.0, 1.0, 0.14),
                }
                ULayout {
                    display: UDisplay::Flex,
                    flex_direction: UFlexDirection::Row,
                    align_items: UAlignItems::Center,
                    justify_content: UJustifyContent::SpaceBetween,
                }
                Children [
                    UText {
                        text: opt,
                        font_size: 13.0,
                        font: f,
                        color: { colors.text_primary },
                    }
                    --
                    LumaControlMenuCheck
                    UText {
                        text: check_text,
                        font_size: 13.0,
                        font: ifnt,
                        color: { colors.primary },
                    }
                ]
            }
        })
        .collect();

    let rows_scene = SceneList::new(rows);
    let overlay_scene = bsn! {
        URootUi::screen()
        UZIndex::Global(9999)
        LumaControlOverlay { owner, kind }
        UNode {
            width: UVal::Percent(1.0),
            height: UVal::Percent(1.0),
            background_color: Color::NONE,
        }
        LumaControlBackdrop { owner, kind }
        UInteraction::default()
        ULayout { display: UDisplay::Flex }
        Children [
            UNode {
                width: UVal::Px(menu_width),
                height: UVal::Auto,
                max_height: menu_h,
                background_color: { colors.surface },
                border_radius: UCornerRadius::all(8.0),
                padding: USides::all(4.0),
            }
            USelf {
                position_type: UPositionType::Absolute,
                left: UVal::Px(menu_left),
                top: UVal::Px(menu_top),
            }
            UBorder {
                color: { colors.surface_border },
                width: 1.0,
                radius: UCornerRadius::all(8.0),
            }
            LumaControlPopupCard
            UInteraction::default()
            ULayout {
                display: UDisplay::Flex,
                flex_direction: UFlexDirection::Column,
                gap: 2.0,
            }
            Children [
                @{empty_row}
                --
                {rows_scene}
            ]
        ]
    };
    commands.spawn_scene(overlay_scene).id()
}

fn spawn_date_picker_overlay(
    commands: &mut Commands,
    owner: Entity,
    left: f32,
    top: f32,
    year: i32,
    month: u32,
    selected_day: u32,
    font: Handle<Font>,
    icon_font: Handle<Font>,
) -> Entity {
    let colors = WidgetColors::default();
    let max_days = days_in_month(year, month);
    let header_title = format!("{year:04}-{month:02}");
    let day_buttons: Vec<_> = (1..=max_days)
        .map(|d| {
            let is_sel = d == selected_day;
            let btn = if is_sel {
                Button::new(d.to_string())
                    .primary()
                    .size(ButtonSize::Small)
                    .font(font.clone())
            } else {
                Button::new(d.to_string())
                    .ghost()
                    .size(ButtonSize::Small)
                    .font(font.clone())
            };
            bsn! {
                LumaControlPopupAction {
                    owner,
                    kind: ControlPopupKind::DatePicker,
                    action: ControlPopupActionKind::DateSelectDay(d),
                }
                UNode { width: UVal::Px(32.0) }
                ULayout { display: UDisplay::Flex, justify_content: UJustifyContent::Center }
                Children [ @{btn} ]
            }
        })
        .collect();
    let days_scene = SceneList::new(day_buttons);
    let header_font = font.clone();
    let today_font = font;
    let prev_ifont = icon_font.clone();
    let next_ifont = icon_font;

    let overlay_scene = bsn! {
        URootUi::screen()
        UZIndex::Global(9999)
        LumaControlOverlay { owner, kind: ControlPopupKind::DatePicker }
        UNode {
            width: UVal::Percent(1.0),
            height: UVal::Percent(1.0),
            background_color: Color::NONE,
        }
        LumaControlBackdrop { owner, kind: ControlPopupKind::DatePicker }
        UInteraction::default()
        ULayout { display: UDisplay::Flex }
        Children [
            UNode {
                width: UVal::Px(264.0),
                background_color: { colors.surface },
                border_radius: UCornerRadius::all(10.0),
                padding: USides::all(10.0),
            }
            USelf {
                position_type: UPositionType::Absolute,
                left: UVal::Px(left),
                top: UVal::Px(top),
            }
            UBorder {
                color: { colors.surface_border },
                width: 1.0,
                radius: UCornerRadius::all(10.0),
            }
            LumaControlPopupCard
            UInteraction::default()
            ULayout {
                display: UDisplay::Flex,
                flex_direction: UFlexDirection::Column,
                gap: 8.0,
            }
            Children [
                UNode { width: UVal::Percent(1.0) }
                ULayout {
                    display: UDisplay::Flex,
                    flex_direction: UFlexDirection::Row,
                    justify_content: UJustifyContent::SpaceBetween,
                    align_items: UAlignItems::Center,
                }
                Children [
                    LumaControlPopupAction {
                        owner,
                        kind: ControlPopupKind::DatePicker,
                        action: ControlPopupActionKind::DatePrevMonth,
                    }
                    UNode::default()
                    Children [
                        @{Button::icon_button(Icon::CHEVRON_LEFT).ghost().size(ButtonSize::Small).icon_font(prev_ifont)}
                    ]
                    --
                    UText {
                        text: header_title,
                        font_size: 14.0,
                        font: header_font,
                        color: { colors.text_primary },
                    }
                    --
                    LumaControlPopupAction {
                        owner,
                        kind: ControlPopupKind::DatePicker,
                        action: ControlPopupActionKind::DateNextMonth,
                    }
                    UNode::default()
                    Children [
                        @{Button::icon_button(Icon::CHEVRON_RIGHT).ghost().size(ButtonSize::Small).icon_font(next_ifont)}
                    ]
                ]
                --
                UNode { width: UVal::Px(244.0) }
                ULayout {
                    display: UDisplay::Flex,
                    flex_direction: UFlexDirection::Row,
                    flex_wrap: UFlexWrap::Wrap,
                    gap: 3.0,
                }
                Children [
                    {days_scene}
                ]
                --
                LumaControlPopupAction {
                    owner,
                    kind: ControlPopupKind::DatePicker,
                    action: ControlPopupActionKind::DateSelectToday,
                }
                UNode { width: UVal::Percent(1.0) }
                ULayout { display: UDisplay::Flex, justify_content: UJustifyContent::End }
                Children [
                    @{Button::new("Today (2026-10-15)").outline().size(ButtonSize::Small).font(today_font)}
                ]
            ]
        ]
    };
    commands.spawn_scene(overlay_scene).id()
}

fn spawn_date_range_overlay(
    commands: &mut Commands,
    owner: Entity,
    left: f32,
    top: f32,
    font: Handle<Font>,
) -> Entity {
    let colors = WidgetColors::default();
    let presets = [
        ("Last 7 Days", "Oct 08, 2026", "Oct 15, 2026"),
        ("Sprint Window", "Oct 20", "Oct 28"),
        ("This Month", "Oct 01, 2026", "Oct 31, 2026"),
        ("Q4 2026", "Oct 01, 2026", "Dec 31, 2026"),
    ];
    let preset_nodes: Vec<_> = presets
        .into_iter()
        .map(|(label, s, e)| {
            let f = font.clone();
            bsn! {
                LumaControlPopupAction {
                    owner,
                    kind: ControlPopupKind::DateRangePicker,
                    action: { ControlPopupActionKind::RangePreset(s.to_string(), e.to_string()) },
                }
                UNode { width: UVal::Percent(1.0) }
                ULayout { display: UDisplay::Flex }
                Children [
                    @{Button::new(label).ghost().size(ButtonSize::Small).font(f)}
                ]
            }
        })
        .collect();
    let presets_scene = SceneList::new(preset_nodes);
    let overlay_scene = bsn! {
        URootUi::screen()
        UZIndex::Global(9999)
        LumaControlOverlay { owner, kind: ControlPopupKind::DateRangePicker }
        UNode {
            width: UVal::Percent(1.0),
            height: UVal::Percent(1.0),
            background_color: Color::NONE,
        }
        LumaControlBackdrop { owner, kind: ControlPopupKind::DateRangePicker }
        UInteraction::default()
        ULayout { display: UDisplay::Flex }
        Children [
            UNode {
                width: UVal::Px(220.0),
                background_color: { colors.surface },
                border_radius: UCornerRadius::all(10.0),
                padding: USides::all(8.0),
            }
            USelf {
                position_type: UPositionType::Absolute,
                left: UVal::Px(left),
                top: UVal::Px(top),
            }
            UBorder {
                color: { colors.surface_border },
                width: 1.0,
                radius: UCornerRadius::all(10.0),
            }
            LumaControlPopupCard
            UInteraction::default()
            ULayout {
                display: UDisplay::Flex,
                flex_direction: UFlexDirection::Column,
                gap: 4.0,
            }
            Children [
                {presets_scene}
            ]
        ]
    };
    commands.spawn_scene(overlay_scene).id()
}

fn spawn_time_picker_overlay(
    commands: &mut Commands,
    owner: Entity,
    left: f32,
    top: f32,
    time_display: String,
    font: Handle<Font>,
    icon_font: Handle<Font>,
) -> Entity {
    let colors = WidgetColors::default();
    let presets = ["09:00 AM", "12:00 PM", "14:30 UTC", "18:00 UTC"];
    let preset_nodes: Vec<_> = presets
        .into_iter()
        .map(|p| {
            let f = font.clone();
            bsn! {
                LumaControlPopupAction {
                    owner,
                    kind: ControlPopupKind::TimePicker,
                    action: { ControlPopupActionKind::TimePreset(p.to_string()) },
                }
                UNode::default()
                Children [
                    @{Button::new(p).ghost().size(ButtonSize::Small).font(f)}
                ]
            }
        })
        .collect();
    let presets_scene = SceneList::new(preset_nodes);
    let f1 = font.clone();
    let f2 = font.clone();
    let f3 = font.clone();
    let f4 = font.clone();
    let f5 = font;
    let _ = icon_font;

    let overlay_scene = bsn! {
        URootUi::screen()
        UZIndex::Global(9999)
        LumaControlOverlay { owner, kind: ControlPopupKind::TimePicker }
        UNode {
            width: UVal::Percent(1.0),
            height: UVal::Percent(1.0),
            background_color: Color::NONE,
        }
        LumaControlBackdrop { owner, kind: ControlPopupKind::TimePicker }
        UInteraction::default()
        ULayout { display: UDisplay::Flex }
        Children [
            UNode {
                width: UVal::Px(220.0),
                background_color: { colors.surface },
                border_radius: UCornerRadius::all(10.0),
                padding: USides::all(10.0),
            }
            USelf {
                position_type: UPositionType::Absolute,
                left: UVal::Px(left),
                top: UVal::Px(top),
            }
            UBorder {
                color: { colors.surface_border },
                width: 1.0,
                radius: UCornerRadius::all(10.0),
            }
            LumaControlPopupCard
            UInteraction::default()
            ULayout {
                display: UDisplay::Flex,
                flex_direction: UFlexDirection::Column,
                gap: 8.0,
            }
            Children [
                LumaTimeDisplayText
                UText {
                    text: time_display,
                    font_size: 16.0,
                    font: f1,
                    color: { colors.primary },
                }
                --
                UNode { width: UVal::Percent(1.0) }
                ULayout {
                    display: UDisplay::Flex,
                    flex_direction: UFlexDirection::Row,
                    justify_content: UJustifyContent::SpaceBetween,
                    gap: 4.0,
                }
                Children [
                    LumaControlPopupAction {
                        owner,
                        kind: ControlPopupKind::TimePicker,
                        action: ControlPopupActionKind::TimeAdjustHour(-1),
                    }
                    UNode::default()
                    Children [ @{Button::new("-1h").outline().size(ButtonSize::Small).font(f2)} ]
                    --
                    LumaControlPopupAction {
                        owner,
                        kind: ControlPopupKind::TimePicker,
                        action: ControlPopupActionKind::TimeAdjustHour(1),
                    }
                    UNode::default()
                    Children [ @{Button::new("+1h").outline().size(ButtonSize::Small).font(f3)} ]
                    --
                    LumaControlPopupAction {
                        owner,
                        kind: ControlPopupKind::TimePicker,
                        action: ControlPopupActionKind::TimeAdjustMinute(-15),
                    }
                    UNode::default()
                    Children [ @{Button::new("-15m").outline().size(ButtonSize::Small).font(f4)} ]
                    --
                    LumaControlPopupAction {
                        owner,
                        kind: ControlPopupKind::TimePicker,
                        action: ControlPopupActionKind::TimeAdjustMinute(15),
                    }
                    UNode::default()
                    Children [ @{Button::new("+15m").outline().size(ButtonSize::Small).font(f5)} ]
                ]
                --
                UNode { width: UVal::Percent(1.0) }
                ULayout {
                    display: UDisplay::Flex,
                    flex_direction: UFlexDirection::Row,
                    flex_wrap: UFlexWrap::Wrap,
                    gap: 4.0,
                }
                Children [
                    {presets_scene}
                ]
            ]
        ]
    };
    commands.spawn_scene(overlay_scene).id()
}

fn toggle_multi_select_popup(
    trigger_entity: Entity,
    commands: &mut Commands,
    parents_query: &Query<&ChildOf>,
    children_query: &Query<&Children>,
    chip_query: &Query<&LumaChip>,
    placeholder_query: &Query<(), With<LumaMultiSelectPlaceholder>>,
    chevron_query: &Query<(), With<LumaMultiSelectChevron>>,
    text_query: &Query<&UText>,
    windows: &Query<&Window>,
    cameras: &Query<(&Camera, &GlobalTransform)>,
    transforms: &Query<(&GlobalTransform, &ComputedSize)>,
    multi_query: &mut Query<(Entity, &mut LumaMultiSelect)>,
) {
    if find_ancestor_or_self(trigger_entity, chip_query, parents_query).is_some() {
        return;
    }
    let Some(ms_entity) = find_ancestor_or_self(trigger_entity, multi_query, parents_query) else {
        return;
    };
    let Ok((entity, mut ms)) = multi_query.get_mut(ms_entity) else {
        return;
    };
    if ms.is_open {
        ms.close_overlay(commands);
        return;
    }
    let (font, icon_font) = find_multi_select_fonts(
        entity,
        children_query,
        |e| placeholder_query.contains(e),
        chevron_query,
        text_query,
    );
    let (left, top, width) = compute_overlay_anchor(entity, 220.0, 180.0, windows, cameras, transforms);
    let overlay_id = spawn_menu_overlay(
        commands,
        entity,
        ControlPopupKind::MultiSelect,
        left,
        top,
        width,
        ms.options.clone(),
        &ms.selected_tags,
        font,
        icon_font,
    );
    ms.overlay_entity = Some(overlay_id);
    ms.is_open = true;
}

fn on_multi_select_trigger_click(
    trigger: On<PointerClick>,
    mut commands: Commands,
    parents_query: Query<&ChildOf>,
    children_query: Query<&Children>,
    chip_query: Query<&LumaChip>,
    placeholder_query: Query<(), With<LumaMultiSelectPlaceholder>>,
    chevron_query: Query<(), With<LumaMultiSelectChevron>>,
    text_query: Query<&UText>,
    windows: Query<&Window>,
    cameras: Query<(&Camera, &GlobalTransform)>,
    transforms: Query<(&GlobalTransform, &ComputedSize)>,
    mut multi_query: Query<(Entity, &mut LumaMultiSelect)>,
) {
    if !is_primary_click(&trigger) {
        return;
    }
    toggle_multi_select_popup(
        trigger.entity,
        &mut commands,
        &parents_query,
        &children_query,
        &chip_query,
        &placeholder_query,
        &chevron_query,
        &text_query,
        &windows,
        &cameras,
        &transforms,
        &mut multi_query,
    );
}

fn on_multi_select_trigger_focus(
    trigger: On<UFocusActivate>,
    mut commands: Commands,
    parents_query: Query<&ChildOf>,
    children_query: Query<&Children>,
    chip_query: Query<&LumaChip>,
    placeholder_query: Query<(), With<LumaMultiSelectPlaceholder>>,
    chevron_query: Query<(), With<LumaMultiSelectChevron>>,
    text_query: Query<&UText>,
    windows: Query<&Window>,
    cameras: Query<(&Camera, &GlobalTransform)>,
    transforms: Query<(&GlobalTransform, &ComputedSize)>,
    mut multi_query: Query<(Entity, &mut LumaMultiSelect)>,
) {
    toggle_multi_select_popup(
        trigger.entity,
        &mut commands,
        &parents_query,
        &children_query,
        &chip_query,
        &placeholder_query,
        &chevron_query,
        &text_query,
        &windows,
        &cameras,
        &transforms,
        &mut multi_query,
    );
}

fn sync_multi_select_placeholder(
    ms_entity: Entity,
    has_tags: bool,
    children_query: &Query<&Children>,
    placeholder_query: &mut Query<&mut ULayout, With<LumaMultiSelectPlaceholder>>,
) {
    visit_descendants(ms_entity, children_query, |e| {
        if let Ok(mut layout) = placeholder_query.get_mut(e) {
            layout.display = if has_tags { UDisplay::None } else { UDisplay::Flex };
            return DescendantVisit::Stop;
        }
        DescendantVisit::Continue
    });
}

fn on_multi_select_chip_dismissed(
    trigger: On<ChipDismissed>,
    mut commands: Commands,
    parents_query: Query<&ChildOf>,
    children_query: Query<&Children>,
    chip_query: Query<&LumaChip>,
    mut multi_query: Query<&mut LumaMultiSelect>,
    mut placeholder_query: Query<&mut ULayout, With<LumaMultiSelectPlaceholder>>,
) {
    let Ok(chip) = chip_query.get(trigger.0) else {
        return;
    };
    let Some(ms_entity) = find_ancestor(trigger.0, &multi_query, &parents_query) else {
        return;
    };
    let Ok(mut ms) = multi_query.get_mut(ms_entity) else {
        return;
    };
    if let Some(pos) = ms.selected_tags.iter().position(|t| t == &chip.label) {
        ms.selected_tags.remove(pos);
    }
    sync_multi_select_placeholder(
        ms_entity,
        !ms.selected_tags.is_empty(),
        &children_query,
        &mut placeholder_query,
    );
    commands.trigger(MultiSelectChanged {
        entity: ms_entity,
        selected_tags: ms.selected_tags.clone(),
    });
}

fn on_combobox_input_changed(
    trigger: On<UTextInputChanged>,
    mut commands: Commands,
    parents_query: Query<&ChildOf>,
    children_query: Query<&Children>,
    input_text_query: Query<(), With<LumaInputText>>,
    text_query: Query<&UText>,
    windows: Query<&Window>,
    cameras: Query<(&Camera, &GlobalTransform)>,
    transforms: Query<(&GlobalTransform, &ComputedSize)>,
    mut combo_query: Query<&mut LumaCombobox>,
) {
    let Some(combo_entity) = find_ancestor(trigger.entity, &combo_query, &parents_query) else {
        return;
    };
    let Ok(mut combo) = combo_query.get_mut(combo_entity) else {
        return;
    };
    combo.close_overlay(&mut commands);
    if combo.options.is_empty() {
        return;
    }
    let filter = trigger.value.to_lowercase();
    let filtered: Vec<String> = combo
        .options
        .iter()
        .filter(|opt| filter.is_empty() || opt.to_lowercase().contains(&filter))
        .cloned()
        .collect();
    if filtered.is_empty() {
        return;
    }
    let font = find_descendant_font(combo_entity, &children_query, &input_text_query, &text_query);
    let (left, top, width) =
        compute_overlay_anchor(combo_entity, 220.0, 180.0, &windows, &cameras, &transforms);
    let overlay_id = spawn_menu_overlay(
        &mut commands,
        combo_entity,
        ControlPopupKind::Combobox,
        left,
        top,
        width,
        filtered,
        &[],
        font,
        Handle::default(),
    );
    combo.overlay_entity = Some(overlay_id);
    combo.is_open = true;
}

fn toggle_combobox_popup(
    trigger_entity: Entity,
    commands: &mut Commands,
    parents_query: &Query<&ChildOf>,
    windows: &Query<&Window>,
    cameras: &Query<(&Camera, &GlobalTransform)>,
    transforms: &Query<(&GlobalTransform, &ComputedSize)>,
    combo_query: &mut Query<&mut LumaCombobox>,
    input_query: &Query<&UTextInput>,
    children_query: &Query<&Children>,
    input_text_query: &Query<(), With<LumaInputText>>,
    text_query: &Query<&UText>,
) {
    let Some(combo_entity) = find_ancestor_or_self(trigger_entity, combo_query, parents_query) else {
        return;
    };
    let Ok(mut combo) = combo_query.get_mut(combo_entity) else {
        return;
    };
    if combo.is_open {
        combo.close_overlay(commands);
        return;
    }
    if combo.options.is_empty() {
        return;
    }
    let mut filter = String::new();
    visit_descendants(combo_entity, children_query, |e| {
        if let Ok(input) = input_query.get(e) {
            filter = input.value.to_lowercase();
            return DescendantVisit::Stop;
        }
        DescendantVisit::Continue
    });
    let filtered: Vec<String> = combo
        .options
        .iter()
        .filter(|opt| filter.is_empty() || opt.to_lowercase().contains(&filter))
        .cloned()
        .collect();
    let list = if filtered.is_empty() {
        combo.options.clone()
    } else {
        filtered
    };
    let font = find_descendant_font(combo_entity, children_query, input_text_query, text_query);
    let (left, top, width) =
        compute_overlay_anchor(combo_entity, 220.0, 180.0, windows, cameras, transforms);
    let overlay_id = spawn_menu_overlay(
        commands,
        combo_entity,
        ControlPopupKind::Combobox,
        left,
        top,
        width,
        list,
        &[],
        font,
        Handle::default(),
    );
    combo.overlay_entity = Some(overlay_id);
    combo.is_open = true;
}

fn on_combobox_click(
    trigger: On<PointerClick>,
    mut commands: Commands,
    parents_query: Query<&ChildOf>,
    windows: Query<&Window>,
    cameras: Query<(&Camera, &GlobalTransform)>,
    transforms: Query<(&GlobalTransform, &ComputedSize)>,
    mut combo_query: Query<&mut LumaCombobox>,
    input_query: Query<&UTextInput>,
    children_query: Query<&Children>,
    input_text_query: Query<(), With<LumaInputText>>,
    text_query: Query<&UText>,
) {
    if !is_primary_click(&trigger) {
        return;
    }
    toggle_combobox_popup(
        trigger.entity,
        &mut commands,
        &parents_query,
        &windows,
        &cameras,
        &transforms,
        &mut combo_query,
        &input_query,
        &children_query,
        &input_text_query,
        &text_query,
    );
}

fn on_combobox_focus(
    trigger: On<UFocusActivate>,
    mut commands: Commands,
    parents_query: Query<&ChildOf>,
    windows: Query<&Window>,
    cameras: Query<(&Camera, &GlobalTransform)>,
    transforms: Query<(&GlobalTransform, &ComputedSize)>,
    mut combo_query: Query<&mut LumaCombobox>,
    input_query: Query<&UTextInput>,
    children_query: Query<&Children>,
    input_text_query: Query<(), With<LumaInputText>>,
    text_query: Query<&UText>,
) {
    toggle_combobox_popup(
        trigger.entity,
        &mut commands,
        &parents_query,
        &windows,
        &cameras,
        &transforms,
        &mut combo_query,
        &input_query,
        &children_query,
        &input_text_query,
        &text_query,
    );
}

fn on_control_menu_item_click(
    trigger: On<PointerClick>,
    mut commands: Commands,
    parents_query: Query<&ChildOf>,
    children_query: Query<&Children>,
    item_query: Query<&LumaControlMenuItem>,
    mut multi_query: Query<&mut LumaMultiSelect>,
    mut combo_query: Query<&mut LumaCombobox>,
    mut chip_query: Query<(Entity, &LumaChip)>,
    tags_row_query: Query<(), With<LumaMultiSelectTagsRow>>,
    chevron_query: Query<(), With<LumaMultiSelectChevron>>,
    mut placeholder_query: Query<&mut ULayout, With<LumaMultiSelectPlaceholder>>,
    mut text_input_query: Query<(Entity, &mut UTextInput)>,
    mut text_query: Query<&mut UText>,
    input_text_query: Query<(), With<LumaInputText>>,
    check_query: Query<(), With<LumaControlMenuCheck>>,
    mut row_bg_query: Query<(&mut UNode, &mut UInteractionColors), With<LumaControlMenuItem>>,
) {
    if !is_primary_click(&trigger) {
        return;
    }
    let Some(item_entity) = find_ancestor_or_self(trigger.entity, &item_query, &parents_query) else {
        return;
    };
    let Ok(item) = item_query.get(item_entity) else {
        return;
    };

    match item.kind {
        ControlPopupKind::MultiSelect => {
            let Ok(mut ms) = multi_query.get_mut(item.owner) else {
                return;
            };
            let is_now_selected = if let Some(pos) = ms.selected_tags.iter().position(|t| t == &item.value) {
                ms.selected_tags.remove(pos);
                let val_to_remove = item.value.clone();
                visit_descendants(item.owner, &children_query, |e| {
                    if let Ok((ce, chip)) = chip_query.get_mut(e) {
                        if chip.label == val_to_remove {
                            if let Ok(mut cmd) = commands.get_entity(ce) {
                                cmd.try_despawn();
                            }
                            return DescendantVisit::Stop;
                        }
                    }
                    DescendantVisit::Continue
                });
                false
            } else {
                ms.selected_tags.push(item.value.clone());
                let mut target_row = None;
                visit_descendants(item.owner, &children_query, |e| {
                    if tags_row_query.contains(e) {
                        target_row = Some(e);
                        return DescendantVisit::Stop;
                    }
                    DescendantVisit::Continue
                });
                if let Some(row_entity) = target_row {
                    let (font, icon_font) = find_multi_select_fonts(
                        item.owner,
                        &children_query,
                        |e| placeholder_query.contains(e),
                        &chevron_query,
                        &text_query,
                    );
                    let chip_scene = Chip::new(item.value.clone())
                        .removable(true)
                        .font(font)
                        .icon_font(icon_font);
                    commands.spawn_scene(chip_scene).insert(ChildOf(row_entity));
                }
                true
            };
            if let Ok((mut node, mut ic)) = row_bg_query.get_mut(item_entity) {
                let bg = if is_now_selected {
                    Color::srgba(0.12, 0.84, 0.60, 0.16)
                } else {
                    Color::NONE
                };
                node.background_color = bg;
                ic.normal = bg;
            }
            visit_descendants(item_entity, &children_query, |e| {
                if check_query.contains(e) {
                    if let Ok(mut txt) = text_query.get_mut(e) {
                        txt.text = if is_now_selected {
                            Icon::CHECK.to_string()
                        } else {
                            String::new()
                        };
                        return DescendantVisit::Stop;
                    }
                }
                DescendantVisit::Continue
            });
            sync_multi_select_placeholder(
                item.owner,
                !ms.selected_tags.is_empty(),
                &children_query,
                &mut placeholder_query,
            );
            commands.trigger(MultiSelectChanged {
                entity: item.owner,
                selected_tags: ms.selected_tags.clone(),
            });
        }
        ControlPopupKind::Combobox => {
            let Ok(mut combo) = combo_query.get_mut(item.owner) else {
                return;
            };
            let chosen = item.value.clone();
            visit_descendants(item.owner, &children_query, |e| {
                if let Ok((_, mut ti)) = text_input_query.get_mut(e) {
                    ti.value = chosen.clone();
                    ti.cursor_position = chosen.len();
                    ti.selection = None;
                    let text_col = ti.text_color;
                    visit_descendants(e, &children_query, |te| {
                        if input_text_query.contains(te) {
                            if let Ok(mut txt) = text_query.get_mut(te) {
                                txt.text = chosen.clone();
                                txt.color = text_col;
                                return DescendantVisit::Stop;
                            }
                        }
                        DescendantVisit::Continue
                    });
                    return DescendantVisit::Stop;
                }
                DescendantVisit::Continue
            });
            combo.close_overlay(&mut commands);
            commands.trigger(ComboboxSelected {
                entity: item.owner,
                value: chosen,
            });
        }
        _ => {}
    }
}

fn on_date_picker_click(
    trigger: On<ButtonClicked>,
    mut commands: Commands,
    parents_query: Query<&ChildOf>,
    children_query: Query<&Children>,
    btn_label_query: Query<(), With<LumaButtonLabel>>,
    btn_icon_query: Query<(), With<LumaButtonIcon>>,
    text_query: Query<&UText>,
    windows: Query<&Window>,
    cameras: Query<(&Camera, &GlobalTransform)>,
    transforms: Query<(&GlobalTransform, &ComputedSize)>,
    mut dp_query: Query<(Entity, &mut LumaDatePicker)>,
) {
    let Some(dp_entity) = find_ancestor(trigger.0, &dp_query, &parents_query) else {
        return;
    };
    let Ok((entity, mut dp)) = dp_query.get_mut(dp_entity) else {
        return;
    };
    if dp.is_open {
        dp.close_overlay(&mut commands);
        return;
    }
    let font = find_descendant_font(entity, &children_query, &btn_label_query, &text_query);
    let icon_font = find_descendant_font(entity, &children_query, &btn_icon_query, &text_query);
    let (left, top, _) = compute_overlay_anchor(entity, 264.0, 260.0, &windows, &cameras, &transforms);
    let overlay_id = spawn_date_picker_overlay(
        &mut commands,
        entity,
        left,
        top,
        dp.year,
        dp.month,
        dp.day,
        font,
        icon_font,
    );
    dp.overlay_entity = Some(overlay_id);
    dp.is_open = true;
}

fn on_date_range_picker_click(
    trigger: On<ButtonClicked>,
    mut commands: Commands,
    parents_query: Query<&ChildOf>,
    children_query: Query<&Children>,
    btn_label_query: Query<(), With<LumaButtonLabel>>,
    text_query: Query<&UText>,
    windows: Query<&Window>,
    cameras: Query<(&Camera, &GlobalTransform)>,
    transforms: Query<(&GlobalTransform, &ComputedSize)>,
    mut drp_query: Query<(Entity, &mut LumaDateRangePicker)>,
) {
    let Some(drp_entity) = find_ancestor(trigger.0, &drp_query, &parents_query) else {
        return;
    };
    let Ok((entity, mut drp)) = drp_query.get_mut(drp_entity) else {
        return;
    };
    if drp.is_open {
        drp.close_overlay(&mut commands);
        return;
    }
    let font = find_descendant_font(entity, &children_query, &btn_label_query, &text_query);
    let (left, top, _) = compute_overlay_anchor(entity, 220.0, 170.0, &windows, &cameras, &transforms);
    let overlay_id = spawn_date_range_overlay(&mut commands, entity, left, top, font);
    drp.overlay_entity = Some(overlay_id);
    drp.is_open = true;
}

fn on_time_picker_click(
    trigger: On<ButtonClicked>,
    mut commands: Commands,
    parents_query: Query<&ChildOf>,
    children_query: Query<&Children>,
    btn_label_query: Query<(), With<LumaButtonLabel>>,
    btn_icon_query: Query<(), With<LumaButtonIcon>>,
    text_query: Query<&UText>,
    windows: Query<&Window>,
    cameras: Query<(&Camera, &GlobalTransform)>,
    transforms: Query<(&GlobalTransform, &ComputedSize)>,
    mut tp_query: Query<(Entity, &mut LumaTimePicker)>,
) {
    let Some(tp_entity) = find_ancestor(trigger.0, &tp_query, &parents_query) else {
        return;
    };
    let Ok((entity, mut tp)) = tp_query.get_mut(tp_entity) else {
        return;
    };
    if tp.is_open {
        tp.close_overlay(&mut commands);
        return;
    }
    let font = find_descendant_font(entity, &children_query, &btn_label_query, &text_query);
    let icon_font = find_descendant_font(entity, &children_query, &btn_icon_query, &text_query);
    let (left, top, _) = compute_overlay_anchor(entity, 220.0, 170.0, &windows, &cameras, &transforms);
    let overlay_id = spawn_time_picker_overlay(
        &mut commands,
        entity,
        left,
        top,
        tp.time.clone(),
        font,
        icon_font,
    );
    tp.overlay_entity = Some(overlay_id);
    tp.is_open = true;
}

fn on_control_popup_action(
    trigger: On<ButtonClicked>,
    mut commands: Commands,
    parents_query: Query<&ChildOf>,
    children_query: Query<&Children>,
    windows: Query<&Window>,
    cameras: Query<(&Camera, &GlobalTransform)>,
    transforms: Query<(&GlobalTransform, &ComputedSize)>,
    action_query: Query<&LumaControlPopupAction>,
    mut dp_query: Query<&mut LumaDatePicker>,
    mut drp_query: Query<&mut LumaDateRangePicker>,
    mut tp_query: Query<&mut LumaTimePicker>,
    mut text_query: Query<&mut UText>,
    btn_label_query: Query<(), With<LumaButtonLabel>>,
    btn_icon_query: Query<(), With<LumaButtonIcon>>,
    time_disp_query: Query<(), With<LumaTimeDisplayText>>,
) {
    let Some(act_entity) = find_ancestor(trigger.0, &action_query, &parents_query) else {
        return;
    };
    let Ok(act) = action_query.get(act_entity) else {
        return;
    };

    match (&act.kind, &act.action) {
        (ControlPopupKind::DatePicker, ControlPopupActionKind::DatePrevMonth)
        | (ControlPopupKind::DatePicker, ControlPopupActionKind::DateNextMonth) => {
            let Ok(mut dp) = dp_query.get_mut(act.owner) else {
                return;
            };
            if matches!(act.action, ControlPopupActionKind::DatePrevMonth) {
                if dp.month == 1 {
                    dp.month = 12;
                    dp.year -= 1;
                } else {
                    dp.month -= 1;
                }
            } else if dp.month == 12 {
                dp.month = 1;
                dp.year += 1;
            } else {
                dp.month += 1;
            }
            dp.day = dp.day.min(days_in_month(dp.year, dp.month));
            dp.close_overlay(&mut commands);
            let font = find_descendant_font(act.owner, &children_query, &btn_label_query, &text_query);
            let icon_font = find_descendant_font(act.owner, &children_query, &btn_icon_query, &text_query);
            let (left, top, _) =
                compute_overlay_anchor(act.owner, 264.0, 260.0, &windows, &cameras, &transforms);
            let overlay_id = spawn_date_picker_overlay(
                &mut commands,
                act.owner,
                left,
                top,
                dp.year,
                dp.month,
                dp.day,
                font,
                icon_font,
            );
            dp.overlay_entity = Some(overlay_id);
            dp.is_open = true;
        }
        (ControlPopupKind::DatePicker, ControlPopupActionKind::DateSelectDay(d)) => {
            let Ok(mut dp) = dp_query.get_mut(act.owner) else {
                return;
            };
            dp.day = *d;
            dp.date = format!("{:04}-{:02}-{:02}", dp.year, dp.month, dp.day);
            set_descendant_button_label(
                act.owner,
                &dp.date,
                &children_query,
                &mut text_query,
                &btn_label_query,
            );
            dp.close_overlay(&mut commands);
            commands.trigger(DateChanged {
                entity: act.owner,
                date: dp.date.clone(),
            });
        }
        (ControlPopupKind::DatePicker, ControlPopupActionKind::DateSelectToday) => {
            let Ok(mut dp) = dp_query.get_mut(act.owner) else {
                return;
            };
            dp.year = 2026;
            dp.month = 10;
            dp.day = 15;
            dp.date = "2026-10-15".to_string();
            set_descendant_button_label(
                act.owner,
                &dp.date,
                &children_query,
                &mut text_query,
                &btn_label_query,
            );
            dp.close_overlay(&mut commands);
            commands.trigger(DateChanged {
                entity: act.owner,
                date: dp.date.clone(),
            });
        }
        (ControlPopupKind::DateRangePicker, ControlPopupActionKind::RangePreset(start, end)) => {
            let Ok(mut drp) = drp_query.get_mut(act.owner) else {
                return;
            };
            drp.start = start.clone();
            drp.end = end.clone();
            let label = format!("{}  →  {}", drp.start, drp.end);
            set_descendant_button_label(
                act.owner,
                &label,
                &children_query,
                &mut text_query,
                &btn_label_query,
            );
            drp.close_overlay(&mut commands);
            commands.trigger(DateRangeChanged {
                entity: act.owner,
                start: drp.start.clone(),
                end: drp.end.clone(),
            });
        }
        (ControlPopupKind::TimePicker, ControlPopupActionKind::TimeAdjustHour(dh)) => {
            let Ok(mut tp) = tp_query.get_mut(act.owner) else {
                return;
            };
            tp.hour = (tp.hour as i32 + dh).rem_euclid(24) as u8;
            tp.time = format_hm_time(tp.hour, tp.minute, &tp.suffix);
            set_descendant_button_label(
                act.owner,
                &tp.time,
                &children_query,
                &mut text_query,
                &btn_label_query,
            );
            if let Some(overlay) = tp.overlay_entity {
                visit_descendants(overlay, &children_query, |e| {
                    if time_disp_query.contains(e) {
                        if let Ok(mut txt) = text_query.get_mut(e) {
                            txt.text = tp.time.clone();
                            return DescendantVisit::Stop;
                        }
                    }
                    DescendantVisit::Continue
                });
            }
            commands.trigger(TimeChanged {
                entity: act.owner,
                time: tp.time.clone(),
            });
        }
        (ControlPopupKind::TimePicker, ControlPopupActionKind::TimeAdjustMinute(dm)) => {
            let Ok(mut tp) = tp_query.get_mut(act.owner) else {
                return;
            };
            let total_mins = tp.hour as i32 * 60 + tp.minute as i32 + dm;
            let wrapped = total_mins.rem_euclid(24 * 60);
            tp.hour = (wrapped / 60) as u8;
            tp.minute = (wrapped % 60) as u8;
            tp.time = format_hm_time(tp.hour, tp.minute, &tp.suffix);
            set_descendant_button_label(
                act.owner,
                &tp.time,
                &children_query,
                &mut text_query,
                &btn_label_query,
            );
            if let Some(overlay) = tp.overlay_entity {
                visit_descendants(overlay, &children_query, |e| {
                    if time_disp_query.contains(e) {
                        if let Ok(mut txt) = text_query.get_mut(e) {
                            txt.text = tp.time.clone();
                            return DescendantVisit::Stop;
                        }
                    }
                    DescendantVisit::Continue
                });
            }
            commands.trigger(TimeChanged {
                entity: act.owner,
                time: tp.time.clone(),
            });
        }
        (ControlPopupKind::TimePicker, ControlPopupActionKind::TimePreset(preset)) => {
            let Ok(mut tp) = tp_query.get_mut(act.owner) else {
                return;
            };
            let (h, m, sfx) = parse_hm_time(preset);
            tp.hour = h;
            tp.minute = m;
            tp.suffix = sfx;
            tp.time = preset.clone();
            set_descendant_button_label(
                act.owner,
                &tp.time,
                &children_query,
                &mut text_query,
                &btn_label_query,
            );
            tp.close_overlay(&mut commands);
            commands.trigger(TimeChanged {
                entity: act.owner,
                time: tp.time.clone(),
            });
        }
        _ => {}
    }
}

fn on_control_backdrop_click(
    trigger: On<PointerClick>,
    mut commands: Commands,
    parents_query: Query<&ChildOf>,
    card_query: Query<&LumaControlPopupCard>,
    backdrop_query: Query<&LumaControlBackdrop>,
    mut multi_query: Query<&mut LumaMultiSelect>,
    mut combo_query: Query<&mut LumaCombobox>,
    mut dp_query: Query<&mut LumaDatePicker>,
    mut drp_query: Query<&mut LumaDateRangePicker>,
    mut tp_query: Query<&mut LumaTimePicker>,
) {
    if !is_primary_click(&trigger) {
        return;
    }
    if find_ancestor_or_self(trigger.entity, &card_query, &parents_query).is_some() {
        return;
    }
    let Some(backdrop_entity) = find_ancestor_or_self(trigger.entity, &backdrop_query, &parents_query)
    else {
        return;
    };
    let Ok(backdrop) = backdrop_query.get(backdrop_entity) else {
        return;
    };

    match backdrop.kind {
        ControlPopupKind::MultiSelect => {
            if let Ok(mut ms) = multi_query.get_mut(backdrop.owner) {
                ms.close_overlay(&mut commands);
            }
        }
        ControlPopupKind::Combobox => {
            if let Ok(mut cb) = combo_query.get_mut(backdrop.owner) {
                cb.close_overlay(&mut commands);
            }
        }
        ControlPopupKind::DatePicker => {
            if let Ok(mut dp) = dp_query.get_mut(backdrop.owner) {
                dp.close_overlay(&mut commands);
            }
        }
        ControlPopupKind::DateRangePicker => {
            if let Ok(mut drp) = drp_query.get_mut(backdrop.owner) {
                drp.close_overlay(&mut commands);
            }
        }
        ControlPopupKind::TimePicker => {
            if let Ok(mut tp) = tp_query.get_mut(backdrop.owner) {
                tp.close_overlay(&mut commands);
            }
        }
    }
}

fn close_all_open<T: Component<Mutability = bevy::ecs::component::Mutable> + OverlayState>(
    commands: &mut Commands,
    q: &mut Query<&mut T>,
) {
    for mut item in q.iter_mut() {
        item.close_overlay(commands);
    }
}

fn close_control_popups_on_escape(
    mut commands: Commands,
    keyboard: Option<Res<ButtonInput<KeyCode>>>,
    mut multi_query: Query<&mut LumaMultiSelect>,
    mut combo_query: Query<&mut LumaCombobox>,
    mut dp_query: Query<&mut LumaDatePicker>,
    mut drp_query: Query<&mut LumaDateRangePicker>,
    mut tp_query: Query<&mut LumaTimePicker>,
) {
    let Some(kb) = keyboard else {
        return;
    };
    if !kb.just_pressed(KeyCode::Escape) {
        return;
    }
    close_all_open(&mut commands, &mut multi_query);
    close_all_open(&mut commands, &mut combo_query);
    close_all_open(&mut commands, &mut dp_query);
    close_all_open(&mut commands, &mut drp_query);
    close_all_open(&mut commands, &mut tp_query);
}

fn cleanup_orphaned_control_overlays(
    mut commands: Commands,
    overlay_query: Query<(Entity, &LumaControlOverlay)>,
    multi_query: Query<(), With<LumaMultiSelect>>,
    combo_query: Query<(), With<LumaCombobox>>,
    dp_query: Query<(), With<LumaDatePicker>>,
    drp_query: Query<(), With<LumaDateRangePicker>>,
    tp_query: Query<(), With<LumaTimePicker>>,
) {
    for (overlay_entity, overlay) in overlay_query.iter() {
        let alive = match overlay.kind {
            ControlPopupKind::MultiSelect => multi_query.contains(overlay.owner),
            ControlPopupKind::Combobox => combo_query.contains(overlay.owner),
            ControlPopupKind::DatePicker => dp_query.contains(overlay.owner),
            ControlPopupKind::DateRangePicker => drp_query.contains(overlay.owner),
            ControlPopupKind::TimePicker => tp_query.contains(overlay.owner),
        };
        if !alive {
            if let Ok(mut cmd) = commands.get_entity(overlay_entity) {
                cmd.try_despawn();
            }
        }
    }
}

fn on_range_slider_changed(
    trigger: On<SliderChanged>,
    mut commands: Commands,
    parents_query: Query<&ChildOf>,
    min_box_query: Query<&LumaRangeMinSlider>,
    max_box_query: Query<&LumaRangeMaxSlider>,
    mut range_query: Query<&mut LumaRangeSlider>,
    mut slider_query: Query<&mut LumaSlider>,
) {
    let Some(range_entity) = find_ancestor(trigger.entity, &range_query, &parents_query) else {
        return;
    };
    let Ok(mut range) = range_query.get_mut(range_entity) else {
        return;
    };
    if find_ancestor(trigger.entity, &min_box_query, &parents_query).is_some() {
        let clamped = trigger.value.min(range.max_value).clamp(range.min, range.max);
        range.min_value = clamped;
        if let Ok(mut slider) = slider_query.get_mut(trigger.entity) {
            if (slider.value - clamped).abs() > f32::EPSILON {
                slider.value = clamped;
            }
        }
    } else if find_ancestor(trigger.entity, &max_box_query, &parents_query).is_some() {
        let clamped = trigger.value.max(range.min_value).clamp(range.min, range.max);
        range.max_value = clamped;
        if let Ok(mut slider) = slider_query.get_mut(trigger.entity) {
            if (slider.value - clamped).abs() > f32::EPSILON {
                slider.value = clamped;
            }
        }
    } else {
        return;
    }

    commands.trigger(RangeSliderChanged {
        entity: range_entity,
        min_value: range.min_value,
        max_value: range.max_value,
    });
}

fn trigger_file_uploader(entity: Entity, commands: &mut Commands) {
    commands.trigger(FileUploadRequested { entity });
}

fn on_file_uploader_click(
    trigger: On<PointerClick>,
    mut commands: Commands,
    parents_query: Query<&ChildOf>,
    uploader_query: Query<&LumaFileUploader>,
) {
    if !is_primary_click(&trigger) {
        return;
    }
    if let Some(entity) = find_ancestor_or_self(trigger.entity, &uploader_query, &parents_query) {
        trigger_file_uploader(entity, &mut commands);
    }
}

fn on_file_uploader_activate(
    trigger: On<UFocusActivate>,
    mut commands: Commands,
    parents_query: Query<&ChildOf>,
    uploader_query: Query<&LumaFileUploader>,
) {
    if let Some(entity) = find_ancestor_or_self(trigger.entity, &uploader_query, &parents_query) {
        trigger_file_uploader(entity, &mut commands);
    }
}

fn on_otp_input_changed(
    trigger: On<UTextInputChanged>,
    mut commands: Commands,
    parents_query: Query<&ChildOf>,
    children_query: Query<&Children>,
    mut focus_state: Option<ResMut<UFocusState>>,
    otp_box_query: Query<&LumaOtpBox>,
    mut otp_query: Query<&mut LumaOtpInput>,
    input_query: Query<&UTextInput>,
) {
    let Some(box_entity) = find_ancestor(trigger.entity, &otp_box_query, &parents_query) else {
        return;
    };
    let Ok(otp_box) = otp_box_query.get(box_entity) else {
        return;
    };
    let Some(otp_entity) = find_ancestor(box_entity, &otp_query, &parents_query) else {
        return;
    };
    let Ok(mut otp) = otp_query.get_mut(otp_entity) else {
        return;
    };

    let mut digits: Vec<(usize, String, Entity)> = Vec::new();
    visit_descendants(otp_entity, &children_query, |e| {
        if let Ok(b) = otp_box_query.get(e) {
            visit_descendants(e, &children_query, |ie| {
                if let Ok(ti) = input_query.get(ie) {
                    let val = if ie == trigger.entity {
                        trigger.value.clone()
                    } else {
                        ti.value.clone()
                    };
                    digits.push((b.index, val, ie));
                    return DescendantVisit::Stop;
                }
                DescendantVisit::Continue
            });
            return DescendantVisit::SkipChildren;
        }
        DescendantVisit::Continue
    });
    digits.sort_by_key(|(idx, _, _)| *idx);
    let full_code: String = digits.iter().map(|(_, v, _)| v.as_str()).collect();
    otp.code = full_code.clone();

    if !trigger.value.is_empty() && otp_box.index + 1 < otp.length {
        if let Some((_, _, next_input)) = digits.iter().find(|(idx, _, _)| *idx == otp_box.index + 1) {
            if let Some(fs) = focus_state.as_deref_mut() {
                set_focus(&mut commands, fs, Some(*next_input), FocusModality::Keyboard);
            }
        }
    }

    commands.trigger(OtpChanged {
        entity: otp_entity,
        code: full_code.clone(),
    });
    if otp.length > 0 && full_code.chars().count() >= otp.length {
        commands.trigger(OtpCompleted {
            entity: otp_entity,
            code: full_code,
        });
    }
}

fn on_phone_input_changed(
    trigger: On<UTextInputChanged>,
    mut commands: Commands,
    parents_query: Query<&ChildOf>,
    mut phone_query: Query<&mut LumaPhoneInput>,
) {
    let Some(phone_entity) = find_ancestor(trigger.entity, &phone_query, &parents_query) else {
        return;
    };
    let Ok(mut phone) = phone_query.get_mut(phone_entity) else {
        return;
    };
    phone.number = trigger.value.clone();
    commands.trigger(PhoneNumberChanged {
        entity: phone_entity,
        country_code: phone.country_code.clone(),
        number: phone.number.clone(),
    });
}

fn on_reaction_picker_click(
    trigger: On<ButtonClicked>,
    mut commands: Commands,
    parents_query: Query<&ChildOf>,
    children_query: Query<&Children>,
    item_query: Query<&LumaReactionItem>,
    mut picker_query: Query<&mut LumaReactionPicker>,
    mut btn_query: Query<(&mut LumaButton, &mut UNode, &mut UBorder, &mut UInteractionColors)>,
    mut text_query: Query<&mut UText>,
    btn_parts_query: Query<(), Or<(With<LumaButtonLabel>, With<LumaButtonIcon>)>>,
) {
    let Some(item_entity) = find_ancestor(trigger.0, &item_query, &parents_query) else {
        return;
    };
    let Ok(item) = item_query.get(item_entity) else {
        return;
    };
    let Some(picker_entity) = find_ancestor(item_entity, &picker_query, &parents_query) else {
        return;
    };
    let Ok(mut picker) = picker_query.get_mut(picker_entity) else {
        return;
    };

    let selected = if picker.selected_index == Some(item.index) {
        picker.selected_index = None;
        picker.selected_reaction = None;
        false
    } else {
        picker.selected_index = Some(item.index);
        picker.selected_reaction = Some(item.reaction.clone());
        true
    };

    let active_idx = picker.selected_index;
    visit_descendants(picker_entity, &children_query, |e| {
        if let Ok(other) = item_query.get(e) {
            let variant = if active_idx == Some(other.index) {
                ButtonVariant::Primary
            } else {
                ButtonVariant::Ghost
            };
            visit_descendants(e, &children_query, |be| {
                if let Ok((mut lb, mut node, mut border, mut ic)) = btn_query.get_mut(be) {
                    let ch = children_query.get(be).ok();
                    apply_button_variant_visuals(
                        variant,
                        &mut lb,
                        &mut node,
                        &mut border,
                        &mut ic,
                        ch,
                        &children_query,
                        &mut text_query,
                        &btn_parts_query,
                    );
                    return DescendantVisit::Stop;
                }
                DescendantVisit::Continue
            });
            return DescendantVisit::SkipChildren;
        }
        DescendantVisit::Continue
    });

    commands.trigger(ReactionSelected {
        entity: picker_entity,
        index: item.index,
        reaction: item.reaction.clone(),
        selected,
    });
}

fn on_rich_text_toolbar_click(
    trigger: On<ButtonClicked>,
    mut commands: Commands,
    parents_query: Query<&ChildOf>,
    children_query: Query<&Children>,
    action_query: Query<&LumaRichTextAction>,
    mut toolbar_query: Query<&mut LumaRichTextToolbar>,
    mut btn_query: Query<(&mut LumaButton, &mut UNode, &mut UBorder, &mut UInteractionColors)>,
    mut text_query: Query<&mut UText>,
    btn_parts_query: Query<(), Or<(With<LumaButtonLabel>, With<LumaButtonIcon>)>>,
) {
    let Some(act_entity) = find_ancestor(trigger.0, &action_query, &parents_query) else {
        return;
    };
    let Ok(act) = action_query.get(act_entity) else {
        return;
    };
    let Some(toolbar_entity) = find_ancestor(act_entity, &toolbar_query, &parents_query) else {
        return;
    };
    let Ok(mut toolbar) = toolbar_query.get_mut(toolbar_entity) else {
        return;
    };

    let active = match act.format {
        RichTextFormat::Bold => {
            toolbar.bold = !toolbar.bold;
            toolbar.bold
        }
        RichTextFormat::Italic => {
            toolbar.italic = !toolbar.italic;
            toolbar.italic
        }
        RichTextFormat::Underline => {
            toolbar.underline = !toolbar.underline;
            toolbar.underline
        }
        RichTextFormat::Link => true,
    };

    if act.format != RichTextFormat::Link {
        let variant = if active {
            ButtonVariant::Secondary
        } else {
            ButtonVariant::Ghost
        };
        if let Ok((mut lb, mut node, mut border, mut ic)) = btn_query.get_mut(trigger.0) {
            let ch = children_query.get(trigger.0).ok();
            apply_button_variant_visuals(
                variant,
                &mut lb,
                &mut node,
                &mut border,
                &mut ic,
                ch,
                &children_query,
                &mut text_query,
                &btn_parts_query,
            );
        }
    }

    commands.trigger(RichTextFormatToggled {
        entity: toolbar_entity,
        format: act.format,
        active,
    });
}

fn commit_tag_to_tag_input(
    commands: &mut Commands,
    tag_entity: Entity,
    raw_value: &str,
    input_entity: Entity,
    tag_input: &mut LumaTagInput,
    children_query: &Query<&Children>,
    chips_query: &Query<(), With<LumaTagInputChips>>,
    tag_icon_font_query: &Query<(), With<LumaTagInputIconFont>>,
    text_input_query: &mut Query<&mut UTextInput>,
    input_text_query: &Query<(), With<LumaInputText>>,
    text_query: &mut Query<&mut UText>,
) {
    let tag = raw_value.trim_end_matches(',').trim().to_string();
    if let Ok(mut ti) = text_input_query.get_mut(input_entity) {
        ti.value.clear();
        ti.cursor_position = 0;
        ti.selection = None;
        let placeholder = ti.placeholder.clone();
        let placeholder_color = ti.placeholder_color;
        visit_descendants(input_entity, children_query, |e| {
            if input_text_query.contains(e) {
                if let Ok(mut txt) = text_query.get_mut(e) {
                    txt.text = placeholder.clone();
                    txt.color = placeholder_color;
                    return DescendantVisit::Stop;
                }
            }
            DescendantVisit::Continue
        });
    }
    if tag.is_empty() || tag_input.tags.contains(&tag) {
        return;
    }
    tag_input.tags.push(tag.clone());
    let mut chips_row = None;
    visit_descendants(tag_entity, children_query, |e| {
        if chips_query.contains(e) {
            chips_row = Some(e);
            return DescendantVisit::Stop;
        }
        DescendantVisit::Continue
    });
    if let Some(row) = chips_row {
        let font = find_descendant_font(tag_entity, children_query, input_text_query, text_query);
        let icon_font = find_descendant_font(tag_entity, children_query, tag_icon_font_query, text_query);
        let chip = Chip::new(tag)
            .removable(true)
            .font(font)
            .icon_font(icon_font);
        commands.spawn_scene(chip).insert(ChildOf(row));
    }
    commands.trigger(TagsChanged {
        entity: tag_entity,
        tags: tag_input.tags.clone(),
    });
}

fn on_tag_input_submit(
    trigger: On<UTextInputSubmit>,
    mut commands: Commands,
    parents_query: Query<&ChildOf>,
    children_query: Query<&Children>,
    field_query: Query<&LumaTagInputField>,
    mut tag_input_query: Query<&mut LumaTagInput>,
    chips_query: Query<(), With<LumaTagInputChips>>,
    tag_icon_font_query: Query<(), With<LumaTagInputIconFont>>,
    mut text_input_query: Query<&mut UTextInput>,
    input_text_query: Query<(), With<LumaInputText>>,
    mut text_query: Query<&mut UText>,
) {
    if find_ancestor(trigger.entity, &field_query, &parents_query).is_none() {
        return;
    }
    let Some(tag_entity) = find_ancestor(trigger.entity, &tag_input_query, &parents_query) else {
        return;
    };
    let Ok(mut tag_input) = tag_input_query.get_mut(tag_entity) else {
        return;
    };
    commit_tag_to_tag_input(
        &mut commands,
        tag_entity,
        &trigger.value,
        trigger.entity,
        &mut tag_input,
        &children_query,
        &chips_query,
        &tag_icon_font_query,
        &mut text_input_query,
        &input_text_query,
        &mut text_query,
    );
}

fn on_tag_input_changed(
    trigger: On<UTextInputChanged>,
    mut commands: Commands,
    parents_query: Query<&ChildOf>,
    children_query: Query<&Children>,
    field_query: Query<&LumaTagInputField>,
    mut tag_input_query: Query<&mut LumaTagInput>,
    chips_query: Query<(), With<LumaTagInputChips>>,
    tag_icon_font_query: Query<(), With<LumaTagInputIconFont>>,
    mut text_input_query: Query<&mut UTextInput>,
    input_text_query: Query<(), With<LumaInputText>>,
    mut text_query: Query<&mut UText>,
) {
    if !trigger.value.ends_with(',') {
        return;
    }
    if find_ancestor(trigger.entity, &field_query, &parents_query).is_none() {
        return;
    }
    let Some(tag_entity) = find_ancestor(trigger.entity, &tag_input_query, &parents_query) else {
        return;
    };
    let Ok(mut tag_input) = tag_input_query.get_mut(tag_entity) else {
        return;
    };
    commit_tag_to_tag_input(
        &mut commands,
        tag_entity,
        &trigger.value,
        trigger.entity,
        &mut tag_input,
        &children_query,
        &chips_query,
        &tag_icon_font_query,
        &mut text_input_query,
        &input_text_query,
        &mut text_query,
    );
}

fn on_tag_input_chip_dismissed(
    trigger: On<ChipDismissed>,
    mut commands: Commands,
    parents_query: Query<&ChildOf>,
    chip_query: Query<&LumaChip>,
    mut tag_input_query: Query<&mut LumaTagInput>,
) {
    let Ok(chip) = chip_query.get(trigger.0) else {
        return;
    };
    let Some(tag_entity) = find_ancestor(trigger.0, &tag_input_query, &parents_query) else {
        return;
    };
    let Ok(mut tag_input) = tag_input_query.get_mut(tag_entity) else {
        return;
    };
    if let Some(pos) = tag_input.tags.iter().position(|t| t == &chip.label) {
        tag_input.tags.remove(pos);
        commands.trigger(TagsChanged {
            entity: tag_entity,
            tags: tag_input.tags.clone(),
        });
    }
}

fn on_voice_input_click(
    trigger: On<ButtonClicked>,
    mut commands: Commands,
    parents_query: Query<&ChildOf>,
    children_query: Query<&Children>,
    mut voice_query: Query<&mut LumaVoiceInput>,
    mut btn_query: Query<(&mut LumaButton, &mut UNode, &mut UBorder, &mut UInteractionColors)>,
    mut text_query: Query<&mut UText>,
    btn_parts_query: Query<(), Or<(With<LumaButtonLabel>, With<LumaButtonIcon>)>>,
) {
    let Some(voice_entity) = find_ancestor(trigger.0, &voice_query, &parents_query) else {
        return;
    };
    let Ok(mut voice) = voice_query.get_mut(voice_entity) else {
        return;
    };
    voice.is_recording = !voice.is_recording;
    let variant = if voice.is_recording {
        ButtonVariant::Danger
    } else {
        ButtonVariant::Secondary
    };
    if let Ok((mut lb, mut node, mut border, mut ic)) = btn_query.get_mut(trigger.0) {
        let ch = children_query.get(trigger.0).ok();
        apply_button_variant_visuals(
            variant,
            &mut lb,
            &mut node,
            &mut border,
            &mut ic,
            ch,
            &children_query,
            &mut text_query,
            &btn_parts_query,
        );
    }
    commands.trigger(VoiceRecordingToggled {
        entity: voice_entity,
        is_recording: voice.is_recording,
    });
}

fn on_password_strength_changed(
    trigger: On<UTextInputChanged>,
    parents_query: Query<&ChildOf>,
    children_query: Query<&Children>,
    pw_query: Query<(), With<LumaPasswordInput>>,
    mut bar_query: Query<&mut UNode, With<LumaPasswordStrengthBar>>,
) {
    let Some(pw_entity) = find_ancestor(trigger.entity, &pw_query, &parents_query) else {
        return;
    };
    let score = compute_password_strength(&trigger.value);
    let color = strength_color_for_score(score);
    visit_descendants(pw_entity, &children_query, |e| {
        if let Ok(mut bar) = bar_query.get_mut(e) {
            bar.width = UVal::Percent(score);
            bar.background_color = color;
            return DescendantVisit::Stop;
        }
        DescendantVisit::Continue
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
            .register_type::<LumaMultiSelect>()
            .register_type::<LumaMultiSelectTagsRow>()
            .register_type::<LumaMultiSelectPlaceholder>()
            .register_type::<LumaMultiSelectChevron>()
            .register_type::<LumaCombobox>()
            .register_type::<LumaRangeSlider>()
            .register_type::<LumaRangeMinSlider>()
            .register_type::<LumaRangeMaxSlider>()
            .register_type::<LumaDatePicker>()
            .register_type::<LumaDateRangePicker>()
            .register_type::<LumaTimePicker>()
            .register_type::<LumaFileUploader>()
            .register_type::<LumaOtpInput>()
            .register_type::<LumaOtpBox>()
            .register_type::<LumaPhoneInput>()
            .register_type::<LumaReactionPicker>()
            .register_type::<LumaReactionItem>()
            .register_type::<LumaRichTextToolbar>()
            .register_type::<LumaRichTextAction>()
            .register_type::<RichTextFormat>()
            .register_type::<LumaTagInput>()
            .register_type::<LumaTagInputChips>()
            .register_type::<LumaTagInputIconFont>()
            .register_type::<LumaTagInputField>()
            .register_type::<LumaVoiceInput>()
            .register_type::<LumaPasswordInput>()
            .register_type::<LumaPasswordStrengthBar>()
            .add_observer(on_color_picker_swatch_click)
            .add_observer(on_multi_select_trigger_click)
            .add_observer(on_multi_select_trigger_focus)
            .add_observer(on_multi_select_chip_dismissed)
            .add_observer(on_combobox_input_changed)
            .add_observer(on_combobox_click)
            .add_observer(on_combobox_focus)
            .add_observer(on_control_menu_item_click)
            .add_observer(on_date_picker_click)
            .add_observer(on_date_range_picker_click)
            .add_observer(on_time_picker_click)
            .add_observer(on_control_popup_action)
            .add_observer(on_control_backdrop_click)
            .add_observer(on_range_slider_changed)
            .add_observer(on_file_uploader_click)
            .add_observer(on_file_uploader_activate)
            .add_observer(on_otp_input_changed)
            .add_observer(on_phone_input_changed)
            .add_observer(on_reaction_picker_click)
            .add_observer(on_rich_text_toolbar_click)
            .add_observer(on_tag_input_submit)
            .add_observer(on_tag_input_changed)
            .add_observer(on_tag_input_chip_dismissed)
            .add_observer(on_voice_input_click)
            .add_observer(on_password_strength_changed)
            .add_systems(
                Update,
                (close_control_popups_on_escape, cleanup_orphaned_control_overlays),
            );
    }
}

