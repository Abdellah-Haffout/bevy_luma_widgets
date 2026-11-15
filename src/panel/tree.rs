//! Serializable binary split-tree data model for dock layouts.

use bevy::prelude::*;
use serde::{Deserialize, Serialize};

use super::area::DockAreaStyle;

/// Unique identifier of a node ([`DockNode::Leaf`] or [`DockNode::Split`]) inside a [`DockTree`].
#[derive(
    Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord, Hash, Reflect, Serialize, Deserialize,
)]
pub struct NodeId(pub u32);

/// Unique identifier of an open tab entry inside a [`DockLeaf`].
#[derive(
    Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord, Hash, Reflect, Serialize, Deserialize,
)]
pub struct TabId(pub u32);

/// Split axis orientation for a [`DockSplit`].
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, Reflect, Serialize, Deserialize)]
pub enum SplitAxis {
    /// Side-by-side left/right panels (Flex row).
    Horizontal,
    /// Stacked top/bottom panels (Flex column).
    Vertical,
}

/// Target edge when splitting a dock leaf.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, Reflect, Serialize, Deserialize)]
pub enum Edge {
    Left,
    Right,
    Top,
    Bottom,
}

impl Edge {
    /// Returns the split axis produced by splitting along this edge.
    pub fn axis(self) -> SplitAxis {
        match self {
            Edge::Left | Edge::Right => SplitAxis::Horizontal,
            Edge::Top | Edge::Bottom => SplitAxis::Vertical,
        }
    }

    /// Returns `true` if the newly inserted leaf occupies the `first` (left/top) slot.
    pub fn inserts_first(self) -> bool {
        matches!(self, Edge::Left | Edge::Top)
    }
}

/// Single open tab inside a [`DockLeaf`].
#[derive(Clone, Debug, PartialEq, Eq, Reflect, Serialize, Deserialize)]
pub struct DockTabEntry {
    pub tab_id: TabId,
    pub window_id: String,
    pub title: String,
}

impl DockTabEntry {
    /// Creates a new [`DockTabEntry`].
    pub fn new(tab_id: TabId, window_id: impl Into<String>, title: impl Into<String>) -> Self {
        Self {
            tab_id,
            window_id: window_id.into(),
            title: title.into(),
        }
    }
}

/// Leaf region in the dock tree representing one [`DockArea`](super::area::DockArea).
#[derive(Clone, Debug, PartialEq, Eq, Reflect, Serialize, Deserialize)]
pub struct DockLeaf {
    pub id: NodeId,
    /// Optional logical name preserved across splits and persistence.
    #[serde(default)]
    pub name: String,
    pub style: DockAreaStyle,
    pub tabs: Vec<DockTabEntry>,
    pub active: usize,
    pub persistent: bool,
}

impl DockLeaf {
    /// Creates a new leaf with a given id, presentation style, and initial tabs.
    pub fn new(id: NodeId, style: DockAreaStyle, tabs: Vec<DockTabEntry>) -> Self {
        Self {
            id,
            name: format!("area_{}", id.0),
            style,
            tabs,
            active: 0,
            persistent: false,
        }
    }

    /// Sets the logical area identifier for this leaf.
    pub fn with_name(mut self, name: impl Into<String>) -> Self {
        self.name = name.into();
        self
    }

    /// Marks whether this leaf stays alive when empty.
    pub fn with_persistent(mut self, persistent: bool) -> Self {
        self.persistent = persistent;
        self
    }

    /// Returns the currently active tab entry, if any.
    pub fn active_tab(&self) -> Option<&DockTabEntry> {
        self.tabs.get(self.active)
    }

    /// Clamps `active` so it always points to a valid index (or `0` when empty).
    pub fn clamp_active(&mut self) {
        if self.tabs.is_empty() {
            self.active = 0;
        } else if self.active >= self.tabs.len() {
            self.active = self.tabs.len() - 1;
        }
    }
}

/// Binary split container dividing space along [`SplitAxis`] by `fraction`.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct DockSplit {
    pub id: NodeId,
    pub axis: SplitAxis,
    /// Fraction of available space given to `first` (clamped to `[0.05, 0.95]`).
    pub fraction: f32,
    pub first: Box<DockNode>,
    pub second: Box<DockNode>,
}

/// Recursive node inside a [`DockTree`].
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub enum DockNode {
    Split(DockSplit),
    Leaf(DockLeaf),
}

