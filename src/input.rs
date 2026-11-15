//! Styled text input field widget with validation states, icons, and presets.

use bevy::prelude::*;
use bevy::scene::{ResolveContext, ResolvedScene, ResolveSceneError};
use bevy_luma::prelude::*;

use crate::helpers::{
    find_ancestor_or_self, interaction_colors, is_primary_click, visit_descendants,
    ConditionalScene, DescendantVisit,
};
use crate::theme::{InputStyle, WidgetColors};

/// Visual validation or focus state for text input fields.
#[derive(Clone, Debug, PartialEq, Eq, Default, Reflect)]
pub enum InputState {
    #[default]
    Normal,
    Focused,
    Error(String),
    Success,
    Disabled,
}

/// Marker component identifying a high-level Luma text input.
#[derive(Component, Clone, Debug, Default, Reflect)]
pub struct LumaInput {
    pub state: InputState,
    pub style: InputStyle,
    pub has_explicit_style: bool,
    pub has_explicit_font_size: bool,
}

/// Marker component for the password visibility eye toggle button.
#[derive(Component, Clone, Debug, Default, Reflect)]
pub struct LumaPasswordToggle;

/// Marker component for the trailing clear (`X`) button on search/clearable inputs.
#[derive(Component, Clone, Debug, Default, Reflect)]
pub struct LumaInputClear;

/// Marker component for the main display text inside a Luma text input.
#[derive(Component, Clone, Debug, Default, Reflect)]
pub struct LumaInputText;

/// Marker component for leading/trailing icons inside a Luma text input.
#[derive(Component, Clone, Debug, Default, Reflect)]
pub struct LumaInputIcon;

/// Declarative TextInput configuration and scene widget struct.
#[derive(Clone, Debug, Reflect)]
pub struct TextInput {
    pub value: String,
    pub placeholder: String,
    pub state: InputState,
    pub leading_icon: Option<String>,
    pub trailing_icon: Option<String>,
    pub font: Handle<Font>,
    pub icon_font: Handle<Font>,
    pub font_size: f32,
    pub mode: TextInputMode,
    pub max_length: Option<usize>,
    pub style: Option<InputStyle>,
}

impl Default for TextInput {
    fn default() -> Self {
        Self {
            value: String::new(),
            placeholder: String::new(),
            state: InputState::Normal,
            leading_icon: None,
            trailing_icon: None,
            font: Handle::default(),
            icon_font: Handle::default(),
            font_size: 14.0,
            mode: TextInputMode::Normal,
            max_length: None,
            style: None,
        }
    }
}

impl TextInput {
    pub fn new(placeholder: impl Into<String>) -> Self {
        Self {
            placeholder: placeholder.into(),
            ..default()
        }
    }

    pub fn value(mut self, value: impl Into<String>) -> Self {
        self.value = value.into();
        self
    }

    pub fn state(mut self, state: InputState) -> Self {
        self.state = state;
        self
    }

    pub fn leading_icon(mut self, icon: impl Into<String>) -> Self {
        self.leading_icon = Some(icon.into());
        self
    }

    pub fn trailing_icon(mut self, icon: impl Into<String>) -> Self {
        self.trailing_icon = Some(icon.into());
        self
    }

    pub fn font(mut self, font: Handle<Font>) -> Self {
        self.font = font;
        self
    }

    pub fn icon_font(mut self, icon_font: Handle<Font>) -> Self {
        self.icon_font = icon_font;
        self
    }

    pub fn font_size(mut self, font_size: f32) -> Self {
        self.font_size = font_size;
        self
    }

    pub fn mode(mut self, mode: TextInputMode) -> Self {
        self.mode = mode;
        self
    }

    pub fn max_length(mut self, max_length: usize) -> Self {
        self.max_length = Some(max_length);
        self
    }

    pub fn style(mut self, style: InputStyle) -> Self {
        self.style = Some(style);
        self
    }

    /// Preset constructor for a Password input field.
    pub fn password(placeholder: impl Into<String>) -> Self {
        Self {
            placeholder: placeholder.into(),
            mode: TextInputMode::Password,
            trailing_icon: Some(Icon::EYE.to_string()),
            ..default()
        }
    }

    /// Preset constructor for a Search Bar input field.
    pub fn search(placeholder: impl Into<String>) -> Self {
        Self {
            placeholder: placeholder.into(),
            leading_icon: Some(Icon::SEARCH.to_string()),
            style: Some(InputStyle::search()),
            ..default()
        }
    }

