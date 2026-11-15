//! Design tokens, styles, and hierarchical theming for Luma widgets.

use bevy::prelude::*;
use bevy_luma::prelude::*;

/// Cohesive color palette for widget styling.
#[derive(Clone, Debug, PartialEq, Reflect)]
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
            background: Color::srgb_u8(0x0A, 0x0A, 0x0B),
            surface: Color::srgb_u8(0x18, 0x18, 0x1B),
            surface_hover: Color::srgb_u8(0x24, 0x24, 0x28),
            surface_border: Color::srgb_u8(0x2B, 0x2B, 0x30),

            primary: Color::srgb_u8(0x1F, 0xD6, 0x9A),
            primary_hover: Color::srgb_u8(0x3D, 0xE2, 0xAC),
            primary_pressed: Color::srgb_u8(0x17, 0xB8, 0x84),

            secondary: Color::srgb_u8(0x18, 0x18, 0x1B),
            secondary_hover: Color::srgb_u8(0x24, 0x24, 0x28),
            secondary_pressed: Color::srgb_u8(0x2C, 0x2C, 0x31),

            danger: Color::srgb_u8(0x3B, 0x12, 0x13),
            danger_hover: Color::srgb_u8(0x4A, 0x17, 0x18),
            danger_pressed: Color::srgb_u8(0x30, 0x0E, 0x0F),

            success: Color::srgb_u8(0x1F, 0xD6, 0x9A),
            warning: Color::srgb_u8(0xFA, 0xCC, 0x15),
            info: Color::srgb_u8(0x38, 0xBD, 0xF8),

            text_primary: Color::srgb_u8(0xEC, 0xEC, 0xEE),
            text_muted: Color::srgb_u8(0xA1, 0xA1, 0xAA),
            text_dim: Color::srgb_u8(0x74, 0x74, 0x7C),
        }
    }
}

impl WidgetColors {
    /// Foreground text/icon color paired with the primary accent surface.
    pub fn on_primary(&self) -> Color {
        Color::srgb_u8(0x06, 0x22, 0x18)
    }

    /// Standard focus outline color derived from theme accent.
    pub fn focus_ring(&self) -> Color {
        self.primary
    }

    /// Background fill used when interactive controls are disabled.
    pub fn disabled_bg(&self) -> Color {
        Color::srgba(0.11, 0.11, 0.13, 0.60)
    }

    /// Border color used when interactive controls are disabled.
    pub fn disabled_border(&self) -> Color {
        Color::srgba(0.20, 0.20, 0.23, 0.60)
    }

    /// Foreground color used when labels or icons are disabled.
    pub fn disabled_text(&self) -> Color {
        Color::srgba(0.45, 0.45, 0.50, 0.75)
    }

    /// Subtle translucent fill for ghost hover states.
    pub fn ghost_hover(&self) -> Color {
        Color::srgba(1.0, 1.0, 1.0, 0.06)
    }

    /// Subtle translucent fill for ghost pressed states.
    pub fn ghost_pressed(&self) -> Color {
        Color::srgba(1.0, 1.0, 1.0, 0.12)
    }
}

/// Standard corner radius tokens.
#[derive(Clone, Copy, Debug, PartialEq, Reflect)]
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

/// Comprehensive, customizable style definition for a Button widget.
#[derive(Clone, Debug, PartialEq, Reflect)]
pub struct ButtonStyle {
    pub bg_normal: Color,
    pub bg_hover: Color,
    pub bg_pressed: Color,
    pub border_color: Color,
    pub border_width: f32,
    pub text_color: Color,
    pub radius: f32,
    pub padding: USides,
    pub min_height: f32,
    pub font_size: f32,
}

impl Default for ButtonStyle {
    fn default() -> Self {
        Self::primary()
    }
}

impl ButtonStyle {
    pub fn primary() -> Self {
        Self {
            bg_normal: Color::srgb_u8(0x1F, 0xD6, 0x9A),
            bg_hover: Color::srgb_u8(0x3D, 0xE2, 0xAC),
            bg_pressed: Color::srgb_u8(0x17, 0xB8, 0x84),
            border_color: Color::srgb_u8(0x1F, 0xD6, 0x9A),
            border_width: 1.0,
            text_color: Color::srgb_u8(0x06, 0x22, 0x18),
            radius: 8.0,
            padding: USides::axes(14.0, 8.0),
            min_height: 34.0,
            font_size: 14.0,
        }
    }

    pub fn secondary() -> Self {
        Self {
            bg_normal: Color::srgb_u8(0x18, 0x18, 0x1B),
            bg_hover: Color::srgb_u8(0x24, 0x24, 0x28),
            bg_pressed: Color::srgb_u8(0x2C, 0x2C, 0x31),
            border_color: Color::srgb_u8(0x2B, 0x2B, 0x30),
            border_width: 1.0,
            text_color: Color::srgb_u8(0xEC, 0xEC, 0xEE),
            radius: 8.0,
            padding: USides::axes(14.0, 8.0),
            min_height: 34.0,
            font_size: 14.0,
        }
    }

    pub fn outline() -> Self {
        Self {
            bg_normal: Color::srgb_u8(0x12, 0x12, 0x14),
            bg_hover: Color::srgb_u8(0x24, 0x24, 0x28),
            bg_pressed: Color::srgb_u8(0x2C, 0x2C, 0x31),
            border_color: Color::srgb_u8(0x2B, 0x2B, 0x30),
            border_width: 1.0,
            text_color: Color::srgb_u8(0xEC, 0xEC, 0xEE),
            radius: 8.0,
            padding: USides::axes(14.0, 8.0),
            min_height: 34.0,
            font_size: 14.0,
        }
    }

    pub fn ghost() -> Self {
        Self {
            bg_normal: Color::NONE,
            bg_hover: Color::srgba(1.0, 1.0, 1.0, 0.06),
            bg_pressed: Color::srgba(1.0, 1.0, 1.0, 0.12),
            border_color: Color::NONE,
            border_width: 0.0,
            text_color: Color::srgb_u8(0xEC, 0xEC, 0xEE),
            radius: 8.0,
            padding: USides::axes(14.0, 8.0),
            min_height: 34.0,
            font_size: 14.0,
        }
    }

    pub fn danger() -> Self {
        Self {
            bg_normal: Color::srgb_u8(0x3B, 0x12, 0x13),
            bg_hover: Color::srgb_u8(0x4A, 0x17, 0x18),
            bg_pressed: Color::srgb_u8(0x30, 0x0E, 0x0F),
            border_color: Color::srgb_u8(0x52, 0x1C, 0x1D),
            border_width: 1.0,
            text_color: Color::srgb_u8(0xF4, 0x6B, 0x6B),
            radius: 8.0,
            padding: USides::axes(14.0, 8.0),
            min_height: 34.0,
            font_size: 14.0,
        }
    }

    pub fn fab() -> Self {
        Self {
            bg_normal: Color::srgb_u8(0x1F, 0xD6, 0x9A),
            bg_hover: Color::srgb_u8(0x3D, 0xE2, 0xAC),
            bg_pressed: Color::srgb_u8(0x17, 0xB8, 0x84),
            border_color: Color::srgb_u8(0x1F, 0xD6, 0x9A),
            border_width: 1.5,
            text_color: Color::srgb_u8(0x06, 0x22, 0x18),
            radius: 28.0,
            padding: USides::all(0.0),
            min_height: 56.0,
            font_size: 20.0,
        }
    }

    pub fn icon() -> Self {
        Self {
            bg_normal: Color::srgb_u8(0x18, 0x18, 0x1B),
            bg_hover: Color::srgb_u8(0x24, 0x24, 0x28),
            bg_pressed: Color::srgb_u8(0x2C, 0x2C, 0x31),
            border_color: Color::srgb_u8(0x2B, 0x2B, 0x30),
            border_width: 1.0,
            text_color: Color::srgb_u8(0xEC, 0xEC, 0xEE),
            radius: 8.0,
            padding: USides::all(0.0),
            min_height: 34.0,
            font_size: 15.0,
        }
    }

    pub fn icon_circle() -> Self {
        Self {
            radius: 19.0,
            ..Self::icon()
        }
    }

    pub fn pill() -> Self {
        Self {
            radius: 999.0,
            ..Self::primary()
        }
    }
}

/// Customizable style definition for a Toggle Switch widget.
#[derive(Clone, Debug, PartialEq, Reflect)]
pub struct ToggleStyle {
    pub checked_bg: Color,
    pub unchecked_bg: Color,
    pub checked_border: Color,
    pub unchecked_border: Color,
    pub knob_color: Color,
    pub track_width: f32,
    pub track_height: f32,
    pub knob_size: f32,
    pub radius: f32,
}

impl Default for ToggleStyle {
    fn default() -> Self {
        Self {
            checked_bg: Color::srgb_u8(0x1F, 0xD6, 0x9A),
            unchecked_bg: Color::srgb_u8(0x18, 0x18, 0x1B),
            checked_border: Color::srgb_u8(0x1F, 0xD6, 0x9A),
            unchecked_border: Color::srgb_u8(0x2B, 0x2B, 0x30),
            knob_color: Color::srgb_u8(0xEC, 0xEC, 0xEE),
            track_width: 44.0,
            track_height: 24.0,
            knob_size: 20.0,
            radius: 12.0,
        }
    }
}

/// Customizable style definition for an elevated Card container widget.
#[derive(Clone, Debug, PartialEq, Reflect)]
pub struct CardStyle {
    pub bg_color: Color,
    pub border_color: Color,
    pub border_width: f32,
    pub radius: f32,
    pub padding: f32,
}

impl Default for CardStyle {
    fn default() -> Self {
        Self {
            bg_color: Color::srgb_u8(0x18, 0x18, 0x1B),
            border_color: Color::srgb_u8(0x2B, 0x2B, 0x30),
            border_width: 1.0,
            radius: 12.0,
            padding: 16.0,
        }
    }
}

impl CardStyle {
    pub fn elevated() -> Self {
        Self::default()
    }

    pub fn outlined() -> Self {
        Self {
            bg_color: Color::NONE,
            border_color: Color::srgb_u8(0x2B, 0x2B, 0x30),
            border_width: 1.5,
            radius: 12.0,
            padding: 16.0,
        }
    }

    pub fn flat() -> Self {
        Self {
            bg_color: Color::srgb_u8(0x12, 0x12, 0x14),
            border_color: Color::NONE,
            border_width: 0.0,
            radius: 8.0,
            padding: 14.0,
        }
    }

