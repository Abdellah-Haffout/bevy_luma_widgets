//! Declarative [`DockTree`] reconciliation against live Luma [`PanelGroup`] and [`DockArea`] entities.

use std::collections::{HashMap, HashSet};

use bevy::ecs::system::SystemParam;
use bevy::prelude::*;
use bevy_luma::prelude::*;

use super::area::{ActiveDockWindow, DockArea, DockTabContent, DockWindow, IconFontHandle};
use super::registry::WindowRegistry;
use super::split::{Panel, PanelGroup, PanelHandle, panel, panel_group, spawn_panel_handle};
use super::style::{DockStyle, resolve_fonts};
use super::tabs::spawn_dock_area;
use super::tree::{DockLeaf, DockNode, DockSplit, DockTree, NodeId, TabId};

/// Marker component placed on the UI root entity whose children are managed by [`DockTree`].
#[derive(Component, Clone, Copy, Debug, Default, Reflect)]
#[reflect(Component)]
pub struct DockTreeHost;

/// Associates a live [`PanelGroup`] or [`DockArea`] entity with its [`NodeId`] in [`DockTree`].
#[derive(Component, Clone, Copy, Debug, PartialEq, Eq, Hash, Reflect)]
#[reflect(Component)]
pub struct NodeBinding(pub NodeId);

/// Associates a live [`DockArea`] entity with its [`DockLeaf`] [`NodeId`] in [`DockTree`].
#[derive(Component, Clone, Copy, Debug, PartialEq, Eq, Hash, Reflect)]
#[reflect(Component)]
pub struct LeafBinding(pub NodeId);

/// Associates a live [`DockWindow`] entity with its [`TabId`] in [`DockTree`] so window widget state
/// survives tab drags and splits cleanly.
#[derive(Component, Clone, Copy, Debug, PartialEq, Eq, Hash, Reflect)]
#[reflect(Component)]
pub struct TabBinding(pub TabId);

#[derive(Component, Clone, Copy, Debug, Default)]
struct PendingRegistryPopulate;

/// Tracks the last reconciled [`DockTree`] state to avoid unnecessary rebuilds.
#[derive(Resource, Default, Clone, Debug)]
pub(crate) struct ReconciledTreeState {
    pub(crate) last_tree: Option<DockTree>,
    pub(crate) last_host: Option<Entity>,
    pub(crate) workspace_switched: bool,
}

#[derive(PartialEq, Eq)]
enum TreeDiffKind {
    Identical,
    FractionsOnly,
    ActiveOnly,
    Structural,
}

fn classify_diff(prev: &DockNode, next: &DockNode) -> TreeDiffKind {
    match (prev, next) {
        (DockNode::Leaf(a), DockNode::Leaf(b)) => {
            if a.id != b.id
                || a.style != b.style
                || a.persistent != b.persistent
                || a.name != b.name
                || a.tabs != b.tabs
            {
                return TreeDiffKind::Structural;
            }
            if a.active != b.active {
                TreeDiffKind::ActiveOnly
            } else {
                TreeDiffKind::Identical
            }
        }
        (DockNode::Split(a), DockNode::Split(b)) => {
            if a.id != b.id || a.axis != b.axis {
                return TreeDiffKind::Structural;
            }
            let d1 = classify_diff(&a.first, &b.first);
            if d1 == TreeDiffKind::Structural {
                return TreeDiffKind::Structural;
            }
            let d2 = classify_diff(&a.second, &b.second);
            if d2 == TreeDiffKind::Structural {
                return TreeDiffKind::Structural;
            }
            if (a.fraction - b.fraction).abs() > 1e-4 {
                TreeDiffKind::FractionsOnly
            } else if d1 == TreeDiffKind::FractionsOnly || d2 == TreeDiffKind::FractionsOnly {
                TreeDiffKind::FractionsOnly
            } else if d1 == TreeDiffKind::ActiveOnly || d2 == TreeDiffKind::ActiveOnly {
                TreeDiffKind::ActiveOnly
            } else {
                TreeDiffKind::Identical
            }
        }
        _ => TreeDiffKind::Structural,
    }
}

