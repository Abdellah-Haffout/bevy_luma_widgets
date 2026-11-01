//! Toast notification widget showcase example in bevy_luma_widgets.

use bevy::prelude::*;
use bevy_luma_widgets::prelude::*;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins.set(WindowPlugin {
            primary_window: Some(Window {
                title: "Luma Widgets — Toast Notification Example".into(),
                resolution: (960, 720).into(),
                ..default()
            }),
            ..default()
        }))
        .add_plugins(LumaWidgetsPlugin)
        .add_systems(Startup, setup)
        .add_observer(|trigger: On<ToastDismissed>| {
            info!("ToastDismissed: entity={:?}", trigger.event().0);
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
                    text: { "Toast Notification Component".to_string() },
                    font_size: 28.0,
                    font: { font.clone() },
                    color: Color::WHITE,
                }
                --
                UText {
                    text: { "Floating alert cards with close buttons and auto-dismiss timers".to_string() },
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
                @{ Toast::success("Changes Saved", "Your profile preferences have been updated.").font(font.clone()).icon_font(icon_font.clone()) }
                --
                @{ Toast::info("New Message", "Alex sent you an invitation to collaborate.").font(font.clone()).icon_font(icon_font.clone()) }
                --
                @{ Toast::warning("Syncing Paused", "Cloud synchronization was paused due to metering.").font(font.clone()).icon_font(icon_font.clone()) }
                --
                @{ Toast::danger("Upload Failed", "File exceeds maximum upload size of 50 MB.").timed(8.0).font(font.clone()).icon_font(icon_font.clone()) }
            ]
        ]
    });
}