    pub fn interactive() -> Self {
        Self {
            bg_color: Color::srgb_u8(0x18, 0x18, 0x1B),
            border_color: Color::srgb_u8(0x1F, 0xD6, 0x9A),
            border_width: 1.0,
            radius: 12.0,
            padding: 16.0,
        }
    }
}

/// Customizable style definition for a Badge widget.
#[derive(Clone, Debug, PartialEq, Reflect)]
pub struct BadgeStyle {
    pub bg_color: Color,
    pub border_color: Color,
    pub text_color: Color,
    pub radius: f32,
    pub padding: USides,
    pub font_size: f32,
}

impl Default for BadgeStyle {
    fn default() -> Self {
        Self::info()
    }
}

impl BadgeStyle {
    pub fn counter() -> Self {
        Self {
            bg_color: Color::srgb(0.85, 0.22, 0.28),
            border_color: Color::WHITE,
            text_color: Color::WHITE,
            radius: 999.0,
            padding: USides::axes(6.0, 2.0),
            font_size: 10.0,
        }
    }

    pub fn info() -> Self {
        Self {
            bg_color: Color::srgba(0.20, 0.50, 0.95, 0.16),
            border_color: Color::srgba(0.20, 0.50, 0.95, 0.35),
            text_color: Color::srgb(0.45, 0.75, 1.00),
            radius: 8.0,
            padding: USides::axes(8.0, 3.0),
            font_size: 11.0,
        }
    }

    pub fn success() -> Self {
        Self {
            bg_color: Color::srgba(0.16, 0.75, 0.45, 0.16),
            border_color: Color::srgba(0.16, 0.75, 0.45, 0.35),
            text_color: Color::srgb(0.35, 0.90, 0.55),
            radius: 8.0,
            padding: USides::axes(8.0, 3.0),
            font_size: 11.0,
        }
    }

    pub fn warning() -> Self {
        Self {
            bg_color: Color::srgba(0.95, 0.65, 0.15, 0.16),
            border_color: Color::srgba(0.95, 0.65, 0.15, 0.35),
            text_color: Color::srgb(1.00, 0.80, 0.35),
            radius: 8.0,
            padding: USides::axes(8.0, 3.0),
            font_size: 11.0,
        }
    }

    pub fn danger() -> Self {
        Self {
            bg_color: Color::srgba(0.85, 0.22, 0.28, 0.16),
            border_color: Color::srgba(0.85, 0.22, 0.28, 0.35),
            text_color: Color::srgb(1.00, 0.45, 0.50),
            radius: 8.0,
            padding: USides::axes(8.0, 3.0),
            font_size: 11.0,
        }
    }

    pub fn neutral() -> Self {
        Self {
            bg_color: Color::srgba(0.50, 0.55, 0.68, 0.16),
            border_color: Color::srgba(0.50, 0.55, 0.68, 0.35),
            text_color: Color::srgb(0.80, 0.85, 0.92),
            radius: 8.0,
            padding: USides::axes(8.0, 3.0),
            font_size: 11.0,
        }
    }
}

/// Customizable style definition for a Slider widget.
#[derive(Clone, Debug, PartialEq, Reflect)]
pub struct SliderStyle {
    pub track_bg: Color,
    pub fill_color: Color,
    pub thumb_color: Color,
    pub thumb_border: Color,
    pub track_height: f32,
    pub thumb_size: f32,
}

impl Default for SliderStyle {
    fn default() -> Self {
        Self {
            track_bg: Color::srgb_u8(0x18, 0x18, 0x1B),
            fill_color: Color::srgb_u8(0x1F, 0xD6, 0x9A),
            thumb_color: Color::srgb_u8(0xEC, 0xEC, 0xEE),
            thumb_border: Color::srgb_u8(0x1F, 0xD6, 0x9A),
            track_height: 6.0,
            thumb_size: 16.0,
        }
    }
}

/// Customizable style definition for a DragValue numeric scrubber widget.
#[derive(Clone, Debug, PartialEq, Reflect)]
pub struct DragValueStyle {
    pub bg_normal: Color,
    pub bg_hover: Color,
    pub bg_active: Color,
    pub border_color: Color,
    pub border_hover: Color,
    pub border_active: Color,
    pub border_width: f32,
    pub text_color: Color,
    pub accent_color: Color,
    pub fill_color: Color,
    pub radius: f32,
    pub height: f32,
    pub font_size: f32,
}

impl Default for DragValueStyle {
    fn default() -> Self {
        Self {
            bg_normal: Color::srgb_u8(0x12, 0x12, 0x14),
            bg_hover: Color::srgb_u8(0x18, 0x18, 0x1B),
            bg_active: Color::srgb_u8(0x12, 0x12, 0x14),
            border_color: Color::srgb_u8(0x2B, 0x2B, 0x30),
            border_hover: Color::srgb_u8(0x3F, 0x3F, 0x46),
            border_active: Color::srgb_u8(0x1F, 0xD6, 0x9A),
            border_width: 1.0,
            text_color: Color::srgb_u8(0xEC, 0xEC, 0xEE),
            accent_color: Color::srgb_u8(0x74, 0x74, 0x7C),
            fill_color: Color::srgba(0.12, 0.84, 0.60, 0.25),
            radius: 6.0,
            height: 28.0,
            font_size: 13.0,
        }
    }
}

/// Customizable style definition for a Text Input widget.
#[derive(Clone, Debug, PartialEq, Reflect)]
pub struct InputStyle {
    pub bg_normal: Color,
    pub bg_focus: Color,
    pub border_normal: Color,
    pub border_focus: Color,
    pub border_error: Color,
    pub border_success: Color,
    pub border_width: f32,
    pub text_color: Color,
    pub placeholder_color: Color,
    pub radius: f32,
    pub padding: USides,
    pub min_height: f32,
    pub font_size: f32,
}

impl Default for InputStyle {
    fn default() -> Self {
        Self::normal()
    }
}

impl InputStyle {
    pub fn normal() -> Self {
        Self {
            bg_normal: Color::srgb_u8(0x12, 0x12, 0x14),
            bg_focus: Color::srgb_u8(0x18, 0x18, 0x1B),
            border_normal: Color::srgb_u8(0x2B, 0x2B, 0x30),
            border_focus: Color::srgb_u8(0x1F, 0xD6, 0x9A),
            border_error: Color::srgb_u8(0xF4, 0x6B, 0x6B),
            border_success: Color::srgb_u8(0x1F, 0xD6, 0x9A),
            border_width: 1.0,
            text_color: Color::srgb_u8(0xEC, 0xEC, 0xEE),
            placeholder_color: Color::srgb_u8(0x74, 0x74, 0x7C),
            radius: 8.0,
            padding: USides::axes(12.0, 8.0),
            min_height: 34.0,
            font_size: 14.0,
        }
    }

    pub fn search() -> Self {
        Self {
            radius: 999.0,
            padding: USides::axes(18.0, 9.0),
            ..Self::normal()
        }
    }

    pub fn otp() -> Self {
        Self {
            radius: 8.0,
            padding: USides::all(0.0),
            min_height: 48.0,
            font_size: 20.0,
            ..Self::normal()
        }
    }

    pub fn multiline() -> Self {
        Self {
            radius: 8.0,
            padding: USides::all(12.0),
            min_height: 90.0,
            ..Self::normal()
        }
    }
}

/// Customizable style definition for a circular Spinner widget.
#[derive(Clone, Debug, PartialEq, Reflect)]
pub struct SpinnerStyle {
    pub color: Color,
    pub track_color: Color,
    pub size: f32,
    pub thickness: f32,
    pub speed: f32,
}

impl Default for SpinnerStyle {
    fn default() -> Self {
        Self::md()
    }
}

impl SpinnerStyle {
    pub fn sm() -> Self {
        Self {
            color: Color::srgb_u8(0x1F, 0xD6, 0x9A),
            track_color: Color::srgba(1.0, 1.0, 1.0, 0.12),
            size: 16.0,
            thickness: 2.0,
            speed: 6.0,
        }
    }

    pub fn md() -> Self {
        Self {
            color: Color::srgb_u8(0x1F, 0xD6, 0x9A),
            track_color: Color::srgba(1.0, 1.0, 1.0, 0.12),
            size: 24.0,
            thickness: 3.0,
            speed: 6.0,
        }
    }

    pub fn lg() -> Self {
        Self {
            color: Color::srgb_u8(0x1F, 0xD6, 0x9A),
            track_color: Color::srgba(1.0, 1.0, 1.0, 0.12),
            size: 36.0,
            thickness: 4.0,
            speed: 6.0,
        }
    }
}

/// Customizable style definition for a ListItem widget.
#[derive(Clone, Debug, PartialEq, Reflect)]
pub struct ListItemStyle {
    pub bg_normal: Color,
    pub bg_hover: Color,
    pub bg_pressed: Color,
    pub border_color: Color,
    pub border_width: f32,
    pub title_color: Color,
    pub subtitle_color: Color,
    pub radius: f32,
    pub padding: USides,
    pub min_height: f32,
    pub font_size: f32,
    pub subtitle_font_size: f32,
}

impl Default for ListItemStyle {
    fn default() -> Self {
        Self {
            bg_normal: Color::NONE,
            bg_hover: Color::srgb_u8(0x24, 0x24, 0x28),
            bg_pressed: Color::srgb_u8(0x2C, 0x2C, 0x31),
            border_color: Color::NONE,
            border_width: 0.0,
            title_color: Color::srgb_u8(0xEC, 0xEC, 0xEE),
            subtitle_color: Color::srgb_u8(0x74, 0x74, 0x7C),
            radius: 8.0,
            padding: USides::axes(12.0, 8.0),
            min_height: 40.0,
            font_size: 14.0,
            subtitle_font_size: 12.0,
        }
    }
}

/// Customizable style definition for a Select dropdown widget.
#[derive(Clone, Debug, PartialEq, Reflect)]
pub struct SelectStyle {
    pub bg_color: Color,
    pub bg_hover: Color,
    pub border_color: Color,
    pub border_focus: Color,
    pub border_width: f32,
    pub text_color: Color,
    pub radius: f32,
    pub padding: USides,
    pub min_height: f32,
    pub font_size: f32,
    pub menu_bg: Color,
    pub menu_border: Color,
}