fn apply_fractions_in_place(
    node: &DockNode,
    group_bindings: &Query<
        (
            Entity,
            &'static NodeBinding,
            Option<&'static ChildOf>,
            Option<&'static Children>,
            &'static mut PanelGroup,
        ),
        (
            Without<DockArea>,
            Without<DockWindow>,
            Without<Panel>,
            Without<DockTabContent>,
            Without<PanelHandle>,
        ),
    >,
    panels: &mut Query<
        (Entity, &'static mut Panel, Option<&'static Children>),
        (
            Without<PanelGroup>,
            Without<DockArea>,
            Without<DockWindow>,
            Without<DockTabContent>,
            Without<PanelHandle>,
        ),
    >,
) {
    let DockNode::Split(split) = node else {
        return;
    };
    for (_, binding, _, children_opt, _) in group_bindings.iter() {
        if binding.0 != split.id {
            continue;
        }
        let Some(children) = children_opt else {
            continue;
        };
        let mut p_pair = [None, None];
        let mut count = 0;
        for &c in children {
            if panels.contains(c) {
                if count < 2 {
                    p_pair[count] = Some(c);
                }
                count += 1;
            }
        }
        if count == 2
            && let (Some(e0), Some(e1)) = (p_pair[0], p_pair[1])
        {
            let first_ratio = split.fraction.clamp(0.05, 0.95);
            let second_ratio = 1.0 - first_ratio;
            if let Ok((_, mut p1, _)) = panels.get_mut(e0) {
                if (p1.ratio - first_ratio).abs() > 1e-4 {
                    p1.ratio = first_ratio;
                }
            }
            if let Ok((_, mut p2, _)) = panels.get_mut(e1) {
                if (p2.ratio - second_ratio).abs() > 1e-4 {
                    p2.ratio = second_ratio;
                }
            }
        }
    }
    apply_fractions_in_place(&split.first, group_bindings, panels);
    apply_fractions_in_place(&split.second, group_bindings, panels);
}

fn apply_active_in_place(
    node: &DockNode,
    leaf_areas: &mut Query<
        (
            Entity,
            &'static LeafBinding,
            Option<&'static ChildOf>,
            Option<&'static Children>,
            &'static mut DockArea,
            &'static mut ActiveDockWindow,
        ),
        (
            Without<PanelGroup>,
            Without<DockWindow>,
            Without<Panel>,
            Without<DockTabContent>,
            Without<PanelHandle>,
        ),
    >,
    content_q: &Query<
        Option<&'static Children>,
        (
            With<DockTabContent>,
            Without<PanelGroup>,
            Without<DockArea>,
            Without<DockWindow>,
            Without<Panel>,
            Without<PanelHandle>,
        ),
    >,
    live_windows_q: &mut Query<
        (
            Entity,
            &'static TabBinding,
            Option<&'static ChildOf>,
            &'static mut DockWindow,
            &'static mut ULayout,
            &'static mut Visibility,
            &'static mut Transform,
        ),
        (
            Without<DockArea>,
            Without<PanelGroup>,
            Without<Panel>,
            Without<DockTabContent>,
            Without<PanelHandle>,
        ),
    >,
) {
    match node {
        DockNode::Leaf(leaf) => {
            let active_idx = resolved_leaf_active_index(leaf);
            for (_, binding, _, area_children_opt, _, mut active) in leaf_areas.iter_mut() {
                if binding.0 != leaf.id {
                    continue;
                }
                let Some(area_children) = area_children_opt else {
                    continue;
                };
                let Some(content_ent) = area_children.iter().find(|&c| content_q.contains(c))
                else {
                    continue;
                };
                let Ok(Some(content_children)) = content_q.get(content_ent) else {
                    if active.0.is_some() {
                        active.0 = None;
                    }
                    continue;
                };
                let target = active_idx.and_then(|idx| {
                    content_children
                        .iter()
                        .filter(|&e| live_windows_q.contains(e))
                        .nth(idx)
                });
                if active.0 != target {
                    active.0 = target;
                }
                for &c in content_children {
                    if let Ok((win_ent, _, _, _, mut layout, mut vis, mut transform)) =
                        live_windows_q.get_mut(c)
                    {
                        apply_window_active_state(
                            &mut layout,
                            &mut vis,
                            &mut transform,
                            Some(win_ent) == target,
                        );
                    }
                }
            }
        }
        DockNode::Split(split) => {
            apply_active_in_place(&split.first, leaf_areas, content_q, live_windows_q);
            apply_active_in_place(&split.second, leaf_areas, content_q, live_windows_q);
        }
    }
}

