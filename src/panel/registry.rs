//! Window descriptor registry for populating and hot-reloading docked panel contents.

use std::collections::{HashMap, HashSet};
use std::sync::Arc;

use bevy::prelude::*;
use bevy::scene::Scene;

/// Builder callback that populates a docked window container entity with Luma UI children.
pub type DockWindowBuildFn = Arc<dyn Fn(&mut Commands, Entity) + Send + Sync>;

/// Metadata and content builder describing a dockable window type.
#[derive(Clone)]
pub struct DockWindowDescriptor {
    /// Unique identifier for this window kind (e.g. `"inspector"`, `"hierarchy"`).
    pub id: String,
    /// Human-readable display title used in tabs and the `+` add menu.
    pub name: String,
    /// Optional Lucide glyph shown in tabs, sidebars, and popup menus.
    pub icon: Option<String>,
    /// Preferred target area identifier when spawning without an explicit target.
    pub default_area: String,
    /// Sort priority inside the `+` add-window menu (smaller comes first).
    pub priority: i32,
    /// Content factory invoked with `(&mut Commands, content_parent_entity)`.
    pub build: DockWindowBuildFn,
}

impl std::fmt::Debug for DockWindowDescriptor {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("DockWindowDescriptor")
            .field("id", &self.id)
            .field("name", &self.name)
            .field("icon", &self.icon)
            .field("default_area", &self.default_area)
            .field("priority", &self.priority)
            .finish()
    }
}

impl DockWindowDescriptor {
    /// Creates a new [`DockWindowDescriptor`] using an entity-spawning closure.
    pub fn new(
        id: impl Into<String>,
        name: impl Into<String>,
        build: impl Fn(&mut Commands, Entity) + Send + Sync + 'static,
    ) -> Self {
        Self {
            id: id.into(),
            name: name.into(),
            icon: None,
            default_area: "main".to_string(),
            priority: 0,
            build: Arc::new(build),
        }
    }

    /// Creates a [`DockWindowDescriptor`] from a declarative `bsn!` [`Scene`] builder closure.
    pub fn from_scene<S, F>(id: impl Into<String>, name: impl Into<String>, scene_fn: F) -> Self
    where
        S: Scene + 'static,
        F: Fn() -> S + Send + Sync + 'static,
    {
        Self::new(id, name, move |commands, container| {
            let spawned = commands.spawn_scene(scene_fn()).id();
            commands.entity(spawned).insert(ChildOf(container));
        })
    }

    /// Sets an icon glyph for this window type.
    pub fn with_icon(mut self, icon: impl Into<String>) -> Self {
        self.icon = Some(icon.into());
        self
    }

    /// Sets the preferred default dock area identifier.
    pub fn with_default_area(mut self, area_id: impl Into<String>) -> Self {
        self.default_area = area_id.into();
        self
    }

    /// Sets ordering priority in the `+` add-window menu (smaller appears higher).
    pub fn with_priority(mut self, priority: i32) -> Self {
        self.priority = priority;
        self
    }
}

/// Global catalog of dockable window kinds available to the dock tree and `+` popup menu.
#[derive(Resource, Default, Clone, Debug)]
pub struct WindowRegistry {
    descriptors: HashMap<String, DockWindowDescriptor>,
    insertion_order: Vec<String>,
    dirty_ids: HashSet<String>,
}

impl WindowRegistry {
    /// Registers a window descriptor (ignored if its `id` is already present).
    pub fn register(&mut self, descriptor: DockWindowDescriptor) {
        if self.descriptors.contains_key(&descriptor.id) {
            return;
        }
        let id = descriptor.id.clone();
        self.descriptors.insert(id.clone(), descriptor);
        self.insertion_order.push(id);
    }

    /// Registers a new descriptor or replaces an existing one, queueing open instances for live rebuild.
    pub fn register_or_replace(&mut self, descriptor: DockWindowDescriptor) {
        let id = descriptor.id.clone();
        let existed = self.descriptors.insert(id.clone(), descriptor).is_some();
        if !existed {
            self.insertion_order.push(id.clone());
        } else {
            self.dirty_ids.insert(id);
        }
    }

    /// Unregisters a window kind by id and marks any open tabs for cleanup/reconcile.
    pub fn unregister(&mut self, id: &str) -> Option<DockWindowDescriptor> {
        self.insertion_order.retain(|item| item != id);
        self.dirty_ids.remove(id);
        self.descriptors.remove(id)
    }

    /// Returns the descriptor matching `id`.
    pub fn get(&self, id: &str) -> Option<&DockWindowDescriptor> {
        self.descriptors.get(id)
    }

    /// Returns registered descriptors sorted ascending by `(priority, name)`.
    pub fn descriptors(&self) -> Vec<&DockWindowDescriptor> {
        let mut list: Vec<&DockWindowDescriptor> = self.descriptors.values().collect();
        list.sort_by(|a, b| a.priority.cmp(&b.priority).then_with(|| a.name.cmp(&b.name)));
        list
    }

    /// Returns registered window IDs in registration order.
    pub fn registered_ids(&self) -> &[String] {
        &self.insertion_order
    }

    /// Returns whether any descriptor has been replaced since the last drain.
    pub fn has_dirty(&self) -> bool {
        !self.dirty_ids.is_empty()
    }

    /// Drains window IDs queued for live content rebuild.
    pub fn take_dirty_ids(&mut self) -> Vec<String> {
        self.dirty_ids.drain().collect()
    }

    /// Invokes the registered builder for `window_id` with `(commands, container_entity)`.
    pub fn build_into(&self, window_id: &str, commands: &mut Commands, container: Entity) -> bool {
        let Some(desc) = self.descriptors.get(window_id) else {
            return false;
        };
        (desc.build)(commands, container);
        true
    }
}