    /// Preset constructor for an OTP / PIN single-character or segmented digit field.
    pub fn otp() -> Self {
        Self {
            style: Some(InputStyle::otp()),
            ..default()
        }
    }

    /// Preset constructor for a multi-line Text Area field.
    pub fn multiline(placeholder: impl Into<String>) -> Self {
        Self {
            placeholder: placeholder.into(),
            style: Some(InputStyle::multiline()),
            ..default()
        }
    }

    /// Legacy manual spawn helper.
    pub fn spawn(self, commands: &mut Commands) -> Entity {
        let is_disabled = self.state == InputState::Disabled;
        let has_explicit_style = self.style.is_some();
        let has_explicit_font_size = (self.font_size - 14.0).abs() > f32::EPSILON;
        let style = self.style.unwrap_or_default();
        let font_size = if has_explicit_font_size {
            self.font_size
        } else {
            style.font_size
        };
        let input = UTextInput {
            value: self.value,
            placeholder: self.placeholder,
            font_size,
            enabled: !is_disabled,
            max_length: self.max_length,
            mode: self.mode,
            ..default()
        };
        let entity = spawn_text_input(commands, input);
        if let Ok(mut cmd) = commands.get_entity(entity) {
            cmd.insert(LumaInput {
                state: self.state,
                style,
                has_explicit_style,
                has_explicit_font_size,
            });
        }
        entity
    }
}

