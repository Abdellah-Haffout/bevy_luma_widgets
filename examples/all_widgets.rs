//! Master Gallery showcasing ALL widgets in bevy_luma_widgets.
//!
//! Organized into 5 cohesive interactive tabs powered by Bevy Scene Notation (`bsn!`),
//! reactive ECS observers, and hierarchical theming (`LumaStyleScope`).

use bevy::prelude::*;
use bevy_luma_widgets::prelude::*;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins.set(WindowPlugin {
            primary_window: Some(Window {
                title: "Luma Widgets — Complete Component Suite (All 80+ Widgets)".into(),
                resolution: (1280, 1020).into(),
                ..default()
            }),
            ..default()
        }))
        .add_plugins(LumaWidgetsPlugin)
        .add_systems(Startup, setup_all_widgets_gallery)
        .add_observer(|trigger: On<ButtonClicked>| {
            info!("ButtonClicked: {:?}", trigger.event().0);
        })
        .add_observer(|trigger: On<ToggleChanged>| {
            info!("ToggleChanged: checked={}", trigger.event().is_checked);
        })
        .add_observer(|trigger: On<CheckboxChanged>| {
            info!("CheckboxChanged: checked={}", trigger.event().is_checked);
        })
        .add_observer(|trigger: On<RadioChanged>| {
            info!("RadioChanged: group={} val={}", trigger.event().group, trigger.event().value);
        })
        .add_observer(|trigger: On<SliderChanged>| {
            info!("SliderChanged: val={:.2}", trigger.event().value);
        })
        .add_observer(|trigger: On<DragValueChanged>| {
            info!("DragValueChanged: val={:.2}", trigger.event().value);
        })
        .add_observer(|trigger: On<TabSelected>| {
            info!("TabSelected: group={} val={}", trigger.event().group, trigger.event().value);
        })
        .run();
}

fn setup_all_widgets_gallery(mut commands: Commands, theme: Res<Theme>) {
    commands.spawn(Camera2d);
    commands.spawn_scene(all_widgets_scene(&theme));
}

fn all_widgets_scene(theme: &Theme) -> impl Scene + use<> {
    let font = theme.text.font.inter_regular.clone();
    let icon_font = theme.icon.font.clone();

    bsn! {
        URootUi::screen()
        UNode {
            width: UVal::Percent(1.0),
            height: UVal::Percent(1.0),
            background_color: Color::srgb(0.05, 0.06, 0.08),
            padding: USides::all(20.0),
        }
        ULayout {
            display: UDisplay::Flex,
            flex_direction: UFlexDirection::Column,
            align_items: UAlignItems::Center,
            gap: 12.0,
        }
        Children [
            // Header Bar & Navigation Trail
            UNode::default()
            ULayout {
                display: UDisplay::Flex,
                flex_direction: UFlexDirection::Column,
                align_items: UAlignItems::Center,
                gap: 4.0,
            }
            Children [
                UText {
                    text: { "Luma Widgets Master Gallery (All 80+ Widgets)".to_string() },
                    font_size: 24.0,
                    font: { font.clone() },
                    color: Color::WHITE,
                }
                --
                @{
                    luma_breadcrumb_trail((
                        luma_breadcrumb_item(0, "univ", "Univis", false, font.clone()),
                        luma_breadcrumb_separator(icon_font.clone()),
                        luma_breadcrumb_item(1, "luma", "bevy_luma", false, font.clone()),
                        luma_breadcrumb_separator(icon_font.clone()),
                        luma_breadcrumb_item(2, "widgets", "Complete Component Suite", true, font.clone())
                    ))
                }
            ]
            --
            // Top Row: KPI Stat Cards & Overview
            UNode {
                width: UVal::Px(1200.0),
            }
            ULayout {
                display: UDisplay::Flex,
                flex_direction: UFlexDirection::Row,
                justify_content: UJustifyContent::SpaceBetween,
                gap: 12.0,
            }
            Children [
                @{ luma_stat_card_with_trend(
                    "Total UI Components",
                    "80+ Widgets",
                    StatTrend::Up("+100% Core".into()),
                    font.clone(),
                    icon_font.clone(),
                ) }
                --
                @{ luma_stat_card_with_trend(
                    "Design Tokens & Presets",
                    "100+",
                    StatTrend::Up("Styles Ready".into()),
                    font.clone(),
                    icon_font.clone(),
                ) }
                --
                @{ luma_stat_card_full(
                    "Framework Engine",
                    "Bevy 0.19",
                    StatTrend::Up("SDF Shaders".into()),
                    "Hardware anti-aliased rendering",
                    Some(Icon::CPU),
                    font.clone(),
                    icon_font.clone(),
                ) }
                --
                // Header Notification & Keycap quick bar
                UNode {
                    width: UVal::Px(280.0),
                    background_color: Color::srgb(0.10, 0.12, 0.16),
                    border_radius: UCornerRadius::all(12.0),
                    padding: USides::all(14.0),
                }
                UBorder {
                    color: Color::srgb(0.18, 0.22, 0.30),
                    width: 1.0,
                    radius: UCornerRadius::all(12.0),
                }
                ULayout {
                    display: UDisplay::Flex,
                    flex_direction: UFlexDirection::Row,
                    align_items: UAlignItems::Center,
                    justify_content: UJustifyContent::SpaceBetween,
                }
                Children [
                    @{ Badge::counter(38).font(font.clone()) }
                    --
                    @{ luma_kbd_shortcut((
                        luma_kbd("Ctrl", KbdSize::Small, font.clone()),
                        luma_kbd_separator(font.clone()),
                        luma_kbd("K", KbdSize::Small, font.clone())
                    )) }
                    --
                    @{ Tooltip::new("Global Palette").font(font.clone()) }
                ]
            ]
            --
            // Gallery Tab Bar Trigger Switcher (Interactive)
            UNode {
                width: UVal::Px(1200.0),
            }
            ULayout {
                display: UDisplay::Flex,
                flex_direction: UFlexDirection::Row,
                justify_content: UJustifyContent::Center,
            }
            Children [
                @{
                    luma_tab_list((
                        luma_tab_trigger("gallery", "controls", "Controls & Forms", true, font.clone()),
                        luma_tab_trigger("gallery", "cards", "Cards & Surfaces", false, font.clone()),
                        luma_tab_trigger("gallery", "data_views", "Data Views & Metrics", false, font.clone()),
                        luma_tab_trigger("gallery", "navigation", "Navigation & Shells", false, font.clone()),
                        luma_tab_trigger("gallery", "states", "States & Feedback", false, font.clone())
                    ))
                }
            ]
            --
            // 5 Switchable Tab Panels
            @{ controls_tab_scene(font.clone(), icon_font.clone()) }
            --
            @{ cards_tab_scene(font.clone(), icon_font.clone()) }
            --
            @{ data_views_tab_scene(font.clone(), icon_font.clone()) }
            --
            @{ navigation_tab_scene(font.clone(), icon_font.clone()) }
            --
            @{ states_tab_scene(font, icon_font) }
        ]
    }
}