impl DockNode {
    /// Returns this node's [`NodeId`].
    pub fn id(&self) -> NodeId {
        match self {
            DockNode::Split(s) => s.id,
            DockNode::Leaf(l) => l.id,
        }
    }

    /// Returns `true` if `target` exists anywhere in this subtree.
    pub fn contains_node(&self, target: NodeId) -> bool {
        if self.id() == target {
            return true;
        }
        match self {
            DockNode::Split(s) => s.first.contains_node(target) || s.second.contains_node(target),
            DockNode::Leaf(_) => false,
        }
    }

    /// Counts all nodes (splits + leaves) in this subtree.
    pub fn node_count(&self) -> usize {
        match self {
            DockNode::Leaf(_) => 1,
            DockNode::Split(s) => 1 + s.first.node_count() + s.second.node_count(),
        }
    }
}

/// Complete hierarchical layout state driving the panel & docking reconciliation system.
#[derive(Resource, Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct DockTree {
    pub root: DockNode,
    next_id: u32,
}

impl Default for DockTree {
    fn default() -> Self {
        Self::new(DockLeaf::new(NodeId(1), DockAreaStyle::TabBar, Vec::new()).with_persistent(true))
    }
}

impl DockTree {
    /// Builds a [`DockTree`] with a single root leaf.
    pub fn new(root_leaf: DockLeaf) -> Self {
        let mut tree = Self {
            root: DockNode::Leaf(root_leaf),
            next_id: 2,
        };
        tree.sync_next_id();
        tree
    }

    /// Builds a [`DockTree`] from an arbitrary root [`DockNode`], advancing the ID allocator past all existing IDs.
    pub fn from_root(root: DockNode) -> Self {
        let mut tree = Self { root, next_id: 1 };
        tree.sync_next_id();
        tree
    }

    fn sync_next_id(&mut self) {
        let max_id = max_used_id(&self.root);
        self.next_id = max_id.saturating_add(1).max(1);
    }

    /// Allocates a fresh unique [`NodeId`].
    pub fn alloc_node_id(&mut self) -> NodeId {
        let id = self.next_id;
        self.next_id += 1;
        NodeId(id)
    }

    /// Allocates a fresh unique [`TabId`].
    pub fn alloc_tab_id(&mut self) -> TabId {
        let id = self.next_id;
        self.next_id += 1;
        TabId(id)
    }

    /// Returns a reference to the root node.
    pub fn root(&self) -> &DockNode {
        &self.root
    }

    /// Returns a mutable reference to the root node.
    pub fn root_mut(&mut self) -> &mut DockNode {
        &mut self.root
    }

    /// Looks up a leaf by [`NodeId`].
    pub fn find_leaf(&self, id: NodeId) -> Option<&DockLeaf> {
        find_leaf_in(&self.root, id)
    }

    /// Mutably looks up a leaf by [`NodeId`].
    pub fn find_leaf_mut(&mut self, id: NodeId) -> Option<&mut DockLeaf> {
        find_leaf_mut_in(&mut self.root, id)
    }

    /// Looks up a leaf by its logical area name (`DockLeaf::name`).
    pub fn find_leaf_by_name(&self, name: &str) -> Option<&DockLeaf> {
        find_leaf_by_name_in(&self.root, name)
    }

    /// Looks up a split node by [`NodeId`].
    pub fn find_split(&self, id: NodeId) -> Option<&DockSplit> {
        find_split_in(&self.root, id)
    }

    /// Returns all leaf [`NodeId`]s in visual tree traversal order.
    pub fn leaf_ids(&self) -> Vec<NodeId> {
        let mut ids = Vec::new();
        collect_leaf_ids(&self.root, &mut ids);
        ids
    }

    /// Finds the split parent and `is_first` flag for a given child [`NodeId`].
    pub fn parent_split(&self, child_id: NodeId) -> Option<(NodeId, bool)> {
        find_parent_split_in(&self.root, child_id)
    }

    /// Locates `(leaf_id, tab_index)` for a given [`TabId`].
    pub fn find_tab(&self, tab_id: TabId) -> Option<(NodeId, usize)> {
        find_tab_in(&self.root, tab_id)
    }

