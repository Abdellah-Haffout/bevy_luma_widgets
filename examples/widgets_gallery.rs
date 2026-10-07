//! Widgets Gallery showcase using Bevy Scene Notation (BSN).

use bevy::prelude::*;
use bevy_luma_widgets::prelude::*;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins.set(WindowPlugin {
            primary_window: Some(Window {
                title: "Luma Widgets Gallery".into(),
                resolution: (1160, 1020).into(),
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
            padding: USides::all(24.0),
        }
        ULayout {
            display: UDisplay::Flex,
            flex_direction: UFlexDirection::Column,
            align_items: UAlignItems::Center,
            gap: 14.0,
        }
        Children [
            // Gallery Header & Breadcrumbs
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
                    (
                        luma_breadcrumb_trail((
                            luma_breadcrumb_item(0, "home", "Home", false, font.clone()),
                            luma_breadcrumb_separator(icon_font.clone()),
                            luma_breadcrumb_item(1, "components", "Components", false, font.clone()),
                            luma_breadcrumb_separator(icon_font.clone()),
                            luma_breadcrumb_item(2, "widgets", "Gallery", true, font.clone())
                        ))
                    )
                ]
            ),

            // Main Content Card
            (
                UNode {
                    width: UVal::Px(1040.0),
                    background_color: Color::srgb(0.10, 0.12, 0.16),
                    border_radius: UCornerRadius::all(14.0),
                    padding: USides::all(20.0),
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
                    // Section 1: KPI Stat Cards Row
                    (
                        UNode::default()
                        ULayout {
                            display: UDisplay::Flex,
                            flex_direction: UFlexDirection::Row,
                            justify_content: UJustifyContent::SpaceBetween,
                            gap: 12.0,
                        }
                        Children [
                            (luma_stat_card_with_trend(
                                "Active Users",
                                "14,820",
                                StatTrend::Up("+14.2%".into()),
                                font.clone(),
                                icon_font.clone(),
                            )),
                            (luma_stat_card_with_trend(
                                "Server Latency",
                                "18 ms",
                                StatTrend::Down("-8.5%".into()),
                                font.clone(),
                                icon_font.clone(),
                            )),
                            (luma_stat_card_full(
                                "Frame Rate",
                                "144 FPS",
                                StatTrend::Up("+12 FPS".into()),
                                "VSync lock enabled",
                                Some(Icon::CPU),
                                font.clone(),
                                icon_font.clone(),
                            ))
                        ]
                    ),

                    // Section 2: Segmented Controls, Stepper, Rating & Shortcuts
                    (
                        UNode::default()
                        ULayout {
                            display: UDisplay::Flex,
                            flex_direction: UFlexDirection::Row,
                            justify_content: UJustifyContent::SpaceBetween,
                            align_items: UAlignItems::Center,
                        }
                        Children [
                            // Segmented Control
                            (
                                luma_segmented_control(
                                    "view_mode",
                                    "analytics",
                                    (
                                        luma_segment_option("view_mode", "overview", "Overview", false, font.clone()),
                                        luma_segment_option("view_mode", "analytics", "Analytics", true, font.clone()),
                                        luma_segment_option("view_mode", "settings", "Settings", false, font.clone())
                                    )
                                )
                            ),
                            // Stepper Counter
                            (
                                UNode::default()
                                ULayout {
                                    display: UDisplay::Flex,
                                    flex_direction: UFlexDirection::Row,
                                    align_items: UAlignItems::Center,
                                    gap: 8.0,
                                }
                                Children [
                                    (UText {
                                        text: { "Quantity:".to_string() },
                                        font_size: 13.0,
                                        font: { font.clone() },
                                        color: Color::srgb(0.65, 0.70, 0.80),
                                    }),
                                    (luma_stepper(3.0, 1.0, 20.0, 1.0, font.clone(), icon_font.clone()))
                                ]
                            ),
                            // Interactive Star Rating
                            (
                                UNode::default()
                                ULayout {
                                    display: UDisplay::Flex,
                                    flex_direction: UFlexDirection::Row,
                                    align_items: UAlignItems::Center,
                                    gap: 8.0,
                                }
                                Children [
                                    (UText {
                                        text: { "Rating:".to_string() },
                                        font_size: 13.0,
                                        font: { font.clone() },
                                        color: Color::srgb(0.65, 0.70, 0.80),
                                    }),
                                    (luma_interactive_rating(4, icon_font.clone()))
                                ]
                            ),
                            // Keycap shortcut badge
                            (
                                luma_kbd_shortcut((
                                    luma_kbd("Ctrl", KbdSize::Medium, font.clone()),
                                    luma_kbd_separator(font.clone()),
                                    luma_kbd("K", KbdSize::Medium, font.clone())
                                ))
                            )
                        ]
                    ),

                    // Section 3: Buttons & Badges Row
                    (
                        UNode::default()
                        ULayout {
                            display: UDisplay::Flex,
                            flex_direction: UFlexDirection::Row,
                            align_items: UAlignItems::Center,
                            justify_content: UJustifyContent::SpaceBetween,
                        }
                        Children [
                            // Buttons
                            (
                                UNode::default()
                                ULayout {
                                    display: UDisplay::Flex,
                                    flex_direction: UFlexDirection::Row,
                                    align_items: UAlignItems::Center,
                                    gap: 10.0,
                                }
                                Children [
                                    ({ Button::new("Primary").variant(ButtonVariant::Primary).font(font.clone()) }),
                                    ({ Button::new("Secondary").variant(ButtonVariant::Secondary).font(font.clone()) }),
                                    ({ Button::new("Outline").variant(ButtonVariant::Outline).font(font.clone()) }),
                                    ({ Button::new("Ghost").variant(ButtonVariant::Ghost).font(font.clone()) }),
                                    ({ Button::new("Danger").variant(ButtonVariant::Danger).font(font.clone()) }),
                                    ({ Button::new("").icon(Icon::PLAY).variant(ButtonVariant::Primary).icon_font(icon_font.clone()) })
                                ]
                            ),
                            // Badges
                            (
                                UNode::default()
                                ULayout {
                                    display: UDisplay::Flex,
                                    flex_direction: UFlexDirection::Row,
                                    align_items: UAlignItems::Center,
                                    gap: 6.0,
                                }
                                Children [
                                    ({ Badge::new("INFO").variant(BadgeVariant::Info).font(font.clone()) }),
                                    ({ Badge::new("SUCCESS").variant(BadgeVariant::Success).font(font.clone()) }),
                                    ({ Badge::new("WARNING").variant(BadgeVariant::Warning).font(font.clone()) }),
                                    ({ Badge::new("DANGER").variant(BadgeVariant::Danger).font(font.clone()) }),
                                    ({ Badge::new("VERIFIED").icon(Icon::CHECK).variant(BadgeVariant::Success).font(font.clone()).icon_font(icon_font.clone()) })
                                ]
                            )
                        ]
                    ),

                    // Section 4: Removable Chips & Color Swatches Row
                    (
                        UNode::default()
                        ULayout {
                            display: UDisplay::Flex,
                            flex_direction: UFlexDirection::Row,
                            align_items: UAlignItems::Center,
                            justify_content: UJustifyContent::SpaceBetween,
                        }
                        Children [
                            // Chips
                            (
                                UNode::default()
                                ULayout {
                                    display: UDisplay::Flex,
                                    flex_direction: UFlexDirection::Row,
                                    align_items: UAlignItems::Center,
                                    gap: 8.0,
                                }
                                Children [
                                    (luma_chip("Rust 2024", font.clone())),
                                    (luma_chip_with_icon(Icon::CPU, "Multi-Threaded", font.clone(), icon_font.clone())),
                                    (luma_removable_chip("Bevy 0.19", font.clone(), icon_font.clone())),
                                    (luma_removable_chip_with_icon(Icon::CHECK, "SDF Shaders", font.clone(), icon_font.clone()))
                                ]
                            ),
                            // Color Swatches
                            (
                                UNode::default()
                                ULayout {
                                    display: UDisplay::Flex,
                                    flex_direction: UFlexDirection::Row,
                                    align_items: UAlignItems::Center,
                                    gap: 8.0,
                                }
                                Children [
                                    (luma_swatch(Color::srgb(0.20, 0.45, 0.90), true)),
                                    (luma_swatch(Color::srgb(0.16, 0.75, 0.45), false)),
                                    (luma_swatch(Color::srgb(0.95, 0.65, 0.15), false)),
                                    (luma_swatch(Color::srgb(0.85, 0.22, 0.28), false)),
                                    (luma_swatch(Color::srgb(0.65, 0.35, 0.95), false))
                                ]
                            )
                        ]
                    ),

                    // Section 5: Form Controls (Toggles, Checkboxes, Radios)
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
                                    gap: 8.0,
                                }
                                Children [
                                    ({ Toggle::new(true).label("Enable Bloom").font(font.clone()) }),
                                    ({ Toggle::new(false).label("Hardware SDF").font(font.clone()) })
                                ]
                            ),

                            // Checkboxes column
                            (
                                UNode::default()
                                ULayout {
                                    display: UDisplay::Flex,
                                    flex_direction: UFlexDirection::Column,
                                    gap: 8.0,
                                }
                                Children [
                                    ({ Checkbox::new("Spatial Audio", true).font(font.clone()).icon_font(icon_font.clone()) }),
                                    ({ Checkbox::new("V-Sync Lock", false).font(font.clone()).icon_font(icon_font.clone()) })
                                ]
                            ),

                            // Radio Buttons column
                            (
                                UNode::default()
                                ULayout {
                                    display: UDisplay::Flex,
                                    flex_direction: UFlexDirection::Column,
                                    gap: 8.0,
                                }
                                Children [
                                    ({ Radio::new("quality", "low").label("Low").selected(false).font(font.clone()) }),
                                    ({ Radio::new("quality", "med").label("Medium").selected(true).font(font.clone()) }),
                                    ({ Radio::new("quality", "high").label("High").selected(false).font(font.clone()) })
                                ]
                            )
                        ]
                    ),

                    // Section 6: Sliders, Progress Bars, and Avatars Row
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
                                    gap: 10.0,
                                }
                                Children [
                                    (
                                        UNode::default()
                                        ULayout {
                                            display: UDisplay::Flex,
                                            flex_direction: UFlexDirection::Row,
                                            align_items: UAlignItems::Center,
                                            gap: 14.0,
                                        }
                                        Children [
                                            (UText {
                                                text: { "Volume:".to_string() },
                                                font_size: 13.0,
                                                font: { font.clone() },
                                                color: Color::srgb(0.70, 0.75, 0.85),
                                            }),
                                            ({ Slider::new(70.0, 0.0, 100.0, 240.0) })
                                        ]
                                    ),
                                    (
                                        UNode::default()
                                        ULayout {
                                            display: UDisplay::Flex,
                                            flex_direction: UFlexDirection::Row,
                                            align_items: UAlignItems::Center,
                                            gap: 14.0,
                                        }
                                        Children [
                                            (UText {
                                                text: { "Download:".to_string() },
                                                font_size: 13.0,
                                                font: { font.clone() },
                                                color: Color::srgb(0.70, 0.75, 0.85),
                                            }),
                                            ({ ProgressBar::new(0.65).width(UVal::Px(240.0)).variant(ProgressVariant::Success) })
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
                                    gap: 12.0,
                                }
                                Children [
                                    ({ Avatar::new("AH").status(AvatarStatus::Online).font(font.clone()) }),
                                    ({ Avatar::new("JD").status(AvatarStatus::Busy).font(font.clone()) }),
                                    ({ Avatar::new("AI").font(font.clone()) }),
                                    ({ Tooltip::new("Online & Active").font(font.clone()) })
                                ]
                            )
                        ]
                    ),

                    // Section 7: Toast Notification Banner
                    ({
                        Toast::new(
                            "Declarative Struct Architecture & Theming",
                            "All 24 widgets are fully configurable structs with Default, builder chaining, and Android-style inheritance.",
                        )
                        .variant(ToastVariant::Success)
                        .font(font.clone())
                        .icon_font(icon_font.clone())
                    }),

                    // Section 8: Android-Style Theme Scope Demonstration
                    ({
                        luma_style_scope(
                            LumaStyleScope {
                                button: Some(ButtonStyle {
                                    bg_normal: Color::srgb(0.22, 0.14, 0.38),
                                    bg_hover: Color::srgb(0.32, 0.20, 0.52),
                                    bg_pressed: Color::srgb(0.18, 0.10, 0.30),
                                    text_color: Color::srgb(0.95, 0.90, 1.0),
                                    border_color: Color::srgb(0.45, 0.28, 0.75),
                                    radius: 20.0,
                                    ..default()
                                }),
                                ..default()
                            },
                            bsn! {
                                UNode::default()
                                ULayout {
                                    display: UDisplay::Flex,
                                    flex_direction: UFlexDirection::Row,
                                    align_items: UAlignItems::Center,
                                    justify_content: UJustifyContent::SpaceBetween,
                                }
                                Children [
                                    (UText {
                                        text: { "Scoped Style Inheritance:".to_string() },
                                        font_size: 13.0,
                                        font: { font.clone() },
                                        color: Color::srgb(0.75, 0.70, 0.85),
                                    }),
                                    (
                                        UNode::default()
                                        ULayout {
                                            display: UDisplay::Flex,
                                            flex_direction: UFlexDirection::Row,
                                            gap: 10.0,
                                        }
                                        Children [
                                            ({ Button::new("Inherited Pill A").font(font.clone()) }),
                                            ({ Button::new("Inherited Pill B").font(font.clone()) }),
                                        ]
                                    )
                                ]
                            }
                        )
                    })
                ]
            )
        ]
    }
}
