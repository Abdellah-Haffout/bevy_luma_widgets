//! Styled text input field wrapper.

use bevy::prelude::*;
use bevy_luma::prelude::*;

/// Marker component identifying a high-level Luma text input.
#[derive(Component, Clone, Debug, Default, Reflect)]
pub struct LumaInput;

/// Spawns a styled interactive text input field.
pub fn luma_input(commands: &mut Commands, placeholder: impl Into<String>) -> Entity {
    let input = UTextInput {
        placeholder: placeholder.into(),
        placeholder_color: Color::srgb(0.45, 0.50, 0.62),
        text_color: Color::srgb(0.95, 0.96, 0.98),
        font_size: 14.0,
        ..default()
    };

    let entity = spawn_text_input(commands, input);
    commands.entity(entity).insert(LumaInput);
    entity
}

/// Spawns a password text input field with masked character display.
pub fn luma_password_input(commands: &mut Commands, placeholder: impl Into<String>) -> Entity {
    let input = UTextInput {
        placeholder: placeholder.into(),
        placeholder_color: Color::srgb(0.45, 0.50, 0.62),
        text_color: Color::srgb(0.95, 0.96, 0.98),
        font_size: 14.0,
        mode: TextInputMode::Password,
        ..default()
    };

    let entity = spawn_text_input(commands, input);
    commands.entity(entity).insert(LumaInput);
    entity
}
