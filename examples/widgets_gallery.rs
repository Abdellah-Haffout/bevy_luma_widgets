//! Widgets Gallery showcase using Bevy Scene Notation (BSN).

use bevy::prelude::*;
use bevy_luma_widgets::prelude::*;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins.set(WindowPlugin {
            primary_window: Some(Window {
                title: "Luma Widgets Gallery".into(),
                resolution: (1000, 750).into(),
                ..default()
            }),
            ..default()
        }))
        .add_plugins(LumaWidgetsPlugin)
        .add_systems(Startup, setup_gallery)
        .run();
}

fn setup_gallery(mut commands: Commands, theme: Res<Theme>) {
    commands.spawn(Camera2d);
    commands.spawn_scene(gallery_scene(&theme));
}

fn gallery_scene(theme: &Theme) -> impl Scene {
    let font = theme.text.font.inter_regular.clone();
    let icon_font = theme.icon.font.clone();

    bsn! {
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
            gap: 24.0,
            align_items: UAlignItems::Center,
        }
        Children [
            // Gallery Header
            (
                ULayout {
                    display: UDisplay::Flex,
                    flex_direction: UFlexDirection::Column,
                    gap: 6.0,
                    align_items: UAlignItems::Center,
                }
                Children [
                    (UText {
                        text: { "Luma Widgets Gallery".to_string() },
                        font_size: 26.0,
                        font: { font.clone() },
                        color: Color::WHITE,
                    }),
                    (UText {
                        text: { "Clean, accessible, SDF-rendered UI components for Bevy".to_string() },
                        font_size: 14.0,
                        font: { font.clone() },
                        color: Color::srgb(0.55, 0.60, 0.72),
                    })
                ]
            ),

            // Horizontal Divider with label
            (
                luma_divider_with_label("COMPONENTS", font.clone())
            ),

            // Main Content Grid / Card
            (
                UNode {
                    width: UVal::Px(880.0),
                    background_color: Color::srgb(0.10, 0.12, 0.16),
                    border_radius: UCornerRadius::all(14.0),
                    padding: USides::all(28.0),
                }
                UBorder {
                    color: Color::srgb(0.18, 0.22, 0.30),
                    width: 1.0,
                    radius: UCornerRadius::all(14.0),
                }
                ULayout {
                    display: UDisplay::Flex,
                    flex_direction: UFlexDirection::Column,
                    gap: 24.0,
                }
                Children [
                    // Section 1: Buttons
                    (
                        ULayout {
                            display: UDisplay::Flex,
                            flex_direction: UFlexDirection::Row,
                            align_items: UAlignItems::Center,
                            gap: 12.0,
                        }
                        Children [
                            (luma_button("Primary", ButtonVariant::Primary, ButtonSize::Medium, font.clone())),
                            (luma_button("Secondary", ButtonVariant::Secondary, ButtonSize::Medium, font.clone())),
                            (luma_button("Outline", ButtonVariant::Outline, ButtonSize::Medium, font.clone())),
                            (luma_button("Ghost", ButtonVariant::Ghost, ButtonSize::Medium, font.clone())),
                            (luma_button("Danger", ButtonVariant::Danger, ButtonSize::Medium, font.clone())),
                            (luma_icon_button(Icon::PLAY, ButtonVariant::Primary, ButtonSize::Medium, icon_font.clone()))
                        ]
                    ),

                    // Section 2: Badges
                    (
                        ULayout {
                            display: UDisplay::Flex,
                            flex_direction: UFlexDirection::Row,
                            align_items: UAlignItems::Center,
                            gap: 10.0,
                        }
                        Children [
                            (luma_badge("INFO", BadgeVariant::Info, font.clone())),
                            (luma_badge("SUCCESS", BadgeVariant::Success, font.clone())),
                            (luma_badge("WARNING", BadgeVariant::Warning, font.clone())),
                            (luma_badge("DANGER", BadgeVariant::Danger, font.clone())),
                            (luma_badge("NEUTRAL", BadgeVariant::Neutral, font.clone())),
                            (luma_badge_with_icon(Icon::CHECK, "VERIFIED", BadgeVariant::Success, font.clone(), icon_font.clone()))
                        ]
                    ),

                    // Section 3: Form Controls (Toggles, Checkboxes, Radios)
                    (
                        ULayout {
                            display: UDisplay::Flex,
                            flex_direction: UFlexDirection::Row,
                            gap: 36.0,
                            align_items: UAlignItems::Center,
                        }
                        Children [
                            // Toggles
                            (
                                ULayout {
                                    display: UDisplay::Flex,
                                    flex_direction: UFlexDirection::Column,
                                    gap: 12.0,
                                }
                                Children [
                                    (luma_toggle_with_label("Enable Bloom", true, font.clone())),
                                    (luma_toggle_with_label("Hardware SDF", false, font.clone()))
                                ]
                            ),

                            // Checkboxes
                            (
                                ULayout {
                                    display: UDisplay::Flex,
                                    flex_direction: UFlexDirection::Column,
                                    gap: 12.0,
                                }
                                Children [
                                    (luma_checkbox("Spatial Audio", true, font.clone(), icon_font.clone())),
                                    (luma_checkbox("V-Sync Lock", false, font.clone(), icon_font.clone()))
                                ]
                            ),

                            // Radio Buttons
                            (
                                ULayout {
                                    display: UDisplay::Flex,
                                    flex_direction: UFlexDirection::Column,
                                    gap: 12.0,
                                }
                                Children [
                                    (luma_radio("quality", "low", "Low", false, font.clone())),
                                    (luma_radio("quality", "med", "Medium", true, font.clone())),
                                    (luma_radio("quality", "high", "High", false, font.clone()))
                                ]
                            )
                        ]
                    ),

                    // Section 4: Sliders and Progress
                    (
                        ULayout {
                            display: UDisplay::Flex,
                            flex_direction: UFlexDirection::Column,
                            gap: 14.0,
                        }
                        Children [
                            (
                                ULayout {
                                    display: UDisplay::Flex,
                                    flex_direction: UFlexDirection::Row,
                                    align_items: UAlignItems::Center,
                                    gap: 16.0,
                                }
                                Children [
                                    (UText {
                                        text: { "Volume:".to_string() },
                                        font_size: 14.0,
                                        font: { font.clone() },
                                        color: Color::srgb(0.70, 0.75, 0.85),
                                    }),
                                    (luma_slider(70.0, 0.0, 100.0, 260.0))
                                ]
                            ),
                            (
                                ULayout {
                                    display: UDisplay::Flex,
                                    flex_direction: UFlexDirection::Row,
                                    align_items: UAlignItems::Center,
                                    gap: 16.0,
                                }
                                Children [
                                    (UText {
                                        text: { "Download:".to_string() },
                                        font_size: 14.0,
                                        font: { font.clone() },
                                        color: Color::srgb(0.70, 0.75, 0.85),
                                    }),
                                    (luma_progress_bar(0.65, UVal::Px(260.0), 8.0, ProgressVariant::Success))
                                ]
                            )
                        ]
                    )
                ]
            )
        ]
    }
}
