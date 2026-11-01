//! Accordion disclosure panel widget showcase example in bevy_luma_widgets.

use bevy::prelude::*;
use bevy_luma_widgets::prelude::*;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins.set(WindowPlugin {
            primary_window: Some(Window {
                title: "Luma Widgets — Accordion Example".into(),
                resolution: (960, 720).into(),
                ..default()
            }),
            ..default()
        }))
        .add_plugins(LumaWidgetsPlugin)
        .add_systems(Startup, setup)
        .add_observer(|trigger: On<AccordionToggled>| {
            info!("AccordionToggled: entity={:?}, is_expanded={}", trigger.event().entity, trigger.event().is_expanded);
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
                    text: { "Accordion Component".to_string() },
                    font_size: 28.0,
                    font: { font.clone() },
                    color: Color::WHITE,
                }
                --
                UText {
                    text: { "Collapsible disclosure panels with animated chevron toggles and content bodies".to_string() },
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
                gap: 16.0,
            }
            Children [
                @{
                    AccordionItem::new("What is bevy_luma_widgets?")
                        .expanded(true)
                        .font(font.clone())
                        .icon_font(icon_font.clone())
                        .content(bsn! {
                            UText {
                                text: { "bevy_luma_widgets is a modern, GPU signed distance field UI component suite built for Bevy Scene Notation (bsn!).".to_string() },
                                font_size: 14.0,
                                font: { font.clone() },
                                color: Color::srgb(0.75, 0.80, 0.90),
                            }
                        })
                }
                --
                @{
                    AccordionItem::new("How does theme inheritance work?")
                        .expanded(false)
                        .font(font.clone())
                        .icon_font(icon_font.clone())
                        .content(bsn! {
                            UText {
                                text: { "Components inherit styles automatically from ancestor LumaStyleScope nodes, matching Android's XML style hierarchy.".to_string() },
                                font_size: 14.0,
                                font: { font.clone() },
                                color: Color::srgb(0.75, 0.80, 0.90),
                            }
                        })
                }
                --
                @{
                    AccordionItem::new("Can I use reactive ECS observers?")
                        .expanded(false)
                        .font(font.clone())
                        .icon_font(icon_font.clone())
                        .content(bsn! {
                            UText {
                                text: { "Yes! Every interactive widget emits strongly typed Bevy observer events like ButtonClicked, ToggleChanged, and AccordionToggled.".to_string() },
                                font_size: 14.0,
                                font: { font.clone() },
                                color: Color::srgb(0.75, 0.80, 0.90),
                            }
                        })
                }
            ]
        ]
    });
}