fn resolve_tab_window_meta(
    registry: &WindowRegistry,
    window_id: &str,
    title: &str,
) -> (String, Option<String>) {
    let desc = registry.get(window_id);
    let icon = desc.and_then(|d| d.icon.clone());
    let resolved_title = if title.is_empty() {
        desc.map(|d| d.name.clone())
            .unwrap_or_else(|| window_id.to_string())
    } else {
        title.to_string()
    };
    (resolved_title, icon)
}

fn apply_window_active_state(
    layout: &mut ULayout,
    vis: &mut Visibility,
    transform: &mut Transform,
    is_active: bool,
) {
    let target_display = if is_active {
        UDisplay::Flex
    } else {
        UDisplay::None
    };
    if layout.display != target_display {
        layout.display = target_display;
    }
    let target_vis = if is_active {
        Visibility::Inherited
    } else {
        Visibility::Hidden
    };
    if *vis != target_vis {
        *vis = target_vis;
    }
    if is_active && transform.scale == Vec3::ZERO {
        transform.scale = Vec3::ONE;
    }
}

fn resolved_leaf_active_index(leaf: &DockLeaf) -> Option<usize> {
    if leaf.tabs.is_empty() {
        None
    } else {
        Some(leaf.active.min(leaf.tabs.len() - 1))
    }
}

/// Immediately reconciles `DockTree` into a [`DockTreeHost`] entity inside `world` (handy in setup systems or tests).
pub fn reconcile_dock_tree_now(
    commands: &mut Commands,
    host_entity: Entity,
    tree: &DockTree,
    registry: &WindowRegistry,
    style: &DockStyle,
    text_font: Handle<Font>,
    icon_font: Handle<Font>,
) {
    spawn_node_subtree(
        commands,
        host_entity,
        tree.root(),
        registry,
        style,
        &text_font,
        &icon_font,
        &mut HashMap::new(),
    );
}

#[allow(clippy::too_many_arguments)]
fn spawn_node_subtree(
    commands: &mut Commands,
    parent: Entity,
    node: &DockNode,
    registry: &WindowRegistry,
    style: &DockStyle,
    text_font: &Handle<Font>,
    icon_font: &Handle<Font>,
    reusable_windows: &mut HashMap<(TabId, String), Entity>,
) -> Entity {
    match node {
        DockNode::Split(split) => {
            spawn_split_subtree(
                commands,
                parent,
                split,
                registry,
                style,
                text_font,
                icon_font,
                reusable_windows,
            )
        }
        DockNode::Leaf(leaf) => {
            spawn_leaf_area(
                commands,
                parent,
                leaf,
                registry,
                style,
                text_font,
                icon_font,
                reusable_windows,
            )
        }
    }
}

#[allow(clippy::too_many_arguments)]
fn spawn_split_subtree(
    commands: &mut Commands,
    parent: Entity,
    split: &DockSplit,
    registry: &WindowRegistry,
    style: &DockStyle,
    text_font: &Handle<Font>,
    icon_font: &Handle<Font>,
    reusable_windows: &mut HashMap<(TabId, String), Entity>,
) -> Entity {
    let group_ent = commands
        .spawn((
            ChildOf(parent),
            NodeBinding(split.id),
            panel_group(split.axis.into()),
        ))
        .id();

    let first_frac = split.fraction.clamp(0.05, 0.95);
    let second_frac = (1.0 - first_frac).clamp(0.05, 0.95);

    let first_panel = commands
        .spawn((ChildOf(group_ent), panel(first_frac)))
        .id();
    spawn_node_subtree(
        commands,
        first_panel,
        &split.first,
        registry,
        style,
        text_font,
        icon_font,
        reusable_windows,
    );

    spawn_panel_handle(commands, group_ent, style);

    let second_panel = commands
        .spawn((ChildOf(group_ent), panel(second_frac)))
        .id();
    spawn_node_subtree(
        commands,
        second_panel,
        &split.second,
        registry,
        style,
        text_font,
        icon_font,
        reusable_windows,
    );

    group_ent
}

fn leaf_area_id(leaf: &DockLeaf) -> String {
    if leaf.name.is_empty() {
        format!("leaf_{}", leaf.id.0)
    } else {
        leaf.name.clone()
    }
}