// ============================================================================
// TAB 1: CONTROLS & FORMS (Buttons, Inputs, Sliders, DragValue, Pickers)
// ============================================================================

fn controls_tab_scene(font: Handle<Font>, icon_font: Handle<Font>) -> impl Scene + use<> {
    bsn! {
        UNode {
            width: UVal::Px(1200.0),
        }
        LumaTabContent {
            group: { "gallery".to_string() },
            value: { "controls".to_string() },
        }
        ULayout {
            display: UDisplay::Flex,
            flex_direction: UFlexDirection::Row,
            justify_content: UJustifyContent::SpaceBetween,
            gap: 16.0,
        }
        Children [
            // Left Column: Buttons, Inputs, Choices
            UNode {
                width: UVal::Px(590.0),
                background_color: Color::srgb(0.09, 0.11, 0.15),
                border_radius: UCornerRadius::all(14.0),
                padding: USides::all(16.0),
            }
            UBorder {
                color: Color::srgb(0.18, 0.22, 0.30),
                width: 1.0,
                radius: UCornerRadius::all(14.0),
            }
            ULayout {
                display: UDisplay::Flex,
                flex_direction: UFlexDirection::Column,
                gap: 12.0,
            }
            Children [
                UText {
                    text: { "1. Button Suite, FAB & Voice Input".to_string() },
                    font_size: 15.0,
                    font: { font.clone() },
                    color: Color::srgb(0.40, 0.75, 1.0),
                }
                --
                UNode::default()
                ULayout {
                    display: UDisplay::Flex,
                    flex_direction: UFlexDirection::Row,
                    align_items: UAlignItems::Center,
                    gap: 6.0,
                }
                Children [
                    @{ Button::new("Primary").variant(ButtonVariant::Primary).small().font(font.clone()) }
                    --
                    @{ Button::new("Secondary").variant(ButtonVariant::Secondary).small().font(font.clone()) }
                    --
                    @{ Button::new("Outline").variant(ButtonVariant::Outline).small().font(font.clone()) }
                    --
                    @{ Button::new("Loading").loading(true).small().font(font.clone()) }
                    --
                    @{ IconButton::new(Icon::STAR).font(font.clone()).icon_font(icon_font.clone()) }
                    --
                    @{ IconButton::new(Icon::CIRCLE_CHECK_BIG).circle().font(font.clone()).icon_font(icon_font.clone()) }
                    --
                    @{ Fab::new(Icon::PLUS).font(font.clone()).icon_font(icon_font.clone()) }
                    --
                    @{ VoiceInputButton::new() }
                ]
                --
                UText {
                    text: { "2. TextInput, SearchBar, Password & Credentials".to_string() },
                    font_size: 15.0,
                    font: { font.clone() },
                    color: Color::srgb(0.40, 0.75, 1.0),
                }
                --
                @{ SearchBar::new("Search components, documentation, assets...").font(font.clone()).icon_font(icon_font.clone()) }
                --
                @{ PasswordInput::new().value("LumaSecretPass2026!").strength(true, 0.85).font(font.clone()).icon_font(icon_font.clone()) }
                --
                UNode::default()
                ULayout {
                    display: UDisplay::Flex,
                    flex_direction: UFlexDirection::Row,
                    gap: 10.0,
                }
                Children [
                    UNode { width: UVal::Px(120.0) }
                    Children [
                        @{ OtpInput { code: "849201".into(), length: 6, font: font.clone() } }
                    ]
                    --
                    UNode { width: UVal::Px(420.0) }
                    Children [
                        @{ PhoneInput { country_code: "+1".into(), number: "555-0199".into(), font: font.clone() } }
                    ]
                ]
                --
                @{ CreditCardInput { font: font.clone(), icon_font: icon_font.clone() } }
                --
                @{ TextArea::new("Multiline design notes and shader compiler parameters...").font(font.clone()) }
                --
                UText {
                    text: { "3. Form Choices: Toggles, Checkboxes & Radios".to_string() },
                    font_size: 15.0,
                    font: { font.clone() },
                    color: Color::srgb(0.40, 0.75, 1.0),
                }
                --
                UNode::default()
                ULayout {
                    display: UDisplay::Flex,
                    flex_direction: UFlexDirection::Row,
                    justify_content: UJustifyContent::SpaceBetween,
                    align_items: UAlignItems::Center,
                }
                Children [
                    @{ Toggle::new(true).label("Auto-Sync").font(font.clone()) }
                    --
                    @{ Checkbox::new("Dark SDF", true).font(font.clone()).icon_font(icon_font.clone()) }
                    --
                    @{ Radio::new("render_tier", "gpu").label("GPU Accelerated").selected(true).font(font.clone()) }
                ]
            ]
            --
            // Right Column: Dropdowns, Sliders, DragValue, Pickers
            UNode {
                width: UVal::Px(590.0),
                background_color: Color::srgb(0.09, 0.11, 0.15),
                border_radius: UCornerRadius::all(14.0),
                padding: USides::all(16.0),
            }
            UBorder {
                color: Color::srgb(0.18, 0.22, 0.30),
                width: 1.0,
                radius: UCornerRadius::all(14.0),
            }
            ULayout {
                display: UDisplay::Flex,
                flex_direction: UFlexDirection::Column,
                gap: 12.0,
            }
            Children [
                UText {
                    text: { "4. Dropdowns, Combobox, MultiSelect & Tags".to_string() },
                    font_size: 15.0,
                    font: { font.clone() },
                    color: Color::srgb(0.40, 0.75, 1.0),
                }
                --
                UNode::default()
                ULayout {
                    display: UDisplay::Flex,
                    flex_direction: UFlexDirection::Row,
                    gap: 10.0,
                }
                Children [
                    UNode { width: UVal::Px(270.0) }
                    Children [
                        @{
                            Select::new(
                                "Select Cluster...",
                                vec![
                                    SelectOption::new("Production (US-East)", "prod"),
                                    SelectOption::new("Staging Cluster", "stage"),
                                    SelectOption::new("Local Sandbox", "local"),
                                ]
                            )
                            .selected(0)
                            .font(font.clone())
                            .icon_font(icon_font.clone())
                        }
                    ]
                    --
                    UNode { width: UVal::Px(270.0) }
                    Children [
                        @{ MultiSelect::new("Framework tags...").tags(vec!["Bevy".into(), "SDF".into(), "Rust".into()]).options(vec!["Bevy", "SDF", "Rust", "UI", "Shaders", "WGSL"]).font(font.clone()).icon_font(icon_font.clone()) }
                    ]
                ]
                --
                UNode::default()
                ULayout {
                    display: UDisplay::Flex,
                    flex_direction: UFlexDirection::Row,
                    gap: 10.0,
                }
                Children [
                    UNode { width: UVal::Px(270.0) }
                    Children [
                        @{ Combobox::new("Filter branch...").options(vec!["main", "develop", "feat/sdf-ui", "release/v1.0"]).font(font.clone()).icon_font(icon_font.clone()) }
                    ]
                    --
                    UNode { width: UVal::Px(270.0) }
                    Children [
                        @{ TagInput { placeholder: "Add release tags...".into(), font: font.clone(), icon_font: icon_font.clone(), ..default() } }
                    ]
                ]
                --
                @{
                    luma_segmented_control(
                        "tab_mode",
                        "code",
                        (
                            luma_segment_option("tab_mode", "code", "Code", true, font.clone()),
                            luma_segment_option("tab_mode", "preview", "Preview", false, font.clone()),
                            luma_segment_option("tab_mode", "docs", "Docs", false, font.clone())
                        )
                    )
                }
                --
                UText {
                    text: { "5. Ranges, Steppers, Ratings & DragValue Scrubber".to_string() },
                    font_size: 15.0,
                    font: { font.clone() },
                    color: Color::srgb(0.40, 0.75, 1.0),
                }
                --
                UNode::default()
                ULayout {
                    display: UDisplay::Flex,
                    flex_direction: UFlexDirection::Row,
                    justify_content: UJustifyContent::SpaceBetween,
                    align_items: UAlignItems::Center,
                }
                Children [
                    @{ Slider::new(75.0, 0.0, 100.0, 130.0) }
                    --
                    @{ RangeSlider::new(20.0, 80.0, 0.0, 100.0).width(130.0) }
                    --
                    @{ luma_stepper(4.0, 0.0, 10.0, 1.0, font.clone(), icon_font.clone()) }
                    --
                    @{ luma_interactive_rating(5, icon_font.clone()) }
                ]
                --
                // DragValue Row showcasing numeric scrubbers
                UNode::default()
                ULayout {
                    display: UDisplay::Flex,
                    flex_direction: UFlexDirection::Row,
                    justify_content: UJustifyContent::SpaceBetween,
                    align_items: UAlignItems::Center,
                }
                Children [
                    @{ DragValue::new(120.0).prefix("X: ").suffix(" px").speed(1.0).font(font.clone()).icon_font(icon_font.clone()) }
                    --
                    @{ DragValue::new(45.5).prefix("Rot: ").suffix("°").range(0.0, 360.0).speed(0.5).font(font.clone()).icon_font(icon_font.clone()) }
                    --
                    @{ DragValue::new(1.25).prefix("Scale: ").range(0.1, 5.0).speed(0.05).decimals(2).font(font.clone()).icon_font(icon_font.clone()) }
                    --
                    @{ DragValue::new(80.0).prefix("Alpha: ").suffix("%").range(0.0, 100.0).speed(1.0).decimals(0).font(font.clone()).icon_font(icon_font.clone()) }
                ]
                --
                UText {
                    text: { "6. Pickers, File Uploader & Rich Toolbar".to_string() },
                    font_size: 15.0,
                    font: { font.clone() },
                    color: Color::srgb(0.40, 0.75, 1.0),
                }
                --
                UNode::default()
                ULayout {
                    display: UDisplay::Flex,
                    flex_direction: UFlexDirection::Row,
                    justify_content: UJustifyContent::SpaceBetween,
                    align_items: UAlignItems::Center,
                }
                Children [
                    @{ DatePicker { date: "2026-10-28".into(), font: font.clone(), icon_font: icon_font.clone() } }
                    --
                    @{ TimePicker { time: "14:30 UTC".into(), font: font.clone(), icon_font: icon_font.clone() } }
                    --
                    @{ DateRangePicker { start: "Oct 20".into(), end: "Oct 28".into(), font: font.clone(), icon_font: icon_font.clone() } }
                    --
                    @{ ColorPicker::new(Color::srgb(0.20, 0.45, 0.90), "#3373E6") }
                ]
                --
                @{ FileUploader { message: "Drop project shaders or click to browse assets".into(), progress: Some(0.72), font: font.clone(), icon_font: icon_font.clone() } }
                --
                UNode::default()
                ULayout {
                    display: UDisplay::Flex,
                    flex_direction: UFlexDirection::Row,
                    justify_content: UJustifyContent::SpaceBetween,
                    align_items: UAlignItems::Center,
                }
                Children [
                    @{ ReactionPicker::new().icon_font(icon_font.clone()) }
                    --
                    @{ RichTextToolbar::new().icon_font(icon_font) }
                ]
            ]
        ]
    }
}

