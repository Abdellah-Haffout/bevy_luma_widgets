//! Multi-workspace dock layout management, switching, and persistence.

use std::sync::Arc;

use bevy::prelude::*;
use serde::{Deserialize, Serialize};

use super::tree::DockTree;

/// Blueprint for a named workspace preset (e.g. `"Scene"`, `"Animation"`, `"Debug"`).
#[derive(Clone)]
pub struct WorkspaceDescriptor {
    pub id: String,
    pub name: String,
    pub icon: Option<String>,
    pub default_tree: Arc<dyn Fn() -> DockTree + Send + Sync>,
}

impl std::fmt::Debug for WorkspaceDescriptor {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("WorkspaceDescriptor")
            .field("id", &self.id)
            .field("name", &self.name)
            .field("icon", &self.icon)
            .finish()
    }
}

impl WorkspaceDescriptor {
    /// Creates a new [`WorkspaceDescriptor`].
    pub fn new(
        id: impl Into<String>,
        name: impl Into<String>,
        default_tree: impl Fn() -> DockTree + Send + Sync + 'static,
    ) -> Self {
        Self {
            id: id.into(),
            name: name.into(),
            icon: None,
            default_tree: Arc::new(default_tree),
        }
    }

    /// Attaches an optional Lucide glyph shown on the workspace tab.
    pub fn with_icon(mut self, icon: impl Into<String>) -> Self {
        self.icon = Some(icon.into());
        self
    }
}

/// Live workspace entry stored inside [`WorkspaceRegistry`].
#[derive(Clone)]
pub struct WorkspaceEntry {
    pub id: String,
    pub name: String,
    pub icon: Option<String>,
    pub tree: DockTree,
    pub is_builtin: bool,
}

impl std::fmt::Debug for WorkspaceEntry {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("WorkspaceEntry")
            .field("id", &self.id)
            .field("name", &self.name)
            .field("icon", &self.icon)
            .field("is_builtin", &self.is_builtin)
            .finish()
    }
}

/// Serializable state for a single saved workspace.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct WorkspacePersist {
    pub id: String,
    pub name: String,
    pub icon: Option<String>,
    pub tree: DockTree,
}

/// Serializable state for the entire multi-workspace session.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct WorkspacesPersist {
    pub active_id: Option<String>,
    pub workspaces: Vec<WorkspacePersist>,
}

/// Emitted whenever the active workspace switches in [`WorkspaceRegistry`].
#[derive(Event, Clone, Debug)]
pub struct DockWorkspaceChanged {
    pub previous: Option<String>,
    pub current: String,
}

/// Registry holding all open workspaces, their saved [`DockTree`] snapshots, and the active workspace ID.
#[derive(Resource, Default, Clone, Debug)]
pub struct WorkspaceRegistry {
    workspaces: Vec<WorkspaceEntry>,
    defaults: Vec<WorkspaceDescriptor>,
    active_id: Option<String>,
    next_user_counter: u32,
}

impl WorkspaceRegistry {
    /// Registers a workspace blueprint. If it is the first workspace, it becomes active immediately.
    pub fn register(&mut self, descriptor: WorkspaceDescriptor) {
        if let Some(existing) = self.workspaces.iter_mut().find(|w| w.id == descriptor.id) {
            existing.name = descriptor.name.clone();
            existing.icon = descriptor.icon.clone();
            existing.is_builtin = true;
        } else {
            let tree = (descriptor.default_tree)();
            self.workspaces.push(WorkspaceEntry {
                id: descriptor.id.clone(),
                name: descriptor.name.clone(),
                icon: descriptor.icon.clone(),
                tree,
                is_builtin: true,
            });
        }
        if !self.defaults.iter().any(|d| d.id == descriptor.id) {
            self.defaults.push(descriptor.clone());
        }
        if self.active_id.is_none() {
            self.active_id = Some(descriptor.id);
        }
    }

    /// Creates a new custom workspace cloned from `tree` and returns its generated ID.
    pub fn add_workspace(&mut self, name: impl Into<String>, tree: DockTree) -> String {
        self.next_user_counter += 1;
        let mut id = format!("workspace_{}", self.next_user_counter);
        while self.workspaces.iter().any(|w| w.id == id) {
            self.next_user_counter += 1;
            id = format!("workspace_{}", self.next_user_counter);
        }
        self.workspaces.push(WorkspaceEntry {
            id: id.clone(),
            name: name.into(),
            icon: None,
            tree,
            is_builtin: false,
        });
        if self.active_id.is_none() {
            self.active_id = Some(id.clone());
        }
        id
    }

    /// Generates an unused default workspace name like `"Workspace 2"`.
    pub fn next_default_name(&self) -> String {
        let mut n = self.workspaces.len() + 1;
        loop {
            let candidate = format!("Workspace {n}");
            if !self.workspaces.iter().any(|w| w.name == candidate) {
                return candidate;
            }
            n += 1;
        }
    }

