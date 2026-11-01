//! Blender-style DragValue numeric scrubber widget showcase example in bevy_luma_widgets.

use bevy::prelude::*;
use bevy_luma_widgets::prelude::*;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins.set(WindowPlugin {
            primary_window: Some(Window {
                title: "Luma Widgets — Blender-Style DragValue Scrubber".into(),
                resolution: (980, 840).into(),
                ..default()
            }),
            ..default()
        }))
        .add_plugins(LumaWidgetsPlugin)
        .add_systems(Startup, setup)
        .add_observer(|trigger: On<DragValueChanged>| {
            info!("DragValueChanged: entity={:?}, value={:.3}", trigger.event().entity, trigger.event().value);
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
            background_color: Color::srgb(0.05, 0.06, 0.08),
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
                    text: { "Blender-Style DragValue Numeric Controls".to_string() },
                    font_size: 28.0,
                    font: { font.clone() },
                    color: Color::WHITE,
                }
                --
                UText {
                    text: { "Interactive number sliders with real-time fill progress, X/Y/Z coordinate accents, and scrub dragging".to_string() },
                    font_size: 14.0,
                    font: { font.clone() },
                    color: Color::srgb(0.60, 0.65, 0.75),
                }
            ]
            --
            // Main Showcase Container
            UNode {
                width: UVal::Px(780.0),
                background_color: Color::srgb(0.09, 0.11, 0.15),
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
                // Section 1: Blender 3D Transform Coordinates (RGB Axis Tags)
                UText {
                    text: { "1. Blender 3D Transform Channels (Location & Rotation)".to_string() },
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
                    gap: 14.0,
                }
                Children [
                    UText {
                        text: { "Location:".to_string() },
                        font_size: 14.0,
                        font: { font.clone() },
                        color: Color::srgb(0.70, 0.74, 0.82),
                    }
                    --
                    @{ DragValue::blender_x(2.50).suffix(" m").width(UVal::Px(180.0)).font(font.clone()).icon_font(icon_font.clone()) }
                    --
                    @{ DragValue::blender_y(0.00).suffix(" m").width(UVal::Px(180.0)).font(font.clone()).icon_font(icon_font.clone()) }
                    --
                    @{ DragValue::blender_z(-1.25).suffix(" m").width(UVal::Px(180.0)).font(font.clone()).icon_font(icon_font.clone()) }
                ]
                --
                UNode::default()
                ULayout {
                    display: UDisplay::Flex,
                    flex_direction: UFlexDirection::Row,
                    align_items: UAlignItems::Center,
                    gap: 14.0,
                }
                Children [
                    UText {
                        text: { "Rotation:".to_string() },
                        font_size: 14.0,
                        font: { font.clone() },
                        color: Color::srgb(0.70, 0.74, 0.82),
                    }
                    --
                    @{ DragValue::blender_x(0.0).range(-180.0, 180.0).suffix("°").width(UVal::Px(180.0)).font(font.clone()).icon_font(icon_font.clone()) }
                    --
                    @{ DragValue::blender_y(45.0).range(-180.0, 180.0).suffix("°").width(UVal::Px(180.0)).font(font.clone()).icon_font(icon_font.clone()) }
                    --
                    @{ DragValue::blender_z(90.0).range(-180.0, 180.0).suffix("°").width(UVal::Px(180.0)).font(font.clone()).icon_font(icon_font.clone()) }
                ]
                --
                // Section 2: Material & Camera Properties with Real-Time Slider Fill
                UText {
                    text: { "2. Shader & Camera Properties (with Real-Time Fill Sliders)".to_string() },
                    font_size: 16.0,
                    font: { font.clone() },
                    color: Color::srgb(0.40, 0.75, 1.0),
                }
                --
                UNode::default()
                ULayout {
                    display: UDisplay::Flex,
                    flex_direction: UFlexDirection::Column,
                    gap: 10.0,
                }
                Children [
                    @{ DragValue::blender_slider("Roughness", 0.350, 0.0, 1.0).decimals(3).width(UVal::Px(420.0)).font(font.clone()).icon_font(icon_font.clone()) }
                    --
                    @{ DragValue::blender_slider("Metallic", 0.800, 0.0, 1.0).decimals(3).width(UVal::Px(420.0)).font(font.clone()).icon_font(icon_font.clone()) }
                    --
                    @{ DragValue::blender_slider("Index of Refr. (IOR)", 1.450, 1.0, 3.0).decimals(3).width(UVal::Px(420.0)).font(font.clone()).icon_font(icon_font.clone()) }
                    --
                    @{ DragValue::blender_slider("Focal Length", 50.0, 12.0, 200.0).suffix(" mm").decimals(1).width(UVal::Px(420.0)).font(font.clone()).icon_font(icon_font.clone()) }
                ]
                --
                // Section 3: Percentage & Custom Accent Colors
                UText {
                    text: { "3. Percentage Clamped Ranges & Disabled States".to_string() },
                    font_size: 16.0,
                    font: { font.clone() },
                    color: Color::srgb(0.40, 0.75, 1.0),
                }
                --
                UNode::default()
                ULayout {
                    display: UDisplay::Flex,
                    flex_direction: UFlexDirection::Row,
                    gap: 16.0,
                }
                Children [
                    @{ DragValue::blender_slider("Emission", 75.0, 0.0, 100.0).suffix("%").decimals(0).width(UVal::Px(240.0)).font(font.clone()).icon_font(icon_font.clone()) }
                    --
                    @{ DragValue::blender_slider("Subsurface", 25.0, 0.0, 100.0).suffix("%").decimals(0).disabled(true).width(UVal::Px(240.0)).font(font.clone()).icon_font(icon_font.clone()) }
                ]
            ]
        ]
    });
}