    /// Updates the split `fraction` (clamped to `[0.05, 0.95]`). Returns `true` if the split existed and changed.
    pub fn set_fraction(&mut self, split_id: NodeId, fraction: f32) -> bool {
        let clamped = fraction.clamp(0.05, 0.95);
        if let Some(split) = find_split_mut_in(&mut self.root, split_id) {
            if (split.fraction - clamped).abs() > 1e-5 {
                split.fraction = clamped;
                return true;
            }
        }
        false
    }

    /// Sets the active tab index in `leaf_id`, clamping to bounds.
    pub fn set_active(&mut self, leaf_id: NodeId, index: usize) -> bool {
        let Some(leaf) = self.find_leaf_mut(leaf_id) else {
            return false;
        };
        if leaf.tabs.is_empty() {
            return false;
        }
        let target = index.min(leaf.tabs.len() - 1);
        if leaf.active != target {
            leaf.active = target;
            return true;
        }
        false
    }

    /// Appends a new tab to `leaf_id` and makes it active.
    pub fn add_tab(
        &mut self,
        leaf_id: NodeId,
        window_id: impl Into<String>,
        title: impl Into<String>,
    ) -> Option<TabId> {
        let idx = self.find_leaf(leaf_id)?.tabs.len();
        self.insert_tab(leaf_id, idx, window_id, title)
    }

    /// Inserts a new tab at `index` inside `leaf_id` and makes it active.
    pub fn insert_tab(
        &mut self,
        leaf_id: NodeId,
        index: usize,
        window_id: impl Into<String>,
        title: impl Into<String>,
    ) -> Option<TabId> {
        let tab_id = self.alloc_tab_id();
        let entry = DockTabEntry::new(tab_id, window_id, title);
        if self.insert_tab_entry(leaf_id, index, entry) {
            Some(tab_id)
        } else {
            None
        }
    }

    /// Inserts an existing [`DockTabEntry`] at `index` inside `leaf_id` and activates it.
    pub fn insert_tab_entry(&mut self, leaf_id: NodeId, index: usize, entry: DockTabEntry) -> bool {
        let Some(leaf) = self.find_leaf_mut(leaf_id) else {
            return false;
        };
        let insert_at = index.min(leaf.tabs.len());
        leaf.tabs.insert(insert_at, entry);
        leaf.active = insert_at;
        true
    }

    /// Removes the tab at `tab_index` from `leaf_id`, simplifies the tree if a non-persistent leaf becomes empty,
    /// and returns the removed entry.
    pub fn remove_tab(&mut self, leaf_id: NodeId, tab_index: usize) -> Option<DockTabEntry> {
        let leaf = self.find_leaf_mut(leaf_id)?;
        if tab_index >= leaf.tabs.len() {
            return None;
        }
        let removed = leaf.tabs.remove(tab_index);
        if !leaf.tabs.is_empty() {
            if tab_index < leaf.active || leaf.active >= leaf.tabs.len() {
                leaf.active = leaf.active.saturating_sub(1).min(leaf.tabs.len() - 1);
            }
        } else {
            leaf.active = 0;
        }
        self.simplify();
        Some(removed)
    }

    /// Removes the tab matching `tab_id` from wherever it lives, simplifying the tree afterward.
    pub fn remove_tab_by_id(&mut self, tab_id: TabId) -> Option<(NodeId, DockTabEntry)> {
        let (leaf_id, idx) = self.find_tab(tab_id)?;
        let entry = self.remove_tab(leaf_id, idx)?;
        Some((leaf_id, entry))
    }

    /// Moves a tab from `(from_leaf, from_index)` to `(to_leaf, to_index)`, preserving its [`TabId`].
    pub fn move_tab(
        &mut self,
        from_leaf: NodeId,
        from_index: usize,
        to_leaf: NodeId,
        mut to_index: usize,
    ) -> bool {
        let Some(src) = self.find_leaf(from_leaf) else {
            return false;
        };
        if from_index >= src.tabs.len() || self.find_leaf(to_leaf).is_none() {
            return false;
        }
        if from_leaf == to_leaf && from_index < to_index {
            to_index = to_index.saturating_sub(1);
        }
        let Some(src_mut) = self.find_leaf_mut(from_leaf) else {
            return false;
        };
        let entry = src_mut.tabs.remove(from_index);
        src_mut.clamp_active();

        let Some(dst_mut) = self.find_leaf_mut(to_leaf) else {
            return false;
        };
        let insert_at = to_index.min(dst_mut.tabs.len());
        dst_mut.tabs.insert(insert_at, entry);
        dst_mut.active = insert_at;

        self.simplify();
        true
    }

