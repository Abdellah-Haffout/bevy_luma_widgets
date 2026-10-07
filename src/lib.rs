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
//! - Keyboard Keycaps (shortcuts and hotkey combos)
//! - Stat / Metric Cards (KPI dashboards with trend indicators)
//! - Steppers (interactive numeric counters)
//! - Star Ratings (display and interactive ratings)
//! - Breadcrumbs (hierarchical navigation trails)
//! - Segmented Controls (pill toggle button bars)
//! - Chips / Tags (removable tags with icons and avatars)
//! - Toasts (contextual alert toasts with auto-dismiss timers)
//! - Color Swatches (palette previews and color pickers)

pub mod accordion;
pub mod alert;
pub mod avatar;
pub mod badge;
pub mod breadcrumb;
pub mod button;
pub mod card;
pub mod checkbox;
pub mod chip;
pub mod divider;
pub mod input;
pub mod kbd;
pub mod modal;
pub mod progress;
pub mod radio;
pub mod rating;
pub mod segmented_control;
pub mod skeleton;
pub mod slider;
pub mod stat;
pub mod stepper;
pub mod swatch;
pub mod tabs;
pub mod theme;
pub mod toast;
pub mod toggle;
pub mod tooltip;

use bevy::prelude::*;
pub use bevy_luma;

/// Common widget types and builders.
pub mod prelude {
    pub use bevy_luma::prelude::*;

