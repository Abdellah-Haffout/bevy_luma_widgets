//! Interactive multi-workspace tab strip with `+` creation, `X` closing, and double-click inline rename.

use bevy::picking::pointer::PointerButton;
use bevy::prelude::*;
use bevy_luma::interaction_crate::interaction::focus::{
    FocusLost, FocusModality, UFocusState, set_focus,
};
use bevy_luma::interaction_crate::interaction::text_input::{
    UTextInput, UTextInputCancelled, UTextInputCaret, UTextInputCaretMarker,
    UTextInputSelectionMarker, UTextInputSubmit, UTextInputTextMarker,
};
use bevy_luma::layout::text::UTextLabel;
use bevy_luma::prelude::*;

use crate::helpers::find_ancestor_or_self;

use super::area::IconFontHandle;
use super::style::{DockStyle, resolve_fonts, set_interaction_colors_if_changed};
use super::tree::DockTree;
use super::workspace::{DockWorkspaceChanged, WorkspaceRegistry};

/// Marker component placed on the container that should host [`WorkspaceTab`] pills and a `+` button.
#[derive(Component, Clone, Copy, Debug, Default, Reflect)]
#[reflect(Component)]
pub struct WorkspaceTabStrip;

/// Interactive workspace tab inside a [`WorkspaceTabStrip`].
#[derive(Component, Clone, Debug, PartialEq, Eq, Reflect)]
#[reflect(Component)]
pub struct WorkspaceTab {
    pub workspace_id: String,
}

/// Close (`X`) button inside a [`WorkspaceTab`].
#[derive(Component, Clone, Debug, PartialEq, Eq, Reflect)]
#[reflect(Component)]
pub struct WorkspaceTabCloseButton {
    pub workspace_id: String,
}

/// Add (`+`) button inside a [`WorkspaceTabStrip`] creating a workspace cloned from the active tree.
#[derive(Component, Clone, Copy, Debug, Default, Reflect)]
#[reflect(Component)]
pub struct WorkspaceTabAddButton;

/// Inline [`UTextInput`] active while renaming a [`WorkspaceTab`].
#[derive(Component, Clone, Debug, PartialEq, Eq, Reflect)]
#[reflect(Component)]
pub struct WorkspaceRenameInput {
    pub workspace_id: String,
}

#[derive(Component, Clone, Copy, Debug, Default, Reflect)]
#[reflect(Component)]
struct WorkspaceTabAccent;

#[derive(Component, Clone, Copy, Debug, Default, Reflect)]
#[reflect(Component)]
struct WorkspaceTabLabel;

#[derive(Resource, Default, Clone, Debug)]
struct WorkspaceStripState {
    editing_id: Option<String>,
    last_click: Option<(String, f64)>,
    last_signature: Option<WorkspaceStripSignature>,
}

#[derive(Clone, Debug, PartialEq, Eq)]
struct WorkspaceStripSignature {
    items: Vec<(String, String, Option<String>)>,
    editing_id: Option<String>,
}

/// Returns the Luma bundle for spawning a [`WorkspaceTabStrip`] bar.
pub fn workspace_tab_strip() -> impl Bundle {
    let style = DockStyle::default();
    (
        WorkspaceTabStrip,
        UNode {
            width: UVal::Percent(1.0),
            height: UVal::Px(style.workspace_strip_height),
            padding: USides::axes(8.0, 0.0),
            background_color: style.workspace_bg,
            ..default()
        },
        UBorder {
            color: style.tab_bar_border,
            width: 1.0,
            radius: UCornerRadius::default(),
            offset: 0.0,
        },
        ULayout {
            display: UDisplay::Flex,
            flex_direction: UFlexDirection::Row,
            align_items: UAlignItems::Center,
            gap: 4.0,
            ..default()
        },
        USelf {
            flex_grow: Some(0.0),
            flex_shrink: Some(0.0),
            ..default()
        },
    )
}

