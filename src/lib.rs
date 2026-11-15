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

pub mod cards;
pub mod controls;
pub mod data_views;
pub mod navigation;
pub mod states;
pub mod accordion;
pub mod alert;
pub mod avatar;
pub mod badge;
pub mod breadcrumb;
pub mod button;
pub mod button_group;
pub mod card;
pub mod checkbox;
pub mod chip;
pub mod divider;
pub mod drag_value;
pub mod drawer;
pub(crate) mod helpers;
pub mod input;
pub mod kbd;
pub mod list_item;
pub mod modal;
pub mod panel;
pub mod popover;
pub mod progress;
pub mod radio;
pub mod rating;
pub mod segmented_control;
pub mod select;
pub mod skeleton;
pub mod slider;
pub mod spinner;
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
        accordion::*, alert::*, avatar::*, badge::*, breadcrumb::*, button::*, button_group::*, card::*, cards::*,
        checkbox::*, chip::*, controls::*, data_views::*, divider::*, drag_value::*, drawer::*,
        input::*, kbd::*, list_item::*, modal::*, navigation::*, panel::*, popover::*, progress::*,
        radio::*, rating::*, segmented_control::*, select::*, skeleton::*, slider::*, spinner::*,
        stat::*, states::*, stepper::*, swatch::*, tabs::*, theme::*, toast::*, toggle::*,
        tooltip::*, LumaWidgetsPlugin,
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
            ))
            .add_plugins((
                spinner::LumaSpinnerPlugin,
                list_item::LumaListItemPlugin,
                select::LumaSelectPlugin,
                drawer::LumaDrawerPlugin,
                popover::LumaPopoverPlugin,
                drag_value::LumaDragValuePlugin,
                modal::LumaModalPlugin,
                tooltip::LumaTooltipPlugin,
            ))
            .add_plugins((
                theme::LumaThemePlugin,
                input::LumaInputPlugin,
                controls::LumaControlsPlugin,
                navigation::LumaNavigationPlugin,
                cards::LumaCardsPlugin,
                data_views::LumaDataViewsPlugin,
                states::LumaStatesPlugin,
                button_group::LumaButtonGroupPlugin,
                panel::DockPlugin,
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
        assert_eq!(ButtonSize::Small.min_height(), 28.0);
        assert_eq!(ButtonSize::Medium.min_height(), 34.0);
        assert_eq!(ButtonSize::Large.min_height(), 42.0);

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
            ..default()
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
            ..default()
        };

        slider.set_ratio(0.24); // 24.0 -> should round to 20.0
        assert_eq!(slider.value, 20.0);

        slider.set_ratio(0.26); // 26.0 -> should round to 30.0
        assert_eq!(slider.value, 30.0);

        // Step grid anchored at non-zero `min`: 10 + 0.24 * 100 = 34 -> rounds to 35
        let mut offset_slider = LumaSlider {
            value: 10.0,
            min: 10.0,
            max: 110.0,
            step: Some(25.0),
            width: 200.0,
            disabled: false,
            ..default()
        };
        offset_slider.set_ratio(0.24);
        assert_eq!(offset_slider.value, 35.0);
    }

    #[test]
    fn progress_bar_variant_colors() {
        let colors = theme::WidgetColors::default();
        assert_eq!(ProgressVariant::Default.color(), colors.primary);
        assert_eq!(ProgressVariant::Success.color(), colors.success);
        assert_eq!(ProgressVariant::Warning.color(), colors.warning);
        assert_eq!(ProgressVariant::Danger.color(), colors.danger);
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
        use bevy::picking::events::{Pointer, PointerClick, PointerPress};
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
            app.world_mut().trigger(PointerClick {
                entity,
                pointer: Pointer::new(PointerId::Mouse, location.clone()),
                button,
                hit: hit.clone(),
                duration: Duration::ZERO,
                count: 1,
            });
        };

        let trigger_press = |app: &mut App, entity: Entity, button: PointerButton| {
            app.world_mut().trigger(PointerPress {
                entity,
                pointer: Pointer::new(PointerId::Mouse, location.clone()),
                button,
                hit: hit.clone(),
                count: 1,
            });
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

    #[test]
    fn spinner_declarative_builder() {
        let spinner = Spinner::new().size(32.0).speed(8.0).thickness(4.0);
        assert_eq!(spinner.size, 32.0);
        assert_eq!(spinner.speed, 8.0);
        assert_eq!(spinner.thickness, 4.0);

        let sm = Spinner::sm();
        assert_eq!(sm.style.unwrap().size, 16.0);
    }

    #[test]
    fn text_input_presets() {
        let search = TextInput::search("Search query");
        assert!(search.leading_icon.is_some());
        assert_eq!(search.placeholder, "Search query");

        let pwd = TextInput::password("Pass");
        assert_eq!(pwd.mode, TextInputMode::Password);
        assert!(pwd.trailing_icon.is_some());

        let otp = TextInput::otp();
        assert_eq!(otp.style.unwrap().min_height, 48.0);

        let multiline = TextInput::multiline("Notes");
        assert_eq!(multiline.style.unwrap().min_height, 90.0);
    }

    #[test]
    fn button_fab_and_loading() {
        let fab = Button::fab_button("plus");
        assert!(fab.fab);
        assert!(fab.icon_only);

        let loading = Button::new("Submit").loading(true);
        assert!(loading.loading);
    }

    #[test]
    fn badge_counter_formatting() {
        let c5 = Badge::counter(5);
        assert_eq!(c5.label, "5");

        let c150 = Badge::counter(150);
        assert_eq!(c150.label, "99+");
    }

    #[test]
    fn list_item_declarative_builder() {
        let item = ListItem::new("Title")
            .subtitle("Subtitle")
            .leading_icon("star")
            .trailing_text("Active");

        assert_eq!(item.title, "Title");
        assert_eq!(item.subtitle, Some("Subtitle".to_string()));
        assert_eq!(item.leading_icon, Some("star".to_string()));
        assert_eq!(item.trailing_text, Some("Active".to_string()));
    }

    #[test]
    fn select_declarative_builder() {
        let options = vec![
            SelectOption::new("Option A", "a"),
            SelectOption::new("Option B", "b"),
        ];
        let select = Select::new("Choose", options).selected(1);

        assert_eq!(select.placeholder, "Choose");
        assert_eq!(select.options.len(), 2);
        assert_eq!(select.selected_index, Some(1));
    }

    #[test]
    fn drawer_and_popover_builders() {
        let drawer = Drawer::left().title("Sidebar Menu");
        assert_eq!(drawer.placement, DrawerPlacement::Left);
        assert_eq!(drawer.title, Some("Sidebar Menu".to_string()));

        let popover = Popover::new().open(true);
        assert!(popover.is_open);
    }

    #[test]
    fn card_style_presets() {
        let elevated = CardStyle::elevated();
        assert_eq!(elevated.radius, 12.0);

        let outlined = CardStyle::outlined();
        assert_eq!(outlined.bg_color, Color::NONE);
        assert_eq!(outlined.border_width, 1.5);

        let flat = CardStyle::flat();
        assert_eq!(flat.border_width, 0.0);
    }

    #[test]
    fn test_all_100_widgets_instantiation() {
        let mut app = test_app();

        // 1. Input & Controls (30 widgets)
        let _ = app.world_mut().spawn_scene(Button::new("Btn")).unwrap();
        let _ = app.world_mut().spawn_scene(IconButton::new(Icon::PLUS)).unwrap();
        let _ = app.world_mut().spawn_scene(Fab::new(Icon::PLUS)).unwrap();
        let _ = app.world_mut().spawn_scene(SegmentedControl::new("g1", "O1")).unwrap();
        let _ = app.world_mut().spawn_scene(Toggle::new(true)).unwrap();
        let _ = app.world_mut().spawn_scene(Checkbox::new("Check", true)).unwrap();
        let _ = app.world_mut().spawn_scene(Radio::new("g1", "v1")).unwrap();
        let _ = app.world_mut().spawn_scene(TextInput::new("Input placeholder")).unwrap();
        let _ = app.world_mut().spawn_scene(PasswordInput::new()).unwrap();
        let _ = app.world_mut().spawn_scene(SearchBar::new("Search...")).unwrap();
        let _ = app.world_mut().spawn_scene(TextArea::new("Type notes...")).unwrap();
        let _ = app.world_mut().spawn_scene(Stepper::new(1.0, 0.0, 10.0, 1.0)).unwrap();
        let _ = app.world_mut().spawn_scene(Select::new("Sel", vec![SelectOption::new("A", "a")])).unwrap();
        let _ = app.world_mut().spawn_scene(MultiSelect::new("Multi")).unwrap();
        let _ = app.world_mut().spawn_scene(Combobox::new("Combo")).unwrap();
        let _ = app.world_mut().spawn_scene(Slider::new(5.0, 0.0, 10.0, 100.0)).unwrap();
        let _ = app.world_mut().spawn_scene(RangeSlider::new(20.0, 80.0, 0.0, 100.0)).unwrap();
        let _ = app.world_mut().spawn_scene(DatePicker::new("2026-10-16")).unwrap();
        let _ = app.world_mut().spawn_scene(DateRangePicker::new("2026-10-01", "2026-10-16")).unwrap();
        let _ = app.world_mut().spawn_scene(TimePicker::new("12:00 PM")).unwrap();
        let _ = app.world_mut().spawn_scene(ColorPicker::new(Color::srgb(0.2, 0.5, 0.9), "#3388EE")).unwrap();
        let _ = app.world_mut().spawn_scene(FileUploader::new("Drop files here")).unwrap();
        let _ = app.world_mut().spawn_scene(OtpInput::new(6)).unwrap();
        let _ = app.world_mut().spawn_scene(PhoneInput::new("+1")).unwrap();
        let _ = app.world_mut().spawn_scene(Rating::new(5)).unwrap();
        let _ = app.world_mut().spawn_scene(ReactionPicker::default()).unwrap();
        let _ = app.world_mut().spawn_scene(RichTextToolbar::default()).unwrap();
        let _ = app.world_mut().spawn_scene(TagInput::new("Add tags...")).unwrap();
        let _ = app.world_mut().spawn_scene(CreditCardInput::new()).unwrap();
        let _ = app.world_mut().spawn_scene(VoiceInputButton::new()).unwrap();

        // 2. Navigation & Structure (25 widgets)
        let _ = app.world_mut().spawn_scene(Navbar::new("App")).unwrap();
        let _ = app.world_mut().spawn_scene(BottomNav::new()).unwrap();
        let _ = app.world_mut().spawn_scene(Sidebar::new()).unwrap();
        let _ = app.world_mut().spawn_scene(BreadcrumbItem::new("home", "Home")).unwrap();
        let _ = app.world_mut().spawn_scene(Breadcrumbs::new(vec!["Home"])).unwrap();
        let _ = app.world_mut().spawn_scene(TabTrigger::new("group", "tab1", "Tab 1")).unwrap();
        let _ = app.world_mut().spawn_scene(Pagination::new(1, 10)).unwrap();
        let _ = app.world_mut().spawn_scene(StepTracker::new(vec!["Cart"])).unwrap();
        let _ = app.world_mut().spawn_scene(HamburgerMenu::new()).unwrap();
        let _ = app.world_mut().spawn_scene(MegaMenu::new("Solutions")).unwrap();
        let _ = app.world_mut().spawn_scene(Footer::new()).unwrap();
        let _ = app.world_mut().spawn_scene(ContextMenu::new(vec!["Copy"])).unwrap();
        let _ = app.world_mut().spawn_scene(CommandPalette::new()).unwrap();
        let _ = app.world_mut().spawn_scene(CustomScrollbar::new()).unwrap();
        let _ = app.world_mut().spawn_scene(BackToTopButton::new()).unwrap();
        let _ = app.world_mut().spawn_scene(ActionSheet::new("Actions")).unwrap();
        let _ = app.world_mut().spawn_scene(BottomSheet::new("Sheet")).unwrap();
        let _ = app.world_mut().spawn_scene(Splitter::new()).unwrap();
        let _ = app.world_mut().spawn_scene(TreeView::new()).unwrap();
        let _ = app.world_mut().spawn_scene(CarouselControls::new(0, 5)).unwrap();
        let _ = app.world_mut().spawn_scene(TableOfContents::new(vec!["Intro"])).unwrap();
        let _ = app.world_mut().spawn_scene(FilterSortBar::new()).unwrap();
        let _ = app.world_mut().spawn_scene(FloatingToolbar::new()).unwrap();
        let _ = app.world_mut().spawn_scene(WorkspaceSwitcher::new("Main")).unwrap();
        let _ = app.world_mut().spawn_scene(LocaleSelector::new("EN")).unwrap();
        let _ = app.world_mut().spawn_scene(ThemeToggle::new()).unwrap();

        // 3. Data Display & Content (25 widgets)
        let _ = app.world_mut().spawn_scene(Card::new()).unwrap();
        let _ = app.world_mut().spawn_scene(ProductCard::new("Item", "$99")).unwrap();
        let _ = app.world_mut().spawn_scene(UserProfileCard::new("Alice", "@alice")).unwrap();
        let _ = app.world_mut().spawn_scene(PricingCard::new("Pro", "$29")).unwrap();
        let _ = app.world_mut().spawn_scene(ArticleCard::new("Title", "Excerpt...")).unwrap();
        let _ = app.world_mut().spawn_scene(TestimonialCard::new("Great!", "Bob")).unwrap();
        let _ = app.world_mut().spawn_scene(StatCard::new("Users", "12,450")).unwrap();
        let _ = app.world_mut().spawn_scene(AccordionItem::new("Q1")).unwrap();
        let _ = app.world_mut().spawn_scene(Avatar::new("AB")).unwrap();
        let _ = app.world_mut().spawn_scene(AvatarGroup::new(vec!["A".to_string(), "B".to_string()])).unwrap();
        let _ = app.world_mut().spawn_scene(Badge::new("Active")).unwrap();
        let _ = app.world_mut().spawn_scene(NotificationCounter::new(5)).unwrap();
        let _ = app.world_mut().spawn_scene(Chip::new("Rust")).unwrap();
        let _ = app.world_mut().spawn_scene(ListItem::new("List Title")).unwrap();
        let _ = app.world_mut().spawn_scene(Divider::horizontal()).unwrap();
        let _ = app.world_mut().spawn_scene(DataTable::new(vec!["Col1".to_string()], vec![vec!["Val1".to_string()]])).unwrap();
        let _ = app.world_mut().spawn_scene(Timeline::new(vec![("10:00", "Deployed", "v1.0")])).unwrap();
        let _ = app.world_mut().spawn_scene(CommentThread::new("User", "Nice work!")).unwrap();
        let _ = app.world_mut().spawn_scene(ChatBubble::new("Hello!")).unwrap();
        let _ = app.world_mut().spawn_scene(MediaPlayer::new("Track", "Artist")).unwrap();
        let _ = app.world_mut().spawn_scene(ImageLightbox::new("Nature")).unwrap();
        let _ = app.world_mut().spawn_scene(BarChart::new(vec![("Jan", 10.0)])).unwrap();
        let _ = app.world_mut().spawn_scene(DonutChart::new(0.75, "75% Done")).unwrap();
        let _ = app.world_mut().spawn_scene(SocialFeedPost::new("Author", "@auth")).unwrap();
        let _ = app.world_mut().spawn_scene(CodeBlock::new("fn main() {}")).unwrap();

        // 4. Feedback, Overlays & States (22 widgets)
        let _ = app.world_mut().spawn_scene(Modal::new("Modal Title", "Description")).unwrap();
        let _ = app.world_mut().spawn_scene(Alert::new("System Alert", "Notice details")).unwrap();
        let _ = app.world_mut().spawn_scene(Toast::new("Saved", "File saved")).unwrap();
        let _ = app.world_mut().spawn_scene(Tooltip::new("Help text")).unwrap();
        let _ = app.world_mut().spawn_scene(Popover::new()).unwrap();
        let _ = app.world_mut().spawn_scene(ProgressBar::new(0.5)).unwrap();
        let _ = app.world_mut().spawn_scene(Spinner::new()).unwrap();
        let _ = app.world_mut().spawn_scene(Skeleton::new(UVal::Px(100.0), UVal::Px(20.0))).unwrap();
        let _ = app.world_mut().spawn_scene(Kbd::new("Ctrl")).unwrap();
        let _ = app.world_mut().spawn_scene(Drawer::right()).unwrap();
        let _ = app.world_mut().spawn_scene(EmptyState::new("No Data", "Nothing here yet")).unwrap();
        let _ = app.world_mut().spawn_scene(Error404State::new()).unwrap();
        let _ = app.world_mut().spawn_scene(OfflineState::new()).unwrap();
        let _ = app.world_mut().spawn_scene(SuccessScreen::new("Payment Complete", "$49.99")).unwrap();
        let _ = app.world_mut().spawn_scene(Coachmark::new("Feature", "Check this new feature out")).unwrap();
        let _ = app.world_mut().spawn_scene(CookieConsent::new()).unwrap();
        let _ = app.world_mut().spawn_scene(Paywall::new()).unwrap();
        let _ = app.world_mut().spawn_scene(NotificationCenter::new()).unwrap();
        let _ = app.world_mut().spawn_scene(CartDrawer::new()).unwrap();
        let _ = app.world_mut().spawn_scene(KanbanColumn::new("To Do", 2)).unwrap();
        let _ = app.world_mut().spawn_scene(KanbanCard::new("Task 1", "Bug")).unwrap();
        let _ = app.world_mut().spawn_scene(CalendarSchedule::new("Monday")).unwrap();

        app.update();
    }

    #[derive(Resource, Default)]
    struct ActivationCounts {
        buttons: usize,
        toggles: usize,
        checkboxes: usize,
    }

    #[test]
    fn disabled_controls_ignore_activation() {
        let mut app = test_app();
        app.init_resource::<ActivationCounts>();
        app.add_observer(|_: On<ButtonClicked>, mut c: ResMut<ActivationCounts>| c.buttons += 1);
        app.add_observer(|_: On<ToggleChanged>, mut c: ResMut<ActivationCounts>| c.toggles += 1);
        app.add_observer(|_: On<CheckboxChanged>, mut c: ResMut<ActivationCounts>| c.checks_incr());

        app.world_mut().spawn_scene(Button::new("Off").disabled(true)).unwrap();
        app.world_mut().spawn_scene(Button::new("On")).unwrap();
        app.world_mut().spawn_scene(Toggle::new(false).disabled(true)).unwrap();
        app.world_mut().spawn_scene(Toggle::new(false)).unwrap();
        app.world_mut().spawn_scene(Checkbox::new("Off", false).disabled(true)).unwrap();
        app.world_mut().spawn_scene(Checkbox::new("On", false)).unwrap();
        app.update();

        let disabled_entities: Vec<Entity> = {
            let world = app.world_mut();
            let btns = world.query::<(Entity, &LumaButton)>().iter(world).filter(|(_, b)| b.disabled).map(|(e, _)| e).collect::<Vec<_>>();
            let togs = world.query::<(Entity, &LumaToggle)>().iter(world).filter(|(_, t)| t.disabled).map(|(e, _)| e).collect::<Vec<_>>();
            let chks = world.query::<(Entity, &LumaCheckbox)>().iter(world).filter(|(_, c)| c.disabled).map(|(e, _)| e).collect::<Vec<_>>();
            [btns, togs, chks].concat()
        };
        for entity in disabled_entities {
            app.world_mut().trigger(UFocusActivate { entity });
        }
        app.update();

        let counts = app.world().resource::<ActivationCounts>();
        assert_eq!(counts.buttons, 0);
        assert_eq!(counts.toggles, 0);
        assert_eq!(counts.checkboxes, 0);

        let enabled_entities: Vec<Entity> = {
            let world = app.world_mut();
            let btns = world.query::<(Entity, &LumaButton)>().iter(world).filter(|(_, b)| !b.disabled).map(|(e, _)| e).collect::<Vec<_>>();
            let togs = world.query::<(Entity, &LumaToggle)>().iter(world).filter(|(_, t)| !t.disabled).map(|(e, _)| e).collect::<Vec<_>>();
            let chks = world.query::<(Entity, &LumaCheckbox)>().iter(world).filter(|(_, c)| !c.disabled).map(|(e, _)| e).collect::<Vec<_>>();
            [btns, togs, chks].concat()
        };
        for entity in enabled_entities {
            app.world_mut().trigger(UFocusActivate { entity });
        }
        app.update();

        let counts = app.world().resource::<ActivationCounts>();
        assert_eq!(counts.buttons, 1);
        assert_eq!(counts.toggles, 1);
        assert_eq!(counts.checkboxes, 1);
    }

    impl ActivationCounts {
        fn checks_incr(&mut self) {
            self.checkboxes += 1;
        }
    }

    #[test]
    fn toggle_preserves_custom_style_on_activation() {
        let mut app = test_app();
        let custom_checked = Color::srgb(0.90, 0.25, 0.15);
        app.world_mut()
            .spawn_scene(Toggle::new(false).style(ToggleStyle {
                checked_bg: custom_checked,
                ..default()
            }))
            .unwrap();
        app.update();

        let toggle_entity = {
            let mut q = app.world_mut().query::<(Entity, &LumaToggle)>();
            q.single(app.world()).unwrap().0
        };
        assert!(app.world().get::<LumaToggle>(toggle_entity).unwrap().has_explicit_style);

        app.world_mut().trigger(UFocusActivate { entity: toggle_entity });
        app.world_mut().trigger(UFocusDeactivate { entity: toggle_entity });
        app.update();

        let toggle = app.world().get::<LumaToggle>(toggle_entity).unwrap();
        let node = app.world().get::<UNode>(toggle_entity).unwrap();
        let i_colors = app.world().get::<UInteractionColors>(toggle_entity).unwrap();
        assert!(toggle.is_checked);
        assert_eq!(i_colors.normal, custom_checked);
        assert_eq!(node.background_color, custom_checked);
    }

    #[test]
    fn rating_respects_custom_max_stars() {
        let mut app = test_app();
        app.world_mut().spawn_scene(Rating::new(4).max_stars(7)).unwrap();
        app.update();

        {
            let mut r_q = app.world_mut().query::<&LumaRating>();
            let rating = r_q.single(app.world()).unwrap();
            assert_eq!(rating.value, 4);
            assert_eq!(rating.max_stars, 7);
        }

        let star_count = {
            let mut s_q = app.world_mut().query::<&LumaRatingStar>();
            s_q.iter(app.world()).count()
        };
        assert_eq!(star_count, 7);
    }

    #[test]
    fn nested_accordion_isolation() {
        use crate::accordion::{LumaAccordionContent, LumaAccordionHeader, LumaAccordionItem};

        let mut app = test_app();
        app.world_mut()
            .spawn_scene(
                AccordionItem::new("Outer")
                    .expanded(false)
                    .content(AccordionItem::new("Inner").expanded(false)),
            )
            .unwrap();
        app.update();

        let (inner_item, outer_item, inner_header) = {
            let world = app.world_mut();
            let mut items: Vec<(Entity, bool)> = Vec::new();
            let mut q = world.query::<(Entity, &LumaAccordionItem)>();
            for (entity, _) in q.iter(world) {
                let mut cur = world.get::<ChildOf>(entity).map(|c| c.parent());
                let mut is_nested = false;
                while let Some(p) = cur {
                    if world.get::<LumaAccordionContent>(p).is_some() {
                        is_nested = true;
                        break;
                    }
                    cur = world.get::<ChildOf>(p).map(|c| c.parent());
                }
                items.push((entity, is_nested));
            }
            let inner_item = items.iter().find(|(_, nested)| *nested).unwrap().0;
            let outer_item = items.iter().find(|(_, nested)| !*nested).unwrap().0;
            let inner_children = world.get::<Children>(inner_item).unwrap();
            let inner_header = inner_children
                .iter()
                .find(|&c| world.get::<LumaAccordionHeader>(c).is_some())
                .unwrap();
            (inner_item, outer_item, inner_header)
        };

        app.world_mut().trigger(UFocusActivate { entity: inner_header });
        app.update();

        assert!(app.world().get::<LumaAccordionItem>(inner_item).unwrap().is_expanded);
        assert!(!app.world().get::<LumaAccordionItem>(outer_item).unwrap().is_expanded);
    }

    #[test]
    fn style_scope_inheritance_and_explicit_override() {
        let mut app = test_app();
        let scoped_btn = ButtonStyle {
            bg_normal: Color::srgb(0.15, 0.35, 0.75),
            ..default()
        };
        let override_btn = ButtonStyle {
            bg_normal: Color::srgb(0.75, 0.20, 0.30),
            ..default()
        };
        let scoped_chip = ChipStyle {
            bg_color: Color::srgb(0.22, 0.28, 0.45),
            ..default()
        };
        let scoped_toast = ToastStyle {
            bg_color: Color::srgb(0.14, 0.20, 0.28),
            ..default()
        };

        let scoped_btn_bg = scoped_btn.bg_normal;
        let override_btn_bg = override_btn.bg_normal;
        let scoped_chip_bg = scoped_chip.bg_color;
        let scoped_toast_bg = scoped_toast.bg_color;

        app.world_mut()
            .spawn_scene(luma_style_scope(
                LumaStyleScope::new()
                    .button(scoped_btn)
                    .chip(scoped_chip)
                    .toast(scoped_toast),
                bsn! {
                    UNode::default()
                    Children [
                        @{Button::new("Scoped")}
                        --
                        @{Button::new("Explicit").style(override_btn)}
                        --
                        @{Chip::new("Scoped")}
                        --
                        @{Toast::new("Scoped", "Details")}
                    ]
                },
            ))
            .unwrap();
        app.update();

        let mut saw_scoped_btn = false;
        let mut saw_explicit_btn = false;
        let btn_snapshots: Vec<(bool, Color, Vec<Entity>)> = {
            let mut btn_q = app.world_mut().query::<(&LumaButton, &UNode, &Children)>();
            btn_q
                .iter(app.world())
                .map(|(btn, node, children)| {
                    (
                        btn.has_explicit_style,
                        node.background_color,
                        children.iter().collect(),
                    )
                })
                .collect()
        };
        for (has_explicit_style, bg_color, children) in btn_snapshots {
            let labels: Vec<&str> = children
                .iter()
                .filter_map(|&c| app.world().get::<UText>(c))
                .map(|t| t.text.as_str())
                .collect();
            if labels.contains(&"Scoped") {
                saw_scoped_btn = true;
                assert!(!has_explicit_style);
                assert_eq!(bg_color, scoped_btn_bg);
            } else if labels.contains(&"Explicit") {
                saw_explicit_btn = true;
                assert!(has_explicit_style);
                assert_eq!(bg_color, override_btn_bg);
            }
        }
        assert!(saw_scoped_btn && saw_explicit_btn);

        {
            let mut chip_q = app.world_mut().query::<(&LumaChip, &UNode)>();
            let (chip, chip_node) = chip_q.single(app.world()).unwrap();
            assert!(!chip.has_explicit_style);
            assert_eq!(chip.style.bg_color, scoped_chip_bg);
            assert_eq!(chip_node.background_color, scoped_chip_bg);
        }

        {
            let mut toast_q = app.world_mut().query::<(&LumaToast, &UNode)>();
            let (toast, toast_node) = toast_q.single(app.world()).unwrap();
            assert!(!toast.has_explicit_style);
            assert_eq!(toast.style.bg_color, scoped_toast_bg);
            assert_eq!(toast_node.background_color, scoped_toast_bg);
        }
    }

    #[test]
    fn text_input_markers_and_clear_action() {
        use crate::input::{LumaInput, LumaInputClear};

        let mut app = test_app();
        #[derive(Resource, Default)]
        struct Cleared(Option<String>);
        app.init_resource::<Cleared>();
        app.add_observer(|trigger: On<UTextInputChanged>, mut res: ResMut<Cleared>| {
            res.0 = Some(trigger.value.clone());
        });

        app.world_mut()
            .spawn_scene(SearchBar::new("Find..."))
            .unwrap();
        app.update();

        let (input_ent, clear_ent, text_marker_count, caret_marker_count) = {
            let w = app.world_mut();
            let input_ent = w.query_filtered::<Entity, With<LumaInput>>().single(w).unwrap();
            let clear_ent = w.query_filtered::<Entity, With<LumaInputClear>>().single(w).unwrap();
            let tm = w.query::<&UTextInputTextMarker>().iter(w).count();
            let cm = w.query::<&UTextInputCaretMarker>().iter(w).count();
            (input_ent, clear_ent, tm, cm)
        };
        assert_eq!(text_marker_count, 1);
        assert_eq!(caret_marker_count, 1);
        assert!(app.world().get::<UTextInputCaret>(input_ent).is_some());

        app.world_mut().get_mut::<UTextInput>(input_ent).unwrap().value = "Bevy".into();
        app.world_mut().trigger(UFocusActivate { entity: clear_ent });
        app.update();
        assert_eq!(app.world().resource::<Cleared>().0.as_deref(), Some(""));
        assert_eq!(app.world().get::<UTextInput>(input_ent).unwrap().value, "");
    }

    #[test]
    fn controls_interactive_state_transitions() {
        use crate::controls::{
            Combobox, DatePicker, LumaCombobox, LumaDatePicker, LumaMultiSelect, LumaOtpBox,
            LumaOtpInput, LumaPasswordInput, LumaPasswordStrengthBar, LumaRangeMinSlider,
            LumaRangeSlider, LumaReactionItem, LumaRichTextAction, LumaTagInput, LumaTagInputField,
            LumaVoiceInput, MultiSelect, OtpCompleted, OtpInput, PasswordInput, RangeSlider,
            RangeSliderChanged, ReactionPicker, ReactionSelected, RichTextFormat,
            RichTextFormatToggled, RichTextToolbar, TagInput, TagsChanged, VoiceInputButton,
            VoiceRecordingToggled,
        };
        use crate::slider::SliderChanged;

        let mut app = test_app();

        #[derive(Resource, Default)]
        struct ControlEvents {
            range: Option<(f32, f32)>,
            reaction: Option<String>,
            rich_bold: Option<bool>,
            otp: Option<String>,
            tags: Option<Vec<String>>,
            voice: Option<bool>,
        }
        app.init_resource::<ControlEvents>();
        app.add_observer(|t: On<RangeSliderChanged>, mut ev: ResMut<ControlEvents>| {
            ev.range = Some((t.min_value, t.max_value));
        });
        app.add_observer(|t: On<ReactionSelected>, mut ev: ResMut<ControlEvents>| {
            ev.reaction = Some(t.reaction.clone());
        });
        app.add_observer(|t: On<RichTextFormatToggled>, mut ev: ResMut<ControlEvents>| {
            if t.format == RichTextFormat::Bold {
                ev.rich_bold = Some(t.active);
            }
        });
        app.add_observer(|t: On<OtpCompleted>, mut ev: ResMut<ControlEvents>| {
            ev.otp = Some(t.code.clone());
        });
        app.add_observer(|t: On<TagsChanged>, mut ev: ResMut<ControlEvents>| {
            ev.tags = Some(t.tags.clone());
        });
        app.add_observer(|t: On<VoiceRecordingToggled>, mut ev: ResMut<ControlEvents>| {
            ev.voice = Some(t.is_recording);
        });

        app.world_mut()
            .spawn_scene(bsn! {
                UNode::default()
                Children [
                    @{RangeSlider::new(25.0, 50.0, 0.0, 100.0)}
                    --
                    @{ReactionPicker::default()}
                    --
                    @{RichTextToolbar::default()}
                    --
                    @{OtpInput::new(3)}
                    --
                    @{TagInput::new("Add tag...").tags(vec!["Bevy"])}
                    --
                    @{VoiceInputButton::default()}
                    --
                    @{PasswordInput::new().placeholder("Secret").strength(true, 0.25)}
                    --
                    @{MultiSelect::new("Pick").options(vec!["Alpha", "Beta"])}
                    --
                    @{Combobox::new("Search").options(vec!["Rust", "Bevy"])}
                    --
                    @{DatePicker::default()}
                ]
            })
            .unwrap();
        app.update();

        // RangeSlider min clamps at max (50.0) when moved past 50
        let min_slider_ent = {
            let w = app.world_mut();
            let wrap = w.query_filtered::<Entity, With<LumaRangeMinSlider>>().single(w).unwrap();
            w.get::<Children>(wrap)
                .unwrap()
                .iter()
                .find(|&c| w.get::<crate::slider::LumaSlider>(c).is_some())
                .unwrap()
        };
        app.world_mut().trigger(SliderChanged {
            entity: min_slider_ent,
            value: 80.0,
        });
        app.update();
        assert_eq!(app.world().resource::<ControlEvents>().range, Some((50.0, 50.0)));
        assert_eq!(
            app.world_mut().query::<&LumaRangeSlider>().single(app.world()).unwrap().min_value,
            50.0
        );

        // ReactionPicker + RichTextToolbar + VoiceInputButton
        let (reaction_btn, bold_btn, voice_btn) = {
            let w = app.world_mut();
            let rw = w.query_filtered::<Entity, With<LumaReactionItem>>().iter(w).next().unwrap();
            let rb = w.get::<Children>(rw).unwrap().iter().next().unwrap();
            let btw = w
                .query::<(Entity, &LumaRichTextAction)>()
                .iter(w)
                .find(|(_, a)| a.format == RichTextFormat::Bold)
                .unwrap()
                .0;
            let bb = w.get::<Children>(btw).unwrap().iter().next().unwrap();
            let vw = w.query_filtered::<Entity, With<LumaVoiceInput>>().single(w).unwrap();
            let vb = w.get::<Children>(vw).unwrap().iter().next().unwrap();
            (rb, bb, vb)
        };
        app.world_mut().trigger(UFocusActivate { entity: reaction_btn });
        app.world_mut().trigger(UFocusActivate { entity: bold_btn });
        app.world_mut().trigger(UFocusActivate { entity: voice_btn });
        app.update();
        assert!(app.world().resource::<ControlEvents>().reaction.is_some());
        assert_eq!(app.world().resource::<ControlEvents>().rich_bold, Some(true));
        assert_eq!(app.world().resource::<ControlEvents>().voice, Some(true));

        // OtpInput completion
        let mut otp_boxes: Vec<(usize, Entity)> = {
            let w = app.world_mut();
            w.query::<(Entity, &LumaOtpBox)>()
                .iter(w)
                .map(|(e, b)| {
                    let inp = w.get::<Children>(e).unwrap().iter().next().unwrap();
                    (b.index, inp)
                })
                .collect()
        };
        otp_boxes.sort_by_key(|(idx, _)| *idx);
        for (idx, inp) in &otp_boxes {
            let val = (idx + 1).to_string();
            app.world_mut().get_mut::<UTextInput>(*inp).unwrap().value = val.clone();
            app.world_mut().trigger(UTextInputChanged {
                entity: *inp,
                value: val,
            });
        }
        app.update();
        assert_eq!(app.world().resource::<ControlEvents>().otp.as_deref(), Some("123"));
        assert_eq!(
            app.world_mut().query::<&LumaOtpInput>().single(app.world()).unwrap().code,
            "123"
        );

        // TagInput add via submit
        let tag_field = {
            let w = app.world_mut();
            let wrap = w.query_filtered::<Entity, With<LumaTagInputField>>().single(w).unwrap();
            w.get::<Children>(wrap).unwrap().iter().next().unwrap()
        };
        app.world_mut().trigger(UTextInputSubmit {
            entity: tag_field,
            value: "Luma".into(),
        });
        app.update();
        assert_eq!(
            app.world().resource::<ControlEvents>().tags.as_ref(),
            Some(&vec!["Bevy".to_string(), "Luma".to_string()])
        );
        assert_eq!(
            app.world_mut().query::<&LumaTagInput>().single(app.world()).unwrap().tags.len(),
            2
        );

        // PasswordInput live strength bar update
        let (pw_input, bar_ent) = {
            let w = app.world_mut();
            let pw_wrap = w.query_filtered::<Entity, With<LumaPasswordInput>>().single(w).unwrap();
            let bar_ent = w.query_filtered::<Entity, With<LumaPasswordStrengthBar>>().single(w).unwrap();
            let inp = w
                .get::<Children>(pw_wrap)
                .unwrap()
                .iter()
                .find(|&c| w.get::<UTextInput>(c).is_some())
                .unwrap();
            (inp, bar_ent)
        };
        app.world_mut().trigger(UTextInputChanged {
            entity: pw_input,
            value: "Strong#Pass99!".into(),
        });
        app.update();
        let bar_w = app.world().get::<UNode>(bar_ent).unwrap().width;
        assert_eq!(bar_w, UVal::Percent(1.0));

        // MultiSelect, Combobox, DatePicker overlay open toggle
        let (ms_ent, cb_ent, dp_btn) = {
            let w = app.world_mut();
            let ms = w.query_filtered::<Entity, With<LumaMultiSelect>>().single(w).unwrap();
            let cb = w.query_filtered::<Entity, With<LumaCombobox>>().single(w).unwrap();
            let dp = w.query_filtered::<Entity, With<LumaDatePicker>>().single(w).unwrap();
            let dp_btn = w.get::<Children>(dp).unwrap().iter().next().unwrap();
            (ms, cb, dp_btn)
        };
        app.world_mut().trigger(UFocusActivate { entity: ms_ent });
        app.world_mut().trigger(UFocusActivate { entity: cb_ent });
        app.world_mut().trigger(UFocusActivate { entity: dp_btn });
        app.update();
        assert!(app.world_mut().query::<&LumaMultiSelect>().single(app.world()).unwrap().is_open);
        assert!(app.world_mut().query::<&LumaCombobox>().single(app.world()).unwrap().is_open);
        assert!(app.world_mut().query::<&LumaDatePicker>().single(app.world()).unwrap().is_open);
    }

    #[test]
    fn navigation_widgets_interactive_behaviors() {
        use crate::navigation::{
            BottomNav, BottomNavChanged, BottomSheet, BottomSheetToggled, CarouselControls,
            CarouselChanged, FilterSortBar, FilterSortChanged, HamburgerMenu, HamburgerToggled,
            LumaBottomNav, LumaBottomNavItem, LumaBottomSheet, LumaBottomSheetHandle,
            LumaCarouselControls, LumaCarouselDot, LumaFilterChipItem, LumaFilterSortBar,
            LumaHamburgerMenu, LumaNavbar, LumaNavbarItem, LumaPagination, LumaPaginationBtn,
            LumaSidebar, LumaSidebarItem, LumaStepItem, LumaStepTracker, LumaTableOfContents,
            LumaThemeToggle, LumaTocItem, LumaTreeNode, LumaTreeView, Navbar, NavbarItemSelected,
            PageChanged, Pagination, PaginationAction, Sidebar, SidebarItemSelected, StepSelected,
            StepTracker, TableOfContents, ThemeToggle, ThemeToggled, TocSectionSelected,
            TreeNodeToggled, TreeView,
        };

        let mut app = test_app();

        #[derive(Resource, Default)]
        struct NavCounts {
            nav: usize,
            bnav: usize,
            side: usize,
            page: usize,
            step: usize,
            ham: Option<bool>,
            sheet: Option<bool>,
            tree_toggled: Option<bool>,
            carousel: Option<usize>,
            toc: Option<usize>,
            filter: Option<String>,
            dark: Option<bool>,
        }
        app.init_resource::<NavCounts>();
        app.add_observer(|t: On<NavbarItemSelected>, mut r: ResMut<NavCounts>| r.nav = t.index);
        app.add_observer(|t: On<BottomNavChanged>, mut r: ResMut<NavCounts>| r.bnav = t.index);
        app.add_observer(|t: On<SidebarItemSelected>, mut r: ResMut<NavCounts>| r.side = t.index);
        app.add_observer(|t: On<PageChanged>, mut r: ResMut<NavCounts>| r.page = t.page);
        app.add_observer(|t: On<StepSelected>, mut r: ResMut<NavCounts>| r.step = t.index);
        app.add_observer(|t: On<HamburgerToggled>, mut r: ResMut<NavCounts>| r.ham = Some(t.is_open));
        app.add_observer(|t: On<BottomSheetToggled>, mut r: ResMut<NavCounts>| {
            r.sheet = Some(t.expanded)
        });
        app.add_observer(|t: On<TreeNodeToggled>, mut r: ResMut<NavCounts>| {
            r.tree_toggled = Some(t.expanded)
        });
        app.add_observer(|t: On<CarouselChanged>, mut r: ResMut<NavCounts>| {
            r.carousel = Some(t.index)
        });
        app.add_observer(|t: On<TocSectionSelected>, mut r: ResMut<NavCounts>| r.toc = Some(t.index));
        app.add_observer(|t: On<FilterSortChanged>, mut r: ResMut<NavCounts>| {
            r.filter = Some(t.active_filter.clone())
        });
        app.add_observer(|t: On<ThemeToggled>, mut r: ResMut<NavCounts>| r.dark = Some(t.is_dark));

        app.world_mut()
            .spawn_scene(bsn! {
                UNode::default()
                Children [
                    @{Navbar::new("Luma")}
                    --
                    @{BottomNav::new()}
                    --
                    @{Sidebar::new()}
                    --
                    @{Pagination::new(1, 5)}
                    --
                    @{StepTracker::default()}
                    --
                    @{HamburgerMenu::new()}
                    --
                    @{BottomSheet::default()}
                    --
                    @{TreeView::new()}
                    --
                    @{CarouselControls::new(4, 0)}
                    --
                    @{TableOfContents::default()}
                    --
                    @{FilterSortBar::default()}
                    --
                    @{ThemeToggle::new()}
                ]
            })
            .unwrap();
        app.update();

        let (nav_i2, bnav_i1, side_i2, next_page_btn, step_i2, ham_btn, sheet_handle, folder_node, dot_i3, toc_i1, feat_chip, theme_btn) = {
            let w = app.world_mut();
            let nav_i2 = w.query::<(Entity, &LumaNavbarItem)>().iter(w).find(|(_, x)| x.index == 2).unwrap().0;
            let bnav_i1 = w.query::<(Entity, &LumaBottomNavItem)>().iter(w).find(|(_, x)| x.index == 1).unwrap().0;
            let side_i2 = w.query::<(Entity, &LumaSidebarItem)>().iter(w).find(|(_, x)| x.index == 2).unwrap().0;
            let p_wrap = w
                .query::<(Entity, &LumaPaginationBtn)>()
                .iter(w)
                .find(|(_, x)| x.action == PaginationAction::Next)
                .unwrap()
                .0;
            let next_page_btn = w.get::<Children>(p_wrap).unwrap().iter().next().unwrap();
            let step_i2 = w.query::<(Entity, &LumaStepItem)>().iter(w).find(|(_, x)| x.index == 2).unwrap().0;
            let h_wrap = w.query_filtered::<Entity, With<LumaHamburgerMenu>>().single(w).unwrap();
            let ham_btn = w.get::<Children>(h_wrap).unwrap().iter().next().unwrap();
            let sheet_handle = w.query_filtered::<Entity, With<LumaBottomSheetHandle>>().single(w).unwrap();
            let folder_node = w
                .query::<(Entity, &LumaTreeNode)>()
                .iter(w)
                .find(|(_, x)| x.index == 0 && x.is_folder)
                .unwrap()
                .0;
            let dot_i3 = w.query::<(Entity, &LumaCarouselDot)>().iter(w).find(|(_, x)| x.index == 3).unwrap().0;
            let toc_i1 = w.query::<(Entity, &LumaTocItem)>().iter(w).find(|(_, x)| x.index == 1).unwrap().0;
            let feat_chip = w
                .query::<(Entity, &LumaFilterChipItem)>()
                .iter(w)
                .find(|(_, x)| x.label == "Featured")
                .unwrap()
                .0;
            let t_wrap = w.query_filtered::<Entity, With<LumaThemeToggle>>().single(w).unwrap();
            let theme_btn = w.get::<Children>(t_wrap).unwrap().iter().next().unwrap();
            (nav_i2, bnav_i1, side_i2, next_page_btn, step_i2, ham_btn, sheet_handle, folder_node, dot_i3, toc_i1, feat_chip, theme_btn)
        };

        for e in [
            nav_i2,
            bnav_i1,
            side_i2,
            next_page_btn,
            step_i2,
            ham_btn,
            sheet_handle,
            folder_node,
            dot_i3,
            toc_i1,
            feat_chip,
            theme_btn,
        ] {
            app.world_mut().trigger(UFocusActivate { entity: e });
        }
        app.update();

        let counts = app.world().resource::<NavCounts>();
        assert_eq!(counts.nav, 2);
        assert_eq!(counts.bnav, 1);
        assert_eq!(counts.side, 2);
        assert_eq!(counts.page, 2);
        assert_eq!(counts.step, 2);
        assert_eq!(counts.ham, Some(true));
        assert_eq!(counts.sheet, Some(false));
        assert_eq!(counts.tree_toggled, Some(false));
        assert_eq!(counts.carousel, Some(3));
        assert_eq!(counts.toc, Some(1));
        assert_eq!(counts.filter.as_deref(), Some("Featured"));
        assert_eq!(counts.dark, Some(false));

        assert_eq!(app.world_mut().query::<&LumaNavbar>().single(app.world()).unwrap().active_index, 2);
        assert_eq!(app.world_mut().query::<&LumaBottomNav>().single(app.world()).unwrap().active_index, 1);
        assert_eq!(app.world_mut().query::<&LumaSidebar>().single(app.world()).unwrap().active_index, 2);
        assert_eq!(app.world_mut().query::<&LumaPagination>().single(app.world()).unwrap().current_page, 2);
        assert_eq!(app.world_mut().query::<&LumaStepTracker>().single(app.world()).unwrap().current_step, 2);
        assert!(app.world_mut().query::<&LumaHamburgerMenu>().single(app.world()).unwrap().is_open);
        assert!(!app.world_mut().query::<&LumaBottomSheet>().single(app.world()).unwrap().expanded);
        assert_eq!(app.world_mut().query::<&LumaCarouselControls>().single(app.world()).unwrap().current, 3);
        assert_eq!(app.world_mut().query::<&LumaTableOfContents>().single(app.world()).unwrap().active_index, 1);
        assert_eq!(app.world_mut().query::<&LumaFilterSortBar>().single(app.world()).unwrap().active_filter, "Featured");
        assert!(!app.world_mut().query::<&LumaThemeToggle>().single(app.world()).unwrap().is_dark);

        // TreeView child nodes under index 0 ("src") must now be hidden
        let tree_ent = app.world_mut().query_filtered::<Entity, With<LumaTreeView>>().single(app.world()).unwrap();
        let tree_rows: Vec<Entity> = app.world().get::<Children>(tree_ent).unwrap().iter().collect();
        let child_main_rs = tree_rows
            .iter()
            .find(|&&e| app.world().get::<LumaTreeNode>(e).is_some_and(|n| n.index == 1))
            .copied()
            .unwrap();
        assert_eq!(app.world().get::<ULayout>(child_main_rs).unwrap().display, UDisplay::None);
    }

    #[test]
    fn cards_states_and_data_views_interactive_behaviors() {
        use crate::cards::{
            CodeBlock, CodeCopied, LumaCodeBlock, LumaCodeCopyBtn, LumaProfileFollowBtn,
            LumaSocialFeedPost, LumaSocialLikeBtn, LumaUserProfileCard, SocialFeedPost,
            SocialPostLiked, UserProfileCard, UserProfileFollowToggled,
        };
        use crate::data_views::{
            CartCheckoutClicked, CartDrawer, DataTable, DataTableRowSelected, DataTableSorted,
            ImageLightbox, LightboxClosed, LumaCartCheckoutBtn, LumaDataTable, LumaDataTableHeader,
            LumaDataTableRow, LumaImageLightbox, LumaLightboxCloseBtn, LumaMediaBtn,
            LumaMediaPlayer, LumaNotificationItem, MediaAction, MediaPlaybackToggled, MediaPlayer,
            NotificationCenter, NotificationRead,
        };
        use crate::states::{
            Coachmark, CoachmarkNext, CookieConsent, CookieConsentResponded, LumaCoachmark,
            LumaCoachmarkAction, LumaCoachmarkStepText, LumaCookieConsent, LumaCookieConsentAction,
        };

        let mut app = test_app();

        #[derive(Resource, Default)]
        struct ActionLog {
            followed: Option<bool>,
            likes: Option<usize>,
            copied: Option<String>,
            coach_step: Option<usize>,
            cookie_ok: Option<bool>,
            sorted_col: Option<usize>,
            table_row: Option<usize>,
            playing: Option<bool>,
            lightbox_closed: bool,
            notif_read: Option<usize>,
            checkout_clicked: bool,
        }
        app.init_resource::<ActionLog>();
        app.add_observer(|t: On<UserProfileFollowToggled>, mut l: ResMut<ActionLog>| {
            l.followed = Some(t.is_following);
        });
        app.add_observer(|t: On<SocialPostLiked>, mut l: ResMut<ActionLog>| {
            l.likes = Some(t.likes);
        });
        app.add_observer(|t: On<CodeCopied>, mut l: ResMut<ActionLog>| {
            l.copied = Some(t.code.clone());
        });
        app.add_observer(|t: On<CoachmarkNext>, mut l: ResMut<ActionLog>| {
            l.coach_step = Some(t.step);
        });
        app.add_observer(|t: On<CookieConsentResponded>, mut l: ResMut<ActionLog>| {
            l.cookie_ok = Some(t.accepted);
        });
        app.add_observer(|t: On<DataTableSorted>, mut l: ResMut<ActionLog>| {
            l.sorted_col = Some(t.column);
        });
        app.add_observer(|t: On<DataTableRowSelected>, mut l: ResMut<ActionLog>| {
            l.table_row = Some(t.row_index);
        });
        app.add_observer(|t: On<MediaPlaybackToggled>, mut l: ResMut<ActionLog>| {
            l.playing = Some(t.is_playing);
        });
        app.add_observer(|_: On<LightboxClosed>, mut l: ResMut<ActionLog>| {
            l.lightbox_closed = true;
        });
        app.add_observer(|t: On<NotificationRead>, mut l: ResMut<ActionLog>| {
            l.notif_read = Some(t.index);
        });
        app.add_observer(|_: On<CartCheckoutClicked>, mut l: ResMut<ActionLog>| {
            l.checkout_clicked = true;
        });

        app.world_mut()
            .spawn_scene(bsn! {
                UNode::default()
                Children [
                    @{UserProfileCard::new("Alex", "@alex").followers("100")}
                    --
                    @{SocialFeedPost::default()}
                    --
                    @{CodeBlock::default()}
                    --
                    @{Coachmark::default()}
                    --
                    @{CookieConsent::default()}
                    --
                    @{DataTable::default()}
                    --
                    @{MediaPlayer::default().playing(false)}
                    --
                    @{ImageLightbox::default()}
                    --
                    @{NotificationCenter::default()}
                    --
                    @{CartDrawer::default()}
                ]
            })
            .unwrap();
        app.update();

        let (follow_btn, like_btn, copy_btn, next_btn, accept_btn, hdr0, row1, play_btn, lb_close_btn, notif0, checkout_btn) = {
            let w = app.world_mut();
            let fw = w.query_filtered::<Entity, With<LumaProfileFollowBtn>>().single(w).unwrap();
            let follow_btn = w.get::<Children>(fw).unwrap().iter().next().unwrap();
            let lw = w.query_filtered::<Entity, With<LumaSocialLikeBtn>>().single(w).unwrap();
            let like_btn = w.get::<Children>(lw).unwrap().iter().next().unwrap();
            let cw = w.query_filtered::<Entity, With<LumaCodeCopyBtn>>().single(w).unwrap();
            let copy_btn = w.get::<Children>(cw).unwrap().iter().next().unwrap();
            let nw = w
                .query::<(Entity, &LumaCoachmarkAction)>()
                .iter(w)
                .find(|(_, a)| **a == LumaCoachmarkAction::Next)
                .unwrap()
                .0;
            let next_btn = w.get::<Children>(nw).unwrap().iter().next().unwrap();
            let aw = w
                .query::<(Entity, &LumaCookieConsentAction)>()
                .iter(w)
                .find(|(_, a)| **a == LumaCookieConsentAction::Accept)
                .unwrap()
                .0;
            let accept_btn = w.get::<Children>(aw).unwrap().iter().next().unwrap();
            let hdr0 = w.query::<(Entity, &LumaDataTableHeader)>().iter(w).find(|(_, h)| h.col == 0).unwrap().0;
            let row1 = w.query::<(Entity, &LumaDataTableRow)>().iter(w).find(|(_, r)| r.row_index == 1).unwrap().0;
            let pw = w
                .query::<(Entity, &LumaMediaBtn)>()
                .iter(w)
                .find(|(_, b)| b.action == MediaAction::PlayPause)
                .unwrap()
                .0;
            let play_btn = w.get::<Children>(pw).unwrap().iter().next().unwrap();
            let lbw = w.query_filtered::<Entity, With<LumaLightboxCloseBtn>>().single(w).unwrap();
            let lb_close_btn = w.get::<Children>(lbw).unwrap().iter().next().unwrap();
            let notif0 = w.query::<(Entity, &LumaNotificationItem)>().iter(w).find(|(_, n)| n.index == 0).unwrap().0;
            let cow = w.query_filtered::<Entity, With<LumaCartCheckoutBtn>>().single(w).unwrap();
            let checkout_btn = w.get::<Children>(cow).unwrap().iter().next().unwrap();
            (follow_btn, like_btn, copy_btn, next_btn, accept_btn, hdr0, row1, play_btn, lb_close_btn, notif0, checkout_btn)
        };

        for e in [
            follow_btn,
            like_btn,
            copy_btn,
            next_btn,
            accept_btn,
            hdr0,
            row1,
            play_btn,
            lb_close_btn,
            notif0,
            checkout_btn,
        ] {
            app.world_mut().trigger(UFocusActivate { entity: e });
        }
        app.update();

        let log = app.world().resource::<ActionLog>();
        assert_eq!(log.followed, Some(true));
        assert_eq!(log.likes, Some(43));
        assert!(log.copied.as_deref().is_some_and(|c| !c.is_empty()));
        assert_eq!(log.coach_step, Some(2));
        assert_eq!(log.cookie_ok, Some(true));
        assert_eq!(log.sorted_col, Some(0));
        assert_eq!(log.table_row, Some(1));
        assert_eq!(log.playing, Some(true));
        assert!(log.lightbox_closed);
        assert_eq!(log.notif_read, Some(0));
        assert!(log.checkout_clicked);

        assert!(app.world_mut().query::<&LumaUserProfileCard>().single(app.world()).unwrap().is_following);
        assert!(app.world_mut().query::<&LumaSocialFeedPost>().single(app.world()).unwrap().liked);
        assert!(app.world_mut().query::<&LumaCodeBlock>().single(app.world()).unwrap().copied);
        assert!(app.world_mut().query::<&LumaMediaPlayer>().single(app.world()).unwrap().is_playing);
        assert_eq!(app.world_mut().query::<&LumaDataTable>().single(app.world()).unwrap().sort_col, Some(0));
        assert_eq!(app.world_mut().query::<&LumaDataTable>().single(app.world()).unwrap().selected_row, Some(1));

        let step_txt = app
            .world_mut()
            .query_filtered::<&UText, With<LumaCoachmarkStepText>>()
            .single(app.world())
            .unwrap()
            .text
            .clone();
        assert_eq!(step_txt, "Step 2 of 3");

        let cm_ent = app.world_mut().query_filtered::<Entity, With<LumaCoachmark>>().single(app.world()).unwrap();
        assert_eq!(app.world().get::<ULayout>(cm_ent).unwrap().display, UDisplay::Flex);
        let cc_ent = app.world_mut().query_filtered::<Entity, With<LumaCookieConsent>>().single(app.world()).unwrap();
        assert_eq!(app.world().get::<ULayout>(cc_ent).unwrap().display, UDisplay::None);
        let lb_ent = app.world_mut().query_filtered::<Entity, With<LumaImageLightbox>>().single(app.world()).unwrap();
        assert_eq!(app.world().get::<ULayout>(lb_ent).unwrap().display, UDisplay::None);
    }
}


