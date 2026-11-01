//! Controls & Form Inputs showcase example in bevy_luma_widgets.

use bevy::prelude::*;
use bevy_luma_widgets::prelude::*;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins.set(WindowPlugin {
            primary_window: Some(Window {
                title: "Luma Widgets — Controls Suite".into(),
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
                    text: { "Controls & Form Inputs Suite".to_string() },
                    font_size: 26.0,
                    font: { font.clone() },
                    color: Color::WHITE,
                }
                --
                UText {
                    text: { "Specialized input, control, and picker widgets complementing core primitives".to_string() },
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
                // Left Column
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
                        text: { "1. Button Suite, FAB & Voice Input".to_string() },
                        font_size: 15.0,
                        font: { font.clone() },
                        color: Color::srgb(0.40, 0.75, 1.0),
                    }
                    --
                    UNode::default()
                    ULayout {
                        display: UDisplay::Flex,
                        flex_direction: UFlexDirection::Row,
                        align_items: UAlignItems::Center,
                        gap: 8.0,
                    }
                    Children [
                        @{ Button::new("Primary").variant(ButtonVariant::Primary).small().font(font.clone()) }
                        --
                        @{ Button::new("Secondary").variant(ButtonVariant::Secondary).small().font(font.clone()) }
                        --
                        @{ Button::new("Outline").variant(ButtonVariant::Outline).small().font(font.clone()) }
                        --
                        @{ IconButton::new(Icon::STAR).font(font.clone()).icon_font(icon_font.clone()) }
                        --
                        @{ Fab::new(Icon::PLUS).font(font.clone()).icon_font(icon_font.clone()) }
                        --
                        @{ VoiceInputButton::new() }
                    ]
                    --
                    UText {
                        text: { "2. TextInput, SearchBar, Password & Credentials".to_string() },
                        font_size: 15.0,
                        font: { font.clone() },
                        color: Color::srgb(0.40, 0.75, 1.0),
                    }
                    --
                    @{ SearchBar::new("Search components, documentation, assets...").font(font.clone()).icon_font(icon_font.clone()) }
                    --
                    @{ PasswordInput::new().value("LumaSecretPass2026!").strength(true, 0.85).font(font.clone()).icon_font(icon_font.clone()) }
                    --
                    @{ TextArea::new("Multiline design notes and shader compiler parameters...").font(font.clone()) }
                ]
                --
                // Right Column
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
                        text: { "3. Pickers & Color Selection".to_string() },
                        font_size: 15.0,
                        font: { font.clone() },
                        color: Color::srgb(0.40, 0.75, 1.0),
                    }
                    --
                    UNode::default()
                    ULayout {
                        display: UDisplay::Flex,
                        flex_direction: UFlexDirection::Row,
                        justify_content: UJustifyContent::SpaceBetween,
                        align_items: UAlignItems::Center,
                        gap: 8.0,
                    }
                    Children [
                        @{ DatePicker { date: "2026-10-28".into(), font: font.clone(), icon_font: icon_font.clone() } }
                        --
                        @{ TimePicker { time: "14:30 UTC".into(), font: font.clone(), icon_font: icon_font.clone() } }
                        --
                        @{ ColorPicker::new(Color::srgb(0.20, 0.45, 0.90), "#3373E6") }
                    ]
                    --
                    UText {
                        text: { "4. File Upload Dropzone".to_string() },
                        font_size: 15.0,
                        font: { font.clone() },
                        color: Color::srgb(0.40, 0.75, 1.0),
                    }
                    --
                    @{ FileUploader { message: "Drop project shaders or click to browse assets".into(), progress: Some(0.72), font: font.clone(), icon_font: icon_font.clone() } }
                    --
                    UText {
                        text: { "5. Toolbars & Reactions".to_string() },
                        font_size: 15.0,
                        font: { font.clone() },
                        color: Color::srgb(0.40, 0.75, 1.0),
                    }
                    --
                    UNode::default()
                    ULayout {
                        display: UDisplay::Flex,
                        flex_direction: UFlexDirection::Row,
                        justify_content: UJustifyContent::SpaceBetween,
                        align_items: UAlignItems::Center,
                    }
                    Children [
                        @{ ReactionPicker::new().icon_font(icon_font.clone()) }
                        --
                        @{ RichTextToolbar::new().icon_font(icon_font) }
                    ]
                ]
            ]
        ]
    });
}
