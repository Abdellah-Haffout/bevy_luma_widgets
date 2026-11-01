//! Card surface container widget showcase example in bevy_luma_widgets.

use bevy::prelude::*;
use bevy_luma_widgets::prelude::*;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins.set(WindowPlugin {
            primary_window: Some(Window {
                title: "Luma Widgets — Card Container Example".into(),
                resolution: (960, 720).into(),
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

    commands.spawn_scene(bsn! {
        URootUi::screen()
        UNode {
            width: UVal::Percent(1.0),
            height: UVal::Percent(1.0),
            background_color: Color::srgb(0.06, 0.07, 0.09),
            padding: USides::all(32.0),
        }
        ULayout {
            display: UDisplay::Flex,
            flex_direction: UFlexDirection::Column,
            align_items: UAlignItems::Center,
            gap: 20.0,
        }
        Children [
            // Header
            UNode::default()
            ULayout {
                display: UDisplay::Flex,
                flex_direction: UFlexDirection::Column,
                align_items: UAlignItems::Center,
                gap: 6.0,
            }
            Children [
                UText {
                    text: { "Card Component".to_string() },
                    font_size: 28.0,
                    font: { font.clone() },
                    color: Color::WHITE,
                }
                --
                UText {
                    text: { "Elevated SDF surface containers with rounded corners, subtle borders, and flexible padding".to_string() },
                    font_size: 14.0,
                    font: { font.clone() },
                    color: Color::srgb(0.60, 0.65, 0.75),
                }
            ]
            --
            // Main Showcase Container
            UNode {
                width: UVal::Px(720.0),
                background_color: Color::srgb(0.10, 0.12, 0.16),
                border_radius: UCornerRadius::all(14.0),
                padding: USides::all(24.0),
            }
            UBorder {
                color: Color::srgb(0.18, 0.22, 0.30),
                width: 1.0,
                radius: UCornerRadius::all(14.0),
            }
            ULayout {
                display: UDisplay::Flex,
                flex_direction: UFlexDirection::Column,
                gap: 20.0,
            }
            Children [
                UText {
                    text: { "1. Surface Cards Grid".to_string() },
                    font_size: 16.0,
                    font: { font.clone() },
                    color: Color::srgb(0.40, 0.75, 1.0),
                }
                --
                UNode::default()
                ULayout {
                    display: UDisplay::Flex,
                    flex_direction: UFlexDirection::Row,
                    gap: 16.0,
                }
                Children [
                    // Card A
                    UNode {
                        width: UVal::Px(320.0),
                        background_color: Color::srgb(0.12, 0.15, 0.20),
                        border_radius: UCornerRadius::all(12.0),
                        padding: USides::all(18.0),
                    }
                    UBorder {
                        color: Color::srgb(0.20, 0.25, 0.35),
                        width: 1.0,
                        radius: UCornerRadius::all(12.0),
                    }
                    ULayout {
                        display: UDisplay::Flex,
                        flex_direction: UFlexDirection::Column,
                        gap: 10.0,
                    }
                    Children [
                        UText {
                            text: { "Performance Metrics".to_string() },
                            font_size: 18.0,
                            font: { font.clone() },
                            color: Color::WHITE,
                        }
                        --
                        UText {
                            text: { "Frame rate: 120 FPS\nGPU Latency: 4.2ms\nVRAM Usage: 1.4 GB".to_string() },
                            font_size: 13.0,
                            font: { font.clone() },
                            color: Color::srgb(0.70, 0.75, 0.85),
                        }
                    ]
                    --
                    // Card B
                    UNode {
                        width: UVal::Px(320.0),
                        background_color: Color::srgb(0.12, 0.15, 0.20),
                        border_radius: UCornerRadius::all(12.0),
                        padding: USides::all(18.0),
                    }
                    UBorder {
                        color: Color::srgb(0.20, 0.25, 0.35),
                        width: 1.0,
                        radius: UCornerRadius::all(12.0),
                    }
                    ULayout {
                        display: UDisplay::Flex,
                        flex_direction: UFlexDirection::Column,
                        gap: 10.0,
                    }
                    Children [
                        UText {
                            text: { "Render Pipeline".to_string() },
                            font_size: 18.0,
                            font: { font.clone() },
                            color: Color::WHITE,
                        }
                        --
                        UText {
                            text: { "SDF Engine: Active\nMSAA: 4x Enabled\nBloom: Subsurface Lerp".to_string() },
                            font_size: 13.0,
                            font: { font.clone() },
                            color: Color::srgb(0.70, 0.75, 0.85),
                        }
                    ]
                ]
            ]
        ]
    });
}
