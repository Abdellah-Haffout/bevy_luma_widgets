//! Simple test for Col macro

use bevy::prelude::*;
use bevy_luma_widgets::prelude::*;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_plugins(LumaWidgetsPlugin)
        .add_systems(Startup, setup_ui)
        .run();
}

fn setup_ui(mut commands: Commands, theme: Res<Theme>) {
    let font = theme.text.font.inter_regular.clone();

    commands.spawn(Camera2d);

    let scene = luma_ui! {
        URootUi::screen()
        UNode {
            width: UVal::Percent(1.0),
            height: UVal::Percent(1.0),
            background_color: Color::srgb(0.08, 0.09, 0.12),
        }
        ULayout {
            display: UDisplay::Flex,
            justify_content: UJustifyContent::Center,
            align_items: UAlignItems::Center,
        }
        Children [
            Col {
                gap: 16.0,
                children: [
                    ({ luma_ui! { Toggle { label: "Test", default: true, font: font.clone() } }}),
                ]
            },
        ]
    };

    commands.spawn_scene(scene);
}