    /// Splits `leaf_id` along `edge`, creating a new split parent containing the original subtree and `new_leaf`.
    /// Returns the newly allocated split [`NodeId`], or `None` if `leaf_id` was not found.
    pub fn split(
        &mut self,
        leaf_id: NodeId,
        edge: Edge,
        new_leaf: DockLeaf,
        fraction: f32,
    ) -> Option<NodeId> {
        let split_id = self.alloc_node_id();
        let fraction = fraction.clamp(0.05, 0.95);
        if replace_node_with_split(&mut self.root, leaf_id, split_id, edge, new_leaf, fraction) {
            Some(split_id)
        } else {
            None
        }
    }

    /// Moves a single tab out of `from_leaf` into a brand-new leaf created by splitting `target_leaf` along `edge`.
    pub fn move_tab_to_new_split(
        &mut self,
        from_leaf: NodeId,
        from_index: usize,
        target_leaf: NodeId,
        edge: Edge,
        fraction: f32,
    ) -> Option<NodeId> {
        let src = self.find_leaf(from_leaf)?;
        if from_index >= src.tabs.len() || self.find_leaf(target_leaf).is_none() {
            return None;
        }
        // Moving the sole tab of a leaf to split that same leaf is a no-op.
        if from_leaf == target_leaf && src.tabs.len() <= 1 {
            return None;
        }

        let entry = self.find_leaf_mut(from_leaf)?.tabs.remove(from_index);
        self.find_leaf_mut(from_leaf)?.clamp_active();

        let new_leaf_id = self.alloc_node_id();
        let new_leaf = DockLeaf::new(new_leaf_id, DockAreaStyle::TabBar, vec![entry]);
        let split_fraction = if edge.inserts_first() {
            fraction.clamp(0.05, 0.95)
        } else {
            (1.0 - fraction).clamp(0.05, 0.95)
        };
        let split_id = self.split(target_leaf, edge, new_leaf, split_fraction)?;
        self.simplify();
        Some(split_id)
    }

    /// Removes all tabs whose `window_id` matches the given id across the entire tree.
    pub fn remove_window_kind(&mut self, window_id: &str) {
        remove_kind_in(&mut self.root, window_id);
        self.simplify();
    }

    /// Returns `true` if any leaf contains a tab with the specified `window_id`.
    pub fn has_window_kind(&self, window_id: &str) -> bool {
        has_kind_in(&self.root, window_id)
    }

    /// Returns `(leaf_id, tab_index, tab_id)` of the first tab matching `window_id`.
    pub fn find_window_kind(&self, window_id: &str) -> Option<(NodeId, usize, TabId)> {
        find_kind_in(&self.root, window_id)
    }

    /// Renames `window_id` across all open tabs without altering tab IDs or active selection.
    pub fn alias_window_kind(&mut self, old_id: &str, new_id: &str) {
        alias_kind_in(&mut self.root, old_id, new_id);
    }

    /// Collapses empty non-persistent leaves up into their siblings while preserving the root.
    pub fn simplify(&mut self) {
        if let Some(simplified) = simplify_node(std::mem::replace(
            &mut self.root,
            DockNode::Leaf(DockLeaf::new(NodeId(0), DockAreaStyle::TabBar, Vec::new())),
        )) {
            self.root = simplified;
        } else {
            // Root itself became removable -> keep an empty persistent root leaf so the tree remains valid.
            let id = self.alloc_node_id();
            self.root = DockNode::Leaf(
                DockLeaf::new(id, DockAreaStyle::TabBar, Vec::new()).with_persistent(true),
            );
        }
    }
}

fn max_used_id(node: &DockNode) -> u32 {
    match node {
        DockNode::Leaf(leaf) => leaf
            .tabs
            .iter()
            .map(|t| t.tab_id.0)
            .fold(leaf.id.0, u32::max),
        DockNode::Split(split) => split
            .id
            .0
            .max(max_used_id(&split.first))
            .max(max_used_id(&split.second)),
    }
}

fn find_leaf_in(node: &DockNode, id: NodeId) -> Option<&DockLeaf> {
    match node {
        DockNode::Leaf(l) if l.id == id => Some(l),
        DockNode::Leaf(_) => None,
        DockNode::Split(s) => find_leaf_in(&s.first, id).or_else(|| find_leaf_in(&s.second, id)),
    }
}