fn keep_active_workspace_synced(
    tree: Res<DockTree>,
    mut registry: ResMut<WorkspaceRegistry>,
) {
    if tree.is_changed() && registry.active_id().is_some() {
        registry.save_active_tree(&tree);
    }
}

fn workspace_signature_matches(
    sig: &WorkspaceStripSignature,
    registry: &WorkspaceRegistry,
    editing_id: Option<&str>,
) -> bool {
    if sig.editing_id.as_deref() != editing_id {
        return false;
    }
    let workspaces = registry.workspaces();
    sig.items.len() == workspaces.len()
        && sig
            .items
            .iter()
            .zip(workspaces.iter())
            .all(|((s_id, s_name, s_icon), ws)| {
                *s_id == ws.id && *s_name == ws.name && *s_icon == ws.icon
            })
}

fn populate_workspace_tabs(
    mut commands: Commands,
    style: Option<Res<DockStyle>>,
    theme: Option<Res<Theme>>,
    icon_font_res: Option<Res<IconFontHandle>>,
    registry: Res<WorkspaceRegistry>,
    mut strip_state: ResMut<WorkspaceStripState>,
    mut focus_state: Option<ResMut<UFocusState>>,
    strips_q: Query<(Entity, Option<&Children>), (With<WorkspaceTabStrip>, Without<WorkspaceTab>)>,
    added_strips: Query<(), Added<WorkspaceTabStrip>>,
    mut tab_q: Query<(&WorkspaceTab, &mut UNode, &mut UInteractionColors, &Children), Without<WorkspaceTabAccent>>,
    mut accent_q: Query<(&mut UNode, &mut Visibility), (With<WorkspaceTabAccent>, Without<WorkspaceTab>)>,
    mut label_q: Query<&mut UText, (With<WorkspaceTabLabel>, Without<WorkspaceTab>, Without<WorkspaceTabAccent>)>,
) {
    if strips_q.is_empty() {
        return;
    }

    let visual_res_changed = style.as_ref().is_some_and(|s| s.is_changed())
        || theme.as_ref().is_some_and(|t| t.is_changed())
        || icon_font_res.as_ref().is_some_and(|f| f.is_changed());

    if added_strips.is_empty()
        && !registry.is_changed()
        && !visual_res_changed
        && strip_state
            .last_signature
            .as_ref()
            .is_some_and(|sig| sig.editing_id == strip_state.editing_id)
    {
        return;
    }

    let sig_matches = !visual_res_changed
        && strip_state.last_signature.as_ref().is_some_and(|sig| {
            workspace_signature_matches(sig, &registry, strip_state.editing_id.as_deref())
        });
    let needs_rebuild = !added_strips.is_empty() || !sig_matches;

    if !needs_rebuild && !registry.is_changed() && !visual_res_changed {
        return;
    }

    let default_style;
    let style = match style.as_deref() {
        Some(s) => s,
        None => {
            default_style = DockStyle::default();
            &default_style
        }
    };

    if needs_rebuild {
        let (text_font, icon_font) = resolve_fonts(theme.as_deref(), icon_font_res.as_deref());
        let workspaces = registry.workspaces();
        let next_sig = WorkspaceStripSignature {
            items: workspaces
                .iter()
                .map(|w| (w.id.clone(), w.name.clone(), w.icon.clone()))
                .collect(),
            editing_id: strip_state.editing_id.clone(),
        };
        let show_close = workspaces.len() > 1;
        let active_id = registry.active_id().map(str::to_string);

        for (strip_ent, existing_children) in strips_q.iter() {
            if let Some(children) = existing_children {
                for &c in children {
                    if let Ok(mut cmd) = commands.get_entity(c) {
                        cmd.try_despawn();
                    }
                }
            }

            for (ws_id, ws_name, ws_icon) in &next_sig.items {
                let is_active = active_id.as_deref() == Some(ws_id.as_str());
                let is_editing = strip_state.editing_id.as_deref() == Some(ws_id.as_str());
                let bg = if is_active {
                    style.workspace_tab_active_bg
                } else {
                    style.workspace_tab_inactive_bg
                };

                let tab_ent = commands
                    .spawn((
                        ChildOf(strip_ent),
                        WorkspaceTab {
                            workspace_id: ws_id.clone(),
                        },
                        UNode {
                            width: UVal::Auto,
                            height: UVal::Px((style.workspace_strip_height - 6.0).max(22.0)),
                            padding: USides::axes(10.0, 0.0),
                            border_radius: UCornerRadius::all(5.0),
                            background_color: bg,
                            ..default()
                        },
                        ULayout {
                            display: UDisplay::Flex,
                            flex_direction: UFlexDirection::Row,
                            align_items: UAlignItems::Center,
                            gap: 6.0,
                            ..default()
                        },
                        UInteraction::default(),
                        UInteractionColors {
                            normal: bg,
                            hovered: if is_active {
                                bg
                            } else {
                                style.workspace_tab_hover_bg
                            },
                            pressed: style.workspace_tab_active_bg,
                        },
                    ))
                    .id();

                commands.spawn((
                    ChildOf(tab_ent),
                    WorkspaceTabAccent,
                    UNode {
                        width: UVal::Percent(1.0),
                        height: UVal::Px(2.0),
                        background_color: style.accent_color,
                        ..default()
                    },
                    USelf {
                        position_type: UPositionType::Absolute,
                        left: UVal::Px(2.0),
                        right: UVal::Px(2.0),
                        bottom: UVal::Px(0.0),
                        ..default()
                    },
                    if is_active {
                        Visibility::Inherited
                    } else {
                        Visibility::Hidden
                    },
                ));

                if let Some(ic) = ws_icon {
                    commands.spawn((
                        ChildOf(tab_ent),
                        UNode::default(),
                        UText {
                            text: ic.clone(),
                            font_size: 12.5,
                            font: icon_font.clone(),
                            color: if is_active {
                                style.accent_color
                            } else {
                                style.icon_color
                            },
                            ..default()
                        },
                    ));
                }

                if is_editing {
                    let input_ent = spawn_compact_rename_input(
                        &mut commands,
                        tab_ent,
                        ws_id,
                        ws_name,
                        &style,
                        text_font.clone(),
                    );
                    if let Some(fs) = focus_state.as_deref_mut() {
                        set_focus(&mut commands, fs, Some(input_ent), FocusModality::Pointer);
                    }
                } else {
                    commands.spawn((
                        ChildOf(tab_ent),
                        WorkspaceTabLabel,
                        UNode::default(),
                        UText {
                            text: ws_name.clone(),
                            font_size: 12.5,
                            font: text_font.clone(),
                            color: if is_active {
                                style.workspace_text_active
                            } else {
                                style.workspace_text_inactive
                            },
                            ..default()
                        },
                    ));

                    if show_close {
                        let close_btn = commands
                            .spawn((
                                ChildOf(tab_ent),
                                WorkspaceTabCloseButton {
                                    workspace_id: ws_id.clone(),
                                },
                                UNode {
                                    width: UVal::Px(16.0),
                                    height: UVal::Px(16.0),
                                    border_radius: UCornerRadius::all(4.0),
                                    background_color: Color::NONE,
                                    ..default()
                                },
                                ULayout {
                                    display: UDisplay::Flex,
                                    justify_content: UJustifyContent::Center,
                                    align_items: UAlignItems::Center,
                                    ..default()
                                },
                                UInteraction::default(),
                                UInteractionColors {
                                    normal: Color::NONE,
                                    hovered: style.close_hover_bg,
                                    pressed: style.close_hover_bg,
                                },
                            ))
                            .id();
                        commands.spawn((
                            ChildOf(close_btn),
                            UNode::default(),
                            UText {
                                text: Icon::X.to_string(),
                                font_size: 10.5,
                                font: icon_font.clone(),
                                color: style.icon_color,
                                ..default()
                            },
                        ));
                    }
                }
            }

            let add_btn = commands
                .spawn((
                    ChildOf(strip_ent),
                    WorkspaceTabAddButton,
                    UNode {
                        width: UVal::Px(24.0),
                        height: UVal::Px(24.0),
                        border_radius: UCornerRadius::all(5.0),
                        background_color: Color::NONE,
                        ..default()
                    },
                    ULayout {
                        display: UDisplay::Flex,
                        justify_content: UJustifyContent::Center,
                        align_items: UAlignItems::Center,
                        ..default()
                    },
                    UInteraction::default(),
                    UInteractionColors {
                        normal: Color::NONE,
                        hovered: style.action_hover_bg,
                        pressed: style.action_hover_bg,
                    },
                ))
                .id();
            commands.spawn((
                ChildOf(add_btn),
                UNode::default(),
                UText {
                    text: Icon::PLUS.to_string(),
                    font_size: 13.5,
                    font: icon_font.clone(),
                    color: style.icon_color,
                    ..default()
                },
            ));
        }
        strip_state.last_signature = Some(next_sig);
    } else if registry.is_changed() || visual_res_changed {
        let active_id = registry.active_id();
        for (tab, mut node, mut icolors, children) in tab_q.iter_mut() {
            let is_active = active_id == Some(tab.workspace_id.as_str());
            let bg = if is_active {
                style.workspace_tab_active_bg
            } else {
                style.workspace_tab_inactive_bg
            };
            if node.background_color != bg {
                node.background_color = bg;
            }
            let next_icolors = UInteractionColors {
                normal: bg,
                hovered: if is_active {
                    bg
                } else {
                    style.workspace_tab_hover_bg
                },
                pressed: style.workspace_tab_active_bg,
            };
            set_interaction_colors_if_changed(&mut icolors, next_icolors);
            for &c in children {
                if let Ok((mut anode, mut avis)) = accent_q.get_mut(c) {
                    let target_vis = if is_active {
                        Visibility::Inherited
                    } else {
                        Visibility::Hidden
                    };
                    if *avis != target_vis {
                        *avis = target_vis;
                    }
                    if anode.background_color != style.accent_color {
                        anode.background_color = style.accent_color;
                    }
                }
                if let Ok(mut label) = label_q.get_mut(c) {
                    let target_color = if is_active {
                        style.workspace_text_active
                    } else {
                        style.workspace_text_inactive
                    };
                    if label.color != target_color {
                        label.color = target_color;
                    }
                }
            }
        }
    }
}