impl Scene for TextInput {
    fn resolve(
        self,
        context: &mut ResolveContext,
        scene: &mut ResolvedScene,
    ) -> Result<(), ResolveSceneError> {
        let is_disabled = self.state == InputState::Disabled;
        let has_explicit_style = self.style.is_some();
        let has_explicit_font_size = (self.font_size - 14.0).abs() > f32::EPSILON;
        let style = self.style.unwrap_or_default();
        let colors = WidgetColors::default();

        let bg_color = match self.state {
            InputState::Disabled => colors.disabled_bg(),
            InputState::Focused => style.bg_focus,
            _ => style.bg_normal,
        };

        let border_color = match &self.state {
            InputState::Normal => style.border_normal,
            InputState::Focused => style.border_focus,
            InputState::Error(_) => style.border_error,
            InputState::Success => style.border_success,
            InputState::Disabled => colors.disabled_border(),
        };

        let has_leading = self.leading_icon.is_some();
        let leading_text = self.leading_icon.unwrap_or_default();

        let has_trailing = self.trailing_icon.is_some();
        let trailing_text = self.trailing_icon.unwrap_or_default();

        let display_value = if self.value.is_empty() {
            self.placeholder.clone()
        } else if self.mode == TextInputMode::Password {
            "•".repeat(self.value.chars().count())
        } else {
            self.value.clone()
        };

        let input_text_color = if is_disabled {
            colors.disabled_text()
        } else {
            style.text_color
        };
        let placeholder_color = if is_disabled {
            colors.disabled_text()
        } else {
            style.placeholder_color
        };
        let display_color = if is_disabled {
            colors.disabled_text()
        } else if self.value.is_empty() {
            style.placeholder_color
        } else {
            style.text_color
        };

        let value = self.value;
        let placeholder = self.placeholder;
        let font_size = if has_explicit_font_size {
            self.font_size
        } else {
            style.font_size
        };
        let mode = self.mode;
        let max_length = self.max_length;
        let min_height = style.min_height;
        let radius = style.radius;
        let padding = style.padding;
        let border_width = style.border_width;
        let border_focus = style.border_focus;
        let font = self.font;
        let icon_font = self.icon_font;
        let is_password_mode = mode == TextInputMode::Password;
        let is_clear_icon = !is_password_mode && trailing_text == Icon::X;
        let has_trailing_action = is_password_mode || is_clear_icon;
        let caret_color = border_focus;
        let selection_color = Color::srgba(0.35, 0.65, 1.0, 0.28);
        let line_h = font_size * 1.25;
        let toggle_colors = interaction_colors(
            Color::NONE,
            Color::srgba(1.0, 1.0, 1.0, 0.08),
            Color::srgba(1.0, 1.0, 1.0, 0.16),
            is_disabled || !has_trailing_action,
        );
        let clear_colors = toggle_colors.clone();
        let luma_input = LumaInput {
            state: self.state,
            style,
            has_explicit_style,
            has_explicit_font_size,
        };
        let pw_icon_text = trailing_text.clone();
        let pw_icon_font = icon_font.clone();
        let clear_icon_text = trailing_text.clone();
        let clear_icon_font = icon_font.clone();

        let trailing_password = ConditionalScene(is_password_mode.then(move || {
            bsn! {
                UNode {
                    padding: USides::all(4.0),
                    border_radius: UCornerRadius::all(4.0),
                    background_color: Color::NONE,
                }
                LumaPasswordToggle
                UInteraction::default()
                toggle_colors
                UFocusable::default()
                Children [
                    LumaInputIcon
                    UText {
                        text: pw_icon_text,
                        font_size: 16.0,
                        font: pw_icon_font,
                        color: placeholder_color,
                    }
                ]
            }
        }));
        let trailing_clear = ConditionalScene(is_clear_icon.then(move || {
            bsn! {
                UNode {
                    padding: USides::all(4.0),
                    border_radius: UCornerRadius::all(4.0),
                    background_color: Color::NONE,
                }
                LumaInputClear
                UInteraction::default()
                clear_colors
                UFocusable::default()
                Children [
                    LumaInputIcon
                    UText {
                        text: clear_icon_text,
                        font_size: 16.0,
                        font: clear_icon_font,
                        color: placeholder_color,
                    }
                ]
            }
        }));
        let trailing_static_font = icon_font.clone();
        let trailing_static = ConditionalScene((has_trailing && !has_trailing_action).then(move || {
            bsn! {
                UNode {
                    padding: USides::all(4.0),
                }
                Children [
                    LumaInputIcon
                    UText {
                        text: trailing_text,
                        font_size: 16.0,
                        font: trailing_static_font,
                        color: placeholder_color,
                    }
                ]
            }
        }));

        let s = bsn! {
            luma_input
            UTextInput {
                value,
                placeholder,
                placeholder_color,
                text_color: input_text_color,
                font_size,
                enabled: { !is_disabled },
                max_length,
                mode,
            }
            UTextInputCaret {
                color: caret_color,
                width: 1.5,
                selection_color,
            }
            UNode {
                width: UVal::Percent(1.0),
                min_height,
                background_color: bg_color,
                border_radius: UCornerRadius::all(radius),
                padding,
            }
            UBorder {
                color: border_color,
                width: border_width,
                radius: UCornerRadius::all(radius),
            }
            UFocusable {
                enabled: { !is_disabled },
                ..UFocusable::default()
            }
            UFocusVisual::border(border_focus, 2.0)
            UInteraction::default()
            ULayout {
                display: UDisplay::Flex,
                flex_direction: UFlexDirection::Row,
                align_items: UAlignItems::Center,
                gap: 8.0,
            }
            Children [
                UNode::default()
                ULayout {
                    display: { if has_leading { UDisplay::Flex } else { UDisplay::None } },
                }
                Children [
                    LumaInputIcon
                    UText {
                        text: leading_text,
                        font_size: 16.0,
                        font: { icon_font.clone() },
                        color: placeholder_color,
                    }
                ]
                --
                UNode {
                    height: UVal::Px(line_h),
                }
                USelf {
                    flex_grow: { Some(1.0) },
                }
                ULayout {
                    display: UDisplay::Flex,
                    align_items: UAlignItems::Center,
                }
                Children [
                    UNode {
                        width: UVal::Percent(1.0),
                        height: UVal::Percent(1.0),
                    }
                    USelf {
                        position_type: UPositionType::Absolute,
                        left: UVal::Px(-14.0),
                        top: UVal::Px(0.0),
                    }
                    Children [
                        UTextInputSelectionMarker
                        UNode {
                            width: UVal::Px(0.0),
                            height: UVal::Px(line_h),
                            background_color: selection_color,
                            border_radius: UCornerRadius::all(2.0),
                        }
                        USelf {
                            position_type: UPositionType::Absolute,
                            left: UVal::Px(14.0),
                            top: UVal::Px(0.0),
                        }
                        Visibility::Hidden
                        --
                        UTextInputCaretMarker
                        UNode {
                            width: UVal::Px(1.5),
                            height: UVal::Px(line_h),
                            background_color: caret_color,
                        }
                        USelf {
                            position_type: UPositionType::Absolute,
                            left: UVal::Px(14.0),
                            top: UVal::Px(0.0),
                        }
                        Visibility::Hidden
                    ]
                    --
                    LumaInputText
                    UTextInputTextMarker
                    UText {
                        text: display_value,
                        font_size,
                        font,
                        color: display_color,
                        linebreak: LineBreak::NoWrap,
                        autosize: true,
                        overflow: UTextOverflow::Clip,
                    }
                ]
                --
                @{trailing_password}
                --
                @{trailing_clear}
                --
                @{trailing_static}
            ]
        };

        s.resolve(context, scene)
    }
}

