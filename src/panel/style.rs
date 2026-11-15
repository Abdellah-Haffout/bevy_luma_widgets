//! Visual styling, typography helpers, and viewport geometry utilities for the panel system.

use bevy::prelude::*;
use bevy::window::{CursorIcon, SystemCursorIcon};
use bevy_luma::prelude::*;

use crate::theme::LumaWidgetTheme;

use super::area::IconFontHandle;

/// Comprehensive theme and layout tokens for dock areas, tabs, split handles,
/// sidebars, drag previews, popups, and workspace strips.
#[derive(Resource, Clone, Debug, Reflect, PartialEq)]
#[reflect(Resource)]
pub struct DockStyle {
    /// Background color behind the root dock layout.
    pub canvas_bg: Color,
    /// Background fill for dock area containers and tab contents.
    pub area_bg: Color,
    /// Subtle hairline border color around dock areas.
    pub area_border: Color,
    /// Border thickness around dock areas.
    pub area_border_width: f32,
    /// Corner radius applied to dock areas.
    pub area_radius: f32,

    /// Interactive width/height (px) of splitter handles between panels.
    pub handle_thickness: f32,
    /// Idle background color of a splitter handle.
    pub handle_idle: Color,
    /// Hovered background color of a splitter handle.
    pub handle_hover: Color,
    /// Pressed background color of a splitter handle while dragging.
    pub handle_pressed: Color,
    /// Color of the subtle center grip pill inside splitter handles.
    pub handle_pill_color: Color,

    /// Height (px) of the header bar inside `DockAreaStyle::TabBar` panels.
    pub tab_bar_height: f32,
    /// Background color of the tab bar container.
    pub tab_bar_bg: Color,
    /// Bottom separator border color under the tab bar.
    pub tab_bar_border: Color,
    /// Background color for the currently selected tab.
    pub tab_bg_active: Color,
    /// Background color for inactive tabs.
    pub tab_bg_inactive: Color,
    /// Background color when hovering an inactive tab.
    pub tab_bg_hover: Color,
    /// Text color of the active tab label.
    pub tab_text_active: Color,
    /// Text color of inactive tab labels.
    pub tab_text_inactive: Color,
    /// Top/left active accent color on selected tabs and sidebar icons.
    pub accent_color: Color,
    /// Thickness (px) of the active tab indicator stripe.
    pub tab_accent_thickness: f32,
    /// Corner radius on individual tabs.
    pub tab_radius: f32,
    /// Horizontal gap (px) between tabs.
    pub tab_gap: f32,
    /// Font size (px) for tab titles.
    pub tab_font_size: f32,

    /// Icon tint color in idle state.
    pub icon_color: Color,
    /// Icon tint color in hovered/active states.
    pub icon_hover_color: Color,
    /// Hover background on action buttons (`+`, grip).
    pub action_hover_bg: Color,
    /// Hover background on tab close buttons (`X`).
    pub close_hover_bg: Color,
    /// Danger icon color when hovering a close button.
    pub close_hover_color: Color,

    /// Width (px) of the vertical sidebar in `DockAreaStyle::IconSidebar`.
    pub sidebar_width: f32,
    /// Background color of the icon sidebar strip.
    pub sidebar_bg: Color,
    /// Square size (px) of each sidebar icon button.
    pub sidebar_icon_size: f32,
    /// Active button background inside the icon sidebar.
    pub sidebar_icon_active_bg: Color,
    /// Hover button background inside the icon sidebar.
    pub sidebar_icon_hover_bg: Color,
    /// Width (px) of the vertical accent stripe beside active sidebar icons.
    pub sidebar_accent_width: f32,

    /// Overlay fill color shown over target split/dock drop zones.
    pub drop_overlay_fill: Color,
    /// Overlay border color shown around target split/dock drop zones.
    pub drop_overlay_border: Color,
    /// Vertical insertion indicator color shown inside a tab row during drag.
    pub drop_tab_indicator_color: Color,
    /// Drag ghost pill background color.
    pub ghost_bg: Color,
    /// Drag ghost pill border color.
    pub ghost_border: Color,
    /// Drag ghost pill label color.
    pub ghost_text: Color,

    /// Width (px) of the "Add Window" popup menu.
    pub popup_width: f32,
    /// Maximum height (px) of the "Add Window" popup menu.
    pub popup_max_height: f32,
    /// Background color of the "Add Window" popup card.
    pub popup_bg: Color,
    /// Border color of the "Add Window" popup card.
    pub popup_border: Color,
    /// Hover fill color for items inside the "Add Window" popup.
    pub popup_item_hover: Color,
    /// Corner radius of the "Add Window" popup card.
    pub popup_radius: f32,

    /// Height (px) of the workspace tab strip.
    pub workspace_strip_height: f32,
    /// Background fill of the workspace tab strip.
    pub workspace_bg: Color,
    /// Background fill of the active workspace tab.
    pub workspace_tab_active_bg: Color,
    /// Background fill of inactive workspace tabs.
    pub workspace_tab_inactive_bg: Color,
    /// Hover background fill of inactive workspace tabs.
    pub workspace_tab_hover_bg: Color,
    /// Text color of the active workspace tab.
    pub workspace_text_active: Color,
    /// Text color of inactive workspace tabs.
    pub workspace_text_inactive: Color,
}

impl Default for DockStyle {
    fn default() -> Self {
        Self::from_widget_theme(&LumaWidgetTheme::default())
    }
}