fn on_workspace_tab_click(
    mut trigger: On<PointerClick>,
    mut commands: Commands,
    time: Option<Res<Time>>,
    parents: Query<&ChildOf>,
    tabs_q: Query<&WorkspaceTab>,
    close_q: Query<(), With<WorkspaceTabCloseButton>>,
    rename_q: Query<(), With<WorkspaceRenameInput>>,
    mut registry: ResMut<WorkspaceRegistry>,
    mut dock_tree: ResMut<DockTree>,
    mut strip_state: ResMut<WorkspaceStripState>,
) {
    if trigger.event().button != PointerButton::Primary {
        return;
    }
    if find_ancestor_or_self(trigger.entity, &close_q, &parents).is_some()
        || find_ancestor_or_self(trigger.entity, &rename_q, &parents).is_some()
    {
        return;
    }
    let Some(tab_ent) = find_ancestor_or_self(trigger.entity, &tabs_q, &parents) else {
        return;
    };
    let Ok(tab) = tabs_q.get(tab_ent) else {
        return;
    };
    trigger.propagate(false);

    let now = time.as_deref().map(|t| t.elapsed_secs_f64()).unwrap_or(0.0);
    let is_double = strip_state
        .last_click
        .as_ref()
        .is_some_and(|(id, prev)| id == &tab.workspace_id && (now - prev) < 0.35);

    if is_double {
        strip_state.last_click = None;
        strip_state.editing_id = Some(tab.workspace_id.clone());
        return;
    }
    strip_state.last_click = Some((tab.workspace_id.clone(), now));

    if let Some(ev) = registry.switch_active(&tab.workspace_id, &mut dock_tree) {
        commands.trigger(ev);
    }
}