    /// Removes a workspace by ID. Refuses removal if it is the last remaining workspace.
    /// Returns `(removed, new_active_id_if_changed)`.
    pub fn remove_workspace(&mut self, id: &str) -> (bool, Option<String>) {
        if self.workspaces.len() <= 1 {
            return (false, None);
        }
        let Some(pos) = self.workspaces.iter().position(|w| w.id == id) else {
            return (false, None);
        };
        let was_active = self.active_id.as_deref() == Some(id);
        self.workspaces.remove(pos);

        if was_active {
            let next_pos = pos.min(self.workspaces.len() - 1);
            let next_id = self.workspaces.get(next_pos).map(|w| w.id.clone());
            self.active_id = next_id.clone();
            (true, next_id)
        } else {
            (true, None)
        }
    }

    /// Renames a workspace if `new_name` is non-empty after trimming.
    pub fn rename_workspace(&mut self, id: &str, new_name: &str) -> bool {
        let trimmed = new_name.trim();
        if trimmed.is_empty() {
            return false;
        }
        if let Some(ws) = self.workspaces.iter_mut().find(|w| w.id == id) {
            if ws.name != trimmed {
                ws.name = trimmed.to_string();
                return true;
            }
        }
        false
    }

    /// Returns all workspaces in strip display order.
    pub fn workspaces(&self) -> &[WorkspaceEntry] {
        &self.workspaces
    }

    /// Returns the currently active workspace ID.
    pub fn active_id(&self) -> Option<&str> {
        self.active_id.as_deref()
    }

    /// Returns the active [`WorkspaceEntry`], if any.
    pub fn active(&self) -> Option<&WorkspaceEntry> {
        let id = self.active_id.as_deref()?;
        self.workspaces.iter().find(|w| w.id == id)
    }

    /// Persists the live [`DockTree`] into the currently active workspace entry.
    pub fn save_active_tree(&mut self, tree: &DockTree) {
        let Some(active_id) = self.active_id.as_deref() else {
            return;
        };
        if let Some(ws) = self.workspaces.iter_mut().find(|w| w.id == active_id) {
            ws.tree = tree.clone();
        }
    }

    /// Switches the active workspace to `target_id` without emitting ECS events,
    /// saving `live_tree` into the outgoing workspace and writing the incoming tree into `live_tree`.
    pub fn switch_active(
        &mut self,
        target_id: &str,
        live_tree: &mut DockTree,
    ) -> Option<DockWorkspaceChanged> {
        if self.active_id.as_deref() == Some(target_id) {
            return None;
        }
        let target_exists = self.workspaces.iter().any(|w| w.id == target_id);
        if !target_exists {
            return None;
        }

        self.save_active_tree(live_tree);
        let previous = self.active_id.take();
        self.active_id = Some(target_id.to_string());

        if let Some(next_ws) = self.workspaces.iter().find(|w| w.id == target_id) {
            *live_tree = next_ws.tree.clone();
        }

        Some(DockWorkspaceChanged {
            previous,
            current: target_id.to_string(),
        })
    }

    /// Activates the current `active_id` directly (e.g. after `remove_workspace` changed `active_id`),
    /// copying its tree into `live_tree`.
    pub fn load_active_into(&self, live_tree: &mut DockTree) -> bool {
        let Some(ws) = self.active() else {
            return false;
        };
        *live_tree = ws.tree.clone();
        true
    }

    /// Resets the currently active built-in workspace back to its registered default tree.
    pub fn reset_active_to_default(&mut self, live_tree: &mut DockTree) -> bool {
        let Some(active_id) = self.active_id.as_deref() else {
            return false;
        };
        let Some(desc) = self.defaults.iter().find(|d| d.id == active_id) else {
            return false;
        };
        let tree = (desc.default_tree)();
        *live_tree = tree.clone();
        if let Some(ws) = self.workspaces.iter_mut().find(|w| w.id == active_id) {
            ws.tree = tree;
        }
        true
    }

    /// Exports all workspaces into a serializable snapshot.
    pub fn to_persist(&self) -> WorkspacesPersist {
        WorkspacesPersist {
            active_id: self.active_id.clone(),
            workspaces: self
                .workspaces
                .iter()
                .map(|w| WorkspacePersist {
                    id: w.id.clone(),
                    name: w.name.clone(),
                    icon: w.icon.clone(),
                    tree: w.tree.clone(),
                })
                .collect(),
        }
    }

    /// Restores workspaces from a previously saved snapshot, falling back cleanly if empty.
    pub fn restore_from_persist(&mut self, persist: WorkspacesPersist, live_tree: Option<&mut DockTree>) {
        if persist.workspaces.is_empty() {
            return;
        }
        self.workspaces = persist
            .workspaces
            .into_iter()
            .map(|p| {
                let is_builtin = self.defaults.iter().any(|d| d.id == p.id);
                WorkspaceEntry {
                    id: p.id,
                    name: p.name,
                    icon: p.icon,
                    tree: p.tree,
                    is_builtin,
                }
            })
            .collect();
        self.active_id = persist
            .active_id
            .filter(|id| self.workspaces.iter().any(|w| w.id == *id))
            .or_else(|| self.workspaces.first().map(|w| w.id.clone()));

        if let Some(tree_out) = live_tree {
            self.load_active_into(tree_out);
        }
    }
}
