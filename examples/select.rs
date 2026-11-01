//! Select dropdown widget showcase example in bevy_luma_widgets.

use bevy::prelude::*;
use bevy_luma_widgets::prelude::*;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins.set(WindowPlugin {
            primary_window: Some(Window {
                title: "Luma Widgets — Select Dropdown Example".into(),
                resolution: (960, 720).into(),
                ..default()
            }),
            ..default()
        }))
        .add_plugins(LumaWidgetsPlugin)
        .add_systems(Startup, setup)
        .add_observer(|trigger: On<SelectChanged>| {
            info!("SelectChanged: entity={:?}, index={}, value={}", trigger.event().entity, trigger.event().selected_index, trigger.event().value);
        })
        .run();
}

fn setup(mut commands: Commands, theme: Res<Theme>) {
    commands.spawn(Camera2d);
    let font = theme.text.font.inter_regular.clone();
    let icon_font = theme.icon.font.clone();

    let display_modes = vec![
        SelectOption::new("Fullscreen Exclusive", "fullscreen"),
        SelectOption::new("Borderless Windowed", "borderless"),
        SelectOption::new("Windowed (1080p)", "windowed"),
    ];

    let antialiasing_options = vec![
        SelectOption::new("Off", "none"),
        SelectOption::new("FXAA Fast", "fxaa"),
        SelectOption::new("TAA Temporal", "taa"),
        SelectOption::new("MSAA 4x", "msaa4x"),
    ];

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
                    text: { "Select Dropdown Component".to_string() },
                    font_size: 28.0,
                    font: { font.clone() },
                    color: Color::WHITE,
                }
                --
                UText {
                    text: { "Popup single-selection menu with chevron indicator and option list".to_string() },
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
                    text: { "1. Display Mode Selector".to_string() },
                    font_size: 16.0,
                    font: { font.clone() },
                    color: Color::srgb(0.40, 0.75, 1.0),
                }
                --
                @{ Select::new("Choose display mode...", display_modes).selected(1).font(font.clone()).icon_font(icon_font.clone()) }
                --
                UText {
                    text: { "2. Anti-Aliasing Setting".to_string() },
                    font_size: 16.0,
                    font: { font.clone() },
                    color: Color::srgb(0.40, 0.75, 1.0),
                }
                --
                @{ Select::new("Choose anti-aliasing...", antialiasing_options).font(font.clone()).icon_font(icon_font.clone()) }
                --
                UText {
                    text: { "3. Disabled Selector".to_string() },
                    font_size: 16.0,
                    font: { font.clone() },
                    color: Color::srgb(0.40, 0.75, 1.0),
                }
                --
                @{ Select::new("Unavailable on this GPU", vec![]).disabled(true).font(font.clone()).icon_font(icon_font.clone()) }
            ]
        ]
    });
}
