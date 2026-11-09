//! Compact joined button groups (toolbar style): segmented buttons, addons,
//! inline inputs, dropdown triggers, and accent / danger tones sharing one
//! rounded bordered container separated by hairline dividers.
//!
//! ```rust,ignore
//! ButtonGroup::new(vec![
//!     Segment::icon_text(Icon::SAVE, "Save"),
//!     Segment::dropdown(),
//! ])
//! .accent()
//! .font(font)
//! .icon_font(icon_font)
//! ```

use bevy::picking::pointer::PointerButton;
use bevy::prelude::*;
use bevy::scene::{ResolveContext, ResolvedScene, ResolveSceneError};
use bevy::text::LineBreak;
use bevy_luma::prelude::*;

use crate::navigation::SceneList;

// ============================================================================
// Palette
// ============================================================================

/// Full color palette used by [`ButtonGroup`]. The default matches a dark,
/// neutral toolbar look with an emerald accent.
#[derive(Clone, Copy, Debug, Reflect)]
pub struct ButtonGroupPalette {
    pub surface: Color,
    pub surface_hover: Color,
    pub surface_pressed: Color,
    pub border: Color,
    pub divider: Color,
    pub text: Color,
    pub icon: Color,
    pub muted_text: Color,
    pub muted_surface: Color,
    pub input_surface: Color,
    pub accent: Color,
    pub accent_hover: Color,
    pub accent_pressed: Color,
    pub accent_text: Color,
    pub accent_divider: Color,
    pub danger: Color,
    pub danger_hover: Color,
    pub danger_pressed: Color,
    pub danger_text: Color,
    pub danger_border: Color,
    pub danger_divider: Color,
    pub caret: Color,
    pub selection: Color,
}

impl Default for ButtonGroupPalette {
    fn default() -> Self {
        Self {
            surface: Color::srgb_u8(0x18, 0x18, 0x1B),
            surface_hover: Color::srgb_u8(0x24, 0x24, 0x28),
            surface_pressed: Color::srgb_u8(0x2C, 0x2C, 0x31),
            border: Color::srgb_u8(0x2B, 0x2B, 0x30),
            divider: Color::srgb_u8(0x2B, 0x2B, 0x30),
            text: Color::srgb_u8(0xEC, 0xEC, 0xEE),
            icon: Color::srgb_u8(0xD8, 0xD8, 0xDC),
            muted_text: Color::srgb_u8(0x74, 0x74, 0x7C),
            muted_surface: Color::srgb_u8(0x12, 0x12, 0x14),
            input_surface: Color::srgb_u8(0x12, 0x12, 0x14),
            accent: Color::srgb_u8(0x1F, 0xD6, 0x9A),
            accent_hover: Color::srgb_u8(0x3D, 0xE2, 0xAC),
            accent_pressed: Color::srgb_u8(0x17, 0xB8, 0x84),
            accent_text: Color::srgb_u8(0x06, 0x22, 0x18),
            accent_divider: Color::srgb_u8(0x17, 0xAE, 0x7E),
            danger: Color::srgb_u8(0x3B, 0x12, 0x13),
            danger_hover: Color::srgb_u8(0x4A, 0x17, 0x18),
            danger_pressed: Color::srgb_u8(0x30, 0x0E, 0x0F),
            danger_text: Color::srgb_u8(0xF4, 0x6B, 0x6B),
            danger_border: Color::srgb_u8(0x52, 0x1C, 0x1D),
            danger_divider: Color::srgb_u8(0x5A, 0x20, 0x21),
            caret: Color::srgb_u8(0xEC, 0xEC, 0xEE),
            selection: Color::srgba(0.12, 0.84, 0.60, 0.30),
        }
    }
}

/// Dimension metrics of a [`ButtonGroup`].
#[derive(Clone, Copy, Debug, Reflect)]
pub struct ButtonGroupMetrics {
    /// Total outer height including the 1px border.
    pub height: f32,
    pub radius: f32,
    pub font_size: f32,
    pub icon_size: f32,
    pub padding_x: f32,
    pub gap: f32,
}

impl Default for ButtonGroupMetrics {
    fn default() -> Self {
        Self {
            height: 34.0,
            radius: 8.0,
            font_size: 14.0,
            icon_size: 15.0,
            padding_x: 12.0,
            gap: 7.0,
        }
    }
}

// ============================================================================
// Segment description
// ============================================================================

/// Visual tone of a single segment.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Default, Reflect)]
pub enum SegmentTone {
    /// Uses the group's tone.
    #[default]
    Inherit,
    Default,
    /// Darker recessed surface (`#121214`) with primary text.
    Recessed,
    /// Dimmed, non interactive ("Disabled").
    Muted,
    /// Filled emerald accent.
    Accent,
    /// Dark red destructive.
    Danger,
}

/// Kind of popup overlay menu triggered by a segment (if any).
#[derive(Clone, Copy, Debug, PartialEq, Eq, Default, Reflect)]
pub enum SegmentMenuKind {
    #[default]
    None,
    /// Select picker that updates its label to the selected option.
    Select,
    /// Action dropdown menu that triggers an action on selection.
    Dropdown,
}

/// Content kind of a segment.
#[derive(Clone, Debug, PartialEq, Reflect)]
pub enum SegmentKind {
    /// Clickable button cell (text, icon or both).
    Button,
    /// Static addon label such as `https://` or `Amount`.
    Addon,
    /// Inline editable text field.
    Input {
        placeholder: String,
        value: String,
        width: f32,
    },
}

/// One cell inside a [`ButtonGroup`].
#[derive(Clone, Debug, Reflect)]
pub struct Segment {
    pub kind: SegmentKind,
    pub value: String,
    pub label: Option<String>,
    pub icon: Option<String>,
    pub trailing_icon: Option<String>,
    pub badge: Option<String>,
    pub icon_color: Option<Color>,
    pub text_color: Option<Color>,
    pub tone: SegmentTone,
    pub selected: bool,
    pub divider_before: bool,
    pub gap_before: Option<f32>,
    pub width: Option<f32>,
    pub padding_x: Option<f32>,
    pub radius: Option<f32>,
    pub menu_kind: SegmentMenuKind,
    pub menu_options: Vec<String>,
}

impl Default for Segment {
    fn default() -> Self {
        Self {
            kind: SegmentKind::Button,
            value: String::new(),
            label: None,
            icon: None,
            trailing_icon: None,
            badge: None,
            icon_color: None,
            text_color: None,
            tone: SegmentTone::Inherit,
            selected: false,
            divider_before: true,
            gap_before: None,
            width: None,
            padding_x: None,
            radius: None,
            menu_kind: SegmentMenuKind::None,
            menu_options: Vec::new(),
        }
    }
}

impl Segment {
    /// Text-only button cell.
    pub fn text(label: impl Into<String>) -> Self {
        let label = label.into();
        Self {
            value: label.clone(),
            label: Some(label),
            ..default()
        }
    }

    /// Icon-only square button cell.
    pub fn icon(icon: impl Into<String>) -> Self {
        let icon = icon.into();
        Self {
            value: icon.clone(),
            icon: Some(icon),
            ..default()
        }
    }

    /// Leading icon + label button cell.
    pub fn icon_text(icon: impl Into<String>, label: impl Into<String>) -> Self {
        let label = label.into();
        Self {
            value: label.clone(),
            icon: Some(icon.into()),
            label: Some(label),
            ..default()
        }
    }

    /// Label + trailing icon button cell (e.g. `Next →`).
    pub fn text_icon(label: impl Into<String>, icon: impl Into<String>) -> Self {
        let label = label.into();
        Self {
            value: label.clone(),
            trailing_icon: Some(icon.into()),
            label: Some(label),
            ..default()
        }
    }