fn find_leaf_mut_in(node: &mut DockNode, id: NodeId) -> Option<&mut DockLeaf> {
    match node {
        DockNode::Leaf(l) if l.id == id => Some(l),
        DockNode::Leaf(_) => None,
        DockNode::Split(s) => {
            find_leaf_mut_in(&mut s.first, id).or_else(|| find_leaf_mut_in(&mut s.second, id))
        }
    }
}

fn find_leaf_by_name_in<'a>(node: &'a DockNode, name: &str) -> Option<&'a DockLeaf> {
    match node {
        DockNode::Leaf(l) if l.name == name => Some(l),
        DockNode::Leaf(_) => None,
        DockNode::Split(s) => {
            find_leaf_by_name_in(&s.first, name).or_else(|| find_leaf_by_name_in(&s.second, name))
        }
    }
}

fn find_split_in(node: &DockNode, id: NodeId) -> Option<&DockSplit> {
    match node {
        DockNode::Leaf(_) => None,
        DockNode::Split(s) => {
            if s.id == id {
                Some(s)
            } else {
                find_split_in(&s.first, id).or_else(|| find_split_in(&s.second, id))
            }
        }
    }
}

fn find_split_mut_in(node: &mut DockNode, id: NodeId) -> Option<&mut DockSplit> {
    match node {
        DockNode::Leaf(_) => None,
        DockNode::Split(s) => {
            if s.id == id {
                Some(s)
            } else {
                find_split_mut_in(&mut s.first, id).or_else(|| find_split_mut_in(&mut s.second, id))
            }
        }
    }
}

fn collect_leaf_ids(node: &DockNode, out: &mut Vec<NodeId>) {
    match node {
        DockNode::Leaf(l) => out.push(l.id),
        DockNode::Split(s) => {
            collect_leaf_ids(&s.first, out);
            collect_leaf_ids(&s.second, out);
        }
    }
}

fn find_parent_split_in(node: &DockNode, child_id: NodeId) -> Option<(NodeId, bool)> {
    let DockNode::Split(s) = node else {
        return None;
    };
    if s.first.id() == child_id {
        return Some((s.id, true));
    }
    if s.second.id() == child_id {
        return Some((s.id, false));
    }
    find_parent_split_in(&s.first, child_id).or_else(|| find_parent_split_in(&s.second, child_id))
}

fn find_tab_in(node: &DockNode, tab_id: TabId) -> Option<(NodeId, usize)> {
    match node {
        DockNode::Leaf(l) => l
            .tabs
            .iter()
            .position(|t| t.tab_id == tab_id)
            .map(|idx| (l.id, idx)),
        DockNode::Split(s) => find_tab_in(&s.first, tab_id).or_else(|| find_tab_in(&s.second, tab_id)),
    }
}

fn replace_node_with_split(
    node: &mut DockNode,
    target_id: NodeId,
    split_id: NodeId,
    edge: Edge,
    new_leaf: DockLeaf,
    fraction: f32,
) -> bool {
    if node.id() == target_id {
        let placeholder = DockNode::Leaf(DockLeaf::new(NodeId(0), DockAreaStyle::TabBar, Vec::new()));
        let existing = std::mem::replace(node, placeholder);
        let new_node = DockNode::Leaf(new_leaf);
        let (first, second) = if edge.inserts_first() {
            (Box::new(new_node), Box::new(existing))
        } else {
            (Box::new(existing), Box::new(new_node))
        };
        *node = DockNode::Split(DockSplit {
            id: split_id,
            axis: edge.axis(),
            fraction,
            first,
            second,
        });
        return true;
    }

    match node {
        DockNode::Leaf(_) => false,
        DockNode::Split(s) => {
            replace_node_with_split(&mut s.first, target_id, split_id, edge, new_leaf.clone(), fraction)
                || replace_node_with_split(&mut s.second, target_id, split_id, edge, new_leaf, fraction)
        }
    }
}

fn remove_kind_in(node: &mut DockNode, window_id: &str) {
    match node {
        DockNode::Leaf(leaf) => {
            leaf.tabs.retain(|t| t.window_id != window_id);
            leaf.clamp_active();
        }
        DockNode::Split(split) => {
            remove_kind_in(&mut split.first, window_id);
            remove_kind_in(&mut split.second, window_id);
        }
    }
}

