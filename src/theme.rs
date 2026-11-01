//! Design tokens, styles, and hierarchical theming for Luma widgets.

use bevy::prelude::*;
use bevy_luma::prelude::*;

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

/// Comprehensive, customizable style definition for a Button widget.
#[derive(Clone, Debug, Reflect)]
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
            bg_normal: Color::srgb(0.20, 0.45, 0.90),
            bg_hover: Color::srgb(0.28, 0.55, 1.00),
            bg_pressed: Color::srgb(0.14, 0.35, 0.75),
            border_color: Color::srgb(0.30, 0.55, 1.00),
            border_width: 1.0,
            text_color: Color::WHITE,
            radius: 8.0,
            padding: USides::axes(16.0, 9.0),
            min_height: 38.0,
            font_size: 14.0,
        }
    }

    pub fn secondary() -> Self {
        Self {
            bg_normal: Color::srgb(0.18, 0.20, 0.27),
            bg_hover: Color::srgb(0.24, 0.27, 0.36),
            bg_pressed: Color::srgb(0.13, 0.15, 0.20),
            border_color: Color::srgb(0.25, 0.28, 0.38),
            border_width: 1.0,
            text_color: Color::srgb(0.95, 0.96, 0.98),
            radius: 8.0,
            padding: USides::axes(16.0, 9.0),
            min_height: 38.0,
            font_size: 14.0,
        }
    }

    pub fn outline() -> Self {
        Self {
            bg_normal: Color::NONE,
            bg_hover: Color::srgba(1.0, 1.0, 1.0, 0.08),
            bg_pressed: Color::srgba(1.0, 1.0, 1.0, 0.14),
            border_color: Color::srgb(0.35, 0.40, 0.52),
            border_width: 1.0,
            text_color: Color::srgb(0.90, 0.92, 0.96),
            radius: 8.0,
            padding: USides::axes(16.0, 9.0),
            min_height: 38.0,
            font_size: 14.0,
        }
    }

    pub fn ghost() -> Self {
        Self {
            bg_normal: Color::NONE,
            bg_hover: Color::srgba(1.0, 1.0, 1.0, 0.08),
            bg_pressed: Color::srgba(1.0, 1.0, 1.0, 0.14),
            border_color: Color::NONE,
            border_width: 0.0,
            text_color: Color::srgb(0.90, 0.92, 0.96),
            radius: 8.0,
            padding: USides::axes(16.0, 9.0),
            min_height: 38.0,
            font_size: 14.0,
        }
    }

    pub fn danger() -> Self {
        Self {
            bg_normal: Color::srgb(0.85, 0.22, 0.28),
            bg_hover: Color::srgb(0.95, 0.30, 0.36),
            bg_pressed: Color::srgb(0.70, 0.16, 0.22),
            border_color: Color::srgb(0.95, 0.35, 0.40),
            border_width: 1.0,
            text_color: Color::WHITE,
            radius: 8.0,
            padding: USides::axes(16.0, 9.0),
            min_height: 38.0,
            font_size: 14.0,
        }
    }

    pub fn fab() -> Self {
        Self {
            bg_normal: Color::srgb(0.20, 0.45, 0.90),
            bg_hover: Color::srgb(0.28, 0.55, 1.00),
            bg_pressed: Color::srgb(0.14, 0.35, 0.75),
            border_color: Color::srgb(0.35, 0.60, 1.00),
            border_width: 1.5,
            text_color: Color::WHITE,
            radius: 28.0,
            padding: USides::all(0.0),
            min_height: 56.0,
            font_size: 20.0,
        }
    }

    pub fn icon() -> Self {
        Self {
            bg_normal: Color::srgb(0.18, 0.20, 0.27),
            bg_hover: Color::srgb(0.24, 0.27, 0.36),
            bg_pressed: Color::srgb(0.13, 0.15, 0.20),
            border_color: Color::srgb(0.25, 0.28, 0.38),
            border_width: 1.0,
            text_color: Color::srgb(0.95, 0.96, 0.98),
            radius: 8.0,
            padding: USides::all(0.0),
            min_height: 38.0,
            font_size: 16.0,
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
#[derive(Clone, Debug, Reflect)]
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
            checked_bg: Color::srgb(0.20, 0.45, 0.90),
            unchecked_bg: Color::srgb(0.18, 0.22, 0.28),
            checked_border: Color::srgb(0.28, 0.55, 1.00),
            unchecked_border: Color::srgb(0.28, 0.32, 0.40),
            knob_color: Color::WHITE,
            track_width: 44.0,
            track_height: 24.0,
            knob_size: 20.0,
            radius: 12.0,
        }
    }
}

