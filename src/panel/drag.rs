//! Drag-and-drop tab reordering, cross-area docking, edge splitting ghost, and drop overlays.

use bevy::ecs::system::SystemParam;
use bevy::picking::pointer::PointerButton;
use bevy::prelude::*;
use bevy_luma::prelude::*;

use crate::helpers::find_ancestor_or_self;

use super::area::{
    ActiveDockWindow, DockArea, DockAreaStyle, DockSidebarIcon, DockTab, DockTabBar,
    DockTabCloseButton, DockTabContent, DockTabGrip, DockTabRow, DockWindow, IconFontHandle,
};
use super::reconcile::{LeafBinding, TabBinding};
use super::style::{DockStyle, node_screen_rect, primary_window_size, resolve_fonts};
use super::tabs::{count_area_windows, find_area_content_entity, find_area_tab_row};
use super::tree::{DockTree, Edge, NodeId, TabId};

/// Drop target edge within a [`DockArea`].
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, Reflect)]
pub enum DropEdge {
    Center,
    Left,
    Right,
    Top,
    Bottom,
}

impl DropEdge {
    /// Converts this drop zone to a tree [`Edge`] when splitting, or `None` for center docking.
    pub fn to_split_edge(self) -> Option<Edge> {
        match self {
            DropEdge::Center => None,
            DropEdge::Left => Some(Edge::Left),
            DropEdge::Right => Some(Edge::Right),
            DropEdge::Top => Some(Edge::Top),
            DropEdge::Bottom => Some(Edge::Bottom),
        }
    }
}

/// Resolved drop target under the cursor during a dock drag operation.
#[derive(Clone, Debug, PartialEq, Eq)]
pub enum DropTarget {
    /// Dock inside a [`DockArea`] tab list (`Center`) or split along an outer edge.
    Area { area_entity: Entity, edge: DropEdge },
    /// Insert or reorder directly inside an area's horizontal [`DockTabRow`].
    TabBar { area_entity: Entity, insert_index: usize },
}

/// Live metadata for an active drag operation.
#[derive(Clone, Debug)]
pub struct ActiveDockDrag {
    pub source_area: Entity,
    pub source_leaf: Option<NodeId>,
    pub source_tab_index: Option<usize>,
    pub source_tab_id: Option<TabId>,
    pub window_entity: Entity,
    pub window_id: String,
    pub title: String,
    pub icon: Option<String>,
    pub cursor_pos: Vec2,
    pub drop_target: Option<DropTarget>,
    overlay_root: Option<Entity>,
    ghost_entity: Option<Entity>,
    drop_overlay_entity: Option<Entity>,
    tab_indicator_entity: Option<Entity>,
}

/// Global drag-and-drop state resource for dock tabs and sidebars.
#[derive(Resource, Default, Clone, Debug)]
pub struct DockDragState {
    pub active: Option<ActiveDockDrag>,
}

impl DockDragState {
    /// Returns `true` when a dock tab drag is currently in progress.
    pub fn is_dragging(&self) -> bool {
        self.active.is_some()
    }
}

#[derive(Component, Clone, Copy, Debug, Default)]
struct DockDragGhost;

#[derive(Component, Clone, Copy, Debug, Default)]
struct DockDragOverlay;

#[derive(Component, Clone, Copy, Debug, Default)]
struct DockTabDropIndicator;

fn cleanup_drag_visuals(commands: &mut Commands, drag: &mut ActiveDockDrag) {
    if let Some(root) = drag.overlay_root.take() {
        if let Ok(mut cmd) = commands.get_entity(root) {
            cmd.try_despawn();
        }
    }
    drag.ghost_entity = None;
    drag.drop_overlay_entity = None;
    drag.tab_indicator_entity = None;
}