impl Default for SelectStyle {
    fn default() -> Self {
        Self {
            bg_color: Color::srgb_u8(0x18, 0x18, 0x1B),
            bg_hover: Color::srgb_u8(0x24, 0x24, 0x28),
            border_color: Color::srgb_u8(0x2B, 0x2B, 0x30),
            border_focus: Color::srgb_u8(0x1F, 0xD6, 0x9A),
            border_width: 1.0,
            text_color: Color::srgb_u8(0xEC, 0xEC, 0xEE),
            radius: 8.0,
            padding: USides::axes(12.0, 8.0),
            min_height: 34.0,
            font_size: 14.0,
            menu_bg: Color::srgb_u8(0x18, 0x18, 0x1B),
            menu_border: Color::srgb_u8(0x2B, 0x2B, 0x30),
        }
    }
}

/// Customizable style definition for a Drawer/Sheet panel.
#[derive(Clone, Debug, PartialEq, Reflect)]
pub struct DrawerStyle {
    pub bg_color: Color,
    pub border_color: Color,
    pub border_width: f32,
    pub backdrop_color: Color,
    pub radius: f32,
    pub padding: USides,
}

impl Default for DrawerStyle {
    fn default() -> Self {
        Self {
            bg_color: Color::srgb_u8(0x18, 0x18, 0x1B),
            border_color: Color::srgb_u8(0x2B, 0x2B, 0x30),
            border_width: 1.0,
            backdrop_color: Color::srgba(0.0, 0.0, 0.0, 0.65),
            radius: 12.0,
            padding: USides::all(20.0),
        }
    }
}

/// Customizable style definition for a Popover overlay.
#[derive(Clone, Debug, PartialEq, Reflect)]
pub struct PopoverStyle {
    pub bg_color: Color,
    pub border_color: Color,
    pub border_width: f32,
    pub radius: f32,
    pub padding: USides,
}

impl Default for PopoverStyle {
    fn default() -> Self {
        Self {
            bg_color: Color::srgb_u8(0x18, 0x18, 0x1B),
            border_color: Color::srgb_u8(0x2B, 0x2B, 0x30),
            border_width: 1.0,
            radius: 10.0,
            padding: USides::all(12.0),
        }
    }
}

/// Customizable style definition for a Checkbox widget.
#[derive(Clone, Debug, PartialEq, Reflect)]
pub struct CheckboxStyle {
    pub checked_bg: Color,
    pub unchecked_bg: Color,
    pub checked_border: Color,
    pub unchecked_border: Color,
    pub hover_border: Color,
    pub check_color: Color,
    pub label_color: Color,
    pub size: f32,
    pub radius: f32,
    pub border_width: f32,
    pub font_size: f32,
}

impl Default for CheckboxStyle {
    fn default() -> Self {
        let c = WidgetColors::default();
        Self {
            checked_bg: c.primary,
            unchecked_bg: c.surface,
            checked_border: c.primary,
            unchecked_border: c.surface_border,
            hover_border: c.primary_hover,
            check_color: c.on_primary(),
            label_color: c.text_primary,
            size: 20.0,
            radius: 5.0,
            border_width: 1.5,
            font_size: 14.0,
        }
    }
}

/// Customizable style definition for a Radio button widget.
#[derive(Clone, Debug, PartialEq, Reflect)]
pub struct RadioStyle {
    pub selected_bg: Color,
    pub unselected_bg: Color,
    pub selected_border: Color,
    pub unselected_border: Color,
    pub hover_border: Color,
    pub dot_color: Color,
    pub label_color: Color,
    pub size: f32,
    pub dot_size: f32,
    pub border_width: f32,
    pub font_size: f32,
}

impl Default for RadioStyle {
    fn default() -> Self {
        let c = WidgetColors::default();
        Self {
            selected_bg: c.surface,
            unselected_bg: c.surface,
            selected_border: c.primary,
            unselected_border: c.surface_border,
            hover_border: c.primary_hover,
            dot_color: c.primary,
            label_color: c.text_primary,
            size: 20.0,
            dot_size: 10.0,
            border_width: 1.5,
            font_size: 14.0,
        }
    }
}

/// Customizable style definition for a Chip widget.
#[derive(Clone, Debug, PartialEq, Reflect)]
pub struct ChipStyle {
    pub bg_color: Color,
    pub selected_bg: Color,
    pub border_color: Color,
    pub selected_border: Color,
    pub text_color: Color,
    pub selected_text: Color,
    pub border_width: f32,
    pub radius: f32,
    pub padding: USides,
    pub font_size: f32,
}

impl Default for ChipStyle {
    fn default() -> Self {
        let c = WidgetColors::default();
        Self {
            bg_color: c.surface,
            selected_bg: Color::srgba(0.12, 0.84, 0.60, 0.20),
            border_color: c.surface_border,
            selected_border: c.primary,
            text_color: c.text_primary,
            selected_text: c.primary,
            border_width: 1.0,
            radius: WidgetRadii::default().full,
            padding: USides::axes(10.0, 5.0),
            font_size: 12.0,
        }
    }
}

/// Customizable style definition for a ProgressBar widget.
#[derive(Clone, Debug, PartialEq, Reflect)]
pub struct ProgressStyle {
    pub track_color: Color,
    pub fill_color: Color,
    pub border_color: Color,
    pub border_width: f32,
    pub radius: f32,
    pub height: f32,
}

impl Default for ProgressStyle {
    fn default() -> Self {
        let c = WidgetColors::default();
        Self {
            track_color: c.surface,
            fill_color: c.primary,
            border_color: c.surface_border,
            border_width: 0.0,
            radius: WidgetRadii::default().full,
            height: 8.0,
        }
    }
}

/// Customizable style definition for a numeric Stepper widget.
#[derive(Clone, Debug, PartialEq, Reflect)]
pub struct StepperStyle {
    pub bg_color: Color,
    pub button_bg: Color,
    pub button_hover: Color,
    pub button_pressed: Color,
    pub border_color: Color,
    pub text_color: Color,
    pub border_width: f32,
    pub radius: f32,
    pub height: f32,
    pub font_size: f32,
}

impl Default for StepperStyle {
    fn default() -> Self {
        let c = WidgetColors::default();
        Self {
            bg_color: c.surface,
            button_bg: c.surface_hover,
            button_hover: c.secondary_pressed,
            button_pressed: c.primary_pressed,
            border_color: c.surface_border,
            text_color: c.text_primary,
            border_width: 1.0,
            radius: WidgetRadii::default().md,
            height: 34.0,
            font_size: 14.0,
        }
    }
}

/// Customizable style definition for a Toast notification widget.
#[derive(Clone, Debug, PartialEq, Reflect)]
pub struct ToastStyle {
    pub bg_color: Color,
    pub border_color: Color,
    pub title_color: Color,
    pub message_color: Color,
    pub border_width: f32,
    pub radius: f32,
    pub padding: USides,
    pub max_width: f32,
}

impl Default for ToastStyle {
    fn default() -> Self {
        let c = WidgetColors::default();
        Self {
            bg_color: c.surface,
            border_color: c.surface_border,
            title_color: c.text_primary,
            message_color: c.text_muted,
            border_width: 1.0,
            radius: 10.0,
            padding: USides::axes(14.0, 12.0),
            max_width: 360.0,
        }
    }
}

/// Hierarchical style scope component attached to container nodes.
///
/// Any child widget inside a node with [`LumaStyleScope`] inherits these styles
/// unless the child widget defines its own explicit style.
#[derive(Component, Clone, Debug, Default, PartialEq, Reflect)]
pub struct LumaStyleScope {
    pub button: Option<ButtonStyle>,
    pub toggle: Option<ToggleStyle>,
    pub checkbox: Option<CheckboxStyle>,
    pub radio: Option<RadioStyle>,
    pub card: Option<CardStyle>,
    pub badge: Option<BadgeStyle>,
    pub chip: Option<ChipStyle>,
    pub slider: Option<SliderStyle>,
    pub progress: Option<ProgressStyle>,
    pub stepper: Option<StepperStyle>,
    pub toast: Option<ToastStyle>,
    pub input: Option<InputStyle>,
    pub spinner: Option<SpinnerStyle>,
    pub list_item: Option<ListItemStyle>,
    pub select: Option<SelectStyle>,
    pub drawer: Option<DrawerStyle>,
    pub popover: Option<PopoverStyle>,
    pub drag_value: Option<DragValueStyle>,
}

impl LumaStyleScope {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn button(mut self, style: ButtonStyle) -> Self {
        self.button = Some(style);
        self
    }

    pub fn toggle(mut self, style: ToggleStyle) -> Self {
        self.toggle = Some(style);
        self
    }

    pub fn checkbox(mut self, style: CheckboxStyle) -> Self {
        self.checkbox = Some(style);
        self
    }

    pub fn radio(mut self, style: RadioStyle) -> Self {
        self.radio = Some(style);
        self
    }

    pub fn card(mut self, style: CardStyle) -> Self {
        self.card = Some(style);
        self
    }

    pub fn badge(mut self, style: BadgeStyle) -> Self {
        self.badge = Some(style);
        self
    }

    pub fn chip(mut self, style: ChipStyle) -> Self {
        self.chip = Some(style);
        self
    }

    pub fn slider(mut self, style: SliderStyle) -> Self {
        self.slider = Some(style);
        self
    }

    pub fn progress(mut self, style: ProgressStyle) -> Self {
        self.progress = Some(style);
        self
    }

    pub fn stepper(mut self, style: StepperStyle) -> Self {
        self.stepper = Some(style);
        self
    }

    pub fn toast(mut self, style: ToastStyle) -> Self {
        self.toast = Some(style);
        self
    }

    pub fn input(mut self, style: InputStyle) -> Self {
        self.input = Some(style);
        self
    }

    pub fn spinner(mut self, style: SpinnerStyle) -> Self {
        self.spinner = Some(style);
        self
    }

    pub fn list_item(mut self, style: ListItemStyle) -> Self {
        self.list_item = Some(style);
        self
    }

    pub fn select(mut self, style: SelectStyle) -> Self {
        self.select = Some(style);
        self
    }

    pub fn drawer(mut self, style: DrawerStyle) -> Self {
        self.drawer = Some(style);
        self
    }

    pub fn popover(mut self, style: PopoverStyle) -> Self {
        self.popover = Some(style);
        self
    }

    pub fn drag_value(mut self, style: DragValueStyle) -> Self {
        self.drag_value = Some(style);
        self
    }
}