    /// Select trigger: label with a trailing chevron (e.g. `usd ⌄`).
    pub fn select(label: impl Into<String>) -> Self {
        let label = label.into();
        Self {
            kind: SegmentKind::Button,
            value: label.clone(),
            label: Some(label),
            trailing_icon: Some(Icon::CHEVRON_DOWN.to_string()),
            menu_kind: SegmentMenuKind::Select,
            ..default()
        }
    }

    /// Icon-only chevron dropdown cell.
    pub fn dropdown() -> Self {
        Self {
            kind: SegmentKind::Button,
            value: "dropdown".into(),
            icon: Some(Icon::CHEVRON_DOWN.to_string()),
            menu_kind: SegmentMenuKind::Dropdown,
            ..default()
        }
    }

    /// Attaches dropdown or select options to this segment.
    pub fn menu(mut self, options: impl IntoIterator<Item = impl Into<String>>) -> Self {
        self.menu_options = options.into_iter().map(Into::into).collect();
        if self.menu_kind == SegmentMenuKind::None {
            self.menu_kind = SegmentMenuKind::Dropdown;
        }
        self
    }

    /// Static, non interactive addon label.
    pub fn addon(label: impl Into<String>) -> Self {
        let label = label.into();
        Self {
            kind: SegmentKind::Addon,
            value: label.clone(),
            label: Some(label),
            ..default()
        }
    }

    /// Inline text field of a fixed width.
    pub fn input(placeholder: impl Into<String>, width: f32) -> Self {
        Self {
            kind: SegmentKind::Input {
                placeholder: placeholder.into(),
                value: String::new(),
                width,
            },
            value: "input".into(),
            ..default()
        }
    }

    /// Inline text field pre-filled with a value.
    pub fn input_value(value: impl Into<String>, width: f32) -> Self {
        let value = value.into();
        Self {
            kind: SegmentKind::Input {
                placeholder: String::new(),
                value: value.clone(),
                width,
            },
            value,
            ..default()
        }
    }

    /// Leading icon living inside an input area (no divider after it).
    pub fn input_icon(icon: impl Into<String>) -> Self {
        Self {
            kind: SegmentKind::Addon,
            icon: Some(icon.into()),
            tone: SegmentTone::Default,
            ..default()
        }
    }

    pub fn value(mut self, value: impl Into<String>) -> Self {
        self.value = value.into();
        self
    }

    pub fn label(mut self, label: impl Into<String>) -> Self {
        self.label = Some(label.into());
        self
    }

    pub fn icon_color(mut self, color: Color) -> Self {
        self.icon_color = Some(color);
        self
    }

    pub fn text_color(mut self, color: Color) -> Self {
        self.text_color = Some(color);
        self
    }

    pub fn tone(mut self, tone: SegmentTone) -> Self {
        self.tone = tone;
        self
    }

    pub fn recessed(self) -> Self {
        self.tone(SegmentTone::Recessed)
    }

    pub fn accent(self) -> Self {
        self.tone(SegmentTone::Accent)
    }

    pub fn danger(self) -> Self {
        self.tone(SegmentTone::Danger)
    }

    pub fn muted(self) -> Self {
        self.tone(SegmentTone::Muted)
    }

    /// Alias for [`Segment::muted`].
    pub fn disabled(self) -> Self {
        self.muted()
    }

    pub fn selected(mut self, selected: bool) -> Self {
        self.selected = selected;
        self
    }

    /// Attaches a compact pill counter or badge inside the segment (e.g. `"12"` or `"NEW"`).
    pub fn badge(mut self, badge: impl Into<String>) -> Self {
        self.badge = Some(badge.into());
        self
    }

    /// Removes the hairline divider placed before this segment.
    pub fn no_divider(mut self) -> Self {
        self.divider_before = false;
        self
    }

    /// Inserts an empty space (in px) before this segment so it is not attached to the previous segment.
    pub fn gap_before(mut self, gap: f32) -> Self {
        self.gap_before = Some(gap.max(0.0));
        self.divider_before = false;
        self
    }

    /// Forces a fixed segment width.
    pub fn width(mut self, width: f32) -> Self {
        self.width = Some(width);
        self
    }

    /// Overrides the horizontal padding inside this specific segment.
    pub fn padding_x(mut self, padding_x: f32) -> Self {
        self.padding_x = Some(padding_x);
        self
    }

    /// Explicitly sets rounded corners (in px) on this specific segment.
    pub fn radius(mut self, radius: f32) -> Self {
        self.radius = Some(radius.max(0.0));
        self
    }
}

// ============================================================================
// Components & events
// ============================================================================

/// Selection behaviour of a group.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Default, Reflect)]
pub enum GroupSelection {
    /// Plain action buttons.
    #[default]
    None,
    /// Exactly one segment is active (accent filled) at a time.
    Single,
    /// Any number of segments can be toggled active independently (e.g. Bold / Italic / Underline).
    Multiple,
}

/// Root component of a button group container.
#[derive(Component, Clone, Debug, Default, Reflect)]
pub struct LumaButtonGroup {
    pub selection: GroupSelection,
}

/// Component attached to every interactive segment cell.
#[derive(Component, Clone, Debug, Default, Reflect)]
pub struct LumaGroupSegment {
    pub index: usize,
    pub value: String,
    pub selected: bool,
    pub interactive: bool,
    pub base_tone: SegmentTone,
    pub menu_kind: SegmentMenuKind,
    pub menu_options: Vec<String>,
    pub is_menu_open: bool,
    pub overlay_entity: Option<Entity>,
}

/// Marker for segment text / icon glyphs that follow the segment tone color.
#[derive(Component, Clone, Debug, Default, Reflect)]
pub struct LumaGroupSegmentGlyph;

/// Marker for the text label entity inside a segment (can be updated on select).
#[derive(Component, Clone, Debug, Default, Reflect)]
pub struct LumaGroupSegmentLabel;

/// Marker for the chevron icon inside a select / dropdown segment (flips on open/close).
#[derive(Component, Clone, Copy, Debug, Default, Reflect)]
pub struct LumaGroupSegmentChevron {
    pub is_chevron: bool,
}

/// Marker for the independent floating overlay root entity of a button group dropdown.
#[derive(Component, Clone, Debug, Reflect)]
pub struct LumaGroupOverlay {
    pub segment_entity: Entity,
}

impl Default for LumaGroupOverlay {
    fn default() -> Self {
        Self {
            segment_entity: Entity::PLACEHOLDER,
        }
    }
}

/// Fullscreen transparent barrier backdrop capturing outside clicks to dismiss the group dropdown menu.
#[derive(Component, Clone, Debug, Reflect)]
pub struct LumaGroupBackdrop {
    pub segment_entity: Entity,
}

impl Default for LumaGroupBackdrop {
    fn default() -> Self {
        Self {
            segment_entity: Entity::PLACEHOLDER,
        }
    }
}

/// Component attached to each option item row inside a group dropdown menu.
#[derive(Component, Clone, Debug, Reflect)]
pub struct LumaGroupMenuItem {
    pub segment_entity: Entity,
    pub option_text: String,
    pub index: usize,
}

impl Default for LumaGroupMenuItem {
    fn default() -> Self {
        Self {
            segment_entity: Entity::PLACEHOLDER,
            option_text: String::new(),
            index: 0,
        }
    }
}

/// Triggered when a segment inside a [`ButtonGroup`] is clicked or activated.
#[derive(Event, Clone, Debug)]
pub struct ButtonGroupClicked {
    pub group: Entity,
    pub segment: Entity,
    pub index: usize,
    pub value: String,
    pub selected: bool,
}

/// Resource holding the palette used when re-coloring selections at runtime.
#[derive(Resource, Clone, Copy, Debug, Default)]
pub struct ButtonGroupTheme {
    pub palette: ButtonGroupPalette,
}