/// Customizable style definition for an elevated Card container widget.
#[derive(Clone, Debug, Reflect)]
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
            bg_color: Color::srgb(0.11, 0.13, 0.18),
            border_color: Color::srgb(0.20, 0.24, 0.32),
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
            border_color: Color::srgb(0.24, 0.28, 0.38),
            border_width: 1.5,
            radius: 12.0,
            padding: 16.0,
        }
    }

    pub fn flat() -> Self {
        Self {
            bg_color: Color::srgb(0.09, 0.10, 0.14),
            border_color: Color::NONE,
            border_width: 0.0,
            radius: 8.0,
            padding: 14.0,
        }
    }

    pub fn interactive() -> Self {
        Self {
            bg_color: Color::srgb(0.12, 0.14, 0.20),
            border_color: Color::srgb(0.28, 0.45, 0.85),
            border_width: 1.0,
            radius: 12.0,
            padding: 16.0,
        }
    }
}

/// Customizable style definition for a Badge widget.
#[derive(Clone, Debug, Reflect)]
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
#[derive(Clone, Debug, Reflect)]
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
            track_bg: Color::srgb(0.18, 0.22, 0.28),
            fill_color: Color::srgb(0.20, 0.45, 0.90),
            thumb_color: Color::WHITE,
            thumb_border: Color::srgb(0.28, 0.55, 1.00),
            track_height: 6.0,
            thumb_size: 16.0,
        }
    }
}

/// Customizable style definition for a DragValue numeric scrubber widget.
#[derive(Clone, Debug, Reflect)]
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
            bg_normal: Color::srgb(0.14, 0.15, 0.18),
            bg_hover: Color::srgb(0.18, 0.20, 0.25),
            bg_active: Color::srgb(0.12, 0.14, 0.18),
            border_color: Color::srgb(0.22, 0.25, 0.30),
            border_hover: Color::srgb(0.32, 0.38, 0.48),
            border_active: Color::srgb(0.35, 0.65, 1.00),
            border_width: 1.0,
            text_color: Color::srgb(0.92, 0.94, 0.98),
            accent_color: Color::srgb(0.50, 0.55, 0.65),
            fill_color: Color::srgba(0.28, 0.45, 0.70, 0.35),
            radius: 5.0,
            height: 28.0,
            font_size: 13.0,
        }
    }
}

/// Customizable style definition for a Text Input widget.
#[derive(Clone, Debug, Reflect)]
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
            bg_normal: Color::srgb(0.10, 0.12, 0.16),
            bg_focus: Color::srgb(0.12, 0.14, 0.20),
            border_normal: Color::srgb(0.20, 0.24, 0.32),
            border_focus: Color::srgb(0.35, 0.65, 1.00),
            border_error: Color::srgb(0.85, 0.22, 0.28),
            border_success: Color::srgb(0.16, 0.75, 0.45),
            border_width: 1.0,
            text_color: Color::srgb(0.95, 0.96, 0.98),
            placeholder_color: Color::srgb(0.45, 0.50, 0.62),
            radius: 8.0,
            padding: USides::axes(14.0, 9.0),
            min_height: 38.0,
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
#[derive(Clone, Debug, Reflect)]
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
            color: Color::srgb(0.35, 0.65, 1.00),
            track_color: Color::srgba(1.0, 1.0, 1.0, 0.15),
            size: 16.0,
            thickness: 2.0,
            speed: 6.0,
        }
    }

    pub fn md() -> Self {
        Self {
            color: Color::srgb(0.35, 0.65, 1.00),
            track_color: Color::srgba(1.0, 1.0, 1.0, 0.15),
            size: 24.0,
            thickness: 3.0,
            speed: 6.0,
        }
    }

    pub fn lg() -> Self {
        Self {
            color: Color::srgb(0.35, 0.65, 1.00),
            track_color: Color::srgba(1.0, 1.0, 1.0, 0.15),
            size: 36.0,
            thickness: 4.0,
            speed: 6.0,
        }
    }
}