// ============================================================================
// TAB 2: CARDS & SURFACES (Product, UserProfile, Pricing, Article, Social)
// ============================================================================

fn cards_tab_scene(font: Handle<Font>, icon_font: Handle<Font>) -> impl Scene + use<> {
    bsn! {
        UNode {
            width: UVal::Px(1200.0),
        }
        LumaTabContent {
            group: { "gallery".to_string() },
            value: { "cards".to_string() },
        }
        ULayout {
            display: UDisplay::None,
            flex_direction: UFlexDirection::Row,
            justify_content: UJustifyContent::SpaceBetween,
            gap: 16.0,
        }
        Children [
            // Left Column: Product, Profile, Pricing
            UNode {
                width: UVal::Px(590.0),
                background_color: Color::srgb(0.09, 0.11, 0.15),
                border_radius: UCornerRadius::all(14.0),
                padding: USides::all(16.0),
            }
            UBorder {
                color: Color::srgb(0.18, 0.22, 0.30),
                width: 1.0,
                radius: UCornerRadius::all(14.0),
            }
            ULayout {
                display: UDisplay::Flex,
                flex_direction: UFlexDirection::Column,
                gap: 14.0,
            }
            Children [
                UText {
                    text: { "1. Commercial & E-Commerce Product Card".to_string() },
                    font_size: 15.0,
                    font: { font.clone() },
                    color: Color::srgb(0.40, 0.75, 1.0),
                }
                --
                @{
                    ProductCard::new("Bevy Luma Pro Suite", "$49.00")
                        .original_price("$79.00")
                        .discount("38% OFF")
                        .rating(5)
                        .cta("Buy License")
                        .font(font.clone())
                        .icon_font(icon_font.clone())
                }
                --
                UText {
                    text: { "2. Developer Profile & Team Member Card".to_string() },
                    font_size: 15.0,
                    font: { font.clone() },
                    color: Color::srgb(0.40, 0.75, 1.0),
                }
                --
                @{
                    UserProfileCard::new("Abdellah Hanani", "@abdellah")
                        .bio("Systems architect & graphics engineer. Building next-generation SDF UI tooling for Bevy.")
                        .followers("14.2k")
                        .following("482")
                        .font(font.clone())
                        .icon_font(icon_font.clone())
                }
                --
                UText {
                    text: { "3. SaaS Pricing Tier Card".to_string() },
                    font_size: 15.0,
                    font: { font.clone() },
                    color: Color::srgb(0.40, 0.75, 1.0),
                }
                --
                @{
                    PricingCard::new("Enterprise Tier", "$199")
                        .period("/month")
                        .featured(true)
                        .features(vec![
                            "Unlimited SDF Shaders & Shadows".to_string(),
                            "Hierarchical Theme Scope Engine".to_string(),
                            "24/7 SLA Priority Technical Support".to_string(),
                        ])
                        .cta("Start 14-Day Free Trial")
                        .font(font.clone())
                        .icon_font(icon_font.clone())
                }
            ]
            --
            // Right Column: Article, Testimonial, Social Post, CodeBlock
            UNode {
                width: UVal::Px(590.0),
                background_color: Color::srgb(0.09, 0.11, 0.15),
                border_radius: UCornerRadius::all(14.0),
                padding: USides::all(16.0),
            }
            UBorder {
                color: Color::srgb(0.18, 0.22, 0.30),
                width: 1.0,
                radius: UCornerRadius::all(14.0),
            }
            ULayout {
                display: UDisplay::Flex,
                flex_direction: UFlexDirection::Column,
                gap: 14.0,
            }
            Children [
                UText {
                    text: { "4. Editorial Article Card".to_string() },
                    font_size: 15.0,
                    font: { font.clone() },
                    color: Color::srgb(0.40, 0.75, 1.0),
                }
                --
                @{
                    ArticleCard::new(
                        "Reactive Bevy UI with SDF Antialiasing",
                        "Learn how declarative scenes, signed distance fields, and hardware shaders deliver 120 FPS interfaces in Rust.",
                    )
                    .category("Architecture")
                    .date("Oct 2026")
                    .read_time("5 min read")
                    .font(font.clone())
                    .icon_font(icon_font.clone())
                }
                --
                UText {
                    text: { "5. Customer Testimonial Card".to_string() },
                    font_size: 15.0,
                    font: { font.clone() },
                    color: Color::srgb(0.40, 0.75, 1.0),
                }
                --
                @{
                    TestimonialCard::new(
                        "Bevy Luma reduced our UI iteration cycle from hours to minutes. Zero frame drops even on integrated GPUs!",
                        "Sarah Jenkins",
                    )
                    .company("Lead Tools Dev @ Cyberdyne")
                    .rating(5)
                    .font(font.clone())
                    .icon_font(icon_font.clone())
                }
                --
                UText {
                    text: { "6. Social Feed Post & CodeBlock Preview".to_string() },
                    font_size: 15.0,
                    font: { font.clone() },
                    color: Color::srgb(0.40, 0.75, 1.0),
                }
                --
                @{
                    SocialFeedPost::new(
                        "Bevy Engine",
                        "Bevy 0.19 introduces the new Scene Notation system with clean nested syntax and high-performance layout computation!",
                    )
                    .handle("@bevyengine")
                    .time_ago("2h ago")
                    .font(font.clone())
                    .icon_font(icon_font.clone())
                }
                --
                @{
                    CodeBlock::new("fn main() {\n    App::new().add_plugins(LumaWidgetsPlugin).run();\n}")
                        .language("rust")
                        .font(font)
                        .icon_font(icon_font)
                }
            ]
        ]
    }
}

