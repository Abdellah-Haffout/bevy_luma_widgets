//! Realistic Game / App Settings Dialog example using Bevy Scene Notation (BSN).

use bevy::prelude::*;
use bevy_luma_widgets::prelude::*;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins.set(WindowPlugin {
            primary_window: Some(Window {
                title: "Luma Settings Dialog".into(),
                resolution: (900, 700).into(),
                ..default()
            }),
            ..default()
        }))
        .add_plugins(LumaWidgetsPlugin)
        .add_systems(Startup, setup_settings)
        .run();
}

fn setup_settings(mut commands: Commands, theme: Res<Theme>) {
    commands.spawn(Camera2d);
    commands.spawn_scene(settings_scene(&theme));
}

fn settings_scene(theme: &Theme) -> impl Scene {
    let font = theme.text.font.inter_regular.clone();
    let icon_font = theme.icon.font.clone();

    bsn! {
        URootUi::screen()
        UNode {
            width: UVal::Percent(1.0),
            height: UVal::Percent(1.0),
            background_color: Color::srgba(0.04, 0.05, 0.07, 0.85), // Dimmed backdrop
        }
        ULayout {
            display: UDisplay::Flex,
            justify_content: UJustifyContent::Center,
            align_items: UAlignItems::Center,
        }
        Children [
            (
                UNode {
                    width: UVal::Px(480.0),
                    background_color: Color::srgb(0.10, 0.12, 0.17),
                    border_radius: UCornerRadius::all(16.0),
                    padding: USides::all(28.0),
                }
                UBorder {
                    color: Color::srgb(0.20, 0.25, 0.35),
                    width: 1.0,
                    radius: UCornerRadius::all(16.0),
                }
                ULayout {
                    display: UDisplay::Flex,
                    flex_direction: UFlexDirection::Column,
                    gap: 20.0,
                }
                Children [
                    // Dialog Header
                    (
                        UNode {
                            width: UVal::Percent(1.0),
                        }
                        ULayout {
                            display: UDisplay::Flex,
                            flex_direction: UFlexDirection::Row,
                            justify_content: UJustifyContent::SpaceBetween,
                            align_items: UAlignItems::Center,
                        }
                        Children [
                            (UText {
                                text: { "Game Settings".to_string() },
                                font_size: 20.0,
                                font: { font.clone() },
                                color: Color::WHITE,
                            }),
                            (luma_badge("SAVED", BadgeVariant::Success, font.clone()))
                        ]
                    ),

                    (luma_divider()),

                    // Setting 1: Master Volume
                    (
                        UNode {
                            width: UVal::Percent(1.0),
                        }
                        ULayout {
                            display: UDisplay::Flex,
                            flex_direction: UFlexDirection::Column,
                            gap: 8.0,
                        }
                        Children [
                            (UText {
                                text: { "Master Volume".to_string() },
                                font_size: 14.0,
                                font: { font.clone() },
                                color: Color::srgb(0.85, 0.88, 0.95),
                            }),
                            (luma_slider(80.0, 0.0, 100.0, 424.0))
                        ]
                    ),

                    // Setting 2: Graphics Quality
                    (
                        UNode {
                            width: UVal::Percent(1.0),
                        }
                        ULayout {
                            display: UDisplay::Flex,
                            flex_direction: UFlexDirection::Column,
                            gap: 8.0,
                        }
                        Children [
                            (UText {
                                text: { "Graphics Preset".to_string() },
                                font_size: 14.0,
                                font: { font.clone() },
                                color: Color::srgb(0.85, 0.88, 0.95),
                            }),
                            (
                                UNode::default()
                                ULayout {
                                    display: UDisplay::Flex,
                                    flex_direction: UFlexDirection::Row,
                                    gap: 24.0,
                                }
                                Children [
                                    (luma_radio("preset", "low", "Low", false, font.clone())),
                                    (luma_radio("preset", "medium", "Medium", false, font.clone())),
                                    (luma_radio("preset", "high", "High", true, font.clone()))
                                ]
                            )
                        ]
                    ),

                    // Setting 3: Toggles
                    (
                        UNode {
                            width: UVal::Percent(1.0),
                        }
                        ULayout {
                            display: UDisplay::Flex,
                            flex_direction: UFlexDirection::Column,
                            gap: 12.0,
                        }
                        Children [
                            (luma_toggle_with_label("Fullscreen Mode", true, font.clone())),
                            (luma_toggle_with_label("Hardware Anti-Aliasing", true, font.clone())),
                            (luma_checkbox("Show FPS Counter", true, font.clone(), icon_font.clone()))
                        ]
                    ),

                    (luma_divider()),

                    // Dialog Actions Footer
                    (
                        UNode {
                            width: UVal::Percent(1.0),
                        }
                        ULayout {
                            display: UDisplay::Flex,
                            flex_direction: UFlexDirection::Row,
                            justify_content: UJustifyContent::End,
                            gap: 12.0,
                        }
                        Children [
                            (
                                luma_button("Cancel", ButtonVariant::Secondary, ButtonSize::Medium, font.clone())
                                on(|_event: On<Pointer<Click>>| {
                                    info!("Settings cancelled");
                                })
                            ),
                            (
                                luma_button("Save Changes", ButtonVariant::Primary, ButtonSize::Medium, font.clone())
                                on(|_event: On<Pointer<Click>>| {
                                    info!("Settings saved");
                                })
                            )
                        ]
                    )
                ]
            )
        ]
    }
}
