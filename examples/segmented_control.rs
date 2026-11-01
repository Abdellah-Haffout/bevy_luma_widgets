//! SegmentedControl widget showcase example in bevy_luma_widgets.

use bevy::prelude::*;
use bevy_luma_widgets::prelude::*;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins.set(WindowPlugin {
            primary_window: Some(Window {
                title: "Luma Widgets — Segmented Control Example".into(),
                resolution: (960, 720).into(),
                ..default()
            }),
            ..default()
        }))
        .add_plugins(LumaWidgetsPlugin)
        .add_systems(Startup, setup)
        .add_observer(|trigger: On<SegmentChanged>| {
            info!("SegmentChanged: group={}, value={}", trigger.event().group, trigger.event().value);
        })
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
                    text: { "Segmented Control Component".to_string() },
                    font_size: 28.0,
                    font: { font.clone() },
                    color: Color::WHITE,
                }
                --
                UText {
                    text: { "Pill button group for switching between cohesive view modes".to_string() },
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
                    text: { "1. Dashboard View Mode (3 Segments)".to_string() },
                    font_size: 16.0,
                    font: { font.clone() },
                    color: Color::srgb(0.40, 0.75, 1.0),
                }
                --
                @{
                    luma_segmented_control(
                        "view_mode",
                        "overview",
                        (
                            luma_segment_option("view_mode", "overview", "Overview", true, font.clone()),
                            luma_segment_option("view_mode", "analytics", "Analytics", false, font.clone()),
                            luma_segment_option("view_mode", "reports", "Reports", false, font.clone())
                        )
                    )
                }
                --
                UText {
                    text: { "2. Time Range Selector (4 Segments)".to_string() },
                    font_size: 16.0,
                    font: { font.clone() },
                    color: Color::srgb(0.40, 0.75, 1.0),
                }
                --
                @{
                    luma_segmented_control(
                        "time_range",
                        "7d",
                        (
                            luma_segment_option("time_range", "24h", "24 Hours", false, font.clone()),
                            luma_segment_option("time_range", "7d", "7 Days", true, font.clone()),
                            luma_segment_option("time_range", "30d", "30 Days", false, font.clone()),
                            luma_segment_option("time_range", "1y", "1 Year", false, font.clone())
                        )
                    )
                }
            ]
        ]
    });
}