/// Global widget theme resource.
#[derive(Resource, Clone, Debug, Default, PartialEq, Reflect)]
pub struct LumaWidgetTheme {
    pub colors: WidgetColors,
    pub radii: WidgetRadii,
    pub button: ButtonStyle,
    pub toggle: ToggleStyle,
    pub checkbox: CheckboxStyle,
    pub radio: RadioStyle,
    pub card: CardStyle,
    pub badge: BadgeStyle,
    pub chip: ChipStyle,
    pub slider: SliderStyle,
    pub progress: ProgressStyle,
    pub stepper: StepperStyle,
    pub toast: ToastStyle,
    pub input: InputStyle,
    pub spinner: SpinnerStyle,
    pub list_item: ListItemStyle,
    pub select: SelectStyle,
    pub drawer: DrawerStyle,
    pub popover: PopoverStyle,
    pub drag_value: DragValueStyle,
}

/// Creates a container scene that provides an inherited style scope to all its children.
pub fn luma_style_scope(scope: LumaStyleScope, children: impl Scene) -> impl Scene {
    let LumaStyleScope {
        button,
        toggle,
        checkbox,
        radio,
        card,
        badge,
        chip,
        slider,
        progress,
        stepper,
        toast,
        input,
        spinner,
        list_item,
        select,
        drawer,
        popover,
        drag_value,
    } = scope;
    bsn! {
        LumaStyleScope {
            button,
            toggle,
            checkbox,
            radio,
            card,
            badge,
            chip,
            slider,
            progress,
            stepper,
            toast,
            input,
            spinner,
            list_item,
            select,
            drawer,
            popover,
            drag_value,
        }
        UNode::default()
        ULayout {
            display: UDisplay::Flex,
            flex_direction: UFlexDirection::Column,
            gap: 12.0,
        }
        Children [
            @{children}
        ]
    }
}

/// Resolves the effective style for a widget by searching nearest-to-furthest ancestor
/// [`LumaStyleScope`]s first, and falling back to [`LumaWidgetTheme`] if customized.
pub(crate) fn resolve_scoped_style<'a, S: Clone + 'a>(
    entity: Entity,
    parents_query: &Query<&ChildOf>,
    scopes_query: &'a Query<&LumaStyleScope>,
    pick_scope: impl Fn(&LumaStyleScope) -> Option<&S>,
    theme: Option<&'a LumaWidgetTheme>,
    pick_theme: impl Fn(&LumaWidgetTheme) -> &S,
    theme_customized: bool,
) -> Option<S> {
    if let Some(scoped) = parents_query
        .iter_ancestors(entity)
        .find_map(|ancestor| scopes_query.get(ancestor).ok().and_then(&pick_scope))
    {
        return Some(scoped.clone());
    }
    if theme_customized {
        return theme.map(pick_theme).cloned();
    }
    None
}

pub(crate) fn is_theme_customized(theme: Option<&Res<LumaWidgetTheme>>) -> (bool, bool) {
    let Some(t) = theme else {
        return (false, false);
    };
    let live_changed = t.is_changed() && !t.is_added();
    let non_default = **t != LumaWidgetTheme::default() || live_changed;
    (non_default, live_changed)
}

pub fn apply_luma_style_scope_to_buttons(
    parents_query: Query<&ChildOf>,
    scopes_query: Query<&LumaStyleScope>,
    changed_scopes: Query<(), Changed<LumaStyleScope>>,
    added_buttons: Query<(), Added<crate::button::LumaButton>>,
    theme: Option<Res<LumaWidgetTheme>>,
    mut button_query: Query<(
        Entity,
        Ref<crate::button::LumaButton>,
        &mut UNode,
        &mut UBorder,
        &mut UInteractionColors,
        &Children,
    )>,
    mut text_query: Query<(
        &mut UText,
        Option<&crate::button::LumaButtonLabel>,
        Option<&crate::button::LumaButtonIcon>,
    )>,
) {
    let (theme_customized, theme_live_changed) = is_theme_customized(theme.as_ref());
    let live_reapply = !changed_scopes.is_empty() || theme_live_changed;
    if added_buttons.is_empty() && !live_reapply {
        return;
    }

    let theme_ref = theme.as_deref();
    for (entity, button, mut node, mut border, mut colors, children) in button_query.iter_mut() {
        if button.has_explicit_style || !(live_reapply || button.is_added()) {
            continue;
        }
        let Some(style) = resolve_scoped_style(
            entity,
            &parents_query,
            &scopes_query,
            |s| s.button.as_ref(),
            theme_ref,
            |t| &t.button,
            theme_customized,
        ) else {
            continue;
        };

        node.border_radius = UCornerRadius::all(style.radius);
        border.radius = UCornerRadius::all(style.radius);
        border.width = style.border_width;
        if !button.is_icon_only {
            node.padding = style.padding;
        }
        if !button.has_explicit_size {
            node.min_height = style.min_height;
        }

        if button.variant == crate::button::ButtonVariant::Primary {
            let fg = if button.disabled {
                let disabled_bg = WidgetColors::default().disabled_bg();
                let disabled_border = WidgetColors::default().disabled_border();
                node.background_color = disabled_bg;
                border.color = disabled_border;
                colors.normal = disabled_bg;
                colors.hovered = disabled_bg;
                colors.pressed = disabled_bg;
                WidgetColors::default().disabled_text()
            } else if button.loading {
                node.background_color = style.bg_normal;
                border.color = style.border_color;
                colors.normal = style.bg_normal;
                colors.hovered = style.bg_normal;
                colors.pressed = style.bg_normal;
                style.text_color
            } else {
                node.background_color = style.bg_normal;
                border.color = style.border_color;
                colors.normal = style.bg_normal;
                colors.hovered = style.bg_hover;
                colors.pressed = style.bg_pressed;
                style.text_color
            };

            for child in children.iter() {
                if let Ok((mut text, label_marker, icon_marker)) = text_query.get_mut(child) {
                    if label_marker.is_some() || icon_marker.is_some() {
                        text.color = fg;
                        if label_marker.is_some() && !button.has_explicit_size {
                            text.font_size = style.font_size;
                        }
                    }
                }
            }
        } else if !button.has_explicit_size {
            for child in children.iter() {
                if let Ok((mut text, label_marker, _)) = text_query.get_mut(child) {
                    if label_marker.is_some() {
                        text.font_size = style.font_size;
                    }
                }
            }
        }
    }
}

pub fn apply_luma_style_scope_to_cards(
    parents_query: Query<&ChildOf>,
    scopes_query: Query<&LumaStyleScope>,
    changed_scopes: Query<(), Changed<LumaStyleScope>>,
    added_cards: Query<(), Added<crate::card::LumaCard>>,
    theme: Option<Res<LumaWidgetTheme>>,
    mut card_query: Query<(Entity, Ref<crate::card::LumaCard>, &mut UNode, &mut UBorder)>,
) {
    let (theme_customized, theme_live_changed) = is_theme_customized(theme.as_ref());
    let live_reapply = !changed_scopes.is_empty() || theme_live_changed;
    if added_cards.is_empty() && !live_reapply {
        return;
    }

    let theme_ref = theme.as_deref();
    for (entity, card, mut node, mut border) in card_query.iter_mut() {
        if card.has_explicit_style || !(live_reapply || card.is_added()) {
            continue;
        }
        let Some(style) = resolve_scoped_style(
            entity,
            &parents_query,
            &scopes_query,
            |s| s.card.as_ref(),
            theme_ref,
            |t| &t.card,
            theme_customized,
        ) else {
            continue;
        };

        if !card.has_explicit_bg {
            node.background_color = style.bg_color;
        }
        node.border_radius = UCornerRadius::all(style.radius);
        if !card.has_explicit_padding {
            node.padding = USides::all(style.padding);
        }
        if !card.has_explicit_border {
            border.color = style.border_color;
        }
        border.width = style.border_width;
        border.radius = UCornerRadius::all(style.radius);
    }
}

fn scope_apply_state(
    theme: Option<&Res<LumaWidgetTheme>>,
    added_empty: bool,
    changed_scopes_empty: bool,
) -> Option<(bool, bool)> {
    let (theme_customized, theme_live_changed) = is_theme_customized(theme);
    let live_reapply = !changed_scopes_empty || theme_live_changed;
    if added_empty && !live_reapply {
        return None;
    }
    Some((theme_customized, live_reapply))
}

pub fn apply_luma_style_scope_to_toggles(
    parents_query: Query<&ChildOf>,
    scopes_query: Query<&LumaStyleScope>,
    changed_scopes: Query<(), Changed<LumaStyleScope>>,
    theme: Option<Res<LumaWidgetTheme>>,
    mut toggle_params: ParamSet<(
        Query<(), Added<crate::toggle::LumaToggle>>,
        Query<(
            Entity,
            &mut crate::toggle::LumaToggle,
            &mut UNode,
            &mut UBorder,
            &mut UInteractionColors,
            Option<&mut UFocusVisual>,
            &Children,
        )>,
    )>,
    mut knob_query: Query<(&mut crate::toggle::LumaToggleKnob, &mut UNode, &mut USelf), Without<crate::toggle::LumaToggle>>,
) {
    let Some((theme_customized, live_reapply)) =
        scope_apply_state(theme.as_ref(), toggle_params.p0().is_empty(), changed_scopes.is_empty())
    else {
        return;
    };

    let theme_ref = theme.as_deref();
    let palette = WidgetColors::default();
    let mut toggle_query = toggle_params.p1();
    for (entity, mut toggle, mut node, mut border, mut i_colors, focus_visual, children) in
        toggle_query.iter_mut()
    {
        if toggle.has_explicit_style || !(live_reapply || toggle.is_added()) {
            continue;
        }
        let Some(style) = resolve_scoped_style(
            entity,
            &parents_query,
            &scopes_query,
            |s| s.toggle.as_ref(),
            theme_ref,
            |t| &t.toggle,
            theme_customized,
        ) else {
            continue;
        };

        if toggle.style != style {
            toggle.style = style.clone();
        }
        let (bg, border_color, colors) =
            crate::toggle::toggle_interaction_colors(&style, toggle.is_checked, toggle.disabled);
        node.width = UVal::Px(style.track_width);
        node.height = UVal::Px(style.track_height);
        node.background_color = bg;
        node.border_radius = UCornerRadius::all(style.radius);
        border.radius = UCornerRadius::all(style.radius);
        *i_colors = colors;
        crate::helpers::sync_focus_visual_border(focus_visual, &mut border, border_color);

        let travel = (style.track_width - style.knob_size - 4.0).max(0.0);
        let knob_offset = if toggle.is_checked { travel } else { 0.0 };
        let knob_top = (style.track_height - style.knob_size) * 0.5;
        let knob_color = if toggle.disabled {
            palette.disabled_text()
        } else {
            style.knob_color
        };

        for child in children.iter() {
            if let Ok((mut knob, mut knob_node, mut knob_self)) = knob_query.get_mut(child) {
                knob.travel_distance = travel;
                knob.current_offset = knob_offset;
                knob.target_offset = knob_offset;
                knob_node.width = UVal::Px(style.knob_size);
                knob_node.height = UVal::Px(style.knob_size);
                knob_node.background_color = knob_color;
                knob_node.border_radius = UCornerRadius::all(style.knob_size * 0.5);
                knob_self.left = UVal::Px(knob_offset);
                knob_self.top = UVal::Px(knob_top);
            }
        }
    }
}

