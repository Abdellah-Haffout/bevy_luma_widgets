//! Test BSN nested syntax

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

    // Direct bsn! with nested tuples
    let scene = bsn! {
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
            (
                UNode::default()
                ULayout {
                    display: UDisplay::Flex,
                    flex_direction: UFlexDirection::Column,
                    gap: 16.0,
                }
                Children [
                    (UText {
                        text: { "Hello".to_string() },
                        font_size: 20.0,
                        font: { font.clone() },
                        color: Color::WHITE,
                    }),
                    (
                        UNode::default()
                        ULayout {
                            display: UDisplay::Flex,
                            flex_direction: UFlexDirection::Row,
                            gap: 12.0,
                        }
                        Children [
                            ({ Button::new("Btn1").variant(ButtonVariant::Primary).font(font.clone()) }),
                            ({ Button::new("Btn2").variant(ButtonVariant::Secondary).font(font.clone()) }),
                        ]
                    ),
                ]
            ),
        ]
    };

    commands.spawn_scene(scene);
}