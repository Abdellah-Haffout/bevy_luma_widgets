//! Showcase for the "Primitives & Style" architecture in bevy_luma_widgets.
//!
//! Demonstrates how core root components (Button, TextInput, Select, ListItem, Spinner, Badge)
//! power rich UI patterns (Search Bar, Password Input, FAB, Loading State, Dropdowns, and Scoped Themes).

use bevy::prelude::*;
use bevy_luma_widgets::prelude::*;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins.set(WindowPlugin {
            primary_window: Some(Window {
                title: "Luma Widgets — Roots & Styles Showcase".into(),
                resolution: (1200, 960).into(),
                ..default()
            }),
            ..default()
        }))
        .add_plugins(LumaWidgetsPlugin)
        .add_systems(Startup, setup_showcase)
        .run();
}

fn setup_showcase(mut commands: Commands, theme: Res<Theme>) {
    commands.spawn(Camera2d);
    commands.spawn_scene(showcase_scene(&theme));
}

fn showcase_scene(theme: &Theme) -> impl Scene + use<> {
    let font = theme.text.font.inter_regular.clone();
    let icon_font = theme.icon.font.clone();

    bsn! {
        URootUi::screen()
        UNode {
            width: UVal::Percent(1.0),
            height: UVal::Percent(1.0),
            background_color: Color::srgb(0.06, 0.07, 0.09),
            padding: USides::all(24.0),
        }
        ULayout {
            display: UDisplay::Flex,
            flex_direction: UFlexDirection::Column,
            align_items: UAlignItems::Center,
            gap: 16.0,
        }
        Children [
            // Header
            UNode::default()
            ULayout {
                display: UDisplay::Flex,
                flex_direction: UFlexDirection::Column,
                align_items: UAlignItems::Center,
                gap: 4.0,
            }
            Children [
                UText {
                    text: { "Luma Widgets: Roots & Styles Showcase".to_string() },
                    font_size: 26.0,
                    font: { font.clone() },
                    color: Color::WHITE,
                }
                --
                UText {
                    text: { "Demonstrating Primitive Roots with Derived Variants via Style Presets".to_string() },
                    font_size: 14.0,
                    font: { font.clone() },
                    color: Color::srgb(0.60, 0.65, 0.75),
                }
            ]
            --
            // Main Columns Layout
            UNode {
                width: UVal::Px(1120.0),
            }
            ULayout {
                display: UDisplay::Flex,
                flex_direction: UFlexDirection::Row,
                justify_content: UJustifyContent::SpaceBetween,
                gap: 16.0,
            }
            Children [
                // Left Column: Button Root & TextInput Root
                UNode {
                    width: UVal::Px(540.0),
                    background_color: Color::srgb(0.10, 0.12, 0.16),
                    border_radius: UCornerRadius::all(14.0),
                    padding: USides::all(20.0),
                }
                UBorder {
                    color: Color::srgb(0.18, 0.22, 0.30),
                    width: 1.0,
                    radius: UCornerRadius::all(14.0),
                }
                ULayout {
                    display: UDisplay::Flex,
                    flex_direction: UFlexDirection::Column,
                    gap: 18.0,
                }
                Children [
                    // Section: Button Root & Styles
                    UText {
                        text: { "1. Button Root & Derived Styles".to_string() },
                        font_size: 17.0,
                        font: { font.clone() },
                        color: Color::srgb(0.40, 0.75, 1.0),
                    }
                    --
                    UNode::default()
                    ULayout {
                        display: UDisplay::Flex,
                        flex_direction: UFlexDirection::Row,
                        align_items: UAlignItems::Center,
                        gap: 12.0,
                    }
                    Children [
                        // Primary Button
                        @{ Button::new("Primary").font(font.clone()) }
                        --
                        // Loading Button with Spinner
                        @{ Button::new("Loading").loading(true).font(font.clone()) }
                        --
                        // Icon Button
                        @{ Button::icon_button(Icon::STAR).font(font.clone()).icon_font(icon_font.clone()) }
                        --
                        // FAB Button
                        @{ Button::fab_button(Icon::ARROW_UP).icon_font(icon_font.clone()) }
                    ]
                    --
                    // Section: TextInput Root & Presets
                    UText {
                        text: { "2. TextInput Root & Preset Styles".to_string() },
                        font_size: 17.0,
                        font: { font.clone() },
                        color: Color::srgb(0.40, 0.75, 1.0),
                    }
                    --
                    UNode::default()
                    ULayout {
                        display: UDisplay::Flex,
                        flex_direction: UFlexDirection::Column,
                        gap: 10.0,
                    }
                    Children [
                        // Search Bar Preset
                        @{ TextInput::search("Search documentation, widgets...").font(font.clone()).icon_font(icon_font.clone()) }
                        --
                        // Password Input Preset
                        @{ TextInput::password("Enter secret token...").font(font.clone()).icon_font(icon_font.clone()) }
                        --
                        // Input with Error validation state
                        @{ TextInput::new("Invalid email format").value("user@@invalid").state(InputState::Error("Bad email".into())).font(font.clone()) }
                        --
                        // Multi-line Text Area
                        @{ TextInput::multiline("Write comments or markdown description...").font(font.clone()) }
                    ]
                ]
                --
                // Right Column: Navigation, Menus, Overlays & Lists
                UNode {
                    width: UVal::Px(540.0),
                    background_color: Color::srgb(0.10, 0.12, 0.16),
                    border_radius: UCornerRadius::all(14.0),
                    padding: USides::all(20.0),
                }
                UBorder {
                    color: Color::srgb(0.18, 0.22, 0.30),
                    width: 1.0,
                    radius: UCornerRadius::all(14.0),
                }
                ULayout {
                    display: UDisplay::Flex,
                    flex_direction: UFlexDirection::Column,
                    gap: 18.0,
                }
                Children [
                    // Section: Select Dropdown & Badge Counter
                    UNode::default()
                    ULayout {
                        display: UDisplay::Flex,
                        flex_direction: UFlexDirection::Row,
                        justify_content: UJustifyContent::SpaceBetween,
                        align_items: UAlignItems::Center,
                    }
                    Children [
                        UText {
                            text: { "3. Select, Spinner & Counter".to_string() },
                            font_size: 17.0,
                            font: { font.clone() },
                            color: Color::srgb(0.40, 0.75, 1.0),
                        }
                        --
                        // Notification Counter Badge
                        @{ Badge::counter(24).font(font.clone()) }
                    ]
                    --
                    UNode::default()
                    ULayout {
                        display: UDisplay::Flex,
                        flex_direction: UFlexDirection::Row,
                        align_items: UAlignItems::Center,
                        gap: 12.0,
                    }
                    Children [
                        // Select dropdown
                        UNode {
                            width: UVal::Px(320.0),
                        }
                        Children [
                            @{
                                Select::new(
                                    "Choose Environment...",
                                    vec![
                                        SelectOption::new("Production (US-East)", "prod-us"),
                                        SelectOption::new("Staging (EU-West)", "stage-eu"),
                                        SelectOption::new("Local Dev Sandbox", "dev-local"),
                                    ]
                                )
                                .selected(0)
                                .font(font.clone())
                                .icon_font(icon_font.clone())
                            }
                        ]
                        --
                        // Standalone Spinner loaders (sm, md, lg)
                        @{ Spinner::sm() }
                        --
                        @{ Spinner::md() }
                        --
                        @{ Spinner::lg() }
                    ]
                    --
                    // Section: ListItem Root
                    UText {
                        text: { "4. ListItem Root (Settings & Menu Rows)".to_string() },
                        font_size: 17.0,
                        font: { font.clone() },
                        color: Color::srgb(0.40, 0.75, 1.0),
                    }
                    --
                    UNode {
                        background_color: Color::srgb(0.08, 0.09, 0.12),
                        border_radius: UCornerRadius::all(10.0),
                        padding: USides::all(8.0),
                    }
                    ULayout {
                        display: UDisplay::Flex,
                        flex_direction: UFlexDirection::Column,
                        gap: 6.0,
                    }
                    Children [
                        @{
                            ListItem::new("System Notifications")
                                .subtitle("Push alerts enabled for high priority events")
                                .leading_icon(Icon::INFO)
                                .trailing_text("Enabled")
                                .font(font.clone())
                                .icon_font(icon_font.clone())
                        }
                        --
                        @{
                            ListItem::new("Cloud Synchronization")
                                .subtitle("Auto-sync every 5 minutes")
                                .leading_icon(Icon::CIRCLE_CHECK_BIG)
                                .trailing_text("Active")
                                .selected(true)
                                .font(font.clone())
                                .icon_font(icon_font.clone())
                        }
                        --
                        @{
                            ListItem::new("Developer Preferences")
                                .subtitle("Theme inheritance, shaders, debug metrics")
                                .trailing_icon(Icon::CHEVRON_RIGHT)
                                .font(font.clone())
                                .icon_font(icon_font.clone())
                        }
                    ]
                    --
                    // Section: Style Inheritance Scope
                    UText {
                        text: { "5. LumaStyleScope (Scoped Pill Theme)".to_string() },
                        font_size: 17.0,
                        font: { font.clone() },
                        color: Color::srgb(0.40, 0.75, 1.0),
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
                                    gap: 10.0,
                                }
                                Children [
                                    @{ Button::new("Inherited Pill A").font(font.clone()) }
                                    --
                                    @{ Button::new("Inherited Pill B").variant(ButtonVariant::Secondary).font(font.clone()) }
                                    --
                                    @{ Button::new("Danger Override").variant(ButtonVariant::Danger).font(font.clone()) }
                                ]
                            }
                        )
                    }
                ]
            ]
        ]
    }
}