// ============================================================================
// TAB 3: DATA VIEWS & METRICS (DataTable, Charts, Timeline, Kanban, Media)
// ============================================================================

fn data_views_tab_scene(font: Handle<Font>, icon_font: Handle<Font>) -> impl Scene + use<> {
    bsn! {
        UNode {
            width: UVal::Px(1200.0),
        }
        LumaTabContent {
            group: { "gallery".to_string() },
            value: { "data_views".to_string() },
        }
        ULayout {
            display: UDisplay::None,
            flex_direction: UFlexDirection::Row,
            justify_content: UJustifyContent::SpaceBetween,
            gap: 16.0,
        }
        Children [
            // Left Column: Tables, Timelines, Schedules, Kanban
            UNode {
                width: UVal::Px(590.0),
                background_color: Color::srgb(0.09, 0.11, 0.15),
                border_radius: UCornerRadius::all(14.0),
                padding: USides::all(16.0),
            }
            UBorder {
                color: Color::srgb(0.18, 0.22, 0.30),
                width: 1.0,
                radius: UCornerRadius::all(14.0),
            }
            ULayout {
                display: UDisplay::Flex,
                flex_direction: UFlexDirection::Column,
                gap: 12.0,
            }
            Children [
                UText {
                    text: { "1. High-Performance Data Table Grid".to_string() },
                    font_size: 15.0,
                    font: { font.clone() },
                    color: Color::srgb(0.40, 0.75, 1.0),
                }
                --
                @{
                    DataTable::new(
                        vec!["Service", "Status", "Latency", "Region"],
                        vec![
                            vec!["Auth Service", "Healthy", "14ms", "US-East"],
                            vec!["Database Cluster", "Healthy", "4ms", "EU-West"],
                            vec!["Render Worker", "Warning", "128ms", "AP-South"],
                        ]
                    )
                    .font(font.clone())
                }
                --
                UText {
                    text: { "2. Activity Timeline".to_string() },
                    font_size: 15.0,
                    font: { font.clone() },
                    color: Color::srgb(0.40, 0.75, 1.0),
                }
                --
                @{
                    Timeline::new(vec![
                        ("09:00 AM", "Pipeline Started", "Triggered by push to main branch"),
                        ("09:05 AM", "Artifact Built", "Wasm & Native release binaries compiled"),
                        ("09:12 AM", "Deployed to Edge", "Global deployment complete in 18 regions"),
                    ])
                    .font(font.clone())
                }
                --
                UText {
                    text: { "3. Calendar Schedule & Kanban Column".to_string() },
                    font_size: 15.0,
                    font: { font.clone() },
                    color: Color::srgb(0.40, 0.75, 1.0),
                }
                --
                UNode::default()
                ULayout {
                    display: UDisplay::Flex,
                    flex_direction: UFlexDirection::Row,
                    justify_content: UJustifyContent::SpaceBetween,
                    gap: 10.0,
                }
                Children [
                    UNode { width: UVal::Px(280.0) }
                    Children [
                        @{ CalendarSchedule::new("Monday, October 28").font(font.clone()) }
                    ]
                    --
                    UNode { width: UVal::Px(280.0) }
                    Children [
                        @{ KanbanColumn::new("In Review", 3).font(font.clone()) }
                    ]
                ]
            ]
            --
            // Right Column: Charts, Media, Chat, Lightbox, Feeds
            UNode {
                width: UVal::Px(590.0),
                background_color: Color::srgb(0.09, 0.11, 0.15),
                border_radius: UCornerRadius::all(14.0),
                padding: USides::all(16.0),
            }
            UBorder {
                color: Color::srgb(0.18, 0.22, 0.30),
                width: 1.0,
                radius: UCornerRadius::all(14.0),
            }
            ULayout {
                display: UDisplay::Flex,
                flex_direction: UFlexDirection::Column,
                gap: 12.0,
            }
            Children [
                UText {
                    text: { "4. BarChart & DonutChart Shaders".to_string() },
                    font_size: 15.0,
                    font: { font.clone() },
                    color: Color::srgb(0.40, 0.75, 1.0),
                }
                --
                UNode::default()
                ULayout {
                    display: UDisplay::Flex,
                    flex_direction: UFlexDirection::Row,
                    justify_content: UJustifyContent::SpaceBetween,
                    align_items: UAlignItems::Center,
                }
                Children [
                    @{ BarChart::new(vec![("Mon", 0.45_f32), ("Tue", 0.70_f32), ("Wed", 0.90_f32), ("Thu", 0.60_f32), ("Fri", 0.85_f32)]).font(font.clone()) }
                    --
                    @{ DonutChart::new(0.78_f32, "Memory 78%").font(font.clone()) }
                ]
                --
                UText {
                    text: { "5. Media Player & Interactive Lightbox".to_string() },
                    font_size: 15.0,
                    font: { font.clone() },
                    color: Color::srgb(0.40, 0.75, 1.0),
                }
                --
                @{ MediaPlayer::new("Cyberpunk Synthwave Vol. 1", "Luma Studio Audio").progress(0.42_f32).playing(true).font(font.clone()).icon_font(icon_font.clone()) }
                --
                @{ ImageLightbox::new("SDF Buffer Inspection").font(font.clone()).icon_font(icon_font.clone()) }
                --
                UText {
                    text: { "6. Indicators, ChatBubbles & Notification Center".to_string() },
                    font_size: 15.0,
                    font: { font.clone() },
                    color: Color::srgb(0.40, 0.75, 1.0),
                }
                --
                UNode::default()
                ULayout {
                    display: UDisplay::Flex,
                    flex_direction: UFlexDirection::Row,
                    align_items: UAlignItems::Center,
                    justify_content: UJustifyContent::SpaceBetween,
                }
                Children [
                    @{ AvatarGroup::new(vec!["AH", "JD", "AI", "MR", "EL"]).max(3).font(font.clone()) }
                    --
                    @{ NotificationCounter::new(24).font(font.clone()) }
                    --
                    @{ ProgressBar::new(0.68_f32).width(UVal::Px(120.0)).variant(ProgressVariant::Success) }
                    --
                    @{ Spinner::md() }
                    --
                    @{ Skeleton::new(UVal::Px(80.0), UVal::Px(14.0)).radius(4.0) }
                ]
                --
                @{ ChatBubble::new("The SDF shader pipeline is running at 120 FPS smoothly!").outgoing(false).timestamp("10:14 AM").font(font.clone()) }
                --
                @{ ChatBubble::new("Awesome! Shipping the update now.").outgoing(true).timestamp("10:15 AM").font(font.clone()) }
                --
                @{ CommentThread::new("Alex Rivera", "Can we add support for nested sub-routes in the sidebar navigation?").time_ago("45m ago").font(font.clone()) }
                --
                UNode::default()
                ULayout {
                    display: UDisplay::Flex,
                    flex_direction: UFlexDirection::Row,
                    justify_content: UJustifyContent::SpaceBetween,
                    gap: 10.0,
                }
                Children [
                    UNode { width: UVal::Px(280.0) }
                    Children [
                        @{ NotificationCenter::new().font(font.clone()) }
                    ]
                    --
                    UNode { width: UVal::Px(280.0) }
                    Children [
                        @{ CartDrawer::new().subtotal("$128.50").font(font) }
                    ]
                ]
            ]
        ]
    }
}