/// Customizable style definition for a ListItem widget.
#[derive(Clone, Debug, Reflect)]
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
            bg_hover: Color::srgba(1.0, 1.0, 1.0, 0.05),
            bg_pressed: Color::srgba(1.0, 1.0, 1.0, 0.09),
            border_color: Color::NONE,
            border_width: 0.0,
            title_color: Color::srgb(0.95, 0.96, 0.98),
            subtitle_color: Color::srgb(0.60, 0.65, 0.75),
            radius: 8.0,
            padding: USides::axes(12.0, 8.0),
            min_height: 44.0,
            font_size: 14.0,
            subtitle_font_size: 12.0,
        }
    }
}

/// Customizable style definition for a Select dropdown widget.
#[derive(Clone, Debug, Reflect)]
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
            bg_color: Color::srgb(0.10, 0.12, 0.16),
            bg_hover: Color::srgb(0.14, 0.17, 0.23),
            border_color: Color::srgb(0.20, 0.24, 0.32),
            border_focus: Color::srgb(0.35, 0.65, 1.00),
            border_width: 1.0,
            text_color: Color::srgb(0.95, 0.96, 0.98),
            radius: 8.0,
            padding: USides::axes(14.0, 9.0),
            min_height: 38.0,
            font_size: 14.0,
            menu_bg: Color::srgb(0.11, 0.13, 0.18),
            menu_border: Color::srgb(0.22, 0.26, 0.36),
        }
    }
}

/// Customizable style definition for a Drawer/Sheet panel.
#[derive(Clone, Debug, Reflect)]
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
            bg_color: Color::srgb(0.10, 0.12, 0.17),
            border_color: Color::srgb(0.20, 0.24, 0.34),
            border_width: 1.0,
            backdrop_color: Color::srgba(0.0, 0.0, 0.0, 0.6),
            radius: 12.0,
            padding: USides::all(20.0),
        }
    }
}

/// Customizable style definition for a Popover overlay.
#[derive(Clone, Debug, Reflect)]
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
            bg_color: Color::srgb(0.12, 0.14, 0.20),
            border_color: Color::srgb(0.24, 0.28, 0.38),
            border_width: 1.0,
            radius: 10.0,
            padding: USides::all(12.0),
        }
    }
}

/// Hierarchical style scope component attached to container nodes.
///
/// Any child widget inside a node with [`LumaStyleScope`] inherits these styles
/// unless the child widget defines its own explicit style.
#[derive(Component, Clone, Debug, Default, Reflect)]
pub struct LumaStyleScope {
    pub button: Option<ButtonStyle>,
    pub toggle: Option<ToggleStyle>,
    pub card: Option<CardStyle>,
    pub badge: Option<BadgeStyle>,
    pub slider: Option<SliderStyle>,
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

    pub fn card(mut self, style: CardStyle) -> Self {
        self.card = Some(style);
        self
    }

    pub fn badge(mut self, style: BadgeStyle) -> Self {
        self.badge = Some(style);
        self
    }

    pub fn slider(mut self, style: SliderStyle) -> Self {
        self.slider = Some(style);
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
#[derive(Resource, Clone, Debug, Default, Reflect)]
pub struct LumaWidgetTheme {
    pub colors: WidgetColors,
    pub radii: WidgetRadii,
    pub button: ButtonStyle,
    pub toggle: ToggleStyle,
    pub card: CardStyle,
    pub slider: SliderStyle,
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
        card,
        badge,
        slider,
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
            card,
            badge,
            slider,
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