#[allow(clippy::too_many_arguments)]
fn spawn_leaf_area(
    commands: &mut Commands,
    parent: Entity,
    leaf: &DockLeaf,
    registry: &WindowRegistry,
    style: &DockStyle,
    text_font: &Handle<Font>,
    icon_font: &Handle<Font>,
    reusable_windows: &mut HashMap<(TabId, String), Entity>,
) -> Entity {
    let area = DockArea::new(leaf_area_id(leaf))
        .with_style(leaf.style)
        .with_persistent(leaf.persistent);

    let (area_ent, content_ent) = spawn_dock_area(
        commands,
        Some(parent),
        area,
        style,
        text_font.clone(),
        icon_font.clone(),
    );

    commands
        .entity(area_ent)
        .insert((NodeBinding(leaf.id), LeafBinding(leaf.id)));

    let active_idx = resolved_leaf_active_index(leaf);
    let mut active_win_ent = None;
    for (idx, tab) in leaf.tabs.iter().enumerate() {
        let is_active = Some(idx) == active_idx;
        let win_ent = if let Some(existing_ent) =
            reusable_windows.remove(&(tab.tab_id, tab.window_id.clone()))
        {
            let (title, icon) = resolve_tab_window_meta(registry, &tab.window_id, &tab.title);
            commands.entity(existing_ent).insert((
                ChildOf(content_ent),
                DockWindow {
                    window_id: tab.window_id.clone(),
                    name: title,
                    icon,
                    closeable: true,
                },
                TabBinding(tab.tab_id),
            ));
            existing_ent
        } else {
            spawn_tab_window_entity(
                commands,
                content_ent,
                tab.tab_id,
                &tab.window_id,
                &tab.title,
                is_active,
                registry,
                text_font,
            )
        };

        if is_active {
            active_win_ent = Some(win_ent);
        }
    }

    commands
        .entity(area_ent)
        .insert(ActiveDockWindow(active_win_ent));
    area_ent
}

#[allow(clippy::too_many_arguments)]
fn spawn_tab_window_entity(
    commands: &mut Commands,
    content_ent: Entity,
    tab_id: TabId,
    window_id: &str,
    title: &str,
    is_active: bool,
    registry: &WindowRegistry,
    text_font: &Handle<Font>,
) -> Entity {
    let (resolved_title, icon) = resolve_tab_window_meta(registry, window_id, title);

    let mut win = DockWindow::new(window_id, resolved_title);
    if let Some(ic) = icon {
        win = win.with_icon(ic);
    }

    let win_ent = commands
        .spawn((
            ChildOf(content_ent),
            win,
            TabBinding(tab_id),
            UNode {
                width: UVal::Percent(1.0),
                height: UVal::Percent(1.0),
                background_color: Color::NONE,
                ..default()
            },
            ULayout {
                display: if is_active {
                    UDisplay::Flex
                } else {
                    UDisplay::None
                },
                flex_direction: UFlexDirection::Column,
                align_items: UAlignItems::Stretch,
                ..default()
            },
            if is_active {
                Visibility::Inherited
            } else {
                Visibility::Hidden
            },
            UClip { enabled: true },
        ))
        .id();

    if !registry.build_into(window_id, commands, win_ent) {
        commands.entity(win_ent).insert(PendingRegistryPopulate);
        commands.spawn((
            ChildOf(win_ent),
            UNode {
                width: UVal::Percent(1.0),
                height: UVal::Percent(1.0),
                padding: USides::all(16.0),
                background_color: Color::NONE,
                ..default()
            },
            ULayout {
                display: UDisplay::Flex,
                justify_content: UJustifyContent::Center,
                align_items: UAlignItems::Center,
                ..default()
            },
            UText {
                text: format!("Unregistered panel: {window_id}"),
                font_size: 13.0,
                font: text_font.clone(),
                color: Color::srgba(0.65, 0.65, 0.72, 0.8),
                ..default()
            },
        ));
    }

    win_ent
}

#[derive(SystemParam)]
struct ReconcileVisualResources<'w> {
    registry: Res<'w, WindowRegistry>,
    style: Option<Res<'w, DockStyle>>,
    theme: Option<Res<'w, Theme>>,
    icon_font: Option<Res<'w, IconFontHandle>>,
}