/// Spawns a styled interactive text input field scene.
pub fn luma_input(placeholder: impl Into<String>) -> TextInput {
    TextInput::new(placeholder)
}

/// Spawns a password text input field with masked character display scene.
pub fn luma_password_input(placeholder: impl Into<String>) -> TextInput {
    TextInput::password(placeholder)
}

fn toggle_password_visibility(
    target: Entity,
    toggle_query: &Query<&LumaPasswordToggle>,
    parents_query: &Query<&ChildOf>,
    children_query: &Query<&Children>,
    input_query: &mut Query<(&LumaInput, &mut UTextInput)>,
    icon_text_query: &mut Query<&mut UText>,
) {
    let Some(toggle_node) = find_ancestor_or_self(target, toggle_query, parents_query) else {
        return;
    };
    let Some(input_entity) = find_ancestor_or_self(toggle_node, input_query, parents_query) else {
        return;
    };
    let Ok((luma_input, mut text_input)) = input_query.get_mut(input_entity) else {
        return;
    };
    if luma_input.state == InputState::Disabled || !text_input.enabled {
        return;
    }

    let is_password = text_input.mode == TextInputMode::Password;
    text_input.mode = if is_password {
        TextInputMode::Normal
    } else {
        TextInputMode::Password
    };

    visit_descendants(toggle_node, children_query, |entity| {
        if let Ok(mut text) = icon_text_query.get_mut(entity) {
            text.text = if is_password {
                Icon::EYE_OFF.to_string()
            } else {
                Icon::EYE.to_string()
            };
        }
        DescendantVisit::Continue
    });
}

fn on_password_toggle_click(
    trigger: On<PointerClick>,
    toggle_query: Query<&LumaPasswordToggle>,
    parents_query: Query<&ChildOf>,
    children_query: Query<&Children>,
    mut input_query: Query<(&LumaInput, &mut UTextInput)>,
    mut icon_text_query: Query<&mut UText>,
) {
    if !is_primary_click(&trigger) {
        return;
    }
    toggle_password_visibility(
        trigger.entity,
        &toggle_query,
        &parents_query,
        &children_query,
        &mut input_query,
        &mut icon_text_query,
    );
}

fn on_password_toggle_activate(
    trigger: On<UFocusActivate>,
    toggle_query: Query<&LumaPasswordToggle>,
    parents_query: Query<&ChildOf>,
    children_query: Query<&Children>,
    mut input_query: Query<(&LumaInput, &mut UTextInput)>,
    mut icon_text_query: Query<&mut UText>,
) {
    toggle_password_visibility(
        trigger.entity,
        &toggle_query,
        &parents_query,
        &children_query,
        &mut input_query,
        &mut icon_text_query,
    );
}

fn clear_input_field(
    target: Entity,
    commands: &mut Commands,
    clear_query: &Query<&LumaInputClear>,
    parents_query: &Query<&ChildOf>,
    children_query: &Query<&Children>,
    input_query: &mut Query<(Entity, &LumaInput, &mut UTextInput)>,
    text_query: &mut Query<&mut UText, With<LumaInputText>>,
) {
    let Some(clear_node) = find_ancestor_or_self(target, clear_query, parents_query) else {
        return;
    };
    let Some(input_entity) = find_ancestor_or_self(clear_node, input_query, parents_query) else {
        return;
    };
    let Ok((entity, luma_input, mut text_input)) = input_query.get_mut(input_entity) else {
        return;
    };
    if luma_input.state == InputState::Disabled || !text_input.enabled || text_input.value.is_empty() {
        return;
    }

    text_input.value.clear();
    text_input.cursor_position = 0;
    text_input.selection = None;
    let placeholder = text_input.placeholder.clone();
    let placeholder_color = text_input.placeholder_color;

    visit_descendants(entity, children_query, |e| {
        if let Ok(mut txt) = text_query.get_mut(e) {
            txt.text = placeholder.clone();
            txt.color = placeholder_color;
            return DescendantVisit::Stop;
        }
        DescendantVisit::Continue
    });

    commands.trigger(UTextInputChanged {
        entity,
        value: String::new(),
    });
}

