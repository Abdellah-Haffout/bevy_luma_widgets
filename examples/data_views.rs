//! Data Views & Visualizations showcase example in bevy_luma_widgets.

use bevy::prelude::*;
use bevy_luma_widgets::prelude::*;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins.set(WindowPlugin {
            primary_window: Some(Window {
                title: "Luma Widgets — Data Views Suite".into(),
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
                    text: { "Data Views & Visualizations Suite".to_string() },
                    font_size: 26.0,
                    font: { font.clone() },
                    color: Color::WHITE,
                }
                --
                UText {
                    text: { "Data tables, activity timelines, charts, media players, and communication feeds".to_string() },
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
                // Left Column: Tables, Timelines, Schedules
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
                        text: { "1. Data Table Grid".to_string() },
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
                ]
                --
                // Right Column: Charts, Media Player, Chat
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
                        text: { "3. BarChart & DonutChart Shaders".to_string() },
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
                        text: { "4. Media Player & Interactive Audio Controls".to_string() },
                        font_size: 15.0,
                        font: { font.clone() },
                        color: Color::srgb(0.40, 0.75, 1.0),
                    }
                    --
                    @{ MediaPlayer::new("Cyberpunk Synthwave Vol. 1", "Luma Studio Audio").progress(0.42_f32).playing(true).font(font.clone()).icon_font(icon_font.clone()) }
                    --
                    UText {
                        text: { "5. Chat & Communication Bubbles".to_string() },
                        font_size: 15.0,
                        font: { font.clone() },
                        color: Color::srgb(0.40, 0.75, 1.0),
                    }
                    --
                    @{ ChatBubble::new("The SDF shader pipeline is running at 120 FPS smoothly!").outgoing(false).timestamp("10:14 AM").font(font.clone()) }
                    --
                    @{ ChatBubble::new("Awesome! Shipping the update now.").outgoing(true).timestamp("10:15 AM").font(font.clone()) }
                ]
            ]
        ]
    });
}