fn on_workspace_close_click(
    mut trigger: On<PointerClick>,
    mut commands: Commands,
    parents: Query<&ChildOf>,
    close_q: Query<&WorkspaceTabCloseButton>,
    mut registry: ResMut<WorkspaceRegistry>,
    mut dock_tree: ResMut<DockTree>,
) {
    if trigger.event().button != PointerButton::Primary {
        return;
    }
    let Some(btn_ent) = find_ancestor_or_self(trigger.entity, &close_q, &parents) else {
        return;
    };
    let Ok(btn) = close_q.get(btn_ent) else {
        return;
    };
    trigger.propagate(false);

    let prev = registry.active_id().map(str::to_string);
    let (removed, next_active) = registry.remove_workspace(&btn.workspace_id);
    if !removed {
        return;
    }
    if let Some(current) = next_active {
        registry.load_active_into(&mut dock_tree);
        commands.trigger(DockWorkspaceChanged {
            previous: prev,
            current,
        });
    }
}

fn on_workspace_add_click(
    mut trigger: On<PointerClick>,
    mut commands: Commands,
    parents: Query<&ChildOf>,
    add_q: Query<(), With<WorkspaceTabAddButton>>,
    mut registry: ResMut<WorkspaceRegistry>,
    mut dock_tree: ResMut<DockTree>,
) {
    if trigger.event().button != PointerButton::Primary {
        return;
    }
    if find_ancestor_or_self(trigger.entity, &add_q, &parents).is_none() {
        return;
    }
    trigger.propagate(false);

    registry.save_active_tree(&dock_tree);
    let name = registry.next_default_name();
    let cloned_tree = dock_tree.clone();
    let new_id = registry.add_workspace(name, cloned_tree);
    if let Some(ev) = registry.switch_active(&new_id, &mut dock_tree) {
        commands.trigger(ev);
    }
}

