//! Navigation Shells & Menus showcase example in bevy_luma_widgets.

use bevy::prelude::*;
use bevy_luma_widgets::prelude::*;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins.set(WindowPlugin {
            primary_window: Some(Window {
                title: "Luma Widgets — Navigation Suite".into(),
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
                    text: { "Navigation & Application Shells Suite".to_string() },
                    font_size: 26.0,
                    font: { font.clone() },
                    color: Color::WHITE,
                }
                --
                UText {
                    text: { "Top navbars, application sidebars, pagination, steptrackers, and menus".to_string() },
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
                // Left Column: Navbar, Sidebar
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
                        text: { "1. Navbar & Top Navigation Header".to_string() },
                        font_size: 15.0,
                        font: { font.clone() },
                        color: Color::srgb(0.40, 0.75, 1.0),
                    }
                    --
                    @{
                        Navbar::new("LumaStudio")
                            .items(vec!["Dashboard".into(), "Components".into(), "Themes".into(), "Docs".into()])
                            .active(1)
                            .font(font.clone())
                            .icon_font(icon_font.clone())
                    }
                    --
                    UText {
                        text: { "2. Application Sidebar".to_string() },
                        font_size: 15.0,
                        font: { font.clone() },
                        color: Color::srgb(0.40, 0.75, 1.0),
                    }
                    --
                    @{
                        Sidebar::new()
                            .brand("Navigation Engine")
                            .items(vec![
                                (Icon::STAR.to_string(), "Dashboard".into()),
                                (Icon::FOLDER.to_string(), "Widgets".into()),
                                (Icon::SETTINGS.to_string(), "Settings".into()),
                            ])
                            .active(1)
                            .font(font.clone())
                            .icon_font(icon_font.clone())
                    }
                ]
                --
                // Right Column: Pagination, StepTracker, Filter
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
                        text: { "3. Multi-Step Onboarding Tracker".to_string() },
                        font_size: 15.0,
                        font: { font.clone() },
                        color: Color::srgb(0.40, 0.75, 1.0),
                    }
                    --
                    @{
                        StepTracker::new(vec!["Account", "Workspace", "Billing", "Review"])
                            .current(1)
                            .font(font.clone())
                            .icon_font(icon_font.clone())
                    }
                    --
                    UText {
                        text: { "4. Pagination Controls".to_string() },
                        font_size: 15.0,
                        font: { font.clone() },
                        color: Color::srgb(0.40, 0.75, 1.0),
                    }
                    --
                    @{
                        Pagination::new(4, 12)
                            .font(font.clone())
                            .icon_font(icon_font.clone())
                    }
                    --
                    UText {
                        text: { "5. Filter & Sorting Bar".to_string() },
                        font_size: 15.0,
                        font: { font.clone() },
                        color: Color::srgb(0.40, 0.75, 1.0),
                    }
                    --
                    @{ FilterSortBar::new().filter("All Types").sort("Most Recent").font(font.clone()).icon_font(icon_font.clone()) }
                ]
            ]
        ]
    });
}