// ============================================================================
// ButtonGroup widget
// ============================================================================

/// Joined or spaced segmented toolbar group.
#[derive(Clone, Debug, Reflect)]
pub struct ButtonGroup {
    pub segments: Vec<Segment>,
    pub tone: SegmentTone,
    pub selection: GroupSelection,
    /// Whether hairline dividers are shown between segments.
    /// Defaults to `None` (automatically shown when `segment_gap == 0.0` and hidden when `segment_gap > 0.0`).
    pub dividers: Option<bool>,
    /// Spacing (in px) between segments inside the group. Default `0.0` (joined).
    pub segment_gap: f32,
    /// Optional inner track padding (in px) around segments.
    /// When `None` and `segment_gap > 0.0`, segments are rendered as unattached buttons with empty space between them.
    /// When `Some(px)`, segments sit inside a bordered outer track container with `px` padding.
    pub inset: Option<f32>,
    /// Optional explicit corner radius for inner segments when spaced or inset.
    pub segment_radius: Option<f32>,
    /// Whether the outer container border is drawn (`true` by default).
    pub bordered: bool,
    /// When `true`, the outer container is transparent/borderless and each segment gets its own 1px border.
    pub detached: bool,
    /// Layout orientation (`false` = horizontal row, `true` = vertical column).
    pub vertical: bool,
    pub palette: ButtonGroupPalette,
    pub metrics: ButtonGroupMetrics,
    pub font: Handle<Font>,
    pub icon_font: Handle<Font>,
}

impl Default for ButtonGroup {
    fn default() -> Self {
        Self {
            segments: Vec::new(),
            tone: SegmentTone::Default,
            selection: GroupSelection::None,
            dividers: None,
            segment_gap: 0.0,
            inset: None,
            segment_radius: None,
            bordered: true,
            detached: false,
            vertical: false,
            palette: ButtonGroupPalette::default(),
            metrics: ButtonGroupMetrics::default(),
            font: Handle::default(),
            icon_font: Handle::default(),
        }
    }
}

impl ButtonGroup {
    pub fn new(segments: Vec<Segment>) -> Self {
        Self {
            segments,
            ..default()
        }
    }

    /// Whole group filled with the accent color.
    pub fn accent(mut self) -> Self {
        self.tone = SegmentTone::Accent;
        self
    }

    /// Whole group uses the destructive tone.
    pub fn danger(mut self) -> Self {
        self.tone = SegmentTone::Danger;
        self
    }

    /// Whole group uses the darker recessed surface (`#121214`).
    pub fn recessed(mut self) -> Self {
        self.tone = SegmentTone::Recessed;
        self
    }

    /// Single-choice toggle group (selected segment becomes accent).
    pub fn exclusive(mut self) -> Self {
        self.selection = GroupSelection::Single;
        self
    }

    /// Multi-choice toggle group (each segment toggles on/off independently when clicked).
    pub fn multi_select(mut self) -> Self {
        self.selection = GroupSelection::Multiple;
        self
    }

    /// Hide hairline dividers between segments.
    pub fn no_dividers(mut self) -> Self {
        self.dividers = Some(false);
        self
    }

    /// Explicitly control whether hairline dividers are rendered between segments.
    pub fn with_dividers(mut self, enabled: bool) -> Self {
        self.dividers = Some(enabled);
        self
    }

    /// Sets a gap (in px) between segments (e.g. `.gap(4.0)`).
    /// Unless `.inset(...)` is also set, the segments are rendered as physically separated,
    /// unattached buttons with individual `1px` borders and rounded corners.
    pub fn gap(mut self, gap: f32) -> Self {
        self.segment_gap = gap.max(0.0);
        self
    }

    /// Wraps the segments inside an outer bordered track container with `inset` px of inner padding
    /// (e.g. `.recessed().gap(4.0).inset(3.0)`).
    pub fn inset(mut self, inset: f32) -> Self {
        self.inset = Some(inset.max(0.0));
        self
    }

    /// Overrides the corner radius of individual segments inside the group.
    pub fn segment_radius(mut self, radius: f32) -> Self {
        self.segment_radius = Some(radius.max(0.0));
        self
    }

    /// Overrides the outer corner radius of the group.
    pub fn radius(mut self, radius: f32) -> Self {
        self.metrics.radius = radius.max(0.0);
        self
    }

    /// Full capsule / pill shape (`radius = height / 2`).
    pub fn pill(mut self) -> Self {
        self.metrics.radius = (self.metrics.height * 0.5).ceil();
        self
    }

    /// Overrides the outer height of the group.
    pub fn height(mut self, height: f32) -> Self {
        self.metrics.height = height.max(18.0);
        self
    }

    /// Overrides the horizontal padding inside each segment.
    pub fn padding_x(mut self, padding_x: f32) -> Self {
        self.metrics.padding_x = padding_x.max(0.0);
        self
    }

    /// Overrides the inner gap between an icon and label inside a segment.
    pub fn content_gap(mut self, gap: f32) -> Self {
        self.metrics.gap = gap.max(0.0);
        self
    }

    /// Compact size preset (`28px` height, `6px` radius, `12.5px` text).
    pub fn compact(mut self) -> Self {
        self.metrics = ButtonGroupMetrics {
            height: 28.0,
            radius: 6.0,
            font_size: 12.5,
            icon_size: 13.5,
            padding_x: 9.0,
            gap: 5.5,
        };
        self
    }

    /// Default medium size preset (`34px` height, `8px` radius, `14px` text).
    pub fn medium(mut self) -> Self {
        self.metrics = ButtonGroupMetrics::default();
        self
    }

    /// Large size preset (`40px` height, `10px` radius, `15px` text).
    pub fn large(mut self) -> Self {
        self.metrics = ButtonGroupMetrics {
            height: 40.0,
            radius: 10.0,
            font_size: 15.0,
            icon_size: 16.5,
            padding_x: 15.0,
            gap: 8.0,
        };
        self
    }

    /// Removes the outer `1px` border around the group.
    pub fn borderless(mut self) -> Self {
        self.bordered = false;
        self
    }

    /// Detached mode: hides the outer container background/border and gives each
    /// segment its own `1px` border and rounded corners (ideal with `.gap(4.0)`).
    pub fn detached(mut self) -> Self {
        self.detached = true;
        if self.segment_gap == 0.0 {
            self.segment_gap = 4.0;
        }
        self
    }

    /// Arranges segments vertically in a column instead of a horizontal row.
    pub fn vertical(mut self) -> Self {
        self.vertical = true;
        self
    }

    pub fn palette(mut self, palette: ButtonGroupPalette) -> Self {
        self.palette = palette;
        self
    }

