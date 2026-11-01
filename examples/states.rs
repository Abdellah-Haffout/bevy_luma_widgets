//! UI States & Feedback screens showcase example in bevy_luma_widgets.

use bevy::prelude::*;
use bevy_luma_widgets::prelude::*;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins.set(WindowPlugin {
            primary_window: Some(Window {
                title: "Luma Widgets — UI States Suite".into(),
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
                    text: { "UI States & Error Feedback Suite".to_string() },
                    font_size: 26.0,
                    font: { font.clone() },
                    color: Color::WHITE,
                }
                --
                UText {
                    text: { "Empty states, 404 screens, offline alerts, success confirmations, and coachmarks".to_string() },
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
                // Left Column: EmptyState, Error404, OfflineState
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
                        text: { "1. Empty State Placeholder".to_string() },
                        font_size: 15.0,
                        font: { font.clone() },
                        color: Color::srgb(0.40, 0.75, 1.0),
                    }
                    --
                    @{
                        EmptyState::new(
                            "No Data Points Yet",
                            "Create a new project or import a dataset to view interactive metrics.",
                        )
                        .icon(Icon::INBOX)
                        .font(font.clone())
                        .icon_font(icon_font.clone())
                    }
                    --
                    UText {
                        text: { "2. HTTP 404 Route Missing".to_string() },
                        font_size: 15.0,
                        font: { font.clone() },
                        color: Color::srgb(0.40, 0.75, 1.0),
                    }
                    --
                    @{
                        Error404State::new()
                            .code("404")
                            .title("Page Not Found")
                            .message("The requested component route does not exist in the active registry.")
                            .font(font.clone())
                    }
                ]
                --
                // Right Column: Offline, Success, Coachmark
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
                        text: { "3. Offline State Disconnect Banner".to_string() },
                        font_size: 15.0,
                        font: { font.clone() },
                        color: Color::srgb(0.40, 0.75, 1.0),
                    }
                    --
                    @{ OfflineState::new().title("Disconnected from Edge").retry_label("Reconnect Now").font(font.clone()).icon_font(icon_font.clone()) }
                    --
                    UText {
                        text: { "4. Interactive Feature Coachmark / Tooltip Tour".to_string() },
                        font_size: 15.0,
                        font: { font.clone() },
                        color: Color::srgb(0.40, 0.75, 1.0),
                    }
                    --
                    @{
                        Coachmark::new("SDF Shader Antialiasing", "Every component uses signed distance fields calculated in fragment shaders for razor-sharp rendering.")
                            .step("Step 1 of 4")
                            .font(font.clone())
                    }
                ]
            ]
        ]
    });
}