impl DockStyle {
    /// Builds a [`DockStyle`] harmonizing with the supplied [`LumaWidgetTheme`].
    pub fn from_widget_theme(theme: &LumaWidgetTheme) -> Self {
        let c = &theme.colors;
        Self {
            canvas_bg: c.background,
            area_bg: c.surface,
            area_border: c.surface_border,
            area_border_width: 1.0,
            area_radius: 6.0,

            handle_thickness: 6.0,
            handle_idle: Color::srgba(0.06, 0.06, 0.07, 1.0),
            handle_hover: Color::srgba(0.18, 0.22, 0.25, 1.0),
            handle_pressed: c.primary.with_alpha(0.65),
            handle_pill_color: Color::srgba(1.0, 1.0, 1.0, 0.20),

            tab_bar_height: 32.0,
            tab_bar_bg: Color::srgb_u8(0x11, 0x11, 0x14),
            tab_bar_border: c.surface_border,
            tab_bg_active: c.surface,
            tab_bg_inactive: Color::srgba(0.0, 0.0, 0.0, 0.0),
            tab_bg_hover: Color::srgba(1.0, 1.0, 1.0, 0.06),
            tab_text_active: c.text_primary,
            tab_text_inactive: c.text_muted,
            accent_color: c.primary,
            tab_accent_thickness: 2.0,
            tab_radius: 5.0,
            tab_gap: 2.0,
            tab_font_size: 12.5,

            icon_color: c.text_muted,
            icon_hover_color: c.text_primary,
            action_hover_bg: Color::srgba(1.0, 1.0, 1.0, 0.10),
            close_hover_bg: Color::srgba(0.92, 0.28, 0.28, 0.24),
            close_hover_color: Color::srgb_u8(0xFF, 0x7A, 0x7A),

            sidebar_width: 38.0,
            sidebar_bg: Color::srgb_u8(0x11, 0x11, 0x14),
            sidebar_icon_size: 30.0,
            sidebar_icon_active_bg: c.surface_hover,
            sidebar_icon_hover_bg: Color::srgba(1.0, 1.0, 1.0, 0.07),
            sidebar_accent_width: 2.5,

            drop_overlay_fill: c.primary.with_alpha(0.18),
            drop_overlay_border: c.primary,
            drop_tab_indicator_color: c.primary,
            ghost_bg: Color::srgba(0.10, 0.11, 0.14, 0.94),
            ghost_border: c.primary,
            ghost_text: c.text_primary,

            popup_width: 200.0,
            popup_max_height: 320.0,
            popup_bg: Color::srgb_u8(0x16, 0x16, 0x1A),
            popup_border: c.surface_border,
            popup_item_hover: Color::srgba(1.0, 1.0, 1.0, 0.08),
            popup_radius: 8.0,

            workspace_strip_height: 34.0,
            workspace_bg: Color::srgb_u8(0x0D, 0x0D, 0x10),
            workspace_tab_active_bg: c.surface,
            workspace_tab_inactive_bg: Color::NONE,
            workspace_tab_hover_bg: Color::srgba(1.0, 1.0, 1.0, 0.06),
            workspace_text_active: c.text_primary,
            workspace_text_inactive: c.text_muted,
        }
    }
}

/// Resolves the UI text font and icon font from [`Theme`] and optional [`IconFontHandle`].
pub(crate) fn resolve_fonts(
    theme: Option<&Theme>,
    icon_font_res: Option<&IconFontHandle>,
) -> (Handle<Font>, Handle<Font>) {
    let text_font = theme
        .map(|t| t.text.font.inter_regular.clone())
        .unwrap_or_default();
    let icon_font = icon_font_res
        .map(|f| f.0.clone())
        .or_else(|| theme.map(|t| t.icon.font.clone()))
        .unwrap_or_else(|| text_font.clone());
    (text_font, icon_font)
}

/// Returns the active primary window dimensions `(width, height)` in logical pixels,
/// falling back to `(1280.0, 720.0)` in headless contexts.
pub(crate) fn primary_window_size(windows: &Query<&Window>) -> Vec2 {
    windows
        .iter()
        .next()
        .map(|w| Vec2::new(w.width().max(1.0), w.height().max(1.0)))
        .unwrap_or_else(|| Vec2::new(1280.0, 720.0))
}

/// Maps a Luma node's [`GlobalTransform`] and [`ComputedSize`] into viewport screen-space
/// coordinates (`(0, 0)` at top-left, `+X` right, `+Y` down).
pub(crate) fn node_screen_rect(
    gt: &GlobalTransform,
    computed: &ComputedSize,
    windows: &Query<&Window>,
    cameras: &Query<(&Camera, &GlobalTransform)>,
) -> Rect {
    let win = primary_window_size(windows);
    let mut screen_center = Vec2::new(
        (win.x * 0.5) + gt.translation().x,
        (win.y * 0.5) - gt.translation().y,
    );
    for (cam, cam_gt) in cameras.iter() {
        if let Ok(vp) = cam.world_to_viewport(cam_gt, gt.translation()) {
            screen_center = vp;
            break;
        }
    }
    Rect::from_center_size(
        screen_center,
        Vec2::new(computed.width.max(0.0), computed.height.max(0.0)),
    )
}

/// Updates `UInteractionColors` only when at least one state color changed to avoid spurious Bevy change ticks.
pub(crate) fn set_interaction_colors_if_changed(
    colors: &mut Mut<'_, UInteractionColors>,
    next: UInteractionColors,
) {
    let cur: &UInteractionColors = &**colors;
    if cur.normal != next.normal || cur.hovered != next.hovered || cur.pressed != next.pressed {
        **colors = next;
    }
}

/// Updates the cursor icon on the active window entity when present (no-op headlessly).
pub(crate) fn set_window_cursor(
    commands: &mut Commands,
    windows: &Query<Entity, With<Window>>,
    icon: SystemCursorIcon,
) {
    for win_ent in windows.iter() {
        if let Ok(mut cmd) = commands.get_entity(win_ent) {
            cmd.try_insert(CursorIcon::System(icon));
        }
    }
}