fn has_kind_in(node: &DockNode, window_id: &str) -> bool {
    match node {
        DockNode::Leaf(leaf) => leaf.tabs.iter().any(|t| t.window_id == window_id),
        DockNode::Split(split) => {
            has_kind_in(&split.first, window_id) || has_kind_in(&split.second, window_id)
        }
    }
}

fn find_kind_in(node: &DockNode, window_id: &str) -> Option<(NodeId, usize, TabId)> {
    match node {
        DockNode::Leaf(leaf) => leaf
            .tabs
            .iter()
            .enumerate()
            .find(|(_, t)| t.window_id == window_id)
            .map(|(i, t)| (leaf.id, i, t.tab_id)),
        DockNode::Split(split) => {
            find_kind_in(&split.first, window_id).or_else(|| find_kind_in(&split.second, window_id))
        }
    }
}

fn alias_kind_in(node: &mut DockNode, old_id: &str, new_id: &str) {
    match node {
        DockNode::Leaf(leaf) => {
            for tab in &mut leaf.tabs {
                if tab.window_id == old_id {
                    tab.window_id = new_id.to_string();
                }
            }
        }
        DockNode::Split(split) => {
            alias_kind_in(&mut split.first, old_id, new_id);
            alias_kind_in(&mut split.second, old_id, new_id);
        }
    }
}

fn simplify_node(node: DockNode) -> Option<DockNode> {
    match node {
        DockNode::Leaf(mut leaf) => {
            if leaf.tabs.is_empty() && !leaf.persistent {
                None
            } else {
                leaf.clamp_active();
                Some(DockNode::Leaf(leaf))
            }
        }
        DockNode::Split(split) => {
            let first = simplify_node(*split.first);
            let second = simplify_node(*split.second);
            match (first, second) {
                (None, None) => None,
                (Some(only), None) | (None, Some(only)) => Some(only),
                (Some(f), Some(s)) => Some(DockNode::Split(DockSplit {
                    id: split.id,
                    axis: split.axis,
                    fraction: split.fraction.clamp(0.05, 0.95),
                    first: Box::new(f),
                    second: Box::new(s),
                })),
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn tree_split_move_remove_simplify() {
        let leaf1 = DockLeaf::new(
            NodeId(1),
            DockAreaStyle::TabBar,
            vec![
                DockTabEntry::new(TabId(2), "scene", "Scene"),
                DockTabEntry::new(TabId(3), "hierarchy", "Hierarchy"),
            ],
        );
        let mut tree = DockTree::new(leaf1);
        assert_eq!(tree.leaf_ids(), vec![NodeId(1)]);

        let new_leaf_id = tree.alloc_node_id();
        let tab_inspector = tree.alloc_tab_id();
        let new_leaf = DockLeaf::new(
            new_leaf_id,
            DockAreaStyle::TabBar,
            vec![DockTabEntry::new(tab_inspector, "inspector", "Inspector")],
        );
        let split_id = tree.split(NodeId(1), Edge::Right, new_leaf, 0.7).unwrap();
        assert_eq!(tree.leaf_ids(), vec![NodeId(1), new_leaf_id]);
        assert!(tree.set_fraction(split_id, 0.65));

        assert!(tree.move_tab(NodeId(1), 1, new_leaf_id, 1));
        assert_eq!(tree.find_leaf(NodeId(1)).unwrap().tabs.len(), 1);
        assert_eq!(tree.find_leaf(new_leaf_id).unwrap().tabs.len(), 2);

        // Removing the single remaining tab on non-persistent leaf1 collapses the split back to new_leaf_id.
        tree.remove_tab(NodeId(1), 0).unwrap();
        assert_eq!(tree.leaf_ids(), vec![new_leaf_id]);
        assert_eq!(tree.root().id(), new_leaf_id);
    }

    #[test]
    fn tree_persistent_leaf_stays_alive_when_empty() {
        let leaf = DockLeaf::new(NodeId(1), DockAreaStyle::TabBar, vec![])
            .with_persistent(true)
            .with_name("viewport");
        let mut tree = DockTree::new(leaf);
        let t = tree.add_tab(NodeId(1), "console", "Console").unwrap();
        assert_eq!(tree.find_tab(t), Some((NodeId(1), 0)));
        tree.remove_tab_by_id(t);
        assert_eq!(tree.leaf_ids(), vec![NodeId(1)]);
        assert!(tree.find_leaf(NodeId(1)).unwrap().tabs.is_empty());
    }
}
