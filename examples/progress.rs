//! Progress indicators showcase example in bevy_luma_widgets:
//! Linear ProgressBar and CircularProgress (circular ring) with variants, sizes, and dynamic live update.

use bevy::prelude::*;
use bevy_luma_widgets::prelude::*;

/// Marker component for the live animated progress indicators in this demo.
#[derive(Component, Clone, Default, Reflect)]
struct DemoAnimatedProgress;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins.set(WindowPlugin {
            primary_window: Some(Window {
                title: "Luma Widgets — Progress Bar & Circular Progress Example".into(),
                resolution: (1080, 860).into(),
                ..default()
            }),
            ..default()
        }))
        .add_plugins(LumaWidgetsPlugin)
        .add_systems(Startup, setup)
        .add_systems(Update, animate_demo_progress_system)
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
            padding: USides::all(28.0),
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
                    text: { "Progress Components".to_string() },
                    font_size: 26.0,
                    font: { font.clone() },
                    color: Color::WHITE,
                }
                --
                UText {
                    text: { "Linear progress bars and circular progress rings with status variants".to_string() },
                    font_size: 13.0,
                    font: { font.clone() },
                    color: Color::srgb(0.60, 0.65, 0.75),
                }
            ]
            --
            // Main Showcase Container
            UNode {
                width: UVal::Px(840.0),
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
                // Section 1: Linear Progress Bars
                UText {
                    text: { "1. Linear Progress Bars".to_string() },
                    font_size: 15.0,
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
                    @{ ProgressBar::new(0.65).width(UVal::Percent(1.0)).variant(ProgressVariant::Default) }
                    --
                    @{ ProgressBar::new(1.00).width(UVal::Percent(1.0)).variant(ProgressVariant::Success) }
                    --
                    @{ ProgressBar::new(0.45).width(UVal::Percent(1.0)).variant(ProgressVariant::Warning) }
                    --
                    @{ ProgressBar::new(0.20).width(UVal::Percent(1.0)).variant(ProgressVariant::Danger) }
                ]
                --
                // Section 2: Circular Progress Rings
                UText {
                    text: { "2. Circular Progress Rings (Semantic Status Variants)".to_string() },
                    font_size: 15.0,
                    font: { font.clone() },
                    color: Color::srgb(0.40, 0.75, 1.0),
                }
                --
                UNode::default()
                ULayout {
                    display: UDisplay::Flex,
                    flex_direction: UFlexDirection::Row,
                    justify_content: UJustifyContent::SpaceAround,
                    align_items: UAlignItems::Center,
                }
                Children [
                    // Default / Primary 75%
                    UNode::default()
                    ULayout {
                        display: UDisplay::Flex,
                        flex_direction: UFlexDirection::Column,
                        align_items: UAlignItems::Center,
                        gap: 8.0,
                    }
                    Children [
                        @{ CircularProgress::new(0.75).size(72.0).thickness(6.5).variant(ProgressVariant::Default).font(font.clone()) }
                        --
                        UText {
                            text: { "Primary (75%)".to_string() },
                            font_size: 12.0,
                            font: { font.clone() },
                            color: Color::srgb(0.65, 0.70, 0.80),
                        }
                    ]
                    --
                    // Success 100%
                    UNode::default()
                    ULayout {
                        display: UDisplay::Flex,
                        flex_direction: UFlexDirection::Column,
                        align_items: UAlignItems::Center,
                        gap: 8.0,
                    }
                    Children [
                        @{ CircularProgress::new(1.00).size(72.0).thickness(6.5).variant(ProgressVariant::Success).font(font.clone()) }
                        --
                        UText {
                            text: { "Success (100%)".to_string() },
                            font_size: 12.0,
                            font: { font.clone() },
                            color: Color::srgb(0.65, 0.70, 0.80),
                        }
                    ]
                    --
                    // Warning 50%
                    UNode::default()
                    ULayout {
                        display: UDisplay::Flex,
                        flex_direction: UFlexDirection::Column,
                        align_items: UAlignItems::Center,
                        gap: 8.0,
                    }
                    Children [
                        @{ CircularProgress::new(0.50).size(72.0).thickness(6.5).variant(ProgressVariant::Warning).font(font.clone()) }
                        --
                        UText {
                            text: { "Warning (50%)".to_string() },
                            font_size: 12.0,
                            font: { font.clone() },
                            color: Color::srgb(0.65, 0.70, 0.80),
                        }
                    ]
                    --
                    // Danger 25%
                    UNode::default()
                    ULayout {
                        display: UDisplay::Flex,
                        flex_direction: UFlexDirection::Column,
                        align_items: UAlignItems::Center,
                        gap: 8.0,
                    }
                    Children [
                        @{ CircularProgress::new(0.25).size(72.0).thickness(6.5).variant(ProgressVariant::Danger).font(font.clone()) }
                        --
                        UText {
                            text: { "Danger (25%)".to_string() },
                            font_size: 12.0,
                            font: { font.clone() },
                            color: Color::srgb(0.65, 0.70, 0.80),
                        }
                    ]
                ]
                --
                // Section 3: Circular Progress Sizes
                UText {
                    text: { "3. Circular Progress Sizes (sm, md, lg, xl)".to_string() },
                    font_size: 15.0,
                    font: { font.clone() },
                    color: Color::srgb(0.40, 0.75, 1.0),
                }
                --
                UNode::default()
                ULayout {
                    display: UDisplay::Flex,
                    flex_direction: UFlexDirection::Row,
                    justify_content: UJustifyContent::SpaceAround,
                    align_items: UAlignItems::Center,
                }
                Children [
                    // Small 36px
                    UNode::default()
                    ULayout {
                        display: UDisplay::Flex,
                        flex_direction: UFlexDirection::Column,
                        align_items: UAlignItems::Center,
                        gap: 6.0,
                    }
                    Children [
                        @{ CircularProgress::sm(0.60).font(font.clone()) }
                        --
                        UText {
                            text: { "sm (36px)".to_string() },
                            font_size: 11.0,
                            font: { font.clone() },
                            color: Color::srgb(0.65, 0.70, 0.80),
                        }
                    ]
                    --
                    // Medium 64px
                    UNode::default()
                    ULayout {
                        display: UDisplay::Flex,
                        flex_direction: UFlexDirection::Column,
                        align_items: UAlignItems::Center,
                        gap: 6.0,
                    }
                    Children [
                        @{ CircularProgress::md(0.75).font(font.clone()) }
                        --
                        UText {
                            text: { "md (64px)".to_string() },
                            font_size: 11.0,
                            font: { font.clone() },
                            color: Color::srgb(0.65, 0.70, 0.80),
                        }
                    ]
                    --
                    // Large 96px
                    UNode::default()
                    ULayout {
                        display: UDisplay::Flex,
                        flex_direction: UFlexDirection::Column,
                        align_items: UAlignItems::Center,
                        gap: 6.0,
                    }
                    Children [
                        @{ CircularProgress::lg(0.85).font(font.clone()) }
                        --
                        UText {
                            text: { "lg (96px)".to_string() },
                            font_size: 11.0,
                            font: { font.clone() },
                            color: Color::srgb(0.65, 0.70, 0.80),
                        }
                    ]
                    --
                    // Extra-Large 116px
                    UNode::default()
                    ULayout {
                        display: UDisplay::Flex,
                        flex_direction: UFlexDirection::Column,
                        align_items: UAlignItems::Center,
                        gap: 6.0,
                    }
                    Children [
                        @{ CircularProgress::xl(0.92).size(116.0).thickness(9.0).font(font.clone()) }
                        --
                        UText {
                            text: { "xl (116px)".to_string() },
                            font_size: 11.0,
                            font: { font.clone() },
                            color: Color::srgb(0.65, 0.70, 0.80),
                        }
                    ]
                ]
                --
                // Section 4: Real-time Live Animated Sync
                UText {
                    text: { "4. Real-time Live Dynamic Update (Synchronized Linear + Circular)".to_string() },
                    font_size: 15.0,
                    font: { font.clone() },
                    color: Color::srgb(0.40, 0.75, 1.0),
                }
                --
                UNode {
                    background_color: Color::srgba(0.14, 0.17, 0.24, 0.6),
                    border_radius: UCornerRadius::all(10.0),
                    padding: USides::all(16.0),
                }
                UBorder {
                    color: Color::srgb(0.22, 0.27, 0.38),
                    width: 1.0,
                    radius: UCornerRadius::all(10.0),
                }
                ULayout {
                    display: UDisplay::Flex,
                    flex_direction: UFlexDirection::Row,
                    align_items: UAlignItems::Center,
                    gap: 20.0,
                }
                Children [
                    DemoAnimatedProgress
                    @{ CircularProgress::new(0.0).size(80.0).thickness(7.0).variant(ProgressVariant::Success).font(font.clone()) }
                    --
                    UNode::default()
                    USelf {
                        flex_grow: { Some(1.0) },
                    }
                    ULayout {
                        display: UDisplay::Flex,
                        flex_direction: UFlexDirection::Column,
                        gap: 8.0,
                    }
                    Children [
                        UText {
                            text: { "Real-time task synchronization progress...".to_string() },
                            font_size: 13.0,
                            font: { font.clone() },
                            color: Color::srgb(0.85, 0.88, 0.95),
                        }
                        --
                        DemoAnimatedProgress
                        @{ ProgressBar::new(0.0).width(UVal::Percent(1.0)).height(10.0).variant(ProgressVariant::Success) }
                    ]
                ]
            ]
        ]
    });
}

/// Animates the live progress bar and circular progress ring continuously in real-time.
fn animate_demo_progress_system(
    time: Res<Time>,
    mut circular_query: Query<&mut LumaCircularProgress, With<DemoAnimatedProgress>>,
    mut linear_query: Query<&mut LumaProgressBar, With<DemoAnimatedProgress>>,
) {
    let t = (time.elapsed_secs() * 0.25).fract(); // Loops 0.0 -> 1.0 every 4 seconds

    for mut circular in circular_query.iter_mut() {
        circular.value = t;
    }

    for mut linear in linear_query.iter_mut() {
        linear.value = t;
    }
}
