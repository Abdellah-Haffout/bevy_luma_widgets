//! Serializable snapshots of area and dock-tree layout state.

use bevy::prelude::*;
use serde::{Deserialize, Serialize};

use super::area::{ActiveDockWindow, DockArea, DockAreaStyle, DockWindow};
use super::tree::{DockLeaf, DockNode, DockTree};

/// Serializable state of a single docked area and its open tabs.
#[derive(Clone, Debug, Default, PartialEq, Eq, Reflect, Serialize, Deserialize)]
pub struct AreaState {
    pub id: String,
    pub style: DockAreaStyle,
    pub persistent: bool,
    pub tabs: Vec<String>,
    pub active_tab: Option<String>,
}

/// Collection of serialized [`AreaState`] snapshots capturing a workspace layout.
#[derive(Resource, Clone, Debug, Default, PartialEq, Eq, Reflect, Serialize, Deserialize)]
#[reflect(Resource)]
pub struct LayoutState {
    pub areas: Vec<AreaState>,
}

impl LayoutState {
    /// Captures area snapshots directly from a [`DockTree`].
    pub fn from_tree(tree: &DockTree) -> Self {
        let mut areas = Vec::new();
        collect_leaf_states(tree.root(), &mut areas);
        Self { areas }
    }
}

fn collect_leaf_states(node: &DockNode, out: &mut Vec<AreaState>) {
    match node {
        DockNode::Leaf(leaf) => out.push(area_state_from_leaf(leaf)),
        DockNode::Split(split) => {
            collect_leaf_states(&split.first, out);
            collect_leaf_states(&split.second, out);
        }
    }
}

fn area_state_from_leaf(leaf: &DockLeaf) -> AreaState {
    AreaState {
        id: if leaf.name.is_empty() {
            format!("area_{}", leaf.id.0)
        } else {
            leaf.name.clone()
        },
        style: leaf.style,
        persistent: leaf.persistent,
        tabs: leaf.tabs.iter().map(|t| t.window_id.clone()).collect(),
        active_tab: leaf.active_tab().map(|t| t.window_id.clone()),
    }
}

/// Snapshots the current live ECS [`DockArea`] hierarchy into [`LayoutState`].
pub fn capture_layout_state(
    areas: &Query<(Entity, &DockArea, Option<&ActiveDockWindow>, Option<&Children>)>,
    windows: &Query<&DockWindow>,
    children_q: &Query<&Children>,
    content_q: &Query<(), With<super::area::DockTabContent>>,
) -> LayoutState {
    let mut state = LayoutState::default();
    for (_, area, active_opt, area_children) in areas.iter() {
        let mut tabs = Vec::new();
        let active_ent = active_opt.and_then(|a| a.0);
        let mut active_tab = None;

        if let Some(children) = area_children {
            for &child in children {
                if content_q.contains(child) {
                    if let Ok(content_children) = children_q.get(child) {
                        for &win_ent in content_children {
                            if let Ok(win) = windows.get(win_ent) {
                                tabs.push(win.window_id.clone());
                                if Some(win_ent) == active_ent {
                                    active_tab = Some(win.window_id.clone());
                                }
                            }
                        }
                    }
                }
            }
        }

        state.areas.push(AreaState {
            id: area.id.clone(),
            style: area.style,
            persistent: area.persistent,
            tabs,
            active_tab,
        });
    }
    state
}