#[derive(SystemParam)]
struct DragSourceQueries<'w, 's> {
    parents: Query<'w, 's, &'static ChildOf>,
    children_q: Query<'w, 's, &'static Children>,
    tabs_q: Query<'w, 's, &'static DockTab>,
    close_q: Query<'w, 's, (), With<DockTabCloseButton>>,
    sidebar_icons_q: Query<'w, 's, &'static DockSidebarIcon>,
    grips_q: Query<'w, 's, &'static DockTabGrip>,
    content_q: Query<'w, 's, (), With<DockTabContent>>,
    active_q: Query<'w, 's, &'static ActiveDockWindow>,
    window_meta_q: Query<'w, 's, (&'static DockWindow, Option<&'static TabBinding>)>,
    leaf_bindings: Query<'w, 's, &'static LeafBinding>,
}

fn on_dock_drag_start(
    mut trigger: On<PointerDragStart>,
    mut commands: Commands,
    style: Option<Res<DockStyle>>,
    theme: Option<Res<Theme>>,
    icon_font_res: Option<Res<IconFontHandle>>,
    mut drag_state: ResMut<DockDragState>,
    dock_tree: Option<Res<DockTree>>,
    windows: Query<&Window>,
    q: DragSourceQueries,
) {
    if trigger.event().button != PointerButton::Primary {
        return;
    }
    // Never initiate tab drag when pressing directly inside a close button.
    if find_ancestor_or_self(trigger.entity, &q.close_q, &q.parents).is_some() {
        return;
    }

    let resolved: Option<(Entity, Entity)> =
        if let Some(tab_ent) = find_ancestor_or_self(trigger.entity, &q.tabs_q, &q.parents) {
            q.tabs_q
                .get(tab_ent)
                .ok()
                .map(|t| (t.area_entity, t.window_entity))
        } else if let Some(sb_ent) =
            find_ancestor_or_self(trigger.entity, &q.sidebar_icons_q, &q.parents)
        {
            q.sidebar_icons_q
                .get(sb_ent)
                .ok()
                .map(|s| (s.area_entity, s.window_entity))
        } else if let Some(grip_ent) = find_ancestor_or_self(trigger.entity, &q.grips_q, &q.parents)
        {
            q.grips_q.get(grip_ent).ok().and_then(|g| {
                let win_ent = q
                    .active_q
                    .get(g.area_entity)
                    .ok()
                    .and_then(|a| a.0)
                    .or_else(|| {
                        find_area_content_entity(g.area_entity, &q.children_q, &q.content_q)
                            .and_then(|ce| {
                                q.children_q
                                    .get(ce)
                                    .ok()
                                    .and_then(|c| c.iter().find(|&e| q.window_meta_q.contains(e)))
                            })
                    })?;
                Some((g.area_entity, win_ent))
            })
        } else {
            None
        };

    let Some((source_area, window_entity)) = resolved else {
        return;
    };
    let Ok((win_meta, tab_binding)) = q.window_meta_q.get(window_entity) else {
        return;
    };

    // Clean up any leftover overlay before starting a new drag.
    if let Some(mut prev) = drag_state.active.take() {
        cleanup_drag_visuals(&mut commands, &mut prev);
    }

    trigger.propagate(false);

    let source_leaf = q.leaf_bindings.get(source_area).ok().map(|b| b.0);
    let source_tab_id = tab_binding.map(|b| b.0);
    let source_tab_index = if let (Some(tree), Some(tid)) = (dock_tree.as_deref(), source_tab_id) {
        tree.find_tab(tid).map(|(_, idx)| idx)
    } else {
        find_area_content_entity(source_area, &q.children_q, &q.content_q).and_then(|ce| {
            q.children_q.get(ce).ok().and_then(|children| {
                children
                    .iter()
                    .filter(|&c| q.window_meta_q.contains(c))
                    .position(|c| c == window_entity)
            })
        })
    };

    let style = style.as_deref().cloned().unwrap_or_default();
    let (text_font, icon_font) = resolve_fonts(theme.as_deref(), icon_font_res.as_deref());
    let win_size = primary_window_size(&windows);
    let cursor_pos = trigger.pointer.position;
    let ghost_left = (cursor_pos.x + 14.0).clamp(4.0, (win_size.x - 160.0).max(4.0));
    let ghost_top = (cursor_pos.y + 14.0).clamp(4.0, (win_size.y - 36.0).max(4.0));

    let overlay_root = commands
        .spawn((
            URootUi::screen(),
            UZIndex::Global(9998),
            UNode {
                width: UVal::Percent(1.0),
                height: UVal::Percent(1.0),
                background_color: Color::NONE,
                ..default()
            },
            ULayout {
                display: UDisplay::Flex,
                ..default()
            },
        ))
        .id();

    let drop_overlay_entity = commands
        .spawn((
            ChildOf(overlay_root),
            DockDragOverlay,
            UNode {
                width: UVal::Px(0.0),
                height: UVal::Px(0.0),
                background_color: style.drop_overlay_fill,
                border_radius: UCornerRadius::all(4.0),
                ..default()
            },
            USelf {
                position_type: UPositionType::Absolute,
                left: UVal::Px(0.0),
                top: UVal::Px(0.0),
                ..default()
            },
            UBorder {
                color: style.drop_overlay_border,
                width: 2.0,
                radius: UCornerRadius::all(4.0),
                offset: 0.0,
            },
            Visibility::Hidden,
        ))
        .id();

    let tab_indicator_entity = commands
        .spawn((
            ChildOf(overlay_root),
            DockTabDropIndicator,
            UNode {
                width: UVal::Px(3.0),
                height: UVal::Px(style.tab_bar_height - 6.0),
                background_color: style.drop_tab_indicator_color,
                border_radius: UCornerRadius::all(2.0),
                ..default()
            },
            USelf {
                position_type: UPositionType::Absolute,
                left: UVal::Px(0.0),
                top: UVal::Px(0.0),
                ..default()
            },
            Visibility::Hidden,
        ))
        .id();

    let ghost_entity = commands
        .spawn((
            ChildOf(overlay_root),
            DockDragGhost,
            UNode {
                width: UVal::Auto,
                height: UVal::Px(28.0),
                padding: USides::axes(10.0, 4.0),
                background_color: style.ghost_bg,
                border_radius: UCornerRadius::all(6.0),
                ..default()
            },
            USelf {
                position_type: UPositionType::Absolute,
                left: UVal::Px(ghost_left),
                top: UVal::Px(ghost_top),
                ..default()
            },
            UBorder {
                color: style.ghost_border,
                width: 1.5,
                radius: UCornerRadius::all(6.0),
                offset: 0.0,
            },
            UShadow::drop(Color::srgba(0.0, 0.0, 0.0, 0.55), Vec2::new(0.0, 6.0), 12.0),
            ULayout {
                display: UDisplay::Flex,
                flex_direction: UFlexDirection::Row,
                align_items: UAlignItems::Center,
                gap: 6.0,
                ..default()
            },
        ))
        .id();

    if let Some(ic) = &win_meta.icon {
        commands.spawn((
            ChildOf(ghost_entity),
            UNode::default(),
            UText {
                text: ic.clone(),
                font_size: 13.0,
                font: icon_font,
                color: style.accent_color,
                ..default()
            },
        ));
    }

    commands.spawn((
        ChildOf(ghost_entity),
        UNode::default(),
        UText {
            text: win_meta.name.clone(),
            font_size: 12.5,
            font: text_font,
            color: style.ghost_text,
            ..default()
        },
    ));

    drag_state.active = Some(ActiveDockDrag {
        source_area,
        source_leaf,
        source_tab_index,
        source_tab_id,
        window_entity,
        window_id: win_meta.window_id.clone(),
        title: win_meta.name.clone(),
        icon: win_meta.icon.clone(),
        cursor_pos,
        drop_target: None,
        overlay_root: Some(overlay_root),
        ghost_entity: Some(ghost_entity),
        drop_overlay_entity: Some(drop_overlay_entity),
        tab_indicator_entity: Some(tab_indicator_entity),
    });
}