// ============================================================================
// TAB 4: NAVIGATION & SHELLS (Navbar, Sidebar, CommandPalette, Menus, Tree)
// ============================================================================

fn navigation_tab_scene(font: Handle<Font>, icon_font: Handle<Font>) -> impl Scene + use<> {
    bsn! {
        UNode {
            width: UVal::Px(1200.0),
        }
        LumaTabContent {
            group: { "gallery".to_string() },
            value: { "navigation".to_string() },
        }
        ULayout {
            display: UDisplay::None,
            flex_direction: UFlexDirection::Row,
            justify_content: UJustifyContent::SpaceBetween,
            gap: 16.0,
        }
        Children [
            // Left Column: Navbar, Sidebar, CommandPalette, TreeView
            UNode {
                width: UVal::Px(590.0),
                background_color: Color::srgb(0.09, 0.11, 0.15),
                border_radius: UCornerRadius::all(14.0),
                padding: USides::all(16.0),
            }
            UBorder {
                color: Color::srgb(0.18, 0.22, 0.30),
                width: 1.0,
                radius: UCornerRadius::all(14.0),
            }
            ULayout {
                display: UDisplay::Flex,
                flex_direction: UFlexDirection::Column,
                gap: 12.0,
            }
            Children [
                UText {
                    text: { "1. Navbar & Top Navigation Header".to_string() },
                    font_size: 15.0,
                    font: { font.clone() },
                    color: Color::srgb(0.40, 0.75, 1.0),
                }
                --
                @{
                    Navbar::new("LumaStudio")
                        .items(vec!["Dashboard".into(), "Components".into(), "Themes".into(), "Docs".into()])
                        .active(1)
                        .font(font.clone())
                        .icon_font(icon_font.clone())
                }
                --
                UText {
                    text: { "2. Application Sidebar & Command Launcher".to_string() },
                    font_size: 15.0,
                    font: { font.clone() },
                    color: Color::srgb(0.40, 0.75, 1.0),
                }
                --
                @{
                    Sidebar::new()
                        .brand("Navigation Engine")
                        .items(vec![
                            (Icon::STAR.to_string(), "Dashboard".into()),
                            (Icon::FOLDER.to_string(), "Widgets".into()),
                            (Icon::SETTINGS.to_string(), "Settings".into()),
                        ])
                        .active(1)
                        .font(font.clone())
                        .icon_font(icon_font.clone())
                }
                --
                @{
                    CommandPalette::new()
                        .commands(vec![
                            "Rebuild Shader Pipeline (Release)".to_string(),
                            "Export Scene as PNG".to_string(),
                            "Toggle Dark / Light Theme".to_string(),
                            "View Component Docs".to_string(),
                        ])
                        .font(font.clone())
                        .icon_font(icon_font.clone())
                }
                --
                UText {
                    text: { "3. TreeView Directory & Extended Breadcrumbs".to_string() },
                    font_size: 15.0,
                    font: { font.clone() },
                    color: Color::srgb(0.40, 0.75, 1.0),
                }
                --
                @{
                    TreeView::new()
                        .items(vec![
                            (0, "src/".into(), true),
                            (1, "lib.rs".into(), false),
                            (1, "controls.rs".into(), false),
                            (1, "drag_value.rs".into(), false),
                            (0, "Cargo.toml".into(), false),
                        ])
                        .font(font.clone())
                        .icon_font(icon_font.clone())
                }
                --
                @{ Breadcrumbs::new(vec!["Workspace", "Production", "Cluster-A", "Nodes"]).font(font.clone()).icon_font(icon_font.clone()) }
            ]
            --
            // Right Column: MegaMenu, ContextMenu, Toolbars, Pagination, Trackers
            UNode {
                width: UVal::Px(590.0),
                background_color: Color::srgb(0.09, 0.11, 0.15),
                border_radius: UCornerRadius::all(14.0),
                padding: USides::all(16.0),
            }
            UBorder {
                color: Color::srgb(0.18, 0.22, 0.30),
                width: 1.0,
                radius: UCornerRadius::all(14.0),
            }
            ULayout {
                display: UDisplay::Flex,
                flex_direction: UFlexDirection::Column,
                gap: 12.0,
            }
            Children [
                UText {
                    text: { "4. MegaMenu & Context Menu".to_string() },
                    font_size: 15.0,
                    font: { font.clone() },
                    color: Color::srgb(0.40, 0.75, 1.0),
                }
                --
                @{
                    MegaMenu::new("Components")
                        .categories(vec![
                            ("Controls".into(), vec!["Buttons".into(), "Sliders".into(), "DragValue".into()]),
                            ("Cards".into(), vec!["Product".into(), "Pricing".into(), "Profiles".into()]),
                        ])
                        .font(font.clone())
                }
                --
                @{ ContextMenu::new(vec!["Inspect Node", "Copy Selector", "Export BSN Scene", "Delete"]).font(font.clone()).icon_font(icon_font.clone()) }
                --
                UText {
                    text: { "5. Action Bars, Splitter, Toggle & Switchers".to_string() },
                    font_size: 15.0,
                    font: { font.clone() },
                    color: Color::srgb(0.40, 0.75, 1.0),
                }
                --
                UNode::default()
                ULayout {
                    display: UDisplay::Flex,
                    flex_direction: UFlexDirection::Row,
                    align_items: UAlignItems::Center,
                    justify_content: UJustifyContent::SpaceBetween,
                }
                Children [
                    @{ HamburgerMenu::new().open(false).icon_font(icon_font.clone()) }
                    --
                    @{ ThemeToggle::new().dark(true).icon_font(icon_font.clone()) }
                    --
                    @{ WorkspaceSwitcher::new("Production-East").font(font.clone()).icon_font(icon_font.clone()) }
                    --
                    @{ LocaleSelector::new("en-US").font(font.clone()).icon_font(icon_font.clone()) }
                    --
                    @{ BackToTopButton::new().icon_font(icon_font.clone()) }
                ]
                --
                @{ FilterSortBar::new().filter("All Components").sort("Most Popular").grid_view(true).font(font.clone()).icon_font(icon_font.clone()) }
                --
                @{ FloatingToolbar::new().icon_font(icon_font.clone()) }
                --
                @{ BottomNav::new().items(vec![("Home".into(), Icon::STAR.into()), ("Search".into(), Icon::SEARCH.into()), ("Profile".into(), Icon::USER.into())]).active(0).font(font.clone()).icon_font(icon_font.clone()) }
                --
                UNode::default()
                ULayout {
                    display: UDisplay::Flex,
                    flex_direction: UFlexDirection::Row,
                    justify_content: UJustifyContent::SpaceBetween,
                    gap: 10.0,
                }
                Children [
                    UNode { width: UVal::Px(270.0) }
                    Children [
                        @{ ActionSheet::new("Quick Actions").actions(vec!["Share Project", "Duplicate Tab", "Export JSON"]).cancel("Cancel").font(font.clone()) }
                    ]
                    --
                    UNode { width: UVal::Px(270.0) }
                    Children [
                        @{ BottomSheet::new("Inspector Details").font(font.clone()) }
                    ]
                ]
                --
                UText {
                    text: { "6. StepTracker, Pagination, TOC, Scrollbar & Footer".to_string() },
                    font_size: 15.0,
                    font: { font.clone() },
                    color: Color::srgb(0.40, 0.75, 1.0),
                }
                --
                @{ StepTracker::new(vec!["Configure", "Build Shaders", "Verify Tests", "Deploy"]).current(2).font(font.clone()).icon_font(icon_font.clone()) }
                --
                @{ Pagination::new(3, 10).font(font.clone()).icon_font(icon_font.clone()) }
                --
                @{ TableOfContents::new(vec!["1. Introduction", "2. Architecture", "3. SDF Shaders", "4. API Reference"]).active(1).font(font.clone()) }
                --
                UNode::default()
                ULayout {
                    display: UDisplay::Flex,
                    flex_direction: UFlexDirection::Row,
                    align_items: UAlignItems::Center,
                    justify_content: UJustifyContent::SpaceBetween,
                }
                Children [
                    @{ Splitter::new().horizontal(true) }
                    --
                    @{ CustomScrollbar::new().progress(0.4_f32).horizontal(false) }
                    --
                    @{ CarouselControls::new(5, 2).icon_font(icon_font) }
                ]
                --
                @{ Footer::new().copyright("© 2026 Univis Luma UI Framework. All rights reserved.").font(font) }
            ]
        ]
    }
}

