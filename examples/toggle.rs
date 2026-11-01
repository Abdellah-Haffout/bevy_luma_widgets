//! Toggle switch widget showcase example in bevy_luma_widgets.

use bevy::prelude::*;
use bevy_luma_widgets::prelude::*;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins.set(WindowPlugin {
            primary_window: Some(Window {
                title: "Luma Widgets — Toggle Switch Example".into(),
                resolution: (960, 720).into(),
                ..default()
            }),
            ..default()
        }))
        .add_plugins(LumaWidgetsPlugin)
        .add_systems(Startup, setup)
        .add_observer(|trigger: On<ToggleChanged>| {
            info!("ToggleChanged: entity={:?}, is_checked={}", trigger.event().entity, trigger.event().is_checked);
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
                    text: { "Toggle Switch Component".to_string() },
                    font_size: 28.0,
                    font: { font.clone() },
                    color: Color::WHITE,
                }
                --
                UText {
                    text: { "Animated binary switches with spring transitions and label attachments".to_string() },
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
                    text: { "1. Interactive Toggles with Labels".to_string() },
                    font_size: 16.0,
                    font: { font.clone() },
                    color: Color::srgb(0.40, 0.75, 1.0),
                }
                --
                UNode::default()
                ULayout {
                    display: UDisplay::Flex,
                    flex_direction: UFlexDirection::Column,
                    gap: 12.0,
                }
                Children [
                    @{ Toggle::new(true).label("Wi-Fi Network").font(font.clone()) }
                    --
                    @{ Toggle::new(false).label("Bluetooth Discovery").font(font.clone()) }
                    --
                    @{ Toggle::new(true).label("Hardware Acceleration").font(font.clone()) }
                ]
                --
                UText {
                    text: { "2. Standalone Switches & States".to_string() },
                    font_size: 16.0,
                    font: { font.clone() },
                    color: Color::srgb(0.40, 0.75, 1.0),
                }
                --
                UNode::default()
                ULayout {
                    display: UDisplay::Flex,
                    flex_direction: UFlexDirection::Row,
                    align_items: UAlignItems::Center,
                    gap: 16.0,
                }
                Children [
                    @{ Toggle::new(true) }
                    --
                    @{ Toggle::new(false) }
                    --
                    @{ Toggle::new(true).label("Disabled Active").disabled(true).font(font.clone()) }
                    --
                    @{ Toggle::new(false).label("Disabled Inactive").disabled(true).font(font.clone()) }
                ]
            ]
        ]
    });
}