#[allow(clippy::too_many_arguments)]
fn on_dock_drag_move(
    trigger: On<PointerDrag>,
    mut drag_state: ResMut<DockDragState>,
    windows: Query<&Window>,
    cameras: Query<(&Camera, &GlobalTransform)>,
    children_q: Query<&Children>,
    areas_q: Query<(Entity, &DockArea, &GlobalTransform, &ComputedSize, &Visibility)>,
    tab_bar_q: Query<(), With<DockTabBar>>,
    tab_row_q: Query<(), With<DockTabRow>>,
    content_q: Query<(), With<DockTabContent>>,
    window_q: Query<&DockWindow>,
    tabs_q: Query<(), With<DockTab>>,
    rects_q: Query<(&GlobalTransform, &ComputedSize)>,
    mut overlay_nodes: Query<
        (&mut UNode, &mut USelf, &mut Visibility),
        (
            Or<(With<DockDragGhost>, With<DockDragOverlay>, With<DockTabDropIndicator>)>,
            Without<DockArea>,
        ),
    >,
) {
    if trigger.event().button != PointerButton::Primary {
        return;
    }
    let Some(active) = drag_state.active.as_mut() else {
        return;
    };

    let pos = trigger.pointer.position;
    active.cursor_pos = pos;
    let win_size = primary_window_size(&windows);

    if let Some(ghost_ent) = active.ghost_entity {
        if let Ok((_, mut g_self, _)) = overlay_nodes.get_mut(ghost_ent) {
            g_self.left = UVal::Px((pos.x + 14.0).clamp(4.0, (win_size.x - 160.0).max(4.0)));
            g_self.top = UVal::Px((pos.y + 14.0).clamp(4.0, (win_size.y - 36.0).max(4.0)));
        }
    }

    // 1. Check tab bars first for precise tab-reordering / tab-bar drop.
    let mut found_target: Option<DropTarget> = None;
    let mut overlay_rect: Option<Rect> = None;
    let mut indicator_pos: Option<(f32, f32, f32)> = None;

    for (area_ent, area, _, _, vis) in areas_q.iter() {
        if *vis == Visibility::Hidden || area.style != DockAreaStyle::TabBar {
            continue;
        }
        let Ok(area_children) = children_q.get(area_ent) else {
            continue;
        };
        let Some(bar_ent) = area_children.iter().find(|&c| tab_bar_q.contains(c)) else {
            continue;
        };
        let Ok((bar_gt, bar_comp)) = rects_q.get(bar_ent) else {
            continue;
        };
        if bar_comp.width <= 1.0 || bar_comp.height <= 1.0 {
            continue;
        }
        let bar_rect = node_screen_rect(bar_gt, bar_comp, &windows, &cameras);
        if !bar_rect.contains(pos) {
            continue;
        }

        let Some(row_ent) = find_area_tab_row(area_ent, &children_q, &tab_bar_q, &tab_row_q)
        else {
            continue;
        };
        let row_rect = rects_q
            .get(row_ent)
            .map(|(gt, comp)| node_screen_rect(gt, comp, &windows, &cameras))
            .unwrap_or(bar_rect);

        let mut tab_count = 0usize;
        let mut insert_index = None;
        let mut indicator_x = row_rect.min.x + 4.0;
        let mut last_max_x: Option<f32> = None;

        if let Ok(row_children) = children_q.get(row_ent) {
            for &child_ent in row_children {
                if tabs_q.contains(child_ent) {
                    if let Ok((c_gt, c_comp)) = rects_q.get(child_ent) {
                        let t_rect = node_screen_rect(c_gt, c_comp, &windows, &cameras);
                        if insert_index.is_none() && pos.x < t_rect.center().x {
                            insert_index = Some(tab_count);
                            indicator_x = t_rect.min.x - 1.5;
                        }
                        last_max_x = Some(t_rect.max.x);
                        tab_count += 1;
                    }
                }
            }
        }

        let insert_index = insert_index.unwrap_or(tab_count);
        if insert_index == tab_count {
            if let Some(max_x) = last_max_x {
                indicator_x = max_x + 1.0;
            }
        }

        // Dropping adjacent to the tab's exact current slot inside the same area is a no-op.
        let is_noop_self = area_ent == active.source_area
            && active
                .source_tab_index
                .is_some_and(|src| insert_index == src || insert_index == src + 1);

        if !is_noop_self {
            found_target = Some(DropTarget::TabBar {
                area_entity: area_ent,
                insert_index,
            });
            indicator_pos = Some((
                indicator_x,
                bar_rect.min.y + 3.0,
                (bar_rect.height() - 6.0).max(16.0),
            ));
        }
        break;
    }

    // 2. Otherwise check full DockArea bounds for center dock or 25% outer split edges.
    if found_target.is_none() {
        for (area_ent, _area, area_gt, area_comp, vis) in areas_q.iter() {
            if *vis == Visibility::Hidden || area_comp.width <= 1.0 || area_comp.height <= 1.0 {
                continue;
            }
            let rect = node_screen_rect(area_gt, area_comp, &windows, &cameras);
            if !rect.contains(pos) {
                continue;
            }

            let w = rect.width().max(1.0);
            let h = rect.height().max(1.0);
            let nx = ((pos.x - rect.min.x) / w).clamp(0.0, 1.0);
            let ny = ((pos.y - rect.min.y) / h).clamp(0.0, 1.0);

            let margins = [
                (nx, DropEdge::Left),
                (1.0 - nx, DropEdge::Right),
                (ny, DropEdge::Top),
                (1.0 - ny, DropEdge::Bottom),
            ];
            let (min_dist, best_edge) = margins
                .into_iter()
                .min_by(|a, b| a.0.total_cmp(&b.0))
                .unwrap();

            let edge = if min_dist < 0.25 {
                best_edge
            } else {
                DropEdge::Center
            };

            // Disallow no-op drop on own area center or splitting an area containing only this single tab.
            let area_tab_count =
                count_area_windows(area_ent, &children_q, &content_q, &window_q);
            if area_ent == active.source_area && (edge == DropEdge::Center || area_tab_count <= 1)
            {
                break;
            }

            found_target = Some(DropTarget::Area {
                area_entity: area_ent,
                edge,
            });
            overlay_rect = Some(match edge {
                DropEdge::Center => rect,
                DropEdge::Left => Rect::from_corners(rect.min, rect.min + Vec2::new(w * 0.5, h)),
                DropEdge::Right => Rect::from_corners(
                    Vec2::new(rect.min.x + w * 0.5, rect.min.y),
                    Vec2::new(rect.max.x, rect.max.y),
                ),
                DropEdge::Top => Rect::from_corners(rect.min, rect.min + Vec2::new(w, h * 0.5)),
                DropEdge::Bottom => Rect::from_corners(
                    Vec2::new(rect.min.x, rect.min.y + h * 0.5),
                    Vec2::new(rect.max.x, rect.max.y),
                ),
            });
            break;
        }
    }

    active.drop_target = found_target;

    if let Some(overlay_ent) = active.drop_overlay_entity {
        if let Ok((mut node, mut uself, mut vis)) = overlay_nodes.get_mut(overlay_ent) {
            if let Some(r) = overlay_rect {
                let left = UVal::Px(r.min.x);
                let top = UVal::Px(r.min.y);
                let width = UVal::Px(r.width());
                let height = UVal::Px(r.height());
                if uself.left != left {
                    uself.left = left;
                }
                if uself.top != top {
                    uself.top = top;
                }
                if node.width != width {
                    node.width = width;
                }
                if node.height != height {
                    node.height = height;
                }
                if *vis != Visibility::Inherited {
                    *vis = Visibility::Inherited;
                }
            } else if *vis != Visibility::Hidden {
                *vis = Visibility::Hidden;
            }
        }
    }

    if let Some(ind_ent) = active.tab_indicator_entity {
        if let Ok((mut node, mut uself, mut vis)) = overlay_nodes.get_mut(ind_ent) {
            if let Some((ix, iy, ih)) = indicator_pos {
                let left = UVal::Px(ix);
                let top = UVal::Px(iy);
                let height = UVal::Px(ih);
                if uself.left != left {
                    uself.left = left;
                }
                if uself.top != top {
                    uself.top = top;
                }
                if node.height != height {
                    node.height = height;
                }
                if *vis != Visibility::Inherited {
                    *vis = Visibility::Inherited;
                }
            } else if *vis != Visibility::Hidden {
                *vis = Visibility::Hidden;
            }
        }
    }
}