// ============================================================================
// TAB 5: STATES & FEEDBACK (Empty, 404, Offline, Success, Modals, Popovers)
// ============================================================================

fn states_tab_scene(font: Handle<Font>, icon_font: Handle<Font>) -> impl Scene + use<> {
    bsn! {
        UNode {
            width: UVal::Px(1200.0),
        }
        LumaTabContent {
            group: { "gallery".to_string() },
            value: { "states".to_string() },
        }
        ULayout {
            display: UDisplay::None,
            flex_direction: UFlexDirection::Row,
            justify_content: UJustifyContent::SpaceBetween,
            gap: 16.0,
        }
        Children [
            // Left Column: EmptyState, Error404, OfflineState, Alerts, Toasts
            UNode {
                width: UVal::Px(590.0),
                background_color: Color::srgb(0.09, 0.11, 0.15),
                border_radius: UCornerRadius::all(14.0),
                padding: USides::all(16.0),
            }
            UBorder {
                color: Color::srgb(0.18, 0.22, 0.30),
                width: 1.0,
                radius: UCornerRadius::all(14.0),
            }
            ULayout {
                display: UDisplay::Flex,
                flex_direction: UFlexDirection::Column,
                gap: 12.0,
            }
            Children [
                UText {
                    text: { "1. System Lifecycle & Empty States".to_string() },
                    font_size: 15.0,
                    font: { font.clone() },
                    color: Color::srgb(0.40, 0.75, 1.0),
                }
                --
                @{
                    EmptyState::new(
                        "No Data Points Yet",
                        "Create a new project or import a dataset to view interactive metrics.",
                    )
                    .icon(Icon::INBOX)
                    .font(font.clone())
                    .icon_font(icon_font.clone())
                }
                --
                @{
                    Error404State::new()
                        .code("404")
                        .title("Page Not Found")
                        .message("The requested component route does not exist in the active registry.")
                        .font(font.clone())
                }
                --
                @{ OfflineState::new().title("Disconnected from Edge").retry_label("Reconnect Now").font(font.clone()).icon_font(icon_font.clone()) }
                --
                UText {
                    text: { "2. Contextual Alerts & Toast Notifications".to_string() },
                    font_size: 15.0,
                    font: { font.clone() },
                    color: Color::srgb(0.40, 0.75, 1.0),
                }
                --
                @{
                    Alert::new(
                        "Theme Inheritance Active",
                        "Descendant widgets automatically resolve styles from parent scopes.",
                    )
                    .variant(AlertVariant::Info)
                    .font(font.clone())
                    .icon_font(icon_font.clone())
                }
                --
                @{
                    Alert::new(
                        "Hardware Acceleration Verified",
                        "All rounded corners and borders are rendered via SDF shaders.",
                    )
                    .variant(AlertVariant::Success)
                    .font(font.clone())
                    .icon_font(icon_font.clone())
                }
                --
                @{ Toast::info("Notification Received", "Background pipeline sync completed.").font(font.clone()).icon_font(icon_font.clone()) }
                --
                @{ Toast::success("Tokens Recompiled", "All CSS variables and theme shaders refreshed.").font(font.clone()).icon_font(icon_font.clone()) }
            ]
            --
            // Right Column: SuccessScreen, Paywall, Coachmark, CookieConsent, Popovers, Scopes
            UNode {
                width: UVal::Px(590.0),
                background_color: Color::srgb(0.09, 0.11, 0.15),
                border_radius: UCornerRadius::all(14.0),
                padding: USides::all(16.0),
            }
            UBorder {
                color: Color::srgb(0.18, 0.22, 0.30),
                width: 1.0,
                radius: UCornerRadius::all(14.0),
            }
            ULayout {
                display: UDisplay::Flex,
                flex_direction: UFlexDirection::Column,
                gap: 12.0,
            }
            Children [
                UText {
                    text: { "3. Conversion & Payment Screens".to_string() },
                    font_size: 15.0,
                    font: { font.clone() },
                    color: Color::srgb(0.40, 0.75, 1.0),
                }
                --
                @{
                    SuccessScreen::new("Payment Confirmed", "$2,400.00")
                        .description("Your subscription to Luma Pro Enterprise is now active.")
                        .done_label("Go to Dashboard")
                        .font(font.clone())
                        .icon_font(icon_font.clone())
                }
                --
                @{
                    Paywall::new()
                        .title("Unlock Enterprise Features")
                        .features(vec![
                            "Real-time Chart Shaders".to_string(),
                            "Custom Theme Scope Engine".to_string(),
                            "24/7 SLA Guarantee".to_string(),
                        ])
                        .font(font.clone())
                        .icon_font(icon_font.clone())
                }
                --
                UText {
                    text: { "4. Onboarding, Overlays & Keycaps".to_string() },
                    font_size: 15.0,
                    font: { font.clone() },
                    color: Color::srgb(0.40, 0.75, 1.0),
                }
                --
                @{
                    Coachmark::new(
                        "New Feature: SDF Antialiasing",
                        "GPU SDF rendering gives pixel-perfect text and curved borders at any scale.",
                    )
                    .step("Step 1 of 3")
                    .font(font.clone())
                }
                --
                @{ CookieConsent::new().message("We use essential cookies to maintain your workspace state.").font(font.clone()) }
                --
                @{ Popover::new().open(true) }
                --
                UNode::default()
                ULayout {
                    display: UDisplay::Flex,
                    flex_direction: UFlexDirection::Row,
                    align_items: UAlignItems::Center,
                    justify_content: UJustifyContent::SpaceBetween,
                }
                Children [
                    @{ Badge::new("SUCCESS").variant(BadgeVariant::Success).font(font.clone()) }
                    --
                    @{ Badge::new("DANGER").variant(BadgeVariant::Danger).font(font.clone()) }
                    --
                    @{ luma_chip("Rust 2024", font.clone()) }
                    --
                    @{ luma_removable_chip("Bevy 0.19", font.clone(), icon_font.clone()) }
                    --
                    @{ luma_swatch(Color::srgb(0.20, 0.45, 0.90), true) }
                    --
                    @{ luma_swatch(Color::srgb(0.16, 0.75, 0.45), false) }
                ]
                --
                UNode::default()
                ULayout {
                    display: UDisplay::Flex,
                    flex_direction: UFlexDirection::Row,
                    align_items: UAlignItems::Center,
                    justify_content: UJustifyContent::SpaceBetween,
                }
                Children [
                    @{ Avatar::new("AH").status(AvatarStatus::Online).font(font.clone()) }
                    --
                    @{ Avatar::new("JD").status(AvatarStatus::Busy).font(font.clone()) }
                    --
                    @{ Tooltip::new("Interactive Tooltip").font(font.clone()) }
                    --
                    @{
                        luma_kbd_shortcut((
                            luma_kbd("Ctrl", KbdSize::Small, font.clone()),
                            luma_kbd_separator(font.clone()),
                            luma_kbd("S", KbdSize::Small, font.clone())
                        ))
                    }
                ]
                --
                @{
                    Accordion::new("Expandable System Architecture Details")
                        .expanded(false)
                        .font(font.clone())
                        .icon_font(icon_font.clone())
                        .content(bsn! {
                            UNode::default()
                            Children [
                                UText {
                                    text: { "All 80+ widgets implement Bevy Scene Notation directly with full ECS observer integration.".to_string() },
                                    font_size: 13.0,
                                    font: { font.clone() },
                                    color: Color::srgb(0.70, 0.75, 0.85),
                                }
                            ]
                        })
                }
                --
                @{
                    luma_style_scope(
                        LumaStyleScope::new().button(ButtonStyle::pill()),
                        bsn! {
                            UNode::default()
                            ULayout {
                                display: UDisplay::Flex,
                                flex_direction: UFlexDirection::Row,
                                align_items: UAlignItems::Center,
                                gap: 8.0,
                            }
                            Children [
                                UText {
                                    text: { "Scope Inheritance:".to_string() },
                                    font_size: 12.0,
                                    font: { font.clone() },
                                    color: Color::srgb(0.65, 0.60, 0.80),
                                }
                                --
                                @{ Button::new("Pill A").font(font.clone()) }
                                --
                                @{ Button::new("Pill B").variant(ButtonVariant::Secondary).font(font.clone()) }
                                --
                                @{ Button::new("Danger").variant(ButtonVariant::Danger).font(font) }
                            ]
                        }
                    )
                }
            ]
        ]
    }
}