    pub use crate::{
        accordion::*, alert::*, avatar::*, badge::*, breadcrumb::*, button::*, card::*,
        checkbox::*, chip::*, divider::*, input::*, kbd::*, modal::*, progress::*, radio::*,
        rating::*, segmented_control::*, skeleton::*, slider::*, stat::*, stepper::*, swatch::*,
        tabs::*, theme::*, toast::*, toggle::*, tooltip::*, LumaWidgetsPlugin,
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
            ))
            .add_plugins((
                kbd::LumaKbdPlugin,
                stat::LumaStatPlugin,
                stepper::LumaStepperPlugin,
                rating::LumaRatingPlugin,
                breadcrumb::LumaBreadcrumbPlugin,
                segmented_control::LumaSegmentedControlPlugin,
                chip::LumaChipPlugin,
                toast::LumaToastPlugin,
                swatch::LumaSwatchPlugin,
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
        app.add_plugins((
            MinimalPlugins,
            AssetPlugin::default(),
            TextPlugin,
            bevy::scene::ScenePlugin,
        ))
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

    #[test]
    fn kbd_size_dimensions() {
        assert_eq!(KbdSize::Small.min_height(), 20.0);
        assert_eq!(KbdSize::Medium.min_height(), 24.0);
        assert_eq!(KbdSize::Small.font_size(), 10.0);
        assert_eq!(KbdSize::Medium.font_size(), 12.0);
    }

    #[test]
    fn stat_trend_colors() {
        let (_, up_color, up_icon) = StatTrend::Up("+12%".into()).colors();
        assert_eq!(up_icon, Icon::ARROW_UP);
        assert_eq!(up_color, Color::srgb(0.35, 0.90, 0.55));

        let (_, down_color, down_icon) = StatTrend::Down("-5%".into()).colors();
        assert_eq!(down_icon, Icon::ARROW_DOWN);
        assert_eq!(down_color, Color::srgb(1.00, 0.45, 0.50));
    }

    #[test]
    fn stepper_default_state() {
        let stepper = LumaStepper::default();
        assert_eq!(stepper.value, 0.0);
        assert_eq!(stepper.min, 0.0);
        assert_eq!(stepper.max, 100.0);
        assert_eq!(stepper.step, 1.0);
        assert!(!stepper.disabled);
    }

    #[test]
    fn rating_default_state() {
        let rating = LumaRating::default();
        assert_eq!(rating.value, 5);
        assert_eq!(rating.max_stars, 5);
        assert!(rating.read_only);
    }

    #[test]
    fn toast_variant_colors() {
        let (_, _, info_icon_color, info_icon) = ToastVariant::Info.colors();
        assert_eq!(info_icon, Icon::INFO);
        assert_eq!(info_icon_color, Color::srgb(0.40, 0.70, 1.00));

        let (_, _, success_icon_color, success_icon) = ToastVariant::Success.colors();
        assert_eq!(success_icon, Icon::CHECK);
        assert_eq!(success_icon_color, Color::srgb(0.35, 0.85, 0.55));
    }

    #[test]
    fn swatch_default_state() {
        let swatch = LumaSwatch::default();
        assert_eq!(swatch.color, Color::WHITE);
        assert!(!swatch.selected);
    }

    #[test]
    fn chip_default_state() {
        let chip = LumaChip::default();
        assert_eq!(chip.label, "");
        assert!(!chip.disabled);
    }

    #[test]
    fn segmented_control_default_state() {
        let seg = LumaSegmentedControl::default();
        assert_eq!(seg.group, "");
        assert_eq!(seg.active, "");
    }

    #[test]
    fn button_declarative_builder() {
        let btn = Button::new("Click")
            .variant(ButtonVariant::Danger)
            .size(ButtonSize::Large)
            .disabled(true);
        assert_eq!(btn.label, "Click");
        assert_eq!(btn.variant, ButtonVariant::Danger);
        assert_eq!(btn.size, ButtonSize::Large);
        assert!(btn.disabled);
    }

    #[test]
    fn toggle_declarative_builder() {
        let toggle = Toggle::new(false)
            .label("Sound")
            .checked(true)
            .disabled(false);
        assert!(toggle.is_checked);
        assert_eq!(toggle.label.as_deref(), Some("Sound"));
        assert!(!toggle.disabled);
    }

    #[test]
    fn checkbox_declarative_builder() {
        let cb = Checkbox::new("Auto-save", false)
            .label("Cloud Sync")
            .checked(true);
        assert!(cb.is_checked);
        assert_eq!(cb.label.as_deref(), Some("Cloud Sync"));
    }

    #[test]
    fn slider_declarative_builder() {
        let slider = Slider::new(10.0, 0.0, 50.0, 180.0)
            .value(25.0)
            .min(5.0)
            .max(100.0)
            .width(220.0)
            .step(5.0);
        assert_eq!(slider.value, 25.0);
        assert_eq!(slider.min, 5.0);
        assert_eq!(slider.max, 100.0);
        assert_eq!(slider.width, 220.0);
        assert_eq!(slider.step, Some(5.0));
    }

    #[test]
    fn style_scope_builder() {
        let scope = LumaStyleScope::new()
            .button(ButtonStyle::danger())
            .card(CardStyle::default());
        assert!(scope.button.is_some());
        assert!(scope.card.is_some());
        assert!(scope.toggle.is_none());
    }

    #[test]
    fn test_mouse_clicks_on_widgets() {
        use bevy::camera::NormalizedRenderTarget;
        use bevy::picking::backend::HitData;
        use bevy::picking::events::{Click, Pointer, Press};
        use bevy::picking::pointer::{Location, PointerButton, PointerId};
        use core::time::Duration;

        let mut app = test_app();
        app.update();

        let btn = app.world_mut().spawn_scene(Button::new("ClickMe")).unwrap().id();
        let toggle = app.world_mut().spawn_scene(Toggle::new(false)).unwrap().id();
        let checkbox = app.world_mut().spawn_scene(Checkbox::new("Check", false)).unwrap().id();
        let stepper = app.world_mut().spawn_scene(Stepper::new(5.0, 0.0, 10.0, 1.0)).unwrap().id();
        let chip = app.world_mut().spawn_scene(Chip::new("Tag").removable(true)).unwrap().id();
        let toast = app.world_mut().spawn_scene(Toast::new("Title", "Desc")).unwrap().id();
        let swatch = app.world_mut().spawn_scene(Swatch::new(Color::srgb(1.0, 0.0, 0.0))).unwrap().id();
        let radio = app.world_mut().spawn_scene(Radio::new("group", "opt1")).unwrap().id();
        app.update();

        let location = Location {
            target: NormalizedRenderTarget::None { width: 0, height: 0 },
            position: Vec2::ZERO,
        };

        let hit = HitData::new(Entity::PLACEHOLDER, 0.0, None, None);

        let trigger_click = |app: &mut App, entity: Entity, button: PointerButton| {
            app.world_mut().trigger(Pointer::new(
                PointerId::Mouse,
                location.clone(),
                Click {
                    button,
                    hit: hit.clone(),
                    duration: Duration::ZERO,
                    count: 1,
                },
                entity,
            ));
        };

        let trigger_press = |app: &mut App, entity: Entity, button: PointerButton| {
            app.world_mut().trigger(Pointer::new(
                PointerId::Mouse,
                location.clone(),
                Press {
                    button,
                    hit: hit.clone(),
                    count: 1,
                },
                entity,
            ));
        };

        for entity in [btn, toggle, checkbox, stepper, chip, toast, swatch, radio] {
            // Test Left Click (Primary)
            trigger_press(&mut app, entity, PointerButton::Primary);
            trigger_click(&mut app, entity, PointerButton::Primary);
            app.update();

            // Test Right Click (Secondary)
            trigger_press(&mut app, entity, PointerButton::Secondary);
            trigger_click(&mut app, entity, PointerButton::Secondary);
            app.update();
        }
    }
}

