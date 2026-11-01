//! Button widget showcase example in bevy_luma_widgets.

use bevy::prelude::*;
use bevy_luma_widgets::prelude::*;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins.set(WindowPlugin {
            primary_window: Some(Window {
                title: "Luma Widgets — Button Example".into(),
                resolution: (960, 720).into(),
                ..default()
            }),
            ..default()
        }))
        .add_plugins(LumaWidgetsPlugin)
        .add_systems(Startup, setup)
        .add_observer(|trigger: On<ButtonClicked>| {
            info!("ButtonClicked: {:?}", trigger.event().0);
        })
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
                    text: { "Button Component".to_string() },
                    font_size: 28.0,
                    font: { font.clone() },
                    color: Color::WHITE,
                }
                --
                UText {
                    text: { "Interactive buttons with multiple variants, sizes, icons, and loading states".to_string() },
                    font_size: 14.0,
                    font: { font.clone() },
                    color: Color::srgb(0.60, 0.65, 0.75),
                }
            ]
            --
            // Main Showcase Container
            UNode {
                width: UVal::Px(820.0),
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
                // Section 1: Variants
                UText {
                    text: { "1. Semantic Variants".to_string() },
                    font_size: 16.0,
                    font: { font.clone() },
                    color: Color::srgb(0.40, 0.75, 1.0),
                }
                --
                UNode::default()
                ULayout {
                    display: UDisplay::Flex,
                    flex_direction: UFlexDirection::Row,
                    gap: 12.0,
                }
                Children [
                    @{ Button::new("Primary").variant(ButtonVariant::Primary).font(font.clone()) }
                    --
                    @{ Button::new("Secondary").variant(ButtonVariant::Secondary).font(font.clone()) }
                    --
                    @{ Button::new("Outline").variant(ButtonVariant::Outline).font(font.clone()) }
                    --
                    @{ Button::new("Ghost").variant(ButtonVariant::Ghost).font(font.clone()) }
                    --
                    @{ Button::new("Danger").variant(ButtonVariant::Danger).font(font.clone()) }
                ]
                --
                // Section 2: Sizes
                UText {
                    text: { "2. Sizing Presets".to_string() },
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
                    gap: 12.0,
                }
                Children [
                    @{ Button::new("Small Size").small().font(font.clone()) }
                    --
                    @{ Button::new("Medium Size").medium().font(font.clone()) }
                    --
                    @{ Button::new("Large Size").large().font(font.clone()) }
                ]
                --
                // Section 3: Icons & States
                UText {
                    text: { "3. Icons & Interactive States".to_string() },
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
                    gap: 12.0,
                }
                Children [
                    @{ Button::new("Start Game").icon(Icon::PLAY).font(font.clone()).icon_font(icon_font.clone()) }
                    --
                    @{ Button::icon_button(Icon::SETTINGS).font(font.clone()).icon_font(icon_font.clone()) }
                    --
                    @{ Button::new("Loading...").loading(true).font(font.clone()) }
                    --
                    @{ Button::new("Disabled").disabled(true).font(font.clone()) }
                ]
            ]
        ]
    });
}