pub fn apply_luma_style_scope_to_checkboxes(
    parents_query: Query<&ChildOf>,
    scopes_query: Query<&LumaStyleScope>,
    changed_scopes: Query<(), Changed<LumaStyleScope>>,
    theme: Option<Res<LumaWidgetTheme>>,
    mut checkbox_params: ParamSet<(
        Query<(), Added<crate::checkbox::LumaCheckbox>>,
        Query<(
            Entity,
            &mut crate::checkbox::LumaCheckbox,
            &mut UNode,
            &mut UBorder,
            &mut UInteractionColors,
            Option<&mut UFocusVisual>,
            &Children,
        )>,
    )>,
    mut mark_query: Query<&mut UText, With<crate::checkbox::LumaCheckmark>>,
) {
    let Some((theme_customized, live_reapply)) = scope_apply_state(
        theme.as_ref(),
        checkbox_params.p0().is_empty(),
        changed_scopes.is_empty(),
    ) else {
        return;
    };

    let theme_ref = theme.as_deref();
    let mut checkbox_query = checkbox_params.p1();
    for (entity, mut checkbox, mut node, mut border, mut i_colors, focus_visual, children) in
        checkbox_query.iter_mut()
    {
        if checkbox.has_explicit_style || !(live_reapply || checkbox.is_added()) {
            continue;
        }
        let Some(style) = resolve_scoped_style(
            entity,
            &parents_query,
            &scopes_query,
            |s| s.checkbox.as_ref(),
            theme_ref,
            |t| &t.checkbox,
            theme_customized,
        ) else {
            continue;
        };

        if checkbox.style != style {
            checkbox.style = style.clone();
        }
        let (bg, border_color, icon_color, colors) =
            crate::checkbox::checkbox_visual_state(&style, checkbox.is_checked, checkbox.disabled);
        node.width = UVal::Px(style.size);
        node.height = UVal::Px(style.size);
        node.background_color = bg;
        node.border_radius = UCornerRadius::all(style.radius);
        border.width = style.border_width;
        border.radius = UCornerRadius::all(style.radius);
        *i_colors = colors;
        crate::helpers::sync_focus_visual_border(focus_visual, &mut border, border_color);

        let icon_font_size = (style.size * 0.65).max(10.0);
        for child in children.iter() {
            if let Ok(mut text) = mark_query.get_mut(child) {
                text.color = icon_color;
                text.font_size = icon_font_size;
            }
        }
    }
}

pub fn apply_luma_style_scope_to_radios(
    parents_query: Query<&ChildOf>,
    scopes_query: Query<&LumaStyleScope>,
    changed_scopes: Query<(), Changed<LumaStyleScope>>,
    theme: Option<Res<LumaWidgetTheme>>,
    mut radio_params: ParamSet<(
        Query<(), Added<crate::radio::LumaRadio>>,
        Query<(
            Entity,
            &mut crate::radio::LumaRadio,
            &mut UNode,
            &mut UBorder,
            &mut UInteractionColors,
            Option<&mut UFocusVisual>,
            &Children,
        )>,
    )>,
    mut dot_query: Query<&mut UNode, (With<crate::radio::LumaRadioDot>, Without<crate::radio::LumaRadio>)>,
) {
    let Some((theme_customized, live_reapply)) =
        scope_apply_state(theme.as_ref(), radio_params.p0().is_empty(), changed_scopes.is_empty())
    else {
        return;
    };

    let theme_ref = theme.as_deref();
    let mut radio_query = radio_params.p1();
    for (entity, mut radio, mut node, mut border, mut i_colors, focus_visual, children) in
        radio_query.iter_mut()
    {
        if radio.has_explicit_style || !(live_reapply || radio.is_added()) {
            continue;
        }
        let Some(style) = resolve_scoped_style(
            entity,
            &parents_query,
            &scopes_query,
            |s| s.radio.as_ref(),
            theme_ref,
            |t| &t.radio,
            theme_customized,
        ) else {
            continue;
        };

        if radio.style != style {
            radio.style = style.clone();
        }
        let (bg, border_color, dot_color, colors) =
            crate::radio::radio_visual_state(&style, radio.selected, radio.disabled);
        let circle_radius = style.size * 0.5;
        let dot_radius = style.dot_size * 0.5;
        node.width = UVal::Px(style.size);
        node.height = UVal::Px(style.size);
        node.background_color = bg;
        node.border_radius = UCornerRadius::all(circle_radius);
        border.width = style.border_width;
        border.radius = UCornerRadius::all(circle_radius);
        *i_colors = colors;
        crate::helpers::sync_focus_visual_border(focus_visual, &mut border, border_color);

        for child in children.iter() {
            if let Ok(mut dot_node) = dot_query.get_mut(child) {
                dot_node.width = UVal::Px(style.dot_size);
                dot_node.height = UVal::Px(style.dot_size);
                dot_node.background_color = dot_color;
                dot_node.border_radius = UCornerRadius::all(dot_radius);
            }
        }
    }
}

pub fn apply_luma_style_scope_to_inputs(
    parents_query: Query<&ChildOf>,
    scopes_query: Query<&LumaStyleScope>,
    changed_scopes: Query<(), Changed<LumaStyleScope>>,
    theme: Option<Res<LumaWidgetTheme>>,
    children_query: Query<&Children>,
    mut input_params: ParamSet<(
        Query<(), Added<crate::input::LumaInput>>,
        Query<(
            Entity,
            &mut crate::input::LumaInput,
            &mut UNode,
            &mut UBorder,
            &mut UTextInput,
            Option<&UFocused>,
            &Children,
        )>,
    )>,
    mut text_query: Query<&mut UText, With<crate::input::LumaInputText>>,
) {
    let Some((theme_customized, live_reapply)) =
        scope_apply_state(theme.as_ref(), input_params.p0().is_empty(), changed_scopes.is_empty())
    else {
        return;
    };

    let theme_ref = theme.as_deref();
    let colors = WidgetColors::default();
    let mut input_query = input_params.p1();
    for (entity, mut input, mut node, mut border, mut text_input, focused, children) in
        input_query.iter_mut()
    {
        if input.has_explicit_style || !(live_reapply || input.is_added()) {
            continue;
        }
        let Some(style) = resolve_scoped_style(
            entity,
            &parents_query,
            &scopes_query,
            |s| s.input.as_ref(),
            theme_ref,
            |t| &t.input,
            theme_customized,
        ) else {
            continue;
        };

        if input.style != style {
            input.style = style.clone();
        }
        let is_disabled = input.state == crate::input::InputState::Disabled;
        let is_focused = focused.is_some() || input.state == crate::input::InputState::Focused;

        node.min_height = style.min_height;
        node.padding = style.padding;
        node.border_radius = UCornerRadius::all(style.radius);
        node.background_color = if is_disabled {
            colors.disabled_bg()
        } else if is_focused {
            style.bg_focus
        } else {
            style.bg_normal
        };

        border.width = style.border_width;
        border.radius = UCornerRadius::all(style.radius);
        border.color = match &input.state {
            crate::input::InputState::Disabled => colors.disabled_border(),
            crate::input::InputState::Error(_) => style.border_error,
            crate::input::InputState::Success => style.border_success,
            crate::input::InputState::Focused => style.border_focus,
            crate::input::InputState::Normal => {
                if is_focused {
                    style.border_focus
                } else {
                    style.border_normal
                }
            }
        };

        let input_text_color = if is_disabled {
            colors.disabled_text()
        } else {
            style.text_color
        };
        let placeholder_color = if is_disabled {
            colors.disabled_text()
        } else {
            style.placeholder_color
        };
        let display_color = if is_disabled {
            colors.disabled_text()
        } else if text_input.value.is_empty() {
            style.placeholder_color
        } else {
            style.text_color
        };

        text_input.text_color = input_text_color;
        text_input.placeholder_color = placeholder_color;
        if !input.has_explicit_font_size {
            text_input.font_size = style.font_size;
        }

        for &child in children {
            crate::helpers::visit_descendants(child, &children_query, |e| {
                if let Ok(mut text) = text_query.get_mut(e) {
                    text.color = display_color;
                    if !input.has_explicit_font_size {
                        text.font_size = style.font_size;
                    }
                }
                crate::helpers::DescendantVisit::Continue
            });
        }
    }
}