fn on_dock_drag_end(
    _trigger: On<PointerDragEnd>,
    mut commands: Commands,
    mut drag_state: ResMut<DockDragState>,
    leaf_bindings: Query<&LeafBinding>,
    children_q: Query<&Children>,
    content_q: Query<(), With<DockTabContent>>,
    mut active_q: Query<&mut ActiveDockWindow>,
    mut dock_tree: Option<ResMut<DockTree>>,
) {
    let Some(mut drag) = drag_state.active.take() else {
        return;
    };
    cleanup_drag_visuals(&mut commands, &mut drag);

    let Some(target) = drag.drop_target.take() else {
        return;
    };

    if let (Some(tree), Some(src_leaf), Some(src_idx)) =
        (dock_tree.as_deref_mut(), drag.source_leaf, drag.source_tab_index)
    {
        match target {
            DropTarget::TabBar {
                area_entity,
                insert_index,
            } => {
                if let Ok(dst_leaf) = leaf_bindings.get(area_entity) {
                    tree.move_tab(src_leaf, src_idx, dst_leaf.0, insert_index);
                }
            }
            DropTarget::Area { area_entity, edge } => {
                if let Ok(dst_leaf) = leaf_bindings.get(area_entity) {
                    match edge.to_split_edge() {
                        None => {
                            let dst_len = tree
                                .find_leaf(dst_leaf.0)
                                .map(|l| l.tabs.len())
                                .unwrap_or(0);
                            tree.move_tab(src_leaf, src_idx, dst_leaf.0, dst_len);
                        }
                        Some(split_edge) => {
                            tree.move_tab_to_new_split(src_leaf, src_idx, dst_leaf.0, split_edge, 0.5);
                        }
                    }
                }
            }
        }
        return;
    }

    // Standalone (non-DockTree) area reparenting support when dropping on center/tab-bar.
    let target_area = match target {
        DropTarget::TabBar { area_entity, .. } => Some(area_entity),
        DropTarget::Area {
            area_entity,
            edge: DropEdge::Center,
        } => Some(area_entity),
        _ => None,
    };
    if let Some(dst_area) = target_area {
        if dst_area != drag.source_area {
            if let Some(dst_content) = find_area_content_entity(dst_area, &children_q, &content_q) {
                commands.entity(drag.window_entity).insert(ChildOf(dst_content));
                if let Ok(mut dst_active) = active_q.get_mut(dst_area) {
                    dst_active.0 = Some(drag.window_entity);
                }
                if let Ok(mut src_active) = active_q.get_mut(drag.source_area) {
                    if src_active.0 == Some(drag.window_entity) {
                        src_active.0 = None;
                    }
                }
            }
        }
    }
}

fn cancel_drag_on_escape(
    mut commands: Commands,
    keyboard: Option<Res<ButtonInput<KeyCode>>>,
    mut drag_state: ResMut<DockDragState>,
) {
    if drag_state.is_dragging() && keyboard.is_some_and(|kb| kb.just_pressed(KeyCode::Escape)) {
        if let Some(mut drag) = drag_state.active.take() {
            cleanup_drag_visuals(&mut commands, &mut drag);
        }
    }
}

/// Plugin enabling tab dragging, drop-zone overlays, tab-row insertion indicators, and Escape cancel.
pub struct DockDragPlugin;

impl Plugin for DockDragPlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<DockDragState>()
            .register_type::<DropEdge>()
            .add_systems(Update, cancel_drag_on_escape)
            .add_observer(on_dock_drag_start)
            .add_observer(on_dock_drag_move)
            .add_observer(on_dock_drag_end);
    }
}
