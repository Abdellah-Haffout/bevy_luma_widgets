//! Demo of the `luma_ui!` macro for declarative UI construction.
//!
//! This example demonstrates:
//! - Using `luma_ui!` for root scene and widget composition
//! - Using `luma_col!`, `luma_row!` for layout composition (outside luma_ui!)

use bevy::prelude::*;
use bevy_luma_widgets::prelude::*;
use bevy_luma_widgets::{luma_row, luma_col};

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_plugins(LumaWidgetsPlugin)
        .add_systems(Startup, setup_ui)
        .run();
}

fn setup_ui(mut commands: Commands, theme: Res<Theme>) {
    let font = theme.text.font.inter_regular.clone();
    let icon_font = theme.icon.font.clone();

    commands.spawn(Camera2d);

    // Build individual widget scenes using luma_ui!
    let toggle_vsync = luma_ui! { Toggle { label: "Enable V-Sync", default: true, font: font.clone() } };
    let toggle_fullscreen = luma_ui! { Toggle { label: "Fullscreen Mode", default: false, font: font.clone() } };
    
    let slider_volume = luma_ui! {
        Slider {
            range: (0.0, 1.0),
            default: 0.7,
            step: 0.05,
            width: 280.0,
            font: font.clone(),
        }
    };
    
    let slider_fps = luma_ui! {
        Slider {
            range: (1, 144),
            default: 60,
            step: 1.0,
            width: 280.0,
            font: font.clone(),
        }
    };
    
    let divider_quality = luma_ui! { Divider { label: "Graphics Quality", font: font.clone() } };
    
    let segmented_quality = luma_ui! {
        SegmentedControl {
            group: "quality",
            active: "high",
            options: [
                SegmentOption { group: "quality", value: "low", label: "Low", font: font.clone() },
                SegmentOption { group: "quality", value: "medium", label: "Medium", font: font.clone() },
                SegmentOption { group: "quality", value: "high", label: "High", font: font.clone() },
                SegmentOption { group: "quality", value: "ultra", label: "Ultra", font: font.clone() },
            ],
            font: font.clone(),
        }
    };
    
    let btn_cancel = luma_ui! { Button { label: "Cancel", variant: ButtonVariant::Ghost, size: ButtonSize::Medium, font: font.clone() } };
    let btn_apply = luma_ui! { Button { label: "Apply", variant: ButtonVariant::Primary, size: ButtonSize::Medium, font: font.clone() } };
    let btn_save = luma_ui! {
        Button {
            label: "Save & Close",
            variant: ButtonVariant::Primary,
            size: ButtonSize::Large,
            icon: "SAVE".to_string(),
            font: font.clone(),
            icon_font: icon_font.clone(),
        }
    };

    // Build the action buttons row
    let action_buttons = luma_row!(
        gap: 12.0,
        justify: UJustifyContent::End,
        children: [
            ({ btn_cancel }),
            ({ btn_apply }),
            ({ btn_save }),
        ]
    );

    // Build the form content column
    let form_content = luma_col!(
        gap: 16.0,
        children: [
            ({ toggle_vsync }),
            ({ toggle_fullscreen }),
            ({ slider_volume }),
            ({ slider_fps }),
            ({ divider_quality }),
            ({ segmented_quality }),
            (action_buttons),
        ]
    );

    // Build the header row
    let header = luma_row!(
        gap: 12.0,
        align: UAlignItems::Center,
        children: [
            (UText {
                text: { "Settings Panel".to_string() },
                font_size: 20.0,
                font: { font.clone() },
                color: Color::WHITE,
            }),
        ]
    );

    // Use luma_ui! for the root scene, embedding the pre-built components
    let ui_scene = luma_ui! {
        URootUi::screen()
        UNode {
            width: UVal::Percent(1.0),
            height: UVal::Percent(1.0),
            background_color: Color::srgb(0.08, 0.09, 0.12),
        }
        ULayout {
            display: UDisplay::Flex,
            flex_direction: UFlexDirection::Column,
            justify_content: UJustifyContent::Center,
            align_items: UAlignItems::Center,
            gap: 24.0,
        }
        Children [
            (header),
            (form_content),
        ]
    };

    commands.spawn_scene(ui_scene);
}