pub fn apply_luma_style_scope_to_sliders(
    parents_query: Query<&ChildOf>,
    scopes_query: Query<&LumaStyleScope>,
    changed_scopes: Query<(), Changed<LumaStyleScope>>,
    theme: Option<Res<LumaWidgetTheme>>,
    mut slider_params: ParamSet<(
        Query<(), Added<crate::slider::LumaSlider>>,
        Query<(Entity, &mut crate::slider::LumaSlider, &Children)>,
    )>,
    mut track_query: Query<
        &mut UNode,
        (
            With<crate::slider::LumaSliderTrack>,
            Without<crate::slider::LumaSliderFill>,
            Without<crate::slider::LumaSliderThumb>,
        ),
    >,
    mut fill_query: Query<
        &mut UNode,
        (
            With<crate::slider::LumaSliderFill>,
            Without<crate::slider::LumaSliderTrack>,
            Without<crate::slider::LumaSliderThumb>,
        ),
    >,
    mut thumb_query: Query<
        (&mut UNode, &mut UBorder, &mut USelf),
        (
            With<crate::slider::LumaSliderThumb>,
            Without<crate::slider::LumaSliderTrack>,
            Without<crate::slider::LumaSliderFill>,
        ),
    >,
) {
    let Some((theme_customized, live_reapply)) = scope_apply_state(
        theme.as_ref(),
        slider_params.p0().is_empty(),
        changed_scopes.is_empty(),
    ) else {
        return;
    };

    let theme_ref = theme.as_deref();
    let colors = WidgetColors::default();
    let mut slider_query = slider_params.p1();
    for (entity, mut slider, children) in slider_query.iter_mut() {
        if slider.has_explicit_style || !(live_reapply || slider.is_added()) {
            continue;
        }
        let Some(style) = resolve_scoped_style(
            entity,
            &parents_query,
            &scopes_query,
            |s| s.slider.as_ref(),
            theme_ref,
            |t| &t.slider,
            theme_customized,
        ) else {
            continue;
        };

        if slider.style != style {
            slider.style = style.clone();
        }
        let disabled = slider.disabled;
        let track_radius = UCornerRadius::all(style.track_height * 0.5);
        let thumb_radius = UCornerRadius::all(style.thumb_size * 0.5);
        let thumb_top = (24.0 - style.thumb_size) * 0.5;
        let ratio = slider.ratio();
        let fill_color = if disabled {
            crate::helpers::dim_color(style.fill_color, 0.35)
        } else {
            style.fill_color
        };

        for child in children.iter() {
            if let Ok(mut track_node) = track_query.get_mut(child) {
                track_node.height = UVal::Px(style.track_height);
                track_node.background_color = if disabled {
                    colors.disabled_bg()
                } else {
                    style.track_bg
                };
                track_node.border_radius = track_radius;
            }
            if let Ok(mut fill_node) = fill_query.get_mut(child) {
                fill_node.background_color = fill_color;
                fill_node.border_radius = track_radius;
            }
            if let Ok((mut thumb_node, mut thumb_border, mut thumb_self)) = thumb_query.get_mut(child)
            {
                thumb_node.width = UVal::Px(style.thumb_size);
                thumb_node.height = UVal::Px(style.thumb_size);
                thumb_node.background_color = if disabled {
                    colors.disabled_text()
                } else {
                    style.thumb_color
                };
                thumb_node.border_radius = thumb_radius;
                thumb_border.color = if disabled {
                    colors.disabled_border()
                } else {
                    style.thumb_border
                };
                thumb_border.radius = thumb_radius;
                thumb_self.top = UVal::Px(thumb_top);
                thumb_self.left = UVal::Px((slider.width * ratio) - (style.thumb_size * 0.5));
            }
        }
    }
}

pub fn apply_luma_style_scope_to_drag_values(
    parents_query: Query<&ChildOf>,
    scopes_query: Query<&LumaStyleScope>,
    changed_scopes: Query<(), Changed<LumaStyleScope>>,
    theme: Option<Res<LumaWidgetTheme>>,
    children_query: Query<&Children>,
    mut drag_params: ParamSet<(
        Query<(), Added<crate::drag_value::LumaDragValue>>,
        Query<(
            Entity,
            &mut crate::drag_value::LumaDragValue,
            &mut UNode,
            &mut UBorder,
            &Children,
        )>,
    )>,
    mut fill_query: Query<
        &mut UNode,
        (
            With<crate::drag_value::LumaDragValueFill>,
            Without<crate::drag_value::LumaDragValue>,
        ),
    >,
    mut label_query: Query<
        &mut UText,
        (
            With<crate::drag_value::LumaDragValueLabel>,
            Without<crate::drag_value::LumaDragValueText>,
        ),
    >,
    mut value_query: Query<
        &mut UText,
        (
            With<crate::drag_value::LumaDragValueText>,
            Without<crate::drag_value::LumaDragValueLabel>,
        ),
    >,
) {
    let Some((theme_customized, live_reapply)) =
        scope_apply_state(theme.as_ref(), drag_params.p0().is_empty(), changed_scopes.is_empty())
    else {
        return;
    };

    let theme_ref = theme.as_deref();
    let colors = WidgetColors::default();
    let mut drag_query = drag_params.p1();
    for (entity, mut drag_val, mut node, mut border, children) in drag_query.iter_mut() {
        if drag_val.has_explicit_style || !(live_reapply || drag_val.is_added()) {
            continue;
        }
        let Some(style) = resolve_scoped_style(
            entity,
            &parents_query,
            &scopes_query,
            |s| s.drag_value.as_ref(),
            theme_ref,
            |t| &t.drag_value,
            theme_customized,
        ) else {
            continue;
        };

        if drag_val.style != style {
            drag_val.style = style.clone();
        }
        let disabled = drag_val.disabled;
        node.height = UVal::Px(style.height);
        node.border_radius = UCornerRadius::all(style.radius);
        node.background_color = if disabled {
            colors.disabled_bg()
        } else {
            style.bg_normal
        };
        border.width = style.border_width;
        border.radius = UCornerRadius::all(style.radius);
        border.color = if disabled {
            colors.disabled_border()
        } else {
            style.border_color
        };

        let fill_color = if disabled {
            crate::helpers::dim_color(style.fill_color, 0.35)
        } else {
            style.fill_color
        };
        let label_color = if disabled {
            colors.disabled_text()
        } else {
            style.accent_color
        };
        let text_color = if disabled {
            colors.disabled_text()
        } else {
            style.text_color
        };
        let label_size = (style.font_size - 2.0).max(10.0);

        for &child in children {
            crate::helpers::visit_descendants(child, &children_query, |e| {
                if let Ok(mut fill_node) = fill_query.get_mut(e) {
                    fill_node.background_color = fill_color;
                }
                if let Ok(mut text) = label_query.get_mut(e) {
                    text.color = label_color;
                    text.font_size = label_size;
                }
                if let Ok(mut text) = value_query.get_mut(e) {
                    text.color = text_color;
                    text.font_size = style.font_size;
                }
                crate::helpers::DescendantVisit::Continue
            });
        }
    }
}

pub fn apply_luma_style_scope_to_badges(
    parents_query: Query<&ChildOf>,
    scopes_query: Query<&LumaStyleScope>,
    changed_scopes: Query<(), Changed<LumaStyleScope>>,
    added_badges: Query<(), Added<crate::badge::LumaBadge>>,
    theme: Option<Res<LumaWidgetTheme>>,
    children_query: Query<&Children>,
    mut badge_query: Query<(Entity, Ref<crate::badge::LumaBadge>, &mut UNode, &mut UBorder, &Children)>,
    mut text_query: Query<&mut UText>,
) {
    let Some((theme_customized, live_reapply)) =
        scope_apply_state(theme.as_ref(), added_badges.is_empty(), changed_scopes.is_empty())
    else {
        return;
    };

    let theme_ref = theme.as_deref();
    for (entity, badge, mut node, mut border, children) in badge_query.iter_mut() {
        if badge.has_explicit_style || !(live_reapply || badge.is_added()) {
            continue;
        }
        let Some(style) = resolve_scoped_style(
            entity,
            &parents_query,
            &scopes_query,
            |s| s.badge.as_ref(),
            theme_ref,
            |t| &t.badge,
            theme_customized,
        ) else {
            continue;
        };

        node.border_radius = UCornerRadius::all(style.radius);
        node.padding = style.padding;
        border.radius = UCornerRadius::all(style.radius);
        if badge.is_default_variant {
            node.background_color = style.bg_color;
            border.color = style.border_color;
        }

        for &child in children {
            crate::helpers::visit_descendants(child, &children_query, |e| {
                if let Ok(mut text) = text_query.get_mut(e) {
                    text.font_size = style.font_size;
                    if badge.is_default_variant {
                        text.color = style.text_color;
                    }
                }
                crate::helpers::DescendantVisit::Continue
            });
        }
    }
}

pub fn apply_luma_style_scope_to_spinners(
    parents_query: Query<&ChildOf>,
    scopes_query: Query<&LumaStyleScope>,
    changed_scopes: Query<(), Changed<LumaStyleScope>>,
    theme: Option<Res<LumaWidgetTheme>>,
    mut spinner_params: ParamSet<(
        Query<(), Added<crate::spinner::LumaSpinner>>,
        Query<(
            Entity,
            &mut crate::spinner::LumaSpinner,
            &mut UNode,
            &mut UBorder,
            &mut UGradient,
        )>,
    )>,
) {
    let Some((theme_customized, live_reapply)) = scope_apply_state(
        theme.as_ref(),
        spinner_params.p0().is_empty(),
        changed_scopes.is_empty(),
    ) else {
        return;
    };

    let theme_ref = theme.as_deref();
    let mut spinner_query = spinner_params.p1();
    for (entity, mut spinner, mut node, mut border, mut grad) in spinner_query.iter_mut() {
        if spinner.has_explicit_style || !(live_reapply || spinner.is_added()) {
            continue;
        }
        let Some(style) = resolve_scoped_style(
            entity,
            &parents_query,
            &scopes_query,
            |s| s.spinner.as_ref(),
            theme_ref,
            |t| &t.spinner,
            theme_customized,
        ) else {
            continue;
        };

        let radius = style.size * 0.5;
        spinner.speed = style.speed;
        node.width = UVal::Px(style.size);
        node.height = UVal::Px(style.size);
        node.border_radius = UCornerRadius::all(radius);
        border.width = style.thickness;
        border.radius = UCornerRadius::all(radius);
        *grad = UGradient::angular_stops(vec![
            UGradientStop::new(0.0, style.color),
            UGradientStop::new(0.30, style.track_color),
            UGradientStop::new(1.0, style.track_color),
        ])
        .stepped();
    }
}