fn spawn_compact_rename_input(
    commands: &mut Commands,
    parent: Entity,
    workspace_id: &str,
    initial_value: &str,
    style: &DockStyle,
    font: Handle<Font>,
) -> Entity {
    let font_size = 12.0;
    let caret_h = font_size * 1.15;
    let cursor_pos = initial_value.chars().count();
    let input = UTextInput {
        value: initial_value.to_string(),
        font_size,
        text_color: style.workspace_text_active,
        cursor_position: cursor_pos,
        selection: Some((0, cursor_pos)),
        ..default()
    };

    let input_ent = commands
        .spawn((
            ChildOf(parent),
            WorkspaceRenameInput {
                workspace_id: workspace_id.to_string(),
            },
            UNode {
                width: UVal::Px(112.0),
                height: UVal::Px(22.0),
                padding: USides::axes(6.0, 2.0),
                background_color: style.canvas_bg,
                border_radius: UCornerRadius::all(4.0),
                ..default()
            },
            UBorder {
                color: style.accent_color,
                width: 1.0,
                radius: UCornerRadius::all(4.0),
                offset: 0.0,
            },
            ULayout {
                display: UDisplay::Flex,
                flex_direction: UFlexDirection::Row,
                align_items: UAlignItems::Center,
                ..default()
            },
            UFocusable::new(),
            UInteraction::default(),
            input,
        ))
        .id();

    let sel_ent = commands
        .spawn((
            ChildOf(input_ent),
            UNode {
                width: UVal::Px(0.0),
                height: UVal::Px(caret_h),
                background_color: style.accent_color.with_alpha(0.28),
                border_radius: UCornerRadius::all(2.0),
                ..default()
            },
            USelf {
                position_type: UPositionType::Absolute,
                left: UVal::Px(6.0),
                top: UVal::Px(4.0),
                ..default()
            },
            Visibility::Hidden,
            UTextInputSelectionMarker,
        ))
        .id();

    let text_ent = commands
        .spawn((
            ChildOf(input_ent),
            UNode::default(),
            UTextLabel {
                text: initial_value.to_string(),
                font_size,
                font,
                color: style.workspace_text_active,
                linebreak: bevy::text::LineBreak::NoWrap,
                autosize: true,
                overflow: bevy_luma::layout::text::UTextOverflow::Clip,
                ..default()
            },
            UTextInputTextMarker,
        ))
        .id();

    let caret_ent = commands
        .spawn((
            ChildOf(input_ent),
            UNode {
                width: UVal::Px(1.5),
                height: UVal::Px(caret_h),
                background_color: style.accent_color,
                ..default()
            },
            USelf {
                position_type: UPositionType::Absolute,
                left: UVal::Px(6.0),
                top: UVal::Px(4.0),
                ..default()
            },
            Visibility::Hidden,
            UTextInputCaretMarker,
        ))
        .id();

    commands.entity(input_ent).try_insert(UTextInputCaret {
        text_entity: Some(text_ent),
        caret_entity: Some(caret_ent),
        selection_entity: Some(sel_ent),
        ..default()
    });
    input_ent
}

