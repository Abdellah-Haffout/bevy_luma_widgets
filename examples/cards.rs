//! Compound Cards showcase example in bevy_luma_widgets.

use bevy::prelude::*;
use bevy_luma_widgets::prelude::*;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins.set(WindowPlugin {
            primary_window: Some(Window {
                title: "Luma Widgets — Compound Cards Suite".into(),
                resolution: (1280, 960).into(),
                ..default()
            }),
            ..default()
        }))
        .add_plugins(LumaWidgetsPlugin)
        .add_systems(Startup, setup)
        .run();
}

fn setup(mut commands: Commands, theme: Res<Theme>) {
    commands.spawn(Camera2d);
    let font = theme.text.font.inter_regular.clone();
    let icon_font = theme.icon.font.clone();

    commands.spawn_scene(bsn! {
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
                    text: { "Compound Cards Suite".to_string() },
                    font_size: 26.0,
                    font: { font.clone() },
                    color: Color::WHITE,
                }
                --
                UText {
                    text: { "Product showcases, user profiles, pricing tables, articles, testimonials, and code snippets".to_string() },
                    font_size: 14.0,
                    font: { font.clone() },
                    color: Color::srgb(0.60, 0.65, 0.75),
                }
            ]
            --
            // Main Two-Column Container
            UNode {
                width: UVal::Px(1200.0),
            }
            ULayout {
                display: UDisplay::Flex,
                flex_direction: UFlexDirection::Row,
                justify_content: UJustifyContent::SpaceBetween,
                gap: 16.0,
            }
            Children [
                // Left Column
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
                        text: { "1. Product Card".to_string() },
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
                        text: { "2. Developer Profile Card".to_string() },
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
                ]
                --
                // Right Column
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
                        text: { "3. Editorial Article Card".to_string() },
                        font_size: 15.0,
                        font: { font.clone() },
                        color: Color::srgb(0.40, 0.75, 1.0),
                    }
                    --
                    @{
                        ArticleCard::new(
                            "Reactive Bevy UI with SDF Antialiasing",
                            "Learn how declarative scenes and hardware signed distance field shaders deliver 120 FPS interfaces.",
                        )
                        .category("Architecture")
                        .date("Oct 2026")
                        .read_time("5 min read")
                        .font(font.clone())
                        .icon_font(icon_font.clone())
                    }
                    --
                    UText {
                        text: { "4. Syntax Highlighted CodeBlock".to_string() },
                        font_size: 15.0,
                        font: { font.clone() },
                        color: Color::srgb(0.40, 0.75, 1.0),
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
        ]
    });
}