pub fn apply_luma_style_scope_to_list_items(
    parents_query: Query<&ChildOf>,
    scopes_query: Query<&LumaStyleScope>,
    changed_scopes: Query<(), Changed<LumaStyleScope>>,
    theme: Option<Res<LumaWidgetTheme>>,
    children_query: Query<&Children>,
    mut item_params: ParamSet<(
        Query<(), Added<crate::list_item::LumaListItem>>,
        Query<(
            Entity,
            &mut crate::list_item::LumaListItem,
            &mut UNode,
            &mut UBorder,
            &mut UInteractionColors,
            &Children,
        )>,
    )>,
    mut title_query: Query<
        &mut UText,
        (
            With<crate::list_item::LumaListItemTitle>,
            Without<crate::list_item::LumaListItemSubtitle>,
        ),
    >,
    mut subtitle_query: Query<
        &mut UText,
        (
            With<crate::list_item::LumaListItemSubtitle>,
            Without<crate::list_item::LumaListItemTitle>,
        ),
    >,
) {
    let Some((theme_customized, live_reapply)) =
        scope_apply_state(theme.as_ref(), item_params.p0().is_empty(), changed_scopes.is_empty())
    else {
        return;
    };

    let theme_ref = theme.as_deref();
    let colors = WidgetColors::default();
    let mut item_query = item_params.p1();
    for (entity, mut item, mut node, mut border, mut i_colors, children) in item_query.iter_mut() {
        if item.has_explicit_style || !(live_reapply || item.is_added()) {
            continue;
        }
        let Some(style) = resolve_scoped_style(
            entity,
            &parents_query,
            &scopes_query,
            |s| s.list_item.as_ref(),
            theme_ref,
            |t| &t.list_item,
            theme_customized,
        ) else {
            continue;
        };

        if item.style != style {
            item.style = style.clone();
        }
        let disabled = item.disabled;
        let bg = if disabled {
            colors.disabled_bg()
        } else if item.is_selected {
            style.bg_hover
        } else {
            style.bg_normal
        };
        let border_color = if disabled {
            colors.disabled_border()
        } else {
            style.border_color
        };

        node.min_height = style.min_height;
        node.padding = style.padding;
        node.border_radius = UCornerRadius::all(style.radius);
        node.background_color = bg;
        border.color = border_color;
        border.width = style.border_width;
        border.radius = UCornerRadius::all(style.radius);
        *i_colors = crate::helpers::interaction_colors(
            bg,
            style.bg_hover,
            style.bg_pressed,
            disabled,
        );

        let title_color = if disabled {
            colors.disabled_text()
        } else {
            style.title_color
        };
        let subtitle_color = if disabled {
            colors.disabled_text()
        } else {
            style.subtitle_color
        };

        for &child in children {
            crate::helpers::visit_descendants(child, &children_query, |e| {
                if let Ok(mut text) = title_query.get_mut(e) {
                    text.font_size = style.font_size;
                    text.color = title_color;
                }
                if let Ok(mut text) = subtitle_query.get_mut(e) {
                    text.font_size = style.subtitle_font_size;
                    text.color = subtitle_color;
                }
                crate::helpers::DescendantVisit::Continue
            });
        }
    }
}

pub fn apply_luma_style_scope_to_selects(
    parents_query: Query<&ChildOf>,
    scopes_query: Query<&LumaStyleScope>,
    changed_scopes: Query<(), Changed<LumaStyleScope>>,
    theme: Option<Res<LumaWidgetTheme>>,
    children_query: Query<&Children>,
    mut select_params: ParamSet<(
        Query<(), Added<crate::select::LumaSelect>>,
        Query<(
            Entity,
            &mut crate::select::LumaSelect,
            &mut UNode,
            &mut UBorder,
            &mut UInteractionColors,
            Option<&mut UFocusVisual>,
            &Children,
        )>,
    )>,
    mut text_query: Query<
        &mut UText,
        (
            With<crate::select::LumaSelectText>,
            Without<crate::select::LumaSelectChevron>,
        ),
    >,
    mut chevron_query: Query<
        &mut UText,
        (
            With<crate::select::LumaSelectChevron>,
            Without<crate::select::LumaSelectText>,
        ),
    >,
) {
    let Some((theme_customized, live_reapply)) = scope_apply_state(
        theme.as_ref(),
        select_params.p0().is_empty(),
        changed_scopes.is_empty(),
    ) else {
        return;
    };

    let theme_ref = theme.as_deref();
    let colors = WidgetColors::default();
    let mut select_query = select_params.p1();
    for (entity, mut select, mut node, mut border, mut i_colors, focus_visual, children) in
        select_query.iter_mut()
    {
        if select.has_explicit_style || !(live_reapply || select.is_added()) {
            continue;
        }
        let Some(style) = resolve_scoped_style(
            entity,
            &parents_query,
            &scopes_query,
            |s| s.select.as_ref(),
            theme_ref,
            |t| &t.select,
            theme_customized,
        ) else {
            continue;
        };

        if select.style != style {
            select.style = style.clone();
        }
        let disabled = select.disabled;
        let bg = if disabled {
            colors.disabled_bg()
        } else {
            style.bg_color
        };
        let border_color = if disabled {
            colors.disabled_border()
        } else {
            style.border_color
        };

        node.min_height = style.min_height;
        node.padding = style.padding;
        node.border_radius = UCornerRadius::all(style.radius);
        node.background_color = bg;
        border.color = border_color;
        border.width = style.border_width;
        border.radius = UCornerRadius::all(style.radius);
        *i_colors =
            crate::helpers::interaction_colors(bg, style.bg_hover, style.bg_hover, disabled);
        if let Some(mut fv) = focus_visual {
            *fv = UFocusVisual::border(style.border_focus, 2.0);
        }

        let label_color = if disabled {
            colors.disabled_text()
        } else if select.selected_index.is_none() {
            colors.text_dim
        } else {
            style.text_color
        };
        let icon_color = if disabled {
            colors.disabled_text()
        } else {
            style.text_color
        };

        for &child in children {
            crate::helpers::visit_descendants(child, &children_query, |e| {
                if let Ok(mut text) = text_query.get_mut(e) {
                    text.font_size = style.font_size;
                    text.color = label_color;
                }
                if let Ok(mut text) = chevron_query.get_mut(e) {
                    text.color = icon_color;
                }
                crate::helpers::DescendantVisit::Continue
            });
        }
    }
}

pub fn apply_luma_style_scope_to_chips(
    parents_query: Query<&ChildOf>,
    scopes_query: Query<&LumaStyleScope>,
    changed_scopes: Query<(), Changed<LumaStyleScope>>,
    theme: Option<Res<LumaWidgetTheme>>,
    mut chip_params: ParamSet<(
        Query<(), Added<crate::chip::LumaChip>>,
        Query<(Entity, &mut crate::chip::LumaChip, &mut UNode, &mut UBorder, &Children)>,
    )>,
    mut label_query: Query<&mut UText, With<crate::chip::LumaChipLabel>>,
) {
    let Some((theme_customized, live_reapply)) =
        scope_apply_state(theme.as_ref(), chip_params.p0().is_empty(), changed_scopes.is_empty())
    else {
        return;
    };

    let theme_ref = theme.as_deref();
    let colors = WidgetColors::default();
    let mut chip_query = chip_params.p1();
    for (entity, mut chip, mut node, mut border, children) in chip_query.iter_mut() {
        if chip.has_explicit_style || !(live_reapply || chip.is_added()) {
            continue;
        }
        let Some(style) = resolve_scoped_style(
            entity,
            &parents_query,
            &scopes_query,
            |s| s.chip.as_ref(),
            theme_ref,
            |t| &t.chip,
            theme_customized,
        ) else {
            continue;
        };

        if chip.style != style {
            chip.style = style.clone();
        }
        let disabled = chip.disabled;
        node.padding = style.padding;
        node.border_radius = UCornerRadius::all(style.radius);
        node.background_color = if disabled {
            colors.disabled_bg()
        } else {
            style.bg_color
        };
        border.width = style.border_width;
        border.radius = UCornerRadius::all(style.radius);
        border.color = if disabled {
            colors.disabled_border()
        } else {
            style.border_color
        };

        let label_color = if disabled {
            colors.disabled_text()
        } else {
            style.text_color
        };
        for child in children.iter() {
            if let Ok(mut text) = label_query.get_mut(child) {
                text.font_size = style.font_size;
                text.color = label_color;
            }
        }
    }
}

pub fn apply_luma_style_scope_to_progress_bars(
    parents_query: Query<&ChildOf>,
    scopes_query: Query<&LumaStyleScope>,
    changed_scopes: Query<(), Changed<LumaStyleScope>>,
    theme: Option<Res<LumaWidgetTheme>>,
    mut bar_params: ParamSet<(
        Query<(), Added<crate::progress::LumaProgressBar>>,
        Query<(
            Entity,
            &mut crate::progress::LumaProgressBar,
            &mut UNode,
            &mut UBorder,
            &Children,
        )>,
    )>,
    mut fill_query: Query<
        &mut UNode,
        (
            With<crate::progress::LumaProgressFill>,
            Without<crate::progress::LumaProgressBar>,
        ),
    >,
) {
    let Some((theme_customized, live_reapply)) =
        scope_apply_state(theme.as_ref(), bar_params.p0().is_empty(), changed_scopes.is_empty())
    else {
        return;
    };

    let theme_ref = theme.as_deref();
    let mut bar_query = bar_params.p1();
    for (entity, mut bar, mut node, mut border, children) in bar_query.iter_mut() {
        if bar.has_explicit_style || !(live_reapply || bar.is_added()) {
            continue;
        }
        let Some(style) = resolve_scoped_style(
            entity,
            &parents_query,
            &scopes_query,
            |s| s.progress.as_ref(),
            theme_ref,
            |t| &t.progress,
            theme_customized,
        ) else {
            continue;
        };

        if bar.style != style {
            bar.style = style.clone();
        }
        node.background_color = style.track_color;
        node.border_radius = UCornerRadius::all(style.radius);
        if !bar.has_explicit_height {
            node.height = UVal::Px(style.height);
        }
        border.color = style.border_color;
        border.width = style.border_width;
        border.radius = UCornerRadius::all(style.radius);

        for child in children.iter() {
            if let Ok(mut fill_node) = fill_query.get_mut(child) {
                fill_node.border_radius = UCornerRadius::all(style.radius);
                if bar.is_default_variant {
                    fill_node.background_color = style.fill_color;
                }
            }
        }
    }
}

