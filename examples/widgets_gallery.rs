//! Widgets Gallery showcase using Bevy Scene Notation (BSN).

use bevy::prelude::*;
use bevy_luma_widgets::prelude::*;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins.set(WindowPlugin {
            primary_window: Some(Window {
                title: "Luma Widgets Gallery".into(),
                resolution: (1080, 920).into(),
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
            padding: USides::all(28.0),
        }
        ULayout {
            display: UDisplay::Flex,
            flex_direction: UFlexDirection::Column,
            align_items: UAlignItems::Center,
            gap: 16.0,
        }
        Children [
            // Gallery Header
            (
                UNode::default()
                ULayout {
                    display: UDisplay::Flex,
                    flex_direction: UFlexDirection::Column,
                    align_items: UAlignItems::Center,
                    gap: 6.0,
                }
                Children [
                    (UText {
                        text: { "Luma Widgets Gallery".to_string() },
                        font_size: 26.0,
                        font: { font.clone() },
                        color: Color::WHITE,
                    }),
                    (UText {
                        text: { "Clean, accessible, SDF-rendered UI components with keyboard navigation".to_string() },
                        font_size: 13.0,
                        font: { font.clone() },
                        color: Color::srgb(0.55, 0.60, 0.72),
                    })
                ]
            ),

            // Horizontal Divider with label
            (
                luma_divider_with_label("COMPONENTS", font.clone())
            ),

            // Main Content Card
            (
                UNode {
                    width: UVal::Px(940.0),
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
                    // Section 1: Buttons Row
                    (
                        UNode::default()
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

                    // Section 2: Badges & Tabs Row
                    (
                        UNode::default()
                        ULayout {
                            display: UDisplay::Flex,
                            flex_direction: UFlexDirection::Row,
                            align_items: UAlignItems::Center,
                            justify_content: UJustifyContent::SpaceBetween,
                        }
                        Children [
                            // Badges
                            (
                                UNode::default()
                                ULayout {
                                    display: UDisplay::Flex,
                                    flex_direction: UFlexDirection::Row,
                                    align_items: UAlignItems::Center,
                                    gap: 8.0,
                                }
                                Children [
                                    (luma_badge("INFO", BadgeVariant::Info, font.clone())),
                                    (luma_badge("SUCCESS", BadgeVariant::Success, font.clone())),
                                    (luma_badge("WARNING", BadgeVariant::Warning, font.clone())),
                                    (luma_badge("DANGER", BadgeVariant::Danger, font.clone())),
                                    (luma_badge_with_icon(Icon::CHECK, "VERIFIED", BadgeVariant::Success, font.clone(), icon_font.clone()))
                                ]
                            ),
                            // Tabs
                            (
                                luma_tab_list((
                                    luma_tab_trigger("demo", "overview", "Overview", true, font.clone()),
                                    luma_tab_trigger("demo", "analytics", "Analytics", false, font.clone()),
                                    luma_tab_trigger("demo", "settings", "Settings", false, font.clone())
                                ))
                            )
                        ]
                    ),

                    // Section 3: Form Controls (Toggles, Checkboxes, Radios)
                    (
                        UNode::default()
                        ULayout {
                            display: UDisplay::Flex,
                            flex_direction: UFlexDirection::Row,
                            gap: 32.0,
                            align_items: UAlignItems::Center,
                        }
                        Children [
                            // Toggles column
                            (
                                UNode::default()
                                ULayout {
                                    display: UDisplay::Flex,
                                    flex_direction: UFlexDirection::Column,
                                    gap: 10.0,
                                }
                                Children [
                                    (luma_toggle_with_label("Enable Bloom", true, font.clone())),
                                    (luma_toggle_with_label("Hardware SDF", false, font.clone()))
                                ]
                            ),

                            // Checkboxes column
                            (
                                UNode::default()
                                ULayout {
                                    display: UDisplay::Flex,
                                    flex_direction: UFlexDirection::Column,
                                    gap: 10.0,
                                }
                                Children [
                                    (luma_checkbox("Spatial Audio", true, font.clone(), icon_font.clone())),
                                    (luma_checkbox("V-Sync Lock", false, font.clone(), icon_font.clone()))
                                ]
                            ),

                            // Radio Buttons column
                            (
                                UNode::default()
                                ULayout {
                                    display: UDisplay::Flex,
                                    flex_direction: UFlexDirection::Column,
                                    gap: 10.0,
                                }
                                Children [
                                    (luma_radio("quality", "low", "Low", false, font.clone())),
                                    (luma_radio("quality", "med", "Medium", true, font.clone())),
                                    (luma_radio("quality", "high", "High", false, font.clone()))
                                ]
                            )
                        ]
                    ),

                    // Section 4: Sliders, Progress Bars, and Avatars Row
                    (
                        UNode::default()
                        ULayout {
                            display: UDisplay::Flex,
                            flex_direction: UFlexDirection::Row,
                            justify_content: UJustifyContent::SpaceBetween,
                            align_items: UAlignItems::Center,
                        }
                        Children [
                            // Sliders & Progress
                            (
                                UNode::default()
                                ULayout {
                                    display: UDisplay::Flex,
                                    flex_direction: UFlexDirection::Column,
                                    gap: 12.0,
                                }
                                Children [
                                    (
                                        UNode::default()
                                        ULayout {
                                            display: UDisplay::Flex,
                                            flex_direction: UFlexDirection::Row,
                                            align_items: UAlignItems::Center,
                                            gap: 16.0,
                                        }
                                        Children [
                                            (UText {
                                                text: { "Volume:".to_string() },
                                                font_size: 13.0,
                                                font: { font.clone() },
                                                color: Color::srgb(0.70, 0.75, 0.85),
                                            }),
                                            (luma_slider(70.0, 0.0, 100.0, 240.0))
                                        ]
                                    ),
                                    (
                                        UNode::default()
                                        ULayout {
                                            display: UDisplay::Flex,
                                            flex_direction: UFlexDirection::Row,
                                            align_items: UAlignItems::Center,
                                            gap: 16.0,
                                        }
                                        Children [
                                            (UText {
                                                text: { "Download:".to_string() },
                                                font_size: 13.0,
                                                font: { font.clone() },
                                                color: Color::srgb(0.70, 0.75, 0.85),
                                            }),
                                            (luma_progress_bar(0.65, UVal::Px(240.0), 8.0, ProgressVariant::Success))
                                        ]
                                    )
                                ]
                            ),

                            // Avatars & Tooltip
                            (
                                UNode::default()
                                ULayout {
                                    display: UDisplay::Flex,
                                    flex_direction: UFlexDirection::Row,
                                    align_items: UAlignItems::Center,
                                    gap: 14.0,
                                }
                                Children [
                                    (luma_avatar_with_status("AH", AvatarSize::Medium, AvatarStatus::Online, font.clone())),
                                    (luma_avatar_with_status("JD", AvatarSize::Medium, AvatarStatus::Busy, font.clone())),
                                    (luma_avatar("AI", AvatarSize::Medium, font.clone())),
                                    (luma_tooltip("Online & Active", font.clone()))
                                ]
                            )
                        ]
                    ),

                    // Section 5: Alert Banner Callout
                    (
                        luma_alert(
                            "Keyboard Navigation Enabled",
                            "All widgets support Enter, Space, and Tab key activation. Sliders navigate with arrow keys.",
                            AlertVariant::Info,
                            font.clone(),
                            icon_font.clone(),
                        )
                    )
                ]
            )
        ]
    }
}