#[derive(SystemParam)]
#[allow(clippy::type_complexity)]
struct ReconcileTreeQueries<'w, 's> {
    hosts: Query<'w, 's, (Entity, Option<&'static Children>), With<DockTreeHost>>,
    group_bindings: Query<
        'w,
        's,
        (
            Entity,
            &'static NodeBinding,
            Option<&'static ChildOf>,
            Option<&'static Children>,
            &'static mut PanelGroup,
        ),
        (
            Without<DockArea>,
            Without<DockWindow>,
            Without<Panel>,
            Without<DockTabContent>,
            Without<PanelHandle>,
        ),
    >,
    panels: Query<
        'w,
        's,
        (Entity, &'static mut Panel, Option<&'static Children>),
        (
            Without<PanelGroup>,
            Without<DockArea>,
            Without<DockWindow>,
            Without<DockTabContent>,
            Without<PanelHandle>,
        ),
    >,
    handles: Query<
        'w,
        's,
        Entity,
        (
            With<PanelHandle>,
            Without<PanelGroup>,
            Without<DockArea>,
            Without<DockWindow>,
            Without<Panel>,
            Without<DockTabContent>,
        ),
    >,
    leaf_areas: Query<
        'w,
        's,
        (
            Entity,
            &'static LeafBinding,
            Option<&'static ChildOf>,
            Option<&'static Children>,
            &'static mut DockArea,
            &'static mut ActiveDockWindow,
        ),
        (
            Without<PanelGroup>,
            Without<DockWindow>,
            Without<Panel>,
            Without<DockTabContent>,
            Without<PanelHandle>,
        ),
    >,
    content_q: Query<
        'w,
        's,
        Option<&'static Children>,
        (
            With<DockTabContent>,
            Without<PanelGroup>,
            Without<DockArea>,
            Without<DockWindow>,
            Without<Panel>,
            Without<PanelHandle>,
        ),
    >,
    live_windows_q: Query<
        'w,
        's,
        (
            Entity,
            &'static TabBinding,
            Option<&'static ChildOf>,
            &'static mut DockWindow,
            &'static mut ULayout,
            &'static mut Visibility,
            &'static mut Transform,
        ),
        (
            Without<DockArea>,
            Without<PanelGroup>,
            Without<Panel>,
            Without<DockTabContent>,
            Without<PanelHandle>,
        ),
    >,
}

struct ReconcileContext {
    style: DockStyle,
    text_font: Handle<Font>,
    icon_font: Handle<Font>,
}

#[derive(Default)]
struct ExistingSubtrees {
    groups: HashMap<NodeId, Entity>,
    all_groups: Vec<Entity>,
    leaves: HashMap<NodeId, Entity>,
    all_leaves: Vec<Entity>,
    windows: HashMap<(TabId, String), Entity>,
    all_windows: Vec<Entity>,
    reused_groups: HashSet<Entity>,
    reused_leaves: HashSet<Entity>,
    reused_windows: HashSet<Entity>,
}

fn reconcile_node(
    commands: &mut Commands,
    node: &DockNode,
    parent: Entity,
    ctx: &ReconcileContext,
    registry: &WindowRegistry,
    q: &mut ReconcileTreeQueries,
    existing: &mut ExistingSubtrees,
) -> Entity {
    match node {
        DockNode::Split(split) => {
            reconcile_split_node(commands, split, parent, ctx, registry, q, existing)
        }
        DockNode::Leaf(leaf) => {
            reconcile_leaf_node(commands, leaf, parent, ctx, registry, q, existing)
        }
    }
}

fn reconcile_split_node(
    commands: &mut Commands,
    split: &DockSplit,
    parent: Entity,
    ctx: &ReconcileContext,
    registry: &WindowRegistry,
    q: &mut ReconcileTreeQueries,
    existing: &mut ExistingSubtrees,
) -> Entity {
    let target_dir = split.axis.into();
    let first_frac = split.fraction.clamp(0.05, 0.95);
    let second_frac = (1.0 - first_frac).clamp(0.05, 0.95);

    let reused_split = existing.groups.remove(&split.id).and_then(|group_ent| {
        let Ok((_, _, child_of_opt, children_opt, group)) = q.group_bindings.get(group_ent) else {
            return None;
        };
        if group.direction != target_dir {
            return None;
        }
        let children = children_opt?;
        if children.len() != 3 {
            return None;
        }
        let (p0, handle, p1) = (children[0], children[1], children[2]);
        if !q.panels.contains(p0) || !q.handles.contains(handle) || !q.panels.contains(p1) {
            return None;
        }
        Some((group_ent, child_of_opt.map(|c| c.parent()), p0, p1))
    });

    if let Some((group_ent, old_parent, p0, p1)) = reused_split {
        if old_parent != Some(parent) {
            commands.entity(group_ent).insert(ChildOf(parent));
        }
        if let Ok((_, mut pan0, _)) = q.panels.get_mut(p0)
            && (pan0.ratio - first_frac).abs() > 1e-4
        {
            pan0.ratio = first_frac;
        }
        if let Ok((_, mut pan1, _)) = q.panels.get_mut(p1)
            && (pan1.ratio - second_frac).abs() > 1e-4
        {
            pan1.ratio = second_frac;
        }
        existing.reused_groups.insert(group_ent);
        reconcile_node(commands, &split.first, p0, ctx, registry, q, existing);
        reconcile_node(commands, &split.second, p1, ctx, registry, q, existing);
        return group_ent;
    }

    let group_ent = commands
        .spawn((
            ChildOf(parent),
            NodeBinding(split.id),
            panel_group(target_dir),
        ))
        .id();

    let first_panel = commands.spawn((ChildOf(group_ent), panel(first_frac))).id();
    reconcile_node(
        commands,
        &split.first,
        first_panel,
        ctx,
        registry,
        q,
        existing,
    );

    spawn_panel_handle(commands, group_ent, &ctx.style);

    let second_panel = commands
        .spawn((ChildOf(group_ent), panel(second_frac)))
        .id();
    reconcile_node(
        commands,
        &split.second,
        second_panel,
        ctx,
        registry,
        q,
        existing,
    );

    group_ent
}

fn reconcile_leaf_node(
    commands: &mut Commands,
    leaf: &DockLeaf,
    parent: Entity,
    ctx: &ReconcileContext,
    registry: &WindowRegistry,
    q: &mut ReconcileTreeQueries,
    existing: &mut ExistingSubtrees,
) -> Entity {
    let area_id = leaf_area_id(leaf);

    let reused = existing.leaves.remove(&leaf.id).and_then(|area_ent| {
        let Ok((_, _, child_of_opt, children_opt, mut area, _)) = q.leaf_areas.get_mut(area_ent)
        else {
            return None;
        };
        let content_ent = children_opt?.iter().find(|&c| q.content_q.contains(c))?;
        if area.id != area_id {
            area.id = area_id.clone();
        }
        if area.style != leaf.style {
            area.style = leaf.style;
        }
        if area.persistent != leaf.persistent {
            area.persistent = leaf.persistent;
        }
        if child_of_opt.map(|c| c.parent()) != Some(parent) {
            commands.entity(area_ent).insert(ChildOf(parent));
        }
        existing.reused_leaves.insert(area_ent);
        Some((area_ent, content_ent))
    });

    let (area_ent, content_ent, is_reused_area) = match reused {
        Some((a, c)) => (a, c, true),
        None => {
            let area = DockArea::new(area_id)
                .with_style(leaf.style)
                .with_persistent(leaf.persistent);
            let (a, c) = spawn_dock_area(
                commands,
                Some(parent),
                area,
                &ctx.style,
                ctx.text_font.clone(),
                ctx.icon_font.clone(),
            );
            commands
                .entity(a)
                .insert((NodeBinding(leaf.id), LeafBinding(leaf.id)));
            (a, c, false)
        }
    };

    let active_win_ent =
        reconcile_leaf_windows_in_place(commands, content_ent, leaf, ctx, registry, q, existing);

    if is_reused_area {
        if let Ok((_, _, _, _, _, mut active)) = q.leaf_areas.get_mut(area_ent)
            && active.0 != active_win_ent
        {
            active.0 = active_win_ent;
        }
    } else {
        commands
            .entity(area_ent)
            .insert(ActiveDockWindow(active_win_ent));
    }

    area_ent
}

fn reconcile_leaf_windows_in_place(
    commands: &mut Commands,
    content_ent: Entity,
    leaf: &DockLeaf,
    ctx: &ReconcileContext,
    registry: &WindowRegistry,
    q: &mut ReconcileTreeQueries,
    existing: &mut ExistingSubtrees,
) -> Option<Entity> {
    let existing_content_children: Vec<Entity> = q
        .content_q
        .get(content_ent)
        .ok()
        .flatten()
        .map(|ch| ch.iter().collect())
        .unwrap_or_default();

    let active_idx = resolved_leaf_active_index(leaf);
    let mut active_win_ent = None;
    let mut desired_order = Vec::with_capacity(leaf.tabs.len());
    let mut already_here_set = HashSet::new();
    let mut appended_or_moved = Vec::new();

    for (idx, tab) in leaf.tabs.iter().enumerate() {
        let is_active = Some(idx) == active_idx;
        let key = (tab.tab_id, tab.window_id.clone());
        let (title, icon) = resolve_tab_window_meta(registry, &tab.window_id, &tab.title);

        let win_ent = if let Some(win_ent) = existing.windows.remove(&key)
            && let Ok((_, _, child_of_opt, mut win, mut layout, mut vis, mut transform)) =
                q.live_windows_q.get_mut(win_ent)
        {
            existing.reused_windows.insert(win_ent);
            if win.window_id != tab.window_id {
                win.window_id = tab.window_id.clone();
            }
            if win.name != title {
                win.name = title;
            }
            if win.icon != icon {
                win.icon = icon;
            }
            if !win.closeable {
                win.closeable = true;
            }
            apply_window_active_state(&mut layout, &mut vis, &mut transform, is_active);

            if child_of_opt.map(|c| c.parent()) == Some(content_ent) {
                already_here_set.insert(win_ent);
            } else {
                commands.entity(win_ent).insert(ChildOf(content_ent));
                appended_or_moved.push(win_ent);
            }
            win_ent
        } else {
            let win_ent = spawn_tab_window_entity(
                commands,
                content_ent,
                tab.tab_id,
                &tab.window_id,
                &tab.title,
                is_active,
                registry,
                &ctx.text_font,
            );
            appended_or_moved.push(win_ent);
            win_ent
        };

        if is_active {
            active_win_ent = Some(win_ent);
        }
        desired_order.push(win_ent);
    }

    if !desired_order.is_empty() {
        let mut natural_order: Vec<Entity> = existing_content_children
            .into_iter()
            .filter(|e| already_here_set.contains(e))
            .collect();
        natural_order.extend(appended_or_moved);
        if natural_order != desired_order {
            commands
                .entity(content_ent)
                .insert_children(0, &desired_order);
        }
    }

    active_win_ent
}

fn reconcile_tree_system(
    mut commands: Commands,
    tree: Res<DockTree>,
    res: ReconcileVisualResources,
    mut state: ResMut<ReconciledTreeState>,
    mut q: ReconcileTreeQueries,
) {
    let Ok((host_ent, host_children)) = q.hosts.single() else {
        return;
    };

    let host_changed = state.last_host != Some(host_ent);
    let host_swapped = state.last_host.is_some() && host_changed;
    if !host_changed
        && !state.workspace_switched
        && state.last_tree.is_some()
        && !tree.is_changed()
    {
        return;
    }

    let diff = if host_changed || state.workspace_switched {
        TreeDiffKind::Structural
    } else if let Some(prev) = &state.last_tree {
        classify_diff(prev.root(), tree.root())
    } else {
        TreeDiffKind::Structural
    };

    match diff {
        TreeDiffKind::Identical => return,
        TreeDiffKind::FractionsOnly => {
            apply_fractions_in_place(tree.root(), &q.group_bindings, &mut q.panels);
            apply_active_in_place(
                tree.root(),
                &mut q.leaf_areas,
                &q.content_q,
                &mut q.live_windows_q,
            );
            state.last_tree = Some(tree.clone());
            return;
        }
        TreeDiffKind::ActiveOnly => {
            apply_active_in_place(
                tree.root(),
                &mut q.leaf_areas,
                &q.content_q,
                &mut q.live_windows_q,
            );
            state.last_tree = Some(tree.clone());
            return;
        }
        TreeDiffKind::Structural => {}
    }

    let style = res.style.as_deref().cloned().unwrap_or_default();
    let (text_font, icon_font) = resolve_fonts(res.theme.as_deref(), res.icon_font.as_deref());

    if state.workspace_switched || host_swapped {
        state.workspace_switched = false;
        if let Some(children) = host_children {
            for &c in children {
                if let Ok(mut cmd) = commands.get_entity(c) {
                    cmd.try_despawn();
                }
            }
        }
        spawn_node_subtree(
            &mut commands,
            host_ent,
            tree.root(),
            &res.registry,
            &style,
            &text_font,
            &icon_font,
            &mut HashMap::new(),
        );
        state.last_tree = Some(tree.clone());
        state.last_host = Some(host_ent);
        return;
    }

    state.workspace_switched = false;
    let ctx = ReconcileContext {
        style,
        text_font,
        icon_font,
    };

    let mut existing = ExistingSubtrees::default();
    for (ent, binding, _, _, _) in q.group_bindings.iter() {
        existing.all_groups.push(ent);
        existing.groups.insert(binding.0, ent);
    }
    for (ent, binding, _, _, _, _) in q.leaf_areas.iter() {
        existing.all_leaves.push(ent);
        existing.leaves.insert(binding.0, ent);
    }
    for (ent, tab_binding, _, win, _, _, _) in q.live_windows_q.iter() {
        existing.all_windows.push(ent);
        existing
            .windows
            .insert((tab_binding.0, win.window_id.clone()), ent);
    }

    reconcile_node(
        &mut commands,
        tree.root(),
        host_ent,
        &ctx,
        &res.registry,
        &mut q,
        &mut existing,
    );

    for win_ent in existing.all_windows {
        if !existing.reused_windows.contains(&win_ent)
            && let Ok(mut cmd) = commands.get_entity(win_ent)
        {
            cmd.try_despawn();
        }
    }
    for leaf_ent in existing.all_leaves {
        if !existing.reused_leaves.contains(&leaf_ent)
            && let Ok(mut cmd) = commands.get_entity(leaf_ent)
        {
            cmd.try_despawn();
        }
    }
    for group_ent in existing.all_groups {
        if !existing.reused_groups.contains(&group_ent)
            && let Ok(mut cmd) = commands.get_entity(group_ent)
        {
            cmd.try_despawn();
        }
    }

    state.last_tree = Some(tree.clone());
    state.last_host = Some(host_ent);
}

/// Rebuilds live window contents when a [`WindowRegistry`] entry is replaced or newly registered
/// after initial tree spawn.
fn rebuild_dirty_registry_windows(
    mut commands: Commands,
    mut registry: ResMut<WindowRegistry>,
    pending_q: Query<(), With<PendingRegistryPopulate>>,
    children_q: Query<&Children>,
    mut windows_q: Query<(Entity, &mut DockWindow, Option<&PendingRegistryPopulate>)>,
) {
    // Check dirtiness via read-only `has_dirty()` / `is_changed()` before invoking
    // `take_dirty_ids(&mut self)` so idle frames never mark `WindowRegistry` mutated.
    let has_dirty = registry.has_dirty();
    if !has_dirty && !(registry.is_changed() && !pending_q.is_empty()) {
        return;
    }

    let dirty_ids = if has_dirty {
        registry.take_dirty_ids()
    } else {
        Vec::new()
    };

    for (win_ent, mut win, pending) in windows_q.iter_mut() {
        let should_rebuild = dirty_ids.iter().any(|id| id == &win.window_id)
            || (pending.is_some() && registry.get(&win.window_id).is_some());

        if !should_rebuild {
            continue;
        }

        if let Some(desc) = registry.get(&win.window_id) {
            if win.icon != desc.icon {
                win.icon = desc.icon.clone();
            }
            if win.name.is_empty() && !desc.name.is_empty() {
                win.name = desc.name.clone();
            }
        }
        if pending.is_some() {
            commands.entity(win_ent).remove::<PendingRegistryPopulate>();
        }
        if let Ok(children) = children_q.get(win_ent) {
            for &c in children {
                if let Ok(mut cmd) = commands.get_entity(c) {
                    cmd.try_despawn();
                }
            }
        }
        registry.build_into(&win.window_id, &mut commands, win_ent);
    }
}

fn on_workspace_changed_mark_reset(
    _trigger: On<super::workspace::DockWorkspaceChanged>,
    mut state: ResMut<ReconciledTreeState>,
) {
    state.workspace_switched = true;
    state.last_tree = None;
}

/// Plugin synchronizing [`DockTree`] with live UI entities under [`DockTreeHost`].
pub struct ReconcilePlugin;

impl Plugin for ReconcilePlugin {
    fn build(&self, app: &mut App) {
        super::configure_panel_pipeline(app);
        app.init_resource::<ReconciledTreeState>()
            .init_resource::<DockTree>()
            .init_resource::<WindowRegistry>()
            .register_type::<DockTreeHost>()
            .register_type::<NodeBinding>()
            .register_type::<LeafBinding>()
            .register_type::<TabBinding>()
            .add_observer(on_workspace_changed_mark_reset)
            .add_systems(
                Update,
                (reconcile_tree_system, rebuild_dirty_registry_windows)
                    .chain()
                    .in_set(super::PanelSystemSet::Reconcile),
            );
    }
}