    pub fn metrics(mut self, metrics: ButtonGroupMetrics) -> Self {
        self.metrics = metrics;
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

/// Resolved colors of a segment for a given tone.
#[derive(Clone, Copy)]
struct ToneColors {
    bg: Color,
    hover: Color,
    pressed: Color,
    text: Color,
    icon: Color,
}

fn tone_colors(p: &ButtonGroupPalette, tone: SegmentTone, kind: &SegmentKind) -> ToneColors {
    match tone {
        SegmentTone::Accent => ToneColors {
            bg: p.accent,
            hover: p.accent_hover,
            pressed: p.accent_pressed,
            text: p.accent_text,
            icon: p.accent_text,
        },
        SegmentTone::Danger => ToneColors {
            bg: p.danger,
            hover: p.danger_hover,
            pressed: p.danger_pressed,
            text: p.danger_text,
            icon: p.danger_text,
        },
        SegmentTone::Muted => ToneColors {
            bg: p.muted_surface,
            hover: p.muted_surface,
            pressed: p.muted_surface,
            text: p.muted_text,
            icon: p.muted_text,
        },
        SegmentTone::Recessed => {
            let is_static = !matches!(kind, SegmentKind::Button);
            ToneColors {
                bg: p.input_surface,
                hover: if is_static { p.input_surface } else { p.surface_hover },
                pressed: if is_static { p.input_surface } else { p.surface_pressed },
                text: p.text,
                icon: p.icon,
            }
        }
        SegmentTone::Default | SegmentTone::Inherit => {
            let is_static = !matches!(kind, SegmentKind::Button);
            let bg = if matches!(kind, SegmentKind::Input { .. }) {
                p.input_surface
            } else {
                p.surface
            };
            ToneColors {
                bg,
                hover: if is_static { bg } else { p.surface_hover },
                pressed: if is_static { bg } else { p.surface_pressed },
                text: p.text,
                icon: p.icon,
            }
        }
    }
}

/// Internal cell scene (segment, divider, or spacer) so heterogeneous cells fit in one list.
enum GroupCell {
    Divider {
        color: Color,
        height: f32,
        vertical: bool,
    },
    Spacer {
        size: f32,
        vertical: bool,
    },
    Segment {
        segment: Segment,
        index: usize,
        base_tone: SegmentTone,
        colors: ToneColors,
        radius: UCornerRadius,
        border_width: f32,
        border_color: Color,
        height: f32,
        metrics: ButtonGroupMetrics,
        palette: ButtonGroupPalette,
        font: Handle<Font>,
        icon_font: Handle<Font>,
    },
}

impl Scene for GroupCell {
    fn resolve(
        self,
        context: &mut ResolveContext,
        scene: &mut ResolvedScene,
    ) -> Result<(), ResolveSceneError> {
        match self {
            GroupCell::Divider {
                color,
                height,
                vertical,
            } => {
                let width = if vertical {
                    UVal::Percent(1.0)
                } else {
                    UVal::Px(1.0)
                };
                let h = if vertical {
                    UVal::Px(1.0)
                } else {
                    UVal::Px(height)
                };
                let s = bsn! {
                    UNode {
                        width,
                        height: h,
                        background_color: color,
                    }
                };
                s.resolve(context, scene)
            }
            GroupCell::Spacer { size, vertical } => {
                let width = if vertical {
                    UVal::Px(1.0)
                } else {
                    UVal::Px(size)
                };
                let h = if vertical {
                    UVal::Px(size)
                } else {
                    UVal::Px(1.0)
                };
                let s = bsn! {
                    UNode {
                        width,
                        height: h,
                        background_color: Color::NONE,
                    }
                };
                s.resolve(context, scene)
            }
            GroupCell::Segment {
                segment,
                index,
                base_tone,
                colors,
                radius,
                border_width,
                border_color,
                height,
                metrics,
                palette,
                font,
                icon_font,
            } => match segment.kind.clone() {
                SegmentKind::Input {
                    placeholder,
                    value,
                    width,
                } => resolve_input_cell(
                    context,
                    scene,
                    placeholder,
                    value,
                    width,
                    colors,
                    radius,
                    border_width,
                    border_color,
                    height,
                    metrics,
                    palette,
                    font,
                ),
                kind => resolve_button_cell(
                    context,
                    scene,
                    segment,
                    kind,
                    index,
                    base_tone,
                    colors,
                    radius,
                    border_width,
                    border_color,
                    height,
                    metrics,
                    palette,
                    font,
                    icon_font,
                ),
            },
        }
    }
}

#[allow(clippy::too_many_arguments)]
fn resolve_button_cell(
    context: &mut ResolveContext,
    scene: &mut ResolvedScene,
    segment: Segment,
    kind: SegmentKind,
    index: usize,
    base_tone: SegmentTone,
    colors: ToneColors,
    radius: UCornerRadius,
    border_width: f32,
    border_color: Color,
    height: f32,
    metrics: ButtonGroupMetrics,
    palette: ButtonGroupPalette,
    font: Handle<Font>,
    icon_font: Handle<Font>,
) -> Result<(), ResolveSceneError> {
    let interactive = kind == SegmentKind::Button && segment.tone != SegmentTone::Muted;
    let has_label = segment.label.is_some();
    let has_icon = segment.icon.is_some();
    let has_trailing = segment.trailing_icon.is_some();
    let has_badge = segment.badge.is_some();
    let icon_only = has_icon && !has_label && !has_trailing && !has_badge;
    let is_input_icon = kind == SegmentKind::Addon && icon_only;
    let pad_x = segment.padding_x.unwrap_or(metrics.padding_x);

    let width = match segment.width {
        Some(w) => UVal::Px(w),
        None if icon_only && !is_input_icon => UVal::Px(height),
        None => UVal::Auto,
    };
    let padding = if is_input_icon {
        USides {
            left: pad_x,
            right: 0.0,
            top: 0.0,
            bottom: 0.0,
        }
    } else if icon_only {
        USides::all(0.0)
    } else {
        USides::axes(pad_x, 0.0)
    };

    let text_color = segment.text_color.unwrap_or(colors.text);
    let icon_color = segment.icon_color.unwrap_or(colors.icon);
    let label = segment.label.unwrap_or_default();
    let icon = segment.icon.unwrap_or_default();
    let trailing = segment.trailing_icon.unwrap_or_default();
    let badge_text = segment.badge.unwrap_or_default();
    let value = segment.value;
    let selected = segment.selected;
    let is_dropdown = segment.menu_kind == SegmentMenuKind::Dropdown;
    let is_select = segment.menu_kind == SegmentMenuKind::Select;
    let menu_kind = segment.menu_kind;
    let menu_options = segment.menu_options;
    let font_size = metrics.font_size;
    let badge_font_size = (metrics.font_size - 2.5).max(10.0);
    let badge_h = (height - 12.0).clamp(16.0, 20.0);
    let badge_bg = palette.input_surface;
    let badge_fg = palette.text;
    let icon_size = metrics.icon_size;
    let gap = metrics.gap;
    let bg = colors.bg;
    let hover = colors.hover;
    let pressed = colors.pressed;
    let font_badge = font.clone();
    let icon_font_2 = icon_font.clone();

    let s = bsn! {
        LumaGroupSegment {
            index,
            value,
            selected,
            interactive,
            base_tone,
            menu_kind,
            menu_options,
            is_menu_open: false,
            overlay_entity: None,
        }
        UNode {
            width,
            height: UVal::Px(height),
            padding,
            background_color: bg,
            border_radius: radius,
        }
        UBorder {
            color: border_color,
            width: border_width,
            radius,
        }
        UInteraction::default()
        UInteractionColors {
            normal: bg,
            hovered: hover,
            pressed,
        }
        ULayout {
            display: UDisplay::Flex,
            flex_direction: UFlexDirection::Row,
            justify_content: UJustifyContent::Center,
            align_items: UAlignItems::Center,
            gap,
        }
        Children [
            LumaGroupSegmentChevron { is_chevron: is_dropdown }
            UText {
                text: icon,
                font_size: icon_size,
                font: icon_font,
                color: icon_color,
            }
            ULayout {
                display: { if has_icon { UDisplay::Flex } else { UDisplay::None } },
            }
            --
            LumaGroupSegmentGlyph
            LumaGroupSegmentLabel
            UText {
                text: label,
                font_size,
                font,
                color: text_color,
            }
            ULayout {
                display: { if has_label { UDisplay::Flex } else { UDisplay::None } },
            }
            --
            UNode {
                height: UVal::Px(badge_h),
                padding: USides::axes(6.0, 0.0),
                background_color: badge_bg,
                border_radius: UCornerRadius::all(999.0),
            }
            ULayout {
                display: { if has_badge { UDisplay::Flex } else { UDisplay::None } },
                justify_content: UJustifyContent::Center,
                align_items: UAlignItems::Center,
            }
            Children [
                UText {
                    text: badge_text,
                    font_size: badge_font_size,
                    font: font_badge,
                    color: badge_fg,
                }
            ]
            --
            LumaGroupSegmentChevron { is_chevron: is_select }
            UText {
                text: trailing,
                font_size: icon_size,
                font: icon_font_2,
                color: icon_color,
            }
            ULayout {
                display: { if has_trailing { UDisplay::Flex } else { UDisplay::None } },
            }
        ]
    };
    s.resolve(context, scene)
}

#[allow(clippy::too_many_arguments)]
fn resolve_input_cell(
    context: &mut ResolveContext,
    scene: &mut ResolvedScene,
    placeholder: String,
    value: String,
    width: f32,
    colors: ToneColors,
    radius: UCornerRadius,
    border_width: f32,
    border_color: Color,
    height: f32,
    metrics: ButtonGroupMetrics,
    palette: ButtonGroupPalette,
    font: Handle<Font>,
) -> Result<(), ResolveSceneError> {
    // NOTE: luma's text input positions caret/selection with a fixed 14px left inset.
    let padding = USides {
        left: 14.0,
        right: 10.0,
        top: 0.0,
        bottom: 0.0,
    };
    let font_size = metrics.font_size;
    let line_h = font_size * 1.2;
    let caret_top = ((height - line_h) * 0.5).max(0.0);
    let initial_text = if value.is_empty() {
        placeholder.clone()
    } else {
        value.clone()
    };
    let initial_color = if value.is_empty() {
        palette.muted_text
    } else {
        palette.text
    };
    let bg = colors.bg;
    let text_color = palette.text;
    let placeholder_color = palette.muted_text;
    let caret_color = palette.caret;
    let selection_color = palette.selection;

    let s = bsn! {
        UTextInput {
            value,
            placeholder,
            placeholder_color,
            text_color,
            font_size,
            enabled: true,
        }
        UTextInputCaret {
            color: caret_color,
            width: 1.5,
            selection_color,
        }
        UNode {
            width: UVal::Px(width),
            height: UVal::Px(height),
            padding,
            background_color: bg,
            border_radius: radius,
        }
        UBorder {
            color: border_color,
            width: border_width,
            radius,
        }
        UFocusable::new()
        UInteraction::default()
        ULayout {
            display: UDisplay::Flex,
            flex_direction: UFlexDirection::Row,
            align_items: UAlignItems::Center,
        }
        Children [
            UTextInputSelectionMarker
            UNode {
                width: UVal::Px(0.0),
                height: UVal::Px(line_h),
                background_color: selection_color,
                border_radius: UCornerRadius::all(2.0),
            }
            USelf {
                position_type: UPositionType::Absolute,
                left: UVal::Px(14.0),
                top: UVal::Px(caret_top),
            }
            Visibility::Hidden
            --
            UTextInputTextMarker
            UText {
                text: initial_text,
                font_size,
                font,
                color: initial_color,
                linebreak: LineBreak::NoWrap,
                autosize: true,
                overflow: UTextOverflow::Clip,
            }
            --
            UTextInputCaretMarker
            UNode {
                width: UVal::Px(1.5),
                height: UVal::Px(line_h),
                background_color: caret_color,
            }
            USelf {
                position_type: UPositionType::Absolute,
                left: UVal::Px(14.0),
                top: UVal::Px(caret_top),
            }
            Visibility::Hidden
        ]
    };
    s.resolve(context, scene)
}

impl Scene for ButtonGroup {
    fn resolve(
        self,
        context: &mut ResolveContext,
        scene: &mut ResolvedScene,
    ) -> Result<(), ResolveSceneError> {
        let p = self.palette;
        let m = self.metrics;
        let is_detached = self.detached || (self.segment_gap > 0.0 && self.inset.is_none());
        let inset = if is_detached {
            0.0
        } else {
            self.inset.unwrap_or(1.0)
        };
        let inner_h = if is_detached {
            m.height
        } else {
            (m.height - inset * 2.0).max(16.0)
        };
        let outer_r = (m.radius - inset).max(0.0);
        // Inner corners between two segments remain sharp (0.0) unless `.segment_radius(...)` is explicitly specified.
        let inner_r = self.segment_radius.unwrap_or(0.0);
        let count = self.segments.len();
        let show_dividers = self
            .dividers
            .unwrap_or(!is_detached && self.segment_gap == 0.0);

        let (container_bg, border_color, divider_color) = match self.tone {
            SegmentTone::Accent => (p.accent, p.accent, p.accent_divider),
            SegmentTone::Danger => (p.danger, p.danger_border, p.danger_divider),
            SegmentTone::Recessed => (p.input_surface, p.border, p.divider),
            _ => (p.surface, p.border, p.divider),
        };

        let mut cells: Vec<GroupCell> = Vec::with_capacity(count * 2);
        for (index, mut segment) in self.segments.into_iter().enumerate() {
            if index > 0 {
                if let Some(gap_px) = segment.gap_before.filter(|&g| g > 0.0) {
                    cells.push(GroupCell::Spacer {
                        size: gap_px,
                        vertical: self.vertical,
                    });
                } else if show_dividers && segment.divider_before {
                    cells.push(GroupCell::Divider {
                        color: divider_color,
                        height: inner_h,
                        vertical: self.vertical,
                    });
                }
            }

            let base_tone = if segment.tone == SegmentTone::Inherit {
                if self.selection != GroupSelection::None && self.tone == SegmentTone::Accent {
                    SegmentTone::Default
                } else {
                    self.tone
                }
            } else {
                segment.tone
            };

            let tone = match self.selection {
                GroupSelection::Single | GroupSelection::Multiple => {
                    if segment.selected {
                        SegmentTone::Accent
                    } else {
                        base_tone
                    }
                }
                GroupSelection::None => base_tone,
            };
            let colors = tone_colors(&p, tone, &segment.kind);

            let radius = if let Some(seg_r) = segment.radius {
                UCornerRadius::all(seg_r)
            } else if self.vertical {
                let top = if index == 0 { outer_r } else { inner_r };
                let bottom = if index + 1 == count { outer_r } else { inner_r };
                UCornerRadius {
                    top_left: top,
                    top_right: top,
                    bottom_left: bottom,
                    bottom_right: bottom,
                }
            } else {
                let left = if index == 0 { outer_r } else { inner_r };
                let right = if index + 1 == count { outer_r } else { inner_r };
                UCornerRadius {
                    top_left: left,
                    bottom_left: left,
                    top_right: right,
                    bottom_right: right,
                }
            };

            // Keep static segments in the group's tone.
            if segment.tone == SegmentTone::Inherit {
                segment.tone = tone;
            }

            let seg_border_width = if is_detached { 1.0 } else { 0.0 };
            let seg_border_color = match tone {
                SegmentTone::Accent => p.accent,
                SegmentTone::Danger => p.danger_border,
                _ => p.border,
            };

            cells.push(GroupCell::Segment {
                segment,
                index,
                base_tone,
                colors,
                radius,
                border_width: seg_border_width,
                border_color: seg_border_color,
                height: inner_h,
                metrics: m,
                palette: p,
                font: self.font.clone(),
                icon_font: self.icon_font.clone(),
            });
        }

        let cells = SceneList::new(cells);
        let selection = self.selection;
        let radius = m.radius;
        let outer_height = if self.vertical {
            UVal::Auto
        } else {
            UVal::Px(m.height)
        };
        let outer_bg = if is_detached {
            Color::NONE
        } else {
            container_bg
        };
        let outer_border_width: f32 = if self.bordered && !is_detached {
            1.0
        } else {
            0.0
        };
        let direction = if self.vertical {
            UFlexDirection::Column
        } else {
            UFlexDirection::Row
        };
        let segment_gap = self.segment_gap;

        let s = bsn! {
            LumaButtonGroup {
                selection,
            }
            UNode {
                height: outer_height,
                padding: USides::all(inset),
                background_color: outer_bg,
                border_radius: UCornerRadius::all(radius),
            }
            UBorder {
                color: border_color,
                width: outer_border_width,
                radius: UCornerRadius::all(radius),
            }
            ULayout {
                display: UDisplay::Flex,
                flex_direction: direction,
                align_items: UAlignItems::Center,
                gap: segment_gap,
            }
            Children [
                {cells}
            ]
        };
        s.resolve(context, scene)
    }
}

// ============================================================================
// Interaction
// ============================================================================

fn find_segment(
    entity: Entity,
    segments: &Query<(Entity, &mut LumaGroupSegment, &Children, &ChildOf)>,
    parents: &Query<&ChildOf>,
) -> Option<Entity> {
    if segments.contains(entity) {
        return Some(entity);
    }
    parents.iter_ancestors(entity).find(|e| segments.contains(*e))
}

/// Dedicated scene for the floating dropdown overlay window of a button group segment.
struct GroupOverlayScene {
    segment_entity: Entity,
    menu_left: f32,
    menu_top: f32,
    menu_width: f32,
    menu_height: f32,
    item_height: f32,
    item_gap: f32,
    options: Vec<String>,
    current_value: String,
    font: Handle<Font>,
    icon_font: Handle<Font>,
    palette: ButtonGroupPalette,
}

impl Scene for GroupOverlayScene {
    fn resolve(
        self,
        context: &mut ResolveContext,
        scene: &mut ResolvedScene,
    ) -> Result<(), ResolveSceneError> {
        let segment_entity = self.segment_entity;
        let menu_left = self.menu_left;
        let menu_top = self.menu_top;
        let menu_width = self.menu_width;
        let menu_height = self.menu_height;
        let item_height = self.item_height;
        let item_gap = self.item_gap;
        let p = self.palette;
        let current_val = self.current_value;

        let cur_lower = current_val.to_lowercase();

        let item_rows: Vec<_> = self
            .options
            .into_iter()
            .enumerate()
            .map(|(idx, opt_text)| {
                let opt_lower = opt_text.to_lowercase();
                let is_sel = !cur_lower.is_empty()
                    && (cur_lower == opt_lower
                        || opt_lower.starts_with(&cur_lower)
                        || cur_lower.starts_with(&opt_lower));
                let item_bg = if is_sel {
                    Color::srgba(0.12, 0.84, 0.60, 0.18)
                } else {
                    Color::NONE
                };
                let item_color = if is_sel {
                    p.accent
                } else {
                    p.text
                };
                let opt_text_clone = opt_text.clone();
                let f = self.font.clone();
                let icon_f = self.icon_font.clone();
                let check_text = Icon::CHECK.to_string();

                bsn! {
                    UNode {
                        width: UVal::Percent(1.0),
                        height: UVal::Px(item_height),
                        padding: USides::axes(10.0, 6.0),
                        border_radius: UCornerRadius::all(6.0),
                        background_color: item_bg,
                    }
                    LumaGroupMenuItem {
                        segment_entity,
                        option_text: opt_text_clone,
                        index: idx,
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
                            text: opt_text,
                            font_size: 13.0,
                            font: f,
                            color: item_color,
                        }
                        --
                        UText {
                            text: check_text,
                            font_size: 13.0,
                            font: icon_f,
                            color: { p.accent },
                        }
                        ULayout {
                            display: { if is_sel { UDisplay::Flex } else { UDisplay::None } },
                        }
                    ]
                }
            })
            .collect();

        let options_scene = SceneList::new(item_rows);

        let s = bsn! {
            URootUi::screen()
            UZIndex::Global(9999)
            LumaGroupOverlay {
                segment_entity,
            }
            // Fullscreen invisible barrier backdrop to capture outside clicks
            UNode {
                width: UVal::Percent(1.0),
                height: UVal::Percent(1.0),
                background_color: Color::NONE,
            }
            LumaGroupBackdrop {
                segment_entity,
            }
            UInteraction::default()
            ULayout {
                display: UDisplay::Flex,
            }
            Children [
                // Floating Menu Card
                UNode {
                    width: UVal::Px(menu_width),
                    height: UVal::Auto,
                    max_height: menu_height,
                    background_color: { p.input_surface },
                    border_radius: UCornerRadius::all(8.0),
                    padding: USides::all(4.0),
                }
                USelf {
                    position_type: UPositionType::Absolute,
                    left: UVal::Px(menu_left),
                    top: UVal::Px(menu_top),
                    ..USelf::default()
                }
                UBorder {
                    color: { p.border },
                    width: 1.0,
                    radius: UCornerRadius::all(8.0),
                }
                UShadow::drop(Color::srgba(0.0, 0.0, 0.0, 0.65), Vec2::new(0.0, 8.0), 16.0)
                ULayout {
                    display: UDisplay::Flex,
                    flex_direction: UFlexDirection::Column,
                    gap: item_gap,
                }
                Children [
                    @{options_scene}
                ]
            ]
        };

        s.resolve(context, scene)
    }
}

#[allow(clippy::too_many_arguments)]
fn toggle_group_menu(
    target: Entity,
    commands: &mut Commands,
    theme: &ButtonGroupTheme,
    app_theme: &Theme,
    windows: &Query<&Window>,
    cameras: &Query<(&Camera, &GlobalTransform)>,
    trigger_transform_query: &Query<(&GlobalTransform, &ComputedSize)>,
    segments: &mut Query<(Entity, &mut LumaGroupSegment, &Children, &ChildOf)>,
    chevrons: &mut Query<(&LumaGroupSegmentChevron, &mut UText)>,
    overlays: &Query<(Entity, &LumaGroupOverlay)>,
) {
    let Ok((_, mut seg, children, _)) = segments.get_mut(target) else {
        return;
    };

    if seg.is_menu_open {
        seg.is_menu_open = false;
        if let Some(overlay) = seg.overlay_entity.take() {
            commands.entity(overlay).despawn();
        }
        for child in children.iter() {
            if let Ok((chev, mut text)) = chevrons.get_mut(child) {
                if chev.is_chevron {
                    text.text = Icon::CHEVRON_DOWN.to_string();
                }
            }
        }
        return;
    }

    // Close any other open group overlays first
    for (other_entity, mut other_seg, other_children, _) in segments.iter_mut() {
        if other_entity != target && other_seg.is_menu_open {
            other_seg.is_menu_open = false;
            if let Some(overlay) = other_seg.overlay_entity.take() {
                commands.entity(overlay).despawn();
            }
            for child in other_children.iter() {
                if let Ok((chev, mut text)) = chevrons.get_mut(child) {
                    if chev.is_chevron {
                        text.text = Icon::CHEVRON_DOWN.to_string();
                    }
                }
            }
        }
    }

    // Despawn any dangling overlays
    for (ov_entity, _) in overlays.iter() {
        commands.entity(ov_entity).despawn();
    }

    let Ok((_, mut seg, children, _)) = segments.get_mut(target) else {
        return;
    };

    let window = windows.iter().next();
    let (win_w, win_h) = window.map(|w| (w.width(), w.height())).unwrap_or((1280.0, 720.0));

    let (trigger_w, trigger_h, screen_x, screen_y) = if let Ok((gt, computed)) =
        trigger_transform_query.get(target)
    {
        let mut screen_pos = Vec2::new(
            (win_w * 0.5) + gt.translation().x,
            (win_h * 0.5) - gt.translation().y,
        );
        for (cam, cam_gt) in cameras.iter() {
            if let Ok(viewport_pos) = cam.world_to_viewport(cam_gt, gt.translation()) {
                screen_pos = viewport_pos;
                break;
            }
        }
        (computed.width, computed.height, screen_pos.x, screen_pos.y)
    } else {
        (120.0, 34.0, win_w * 0.5, win_h * 0.5)
    };

    let half_w = trigger_w * 0.5;
    let half_h = trigger_h * 0.5;
    let trigger_left = screen_x - half_w;
    let trigger_right = screen_x + half_w;
    let trigger_top = screen_y - half_h;
    let trigger_bottom = screen_y + half_h;

    let options = if seg.menu_options.is_empty() {
        if !seg.value.is_empty() && seg.value != "dropdown" {
            vec![seg.value.clone()]
        } else {
            vec!["Option 1".to_string()]
        }
    } else {
        seg.menu_options.clone()
    };

    let option_count = options.len();
    let item_height = 32.0;
    let item_gap = 2.0;
    let menu_padding = 8.0;
    let menu_height = if option_count > 0 {
        menu_padding
            + (option_count as f32) * item_height
            + ((option_count.saturating_sub(1)) as f32) * item_gap
    } else {
        menu_padding + item_height
    };

    let max_chars = options.iter().map(|s| s.chars().count()).max().unwrap_or(4);
    let content_w = (max_chars as f32 * 8.0 + 44.0).max(48.0);
    let is_dropdown = seg.menu_kind == SegmentMenuKind::Dropdown;
    let min_w = if is_dropdown && trigger_w < 60.0 {
        content_w.max(88.0)
    } else {
        content_w.max(trigger_w)
    };
    let menu_width = min_w.min(300.0);

    let space_below = win_h - trigger_bottom;
    let space_above = trigger_top;
    let place_above = space_below < (menu_height + 8.0) && space_above > space_below;
    let menu_top = if place_above {
        (trigger_top - menu_height - 4.0).max(4.0)
    } else {
        (trigger_bottom + 4.0).min((win_h - menu_height - 4.0).max(4.0))
    };

    let mut menu_left = if is_dropdown && trigger_w < 70.0 {
        trigger_right - menu_width
    } else {
        trigger_left
    };
    menu_left = menu_left.clamp(4.0, (win_w - menu_width - 4.0).max(4.0));

    let overlay_scene = GroupOverlayScene {
        segment_entity: target,
        menu_left,
        menu_top,
        menu_width,
        menu_height,
        item_height,
        item_gap,
        options,
        current_value: seg.value.clone(),
        font: app_theme.text.font.inter_regular.clone(),
        icon_font: app_theme.icon.font.clone(),
        palette: theme.palette,
    };

    let overlay_id = commands.spawn_scene(overlay_scene).id();
    seg.overlay_entity = Some(overlay_id);
    seg.is_menu_open = true;

    for child in children.iter() {
        if let Ok((chev, mut text)) = chevrons.get_mut(child) {
            if chev.is_chevron {
                text.text = Icon::CHEVRON_UP.to_string();
            }
        }
    }
}

#[allow(clippy::too_many_arguments)]
fn activate_segment(
    target: Entity,
    commands: &mut Commands,
    theme: &ButtonGroupTheme,
    groups: &Query<&LumaButtonGroup>,
    segments: &mut Query<(Entity, &mut LumaGroupSegment, &Children, &ChildOf)>,
    visuals: &mut Query<(
        &mut UNode,
        Option<&mut UInteractionColors>,
        Option<&mut UBorder>,
        &Children,
    )>,
    glyphs: &mut Query<&mut UText, With<LumaGroupSegmentGlyph>>,
) {
    let Ok((_, segment, _, child_of)) = segments.get(target) else {
        return;
    };
    if !segment.interactive {
        return;
    }
    let group_entity = child_of.parent();
    let index = segment.index;
    let value = segment.value.clone();
    let mut final_selected = segment.selected;

    let selection_mode = groups
        .get(group_entity)
        .map(|g| g.selection)
        .unwrap_or(GroupSelection::None);

    match selection_mode {
        GroupSelection::Single => {
            let p = theme.palette;
            let on = tone_colors(&p, SegmentTone::Accent, &SegmentKind::Button);

            for (entity, mut seg, _, parent) in segments.iter_mut() {
                if parent.parent() != group_entity || !seg.interactive {
                    continue;
                }
                seg.selected = entity == target;
                if entity == target {
                    final_selected = true;
                }
                let off = tone_colors(&p, seg.base_tone, &SegmentKind::Button);
                let c = if seg.selected { on } else { off };
                if let Ok((mut node, colors, border, children)) = visuals.get_mut(entity) {
                    node.background_color = c.bg;
                    if let Some(mut colors) = colors {
                        colors.normal = c.bg;
                        colors.hovered = c.hover;
                        colors.pressed = c.pressed;
                    }
                    if let Some(mut border) = border {
                        border.color = if seg.selected { p.accent } else { p.border };
                    }
                    for child in children.iter() {
                        if let Ok(mut text) = glyphs.get_mut(child) {
                            text.color = c.text;
                        }
                    }
                }
            }
        }
        GroupSelection::Multiple => {
            let p = theme.palette;
            let on = tone_colors(&p, SegmentTone::Accent, &SegmentKind::Button);
            if let Ok((entity, mut seg, _, _)) = segments.get_mut(target) {
                seg.selected = !seg.selected;
                final_selected = seg.selected;
                let off = tone_colors(&p, seg.base_tone, &SegmentKind::Button);
                let c = if seg.selected { on } else { off };
                if let Ok((mut node, colors, border, children)) = visuals.get_mut(entity) {
                    node.background_color = c.bg;
                    if let Some(mut colors) = colors {
                        colors.normal = c.bg;
                        colors.hovered = c.hover;
                        colors.pressed = c.pressed;
                    }
                    if let Some(mut border) = border {
                        border.color = if seg.selected { p.accent } else { p.border };
                    }
                    for child in children.iter() {
                        if let Ok(mut text) = glyphs.get_mut(child) {
                            text.color = c.text;
                        }
                    }
                }
            }
        }
        GroupSelection::None => {}
    }

    commands.trigger(ButtonGroupClicked {
        group: group_entity,
        segment: target,
        index,
        value,
        selected: final_selected,
    });
}

#[allow(clippy::too_many_arguments)]
fn on_group_segment_click(
    trigger: On<PointerClick>,
    mut commands: Commands,
    theme: Res<ButtonGroupTheme>,
    app_theme: Res<Theme>,
    windows: Query<&Window>,
    cameras: Query<(&Camera, &GlobalTransform)>,
    trigger_transform_query: Query<(&GlobalTransform, &ComputedSize)>,
    parents: Query<&ChildOf>,
    groups: Query<&LumaButtonGroup>,
    mut segments: Query<(Entity, &mut LumaGroupSegment, &Children, &ChildOf)>,
    mut visuals: Query<(
        &mut UNode,
        Option<&mut UInteractionColors>,
        Option<&mut UBorder>,
        &Children,
    )>,
    mut text_params: ParamSet<(
        Query<(&LumaGroupSegmentChevron, &mut UText)>,
        Query<&mut UText, With<LumaGroupSegmentGlyph>>,
    )>,
    overlays: Query<(Entity, &LumaGroupOverlay)>,
) {
    if trigger.event().button != PointerButton::Primary {
        return;
    }
    if let Some(target) = find_segment(trigger.entity, &segments, &parents) {
        let is_menu = segments
            .get(target)
            .map(|(_, seg, _, _)| seg.menu_kind != SegmentMenuKind::None || !seg.menu_options.is_empty())
            .unwrap_or(false);

        if is_menu {
            let mut chevrons = text_params.p0();
            toggle_group_menu(
                target,
                &mut commands,
                &theme,
                &app_theme,
                &windows,
                &cameras,
                &trigger_transform_query,
                &mut segments,
                &mut chevrons,
                &overlays,
            );
        } else {
            let mut glyphs = text_params.p1();
            activate_segment(
                target,
                &mut commands,
                &theme,
                &groups,
                &mut segments,
                &mut visuals,
                &mut glyphs,
            );
        }
    }
}

#[allow(clippy::too_many_arguments)]
fn on_group_segment_activate(
    trigger: On<UFocusActivate>,
    mut commands: Commands,
    theme: Res<ButtonGroupTheme>,
    app_theme: Res<Theme>,
    windows: Query<&Window>,
    cameras: Query<(&Camera, &GlobalTransform)>,
    trigger_transform_query: Query<(&GlobalTransform, &ComputedSize)>,
    parents: Query<&ChildOf>,
    groups: Query<&LumaButtonGroup>,
    mut segments: Query<(Entity, &mut LumaGroupSegment, &Children, &ChildOf)>,
    mut visuals: Query<(
        &mut UNode,
        Option<&mut UInteractionColors>,
        Option<&mut UBorder>,
        &Children,
    )>,
    mut text_params: ParamSet<(
        Query<(&LumaGroupSegmentChevron, &mut UText)>,
        Query<&mut UText, With<LumaGroupSegmentGlyph>>,
    )>,
    overlays: Query<(Entity, &LumaGroupOverlay)>,
) {
    if let Some(target) = find_segment(trigger.entity, &segments, &parents) {
        let is_menu = segments
            .get(target)
            .map(|(_, seg, _, _)| seg.menu_kind != SegmentMenuKind::None || !seg.menu_options.is_empty())
            .unwrap_or(false);

        if is_menu {
            let mut chevrons = text_params.p0();
            toggle_group_menu(
                target,
                &mut commands,
                &theme,
                &app_theme,
                &windows,
                &cameras,
                &trigger_transform_query,
                &mut segments,
                &mut chevrons,
                &overlays,
            );
        } else {
            let mut glyphs = text_params.p1();
            activate_segment(
                target,
                &mut commands,
                &theme,
                &groups,
                &mut segments,
                &mut visuals,
                &mut glyphs,
            );
        }
    }
}

#[allow(clippy::too_many_arguments)]
fn on_group_menu_item_click(
    trigger: On<PointerClick>,
    mut commands: Commands,
    menu_items: Query<&LumaGroupMenuItem>,
    mut segments: Query<(Entity, &mut LumaGroupSegment, &Children, &ChildOf)>,
    mut text_params: ParamSet<(
        Query<(&LumaGroupSegmentChevron, &mut UText)>,
        Query<&mut UText, With<LumaGroupSegmentLabel>>,
    )>,
    overlays: Query<(Entity, &LumaGroupOverlay)>,
) {
    if trigger.event().button != PointerButton::Primary {
        return;
    }
    let Ok(item) = menu_items.get(trigger.entity) else {
        return;
    };
    let target = item.segment_entity;
    let option_text = item.option_text.clone();

    let Ok((_, mut seg, children, child_of)) = segments.get_mut(target) else {
        return;
    };

    let group_entity = child_of.parent();
    let index = seg.index;
    seg.is_menu_open = false;

    if let Some(overlay) = seg.overlay_entity.take() {
        commands.entity(overlay).despawn();
    }
    for (ov_entity, ov) in overlays.iter() {
        if ov.segment_entity == target {
            commands.entity(ov_entity).despawn();
        }
    }

    {
        let mut chevrons = text_params.p0();
        for child in children.iter() {
            if let Ok((chev, mut text)) = chevrons.get_mut(child) {
                if chev.is_chevron {
                    text.text = Icon::CHEVRON_DOWN.to_string();
                }
            }
        }
    }

    if seg.menu_kind == SegmentMenuKind::Select {
        seg.value = option_text.clone();
        let mut labels = text_params.p1();
        for child in children.iter() {
            if let Ok(mut text) = labels.get_mut(child) {
                text.text = option_text.clone();
            }
        }
    }

    commands.trigger(ButtonGroupClicked {
        group: group_entity,
        segment: target,
        index,
        value: option_text,
        selected: true,
    });
}

fn on_group_backdrop_click(
    trigger: On<PointerClick>,
    mut commands: Commands,
    backdrops: Query<&LumaGroupBackdrop>,
    mut segments: Query<(&mut LumaGroupSegment, &Children)>,
    mut chevrons: Query<(&LumaGroupSegmentChevron, &mut UText)>,
    overlays: Query<(Entity, &LumaGroupOverlay)>,
) {
    if trigger.event().button != PointerButton::Primary {
        return;
    }
    let Ok(backdrop) = backdrops.get(trigger.entity) else {
        return;
    };
    let target = backdrop.segment_entity;

    if let Ok((mut seg, children)) = segments.get_mut(target) {
        seg.is_menu_open = false;
        if let Some(overlay) = seg.overlay_entity.take() {
            commands.entity(overlay).despawn();
        }
        for child in children.iter() {
            if let Ok((chev, mut text)) = chevrons.get_mut(child) {
                if chev.is_chevron {
                    text.text = Icon::CHEVRON_DOWN.to_string();
                }
            }
        }
    }

    for (ov_entity, ov) in overlays.iter() {
        if ov.segment_entity == target {
            commands.entity(ov_entity).despawn();
        }
    }
}

fn close_group_overlay_on_escape(
    mut commands: Commands,
    keyboard: Res<ButtonInput<KeyCode>>,
    mut segments: Query<(&mut LumaGroupSegment, &Children)>,
    mut chevrons: Query<(&LumaGroupSegmentChevron, &mut UText)>,
    overlays: Query<(Entity, &LumaGroupOverlay)>,
) {
    if !keyboard.just_pressed(KeyCode::Escape) {
        return;
    }

    for (ov_entity, ov) in overlays.iter() {
        commands.entity(ov_entity).despawn();
        if let Ok((mut seg, children)) = segments.get_mut(ov.segment_entity) {
            seg.is_menu_open = false;
            seg.overlay_entity = None;
            for child in children.iter() {
                if let Ok((chev, mut text)) = chevrons.get_mut(child) {
                    if chev.is_chevron {
                        text.text = Icon::CHEVRON_DOWN.to_string();
                    }
                }
            }
        }
    }
}

fn cleanup_orphaned_group_overlays(
    mut commands: Commands,
    overlays: Query<(Entity, &LumaGroupOverlay)>,
    segments: Query<&LumaGroupSegment>,
) {
    for (ov_entity, ov) in overlays.iter() {
        if !segments.contains(ov.segment_entity) {
            commands.entity(ov_entity).despawn();
        }
    }
}

/// Plugin registering [`ButtonGroup`] interaction.
pub struct LumaButtonGroupPlugin;

impl Plugin for LumaButtonGroupPlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<ButtonGroupTheme>()
            .register_type::<LumaButtonGroup>()
            .register_type::<LumaGroupSegment>()
            .register_type::<LumaGroupSegmentGlyph>()
            .register_type::<LumaGroupSegmentLabel>()
            .register_type::<LumaGroupSegmentChevron>()
            .register_type::<LumaGroupOverlay>()
            .register_type::<LumaGroupBackdrop>()
            .register_type::<LumaGroupMenuItem>()
            .add_systems(
                Update,
                (
                    close_group_overlay_on_escape,
                    cleanup_orphaned_group_overlays,
                ),
            )
            .add_observer(on_group_segment_click)
            .add_observer(on_group_segment_activate)
            .add_observer(on_group_backdrop_click)
            .add_observer(on_group_menu_item_click);
    }
}