fn on_input_clear_click(
    trigger: On<PointerClick>,
    mut commands: Commands,
    clear_query: Query<&LumaInputClear>,
    parents_query: Query<&ChildOf>,
    children_query: Query<&Children>,
    mut input_query: Query<(Entity, &LumaInput, &mut UTextInput)>,
    mut text_query: Query<&mut UText, With<LumaInputText>>,
) {
    if !is_primary_click(&trigger) {
        return;
    }
    clear_input_field(
        trigger.entity,
        &mut commands,
        &clear_query,
        &parents_query,
        &children_query,
        &mut input_query,
        &mut text_query,
    );
}

fn on_input_clear_activate(
    trigger: On<UFocusActivate>,
    mut commands: Commands,
    clear_query: Query<&LumaInputClear>,
    parents_query: Query<&ChildOf>,
    children_query: Query<&Children>,
    mut input_query: Query<(Entity, &LumaInput, &mut UTextInput)>,
    mut text_query: Query<&mut UText, With<LumaInputText>>,
) {
    clear_input_field(
        trigger.entity,
        &mut commands,
        &clear_query,
        &parents_query,
        &children_query,
        &mut input_query,
        &mut text_query,
    );
}

fn link_luma_text_input_markers(
    children_query: Query<&Children>,
    mut inputs: Query<(Entity, &mut UTextInputCaret), Added<LumaInput>>,
    text_markers: Query<Entity, With<UTextInputTextMarker>>,
    caret_markers: Query<Entity, With<UTextInputCaretMarker>>,
    selection_markers: Query<Entity, With<UTextInputSelectionMarker>>,
) {
    for (entity, mut caret) in inputs.iter_mut() {
        if caret.text_entity.is_some()
            && caret.caret_entity.is_some()
            && caret.selection_entity.is_some()
        {
            continue;
        }
        visit_descendants(entity, &children_query, |child| {
            if caret.text_entity.is_none() && text_markers.contains(child) {
                caret.text_entity = Some(child);
            }
            if caret.caret_entity.is_none() && caret_markers.contains(child) {
                caret.caret_entity = Some(child);
            }
            if caret.selection_entity.is_none() && selection_markers.contains(child) {
                caret.selection_entity = Some(child);
            }
            DescendantVisit::Continue
        });
    }
}

fn sync_input_focus_visuals(
    mut focused_query: Query<(&LumaInput, &mut UNode, &mut UBorder), Added<UFocused>>,
    mut lost_focus: RemovedComponents<UFocused>,
    mut unfocused_query: Query<(&LumaInput, &mut UNode, &mut UBorder), Without<UFocused>>,
) {
    for (input, mut node, mut border) in focused_query.iter_mut() {
        if input.state == InputState::Disabled {
            continue;
        }
        node.background_color = input.style.bg_focus;
        if matches!(input.state, InputState::Normal | InputState::Focused) {
            border.color = input.style.border_focus;
        }
    }

    for entity in lost_focus.read() {
        let Ok((input, mut node, mut border)) = unfocused_query.get_mut(entity) else {
            continue;
        };
        if input.state == InputState::Disabled {
            continue;
        }
        node.background_color = input.style.bg_normal;
        if matches!(input.state, InputState::Normal | InputState::Focused) {
            border.color = input.style.border_normal;
        }
    }
}

/// Plugin registering text input widget systems and observers.
pub struct LumaInputPlugin;

impl Plugin for LumaInputPlugin {
    fn build(&self, app: &mut App) {
        app.register_type::<LumaInput>()
            .register_type::<LumaPasswordToggle>()
            .register_type::<LumaInputClear>()
            .register_type::<LumaInputText>()
            .register_type::<LumaInputIcon>()
            .add_observer(on_password_toggle_click)
            .add_observer(on_password_toggle_activate)
            .add_observer(on_input_clear_click)
            .add_observer(on_input_clear_activate)
            .add_systems(Update, (link_luma_text_input_markers, sync_input_focus_visuals));
    }
}
