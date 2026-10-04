//! # Bevy Luma Widgets
//!
//! A modern, high-level UI widget component suite built on top of the `bevy_luma` SDF UI framework.
//!
//! Provides ready-to-use, accessible, theme-aware widgets designed with Bevy Scene Notation (`bsn!`):
//! - Buttons (Primary, Secondary, Outline, Ghost, Danger)
//! - Toggles / Switches (smooth animated knob)
//! - Checkboxes (custom checked state and label)
//! - Sliders / Range Seekbars (smooth drag, fill bar, thumb)
//! - Progress Bars (continuous or stepped progress)
//! - Badges / Tags (status indicators with variants)
//! - Dividers (horizontal, vertical, or with embedded label)
//! - Cards (elevated SDF surfaces with borders and shadows)
//! - Modals / Dialogs (centered dialogs with backdrop)
//! - Radio Buttons (grouped single-choice selection)
//! - Tabs (tabbed content switching)
//! - Accordions (collapsible disclosure panels)
//! - Alerts (contextual status banners)
//! - Avatars (user initials & presence status)
//! - Skeletons (animated loading placeholder pulse)
//! - Tooltips (floating description bubbles)

pub mod accordion;
pub mod alert;
pub mod avatar;
pub mod badge;
pub mod button;
pub mod card;
pub mod checkbox;
pub mod divider;
pub mod input;
pub mod modal;
pub mod progress;
pub mod radio;
pub mod skeleton;
pub mod slider;
pub mod tabs;
pub mod theme;
pub mod toggle;
pub mod tooltip;

use bevy::prelude::*;
pub use bevy_luma;

/// Common widget types and builders.
pub mod prelude {
    pub use bevy_luma::prelude::*;

    pub use crate::{
        accordion::*, alert::*, avatar::*, badge::*, button::*, card::*, checkbox::*, divider::*,
        input::*, modal::*, progress::*, radio::*, skeleton::*, slider::*, tabs::*, theme::*,
        toggle::*, tooltip::*, LumaWidgetsPlugin,
    };
}

/// Plugin registering all widget systems, observers, and default themes.
pub struct LumaWidgetsPlugin;

impl Plugin for LumaWidgetsPlugin {
    fn build(&self, app: &mut App) {
        if !app.is_plugin_added::<bevy_luma::LumaPlugin>() {
            app.add_plugins(bevy_luma::LumaPlugin);
        }

        app.init_resource::<theme::LumaWidgetTheme>()
            .add_plugins((
                button::LumaButtonPlugin,
                toggle::LumaTogglePlugin,
                checkbox::LumaCheckboxPlugin,
                slider::LumaSliderPlugin,
                progress::LumaProgressPlugin,
                radio::LumaRadioPlugin,
                tabs::LumaTabsPlugin,
                accordion::LumaAccordionPlugin,
                skeleton::LumaSkeletonPlugin,
            ));
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::prelude::*;
    use bevy::{asset::AssetPlugin, text::TextPlugin};

    fn test_app() -> App {
        let mut app = App::new();
        app.add_plugins((MinimalPlugins, AssetPlugin::default(), TextPlugin))
            .add_plugins(LumaWidgetsPlugin);
        app
    }

    #[test]
    fn widget_plugin_registers_theme() {
        let app = test_app();
        assert!(app.world().contains_resource::<LumaWidgetTheme>());
    }

    #[test]
    fn button_size_dimensions() {
        assert_eq!(ButtonSize::Small.min_height(), 30.0);
        assert_eq!(ButtonSize::Medium.min_height(), 38.0);
        assert_eq!(ButtonSize::Large.min_height(), 46.0);

        assert_eq!(ButtonSize::Small.font_size(), 12.0);
        assert_eq!(ButtonSize::Medium.font_size(), 14.0);
        assert_eq!(ButtonSize::Large.font_size(), 16.0);
    }

    #[test]
    fn slider_ratio_calculation() {
        let mut slider = LumaSlider {
            value: 25.0,
            min: 0.0,
            max: 100.0,
            step: None,
            width: 200.0,
            disabled: false,
        };

        assert!((slider.ratio() - 0.25).abs() < f32::EPSILON);

        slider.set_ratio(0.8);
        assert!((slider.value - 80.0).abs() < f32::EPSILON);

        // Clamping check
        slider.set_ratio(1.5);
        assert!((slider.value - 100.0).abs() < f32::EPSILON);

        slider.set_ratio(-0.5);
        assert!((slider.value - 0.0).abs() < f32::EPSILON);
    }

    #[test]
    fn slider_stepped_rounding() {
        let mut slider = LumaSlider {
            value: 0.0,
            min: 0.0,
            max: 100.0,
            step: Some(10.0),
            width: 200.0,
            disabled: false,
        };

        slider.set_ratio(0.24); // 24.0 -> should round to 20.0
        assert_eq!(slider.value, 20.0);

        slider.set_ratio(0.26); // 26.0 -> should round to 30.0
        assert_eq!(slider.value, 30.0);
    }

    #[test]
    fn progress_bar_variant_colors() {
        assert_eq!(ProgressVariant::Default.color(), Color::srgb(0.20, 0.45, 0.90));
        assert_eq!(ProgressVariant::Success.color(), Color::srgb(0.16, 0.75, 0.45));
        assert_eq!(ProgressVariant::Warning.color(), Color::srgb(0.95, 0.65, 0.15));
        assert_eq!(ProgressVariant::Danger.color(), Color::srgb(0.85, 0.22, 0.28));
    }

    #[test]
    fn toggle_default_state() {
        let toggle = LumaToggle::default();
        assert!(!toggle.is_checked);
        assert!(!toggle.disabled);

        let knob = LumaToggleKnob::default();
        assert_eq!(knob.current_offset, 0.0);
        assert_eq!(knob.target_offset, 0.0);
    }

    #[test]
    fn checkbox_default_state() {
        let checkbox = LumaCheckbox::default();
        assert!(!checkbox.is_checked);
        assert!(!checkbox.disabled);
    }

    #[test]
    fn alert_variant_colors() {
        let (info_bg, _, _, info_icon) = AlertVariant::Info.colors();
        assert_eq!(info_icon, Icon::INFO);
        assert_eq!(info_bg, Color::srgba(0.10, 0.25, 0.50, 0.25));

        let (_, _, _, success_icon) = AlertVariant::Success.colors();
        assert_eq!(success_icon, Icon::CIRCLE_CHECK_BIG);
    }

    #[test]
    fn avatar_dimensions() {
        assert_eq!(AvatarSize::Small.size_px(), 32.0);
        assert_eq!(AvatarSize::Medium.size_px(), 40.0);
        assert_eq!(AvatarSize::Large.size_px(), 48.0);
        assert_eq!(AvatarStatus::Online.color(), Color::srgb(0.15, 0.75, 0.40));
    }

    #[test]
    fn tabs_default_state() {
        let tabs = LumaTabs::default();
        assert_eq!(tabs.group, "");
        assert_eq!(tabs.active, "");
    }

    #[test]
    fn accordion_default_state() {
        let acc = LumaAccordionItem::default();
        assert!(!acc.is_expanded);
    }
}