pub fn apply_luma_style_scope_to_steppers(
    parents_query: Query<&ChildOf>,
    scopes_query: Query<&LumaStyleScope>,
    changed_scopes: Query<(), Changed<LumaStyleScope>>,
    theme: Option<Res<LumaWidgetTheme>>,
    children_query: Query<&Children>,
    mut stepper_params: ParamSet<(
        Query<(), Added<crate::stepper::LumaStepper>>,
        Query<(
            Entity,
            &mut crate::stepper::LumaStepper,
            &mut UNode,
            &mut UBorder,
            &Children,
        )>,
    )>,
    mut btn_query: Query<
        (&mut UNode, &mut UInteractionColors),
        (
            With<crate::stepper::LumaStepperButton>,
            Without<crate::stepper::LumaStepper>,
        ),
    >,
    mut icon_query: Query<&mut UText, With<crate::stepper::LumaStepperIcon>>,
    value_box_query: Query<&Children, With<crate::stepper::LumaStepperValueText>>,
    mut plain_text_query: Query<&mut UText, Without<crate::stepper::LumaStepperIcon>>,
) {
    let Some((theme_customized, live_reapply)) = scope_apply_state(
        theme.as_ref(),
        stepper_params.p0().is_empty(),
        changed_scopes.is_empty(),
    ) else {
        return;
    };

    let theme_ref = theme.as_deref();
    let colors = WidgetColors::default();
    let mut stepper_query = stepper_params.p1();
    for (entity, mut stepper, mut node, mut border, children) in stepper_query.iter_mut() {
        if stepper.has_explicit_style || !(live_reapply || stepper.is_added()) {
            continue;
        }
        let Some(style) = resolve_scoped_style(
            entity,
            &parents_query,
            &scopes_query,
            |s| s.stepper.as_ref(),
            theme_ref,
            |t| &t.stepper,
            theme_customized,
        ) else {
            continue;
        };

        if stepper.style != style {
            stepper.style = style.clone();
        }
        let disabled = stepper.disabled;
        let btn_w = style.height.max(28.0);
        let btn_bg = if disabled {
            colors.disabled_bg()
        } else {
            style.button_bg
        };
        let btn_colors = crate::helpers::interaction_colors(
            btn_bg,
            style.button_hover,
            style.button_pressed,
            disabled,
        );
        let fg = if disabled {
            colors.disabled_text()
        } else {
            style.text_color
        };

        node.height = UVal::Px(style.height);
        node.border_radius = UCornerRadius::all(style.radius);
        node.background_color = if disabled {
            colors.disabled_bg()
        } else {
            style.bg_color
        };
        border.width = style.border_width;
        border.radius = UCornerRadius::all(style.radius);
        border.color = if disabled {
            colors.disabled_border()
        } else {
            style.border_color
        };

        for &child in children {
            if let Ok((mut btn_node, mut btn_i_colors)) = btn_query.get_mut(child) {
                btn_node.width = UVal::Px(btn_w);
                btn_node.background_color = btn_bg;
                *btn_i_colors = btn_colors.clone();
            }
            if let Ok(value_children) = value_box_query.get(child) {
                for &vc in value_children {
                    if let Ok(mut text) = plain_text_query.get_mut(vc) {
                        text.font_size = style.font_size;
                        text.color = fg;
                    }
                }
            }
            crate::helpers::visit_descendants(child, &children_query, |e| {
                if let Ok(mut text) = icon_query.get_mut(e) {
                    text.color = fg;
                }
                crate::helpers::DescendantVisit::Continue
            });
        }
    }
}

pub fn apply_luma_style_scope_to_toasts(
    parents_query: Query<&ChildOf>,
    scopes_query: Query<&LumaStyleScope>,
    changed_scopes: Query<(), Changed<LumaStyleScope>>,
    theme: Option<Res<LumaWidgetTheme>>,
    children_query: Query<&Children>,
    mut toast_params: ParamSet<(
        Query<(), Added<crate::toast::LumaToast>>,
        Query<(
            Entity,
            &mut crate::toast::LumaToast,
            &mut UNode,
            &mut UBorder,
            &Children,
        )>,
    )>,
    mut title_query: Query<
        &mut UText,
        (
            With<crate::toast::LumaToastTitle>,
            Without<crate::toast::LumaToastDescription>,
        ),
    >,
    mut desc_query: Query<
        &mut UText,
        (
            With<crate::toast::LumaToastDescription>,
            Without<crate::toast::LumaToastTitle>,
        ),
    >,
) {
    let Some((theme_customized, live_reapply)) =
        scope_apply_state(theme.as_ref(), toast_params.p0().is_empty(), changed_scopes.is_empty())
    else {
        return;
    };

    let theme_ref = theme.as_deref();
    let mut toast_query = toast_params.p1();
    for (entity, mut toast, mut node, mut border, children) in toast_query.iter_mut() {
        if toast.has_explicit_style || !(live_reapply || toast.is_added()) {
            continue;
        }
        let Some(style) = resolve_scoped_style(
            entity,
            &parents_query,
            &scopes_query,
            |s| s.toast.as_ref(),
            theme_ref,
            |t| &t.toast,
            theme_customized,
        ) else {
            continue;
        };

        if toast.style != style {
            toast.style = style.clone();
        }
        node.width = UVal::Px(style.max_width);
        node.padding = style.padding;
        node.border_radius = UCornerRadius::all(style.radius);
        border.width = style.border_width;
        border.radius = UCornerRadius::all(style.radius);
        if toast.is_default_variant {
            node.background_color = style.bg_color;
            border.color = style.border_color;
        }

        for &child in children {
            crate::helpers::visit_descendants(child, &children_query, |e| {
                if let Ok(mut text) = title_query.get_mut(e) {
                    text.color = style.title_color;
                }
                if let Ok(mut text) = desc_query.get_mut(e) {
                    text.color = style.message_color;
                }
                crate::helpers::DescendantVisit::Continue
            });
        }
    }
}

pub fn apply_luma_style_scope_to_drawers(
    parents_query: Query<&ChildOf>,
    scopes_query: Query<&LumaStyleScope>,
    changed_scopes: Query<(), Changed<LumaStyleScope>>,
    theme: Option<Res<LumaWidgetTheme>>,
    mut drawer_params: ParamSet<(
        Query<(), Added<crate::drawer::LumaDrawer>>,
        Query<(Entity, &mut crate::drawer::LumaDrawer, &mut UNode, &Children)>,
    )>,
    mut panel_query: Query<
        (&mut UNode, &mut UBorder),
        (
            With<crate::drawer::LumaDrawerPanel>,
            Without<crate::drawer::LumaDrawer>,
        ),
    >,
) {
    let Some((theme_customized, live_reapply)) = scope_apply_state(
        theme.as_ref(),
        drawer_params.p0().is_empty(),
        changed_scopes.is_empty(),
    ) else {
        return;
    };

    let theme_ref = theme.as_deref();
    let mut drawer_query = drawer_params.p1();
    for (entity, mut drawer, mut backdrop_node, children) in drawer_query.iter_mut() {
        if drawer.has_explicit_style || !(live_reapply || drawer.is_added()) {
            continue;
        }
        let Some(style) = resolve_scoped_style(
            entity,
            &parents_query,
            &scopes_query,
            |s| s.drawer.as_ref(),
            theme_ref,
            |t| &t.drawer,
            theme_customized,
        ) else {
            continue;
        };

        if drawer.style != style {
            drawer.style = style.clone();
        }
        backdrop_node.background_color = style.backdrop_color;
        for child in children.iter() {
            if let Ok((mut panel_node, mut panel_border)) = panel_query.get_mut(child) {
                panel_node.background_color = style.bg_color;
                panel_node.padding = style.padding;
                panel_node.border_radius = UCornerRadius::all(style.radius);
                panel_border.color = style.border_color;
                panel_border.width = style.border_width;
                panel_border.radius = UCornerRadius::all(style.radius);
            }
        }
    }
}

pub fn apply_luma_style_scope_to_popovers(
    parents_query: Query<&ChildOf>,
    scopes_query: Query<&LumaStyleScope>,
    changed_scopes: Query<(), Changed<LumaStyleScope>>,
    theme: Option<Res<LumaWidgetTheme>>,
    mut popover_params: ParamSet<(
        Query<(), Added<crate::popover::LumaPopover>>,
        Query<(Entity, &mut crate::popover::LumaPopover, &mut UNode, &mut UBorder)>,
    )>,
) {
    let Some((theme_customized, live_reapply)) = scope_apply_state(
        theme.as_ref(),
        popover_params.p0().is_empty(),
        changed_scopes.is_empty(),
    ) else {
        return;
    };

    let theme_ref = theme.as_deref();
    let mut popover_query = popover_params.p1();
    for (entity, mut popover, mut node, mut border) in popover_query.iter_mut() {
        if popover.has_explicit_style || !(live_reapply || popover.is_added()) {
            continue;
        }
        let Some(style) = resolve_scoped_style(
            entity,
            &parents_query,
            &scopes_query,
            |s| s.popover.as_ref(),
            theme_ref,
            |t| &t.popover,
            theme_customized,
        ) else {
            continue;
        };

        if popover.style != style {
            popover.style = style.clone();
        }
        node.background_color = style.bg_color;
        node.padding = style.padding;
        node.border_radius = UCornerRadius::all(style.radius);
        border.color = style.border_color;
        border.width = style.border_width;
        border.radius = UCornerRadius::all(style.radius);
    }
}

/// Plugin registering style scopes and default themes.
pub struct LumaThemePlugin;

impl Plugin for LumaThemePlugin {
    fn build(&self, app: &mut App) {
        app.register_type::<WidgetColors>()
            .register_type::<WidgetRadii>()
            .register_type::<ButtonStyle>()
            .register_type::<ToggleStyle>()
            .register_type::<CheckboxStyle>()
            .register_type::<RadioStyle>()
            .register_type::<CardStyle>()
            .register_type::<BadgeStyle>()
            .register_type::<ChipStyle>()
            .register_type::<SliderStyle>()
            .register_type::<ProgressStyle>()
            .register_type::<StepperStyle>()
            .register_type::<ToastStyle>()
            .register_type::<InputStyle>()
            .register_type::<SpinnerStyle>()
            .register_type::<ListItemStyle>()
            .register_type::<SelectStyle>()
            .register_type::<DrawerStyle>()
            .register_type::<PopoverStyle>()
            .register_type::<DragValueStyle>()
            .register_type::<LumaStyleScope>()
            .register_type::<LumaWidgetTheme>()
            .add_systems(
                Update,
                (
                    apply_luma_style_scope_to_buttons,
                    apply_luma_style_scope_to_cards,
                    apply_luma_style_scope_to_toggles,
                    apply_luma_style_scope_to_checkboxes,
                    apply_luma_style_scope_to_radios,
                    apply_luma_style_scope_to_inputs,
                    apply_luma_style_scope_to_sliders,
                    apply_luma_style_scope_to_drag_values,
                    apply_luma_style_scope_to_badges,
                ),
            )
            .add_systems(
                Update,
                (
                    apply_luma_style_scope_to_spinners,
                    apply_luma_style_scope_to_list_items,
                    apply_luma_style_scope_to_selects,
                    apply_luma_style_scope_to_chips,
                    apply_luma_style_scope_to_progress_bars,
                    apply_luma_style_scope_to_steppers,
                    apply_luma_style_scope_to_toasts,
                    apply_luma_style_scope_to_drawers,
                    apply_luma_style_scope_to_popovers,
                ),
            );
    }
}