fn on_workspace_rename_submit(
    trigger: On<UTextInputSubmit>,
    mut commands: Commands,
    rename_q: Query<&WorkspaceRenameInput>,
    mut registry: ResMut<WorkspaceRegistry>,
    mut strip_state: ResMut<WorkspaceStripState>,
) {
    let Ok(rename) = rename_q.get(trigger.entity) else {
        return;
    };
    registry.rename_workspace(&rename.workspace_id, &trigger.value);
    strip_state.editing_id = None;
    if let Ok(mut cmd) = commands.get_entity(trigger.entity) {
        cmd.try_remove::<WorkspaceRenameInput>();
    }
}

fn on_workspace_rename_cancel(
    trigger: On<UTextInputCancelled>,
    mut commands: Commands,
    rename_q: Query<(), With<WorkspaceRenameInput>>,
    mut strip_state: ResMut<WorkspaceStripState>,
) {
    if rename_q.contains(trigger.entity) {
        strip_state.editing_id = None;
        if let Ok(mut cmd) = commands.get_entity(trigger.entity) {
            cmd.try_remove::<WorkspaceRenameInput>();
        }
    }
}

fn on_workspace_rename_blur(
    trigger: On<FocusLost>,
    rename_q: Query<(&WorkspaceRenameInput, &UTextInput)>,
    mut registry: ResMut<WorkspaceRegistry>,
    mut strip_state: ResMut<WorkspaceStripState>,
) {
    if let Ok((rename, input)) = rename_q.get(trigger.entity) {
        registry.rename_workspace(&rename.workspace_id, &input.value);
        strip_state.editing_id = None;
    }
}

/// Plugin enabling multi-workspace tab strip rendering, switching, creation, closing, and inline rename.
pub struct WorkspaceTabsPlugin;

impl Plugin for WorkspaceTabsPlugin {
    fn build(&self, app: &mut App) {
        super::configure_panel_pipeline(app);
        app.init_resource::<WorkspaceRegistry>()
            .init_resource::<WorkspaceStripState>()
            .register_type::<WorkspaceTabStrip>()
            .register_type::<WorkspaceTab>()
            .register_type::<WorkspaceTabCloseButton>()
            .register_type::<WorkspaceTabAddButton>()
            .register_type::<WorkspaceRenameInput>()
            .add_systems(
                Update,
                (keep_active_workspace_synced, populate_workspace_tabs)
                    .chain()
                    .in_set(super::PanelSystemSet::SyncVisuals),
            )
            .add_observer(on_workspace_tab_click)
            .add_observer(on_workspace_close_click)
            .add_observer(on_workspace_add_click)
            .add_observer(on_workspace_rename_submit)
            .add_observer(on_workspace_rename_cancel)
            .add_observer(on_workspace_rename_blur);
    }
}
