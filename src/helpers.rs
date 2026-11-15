//! Internal helper utilities shared across widgets to eliminate duplication
//! in tree traversal, focus visual synchronization, stepping, and disabled styling.

use bevy::ecs::query::{QueryData, QueryFilter};
use bevy::picking::pointer::PointerButton;
use bevy::prelude::*;
use bevy::scene::{ResolveContext, ResolveSceneError, ResolvedScene};
use bevy_luma::prelude::*;

/// Returns `true` only on the original leaf target of a primary mouse button click,
/// preventing double-firing when `PointerClick` bubbles up `ChildOf` ancestors.
pub(crate) fn is_primary_click(trigger: &On<PointerClick>) -> bool {
    trigger.event().button == PointerButton::Primary
        && trigger.entity == trigger.original_event_target()
}

/// Resolves an optional inner scene, emitting nothing when `None`.
pub(crate) struct ConditionalScene<T: Scene>(pub Option<T>);

impl<T: Scene> Scene for ConditionalScene<T> {
    fn resolve(
        self,
        context: &mut ResolveContext,
        scene: &mut ResolvedScene,
    ) -> Result<(), ResolveSceneError> {
        if let Some(inner) = self.0 {
            inner.resolve(context, scene)
        } else {
            Ok(())
        }
    }
}

/// Control flow signal for [`visit_descendants`].
#[derive(Clone, Copy, PartialEq, Eq)]
#[allow(dead_code)]
pub(crate) enum DescendantVisit {
    /// Continue visiting children and siblings.
    Continue,
    /// Do not descend into this entity's children, but continue siblings.
    SkipChildren,
    /// Stop traversal immediately.
    Stop,
}

/// Finds `entity` itself if it matches `query`, otherwise walks its ancestors to find the nearest match.
pub(crate) fn find_ancestor_or_self<'w, 's, D, F>(
    entity: Entity,
    query: &Query<'w, 's, D, F>,
    parents: &Query<&ChildOf>,
) -> Option<Entity>
where
    D: QueryData,
    F: QueryFilter,
{
    if query.contains(entity) {
        Some(entity)
    } else {
        parents.iter_ancestors(entity).find(|&e| query.contains(e))
    }
}

/// Walks ancestors of `entity` (excluding `entity` itself) and returns the nearest match in `query`.
#[allow(dead_code)]
pub(crate) fn find_ancestor<'w, 's, D, F>(
    entity: Entity,
    query: &Query<'w, 's, D, F>,
    parents: &Query<&ChildOf>,
) -> Option<Entity>
where
    D: QueryData,
    F: QueryFilter,
{
    parents.iter_ancestors(entity).find(|&e| query.contains(e))
}

/// Finds `entity` if it matches `query`, else nearest matching ancestor, else first matching direct child.
///
/// Useful for wrapper controls (like a toggle or checkbox with an adjacent label) where the user
/// can click either the control itself, its internal child, or the outer wrapper.
pub(crate) fn find_target_with_children<'w, 's, D, F>(
    entity: Entity,
    query: &Query<'w, 's, D, F>,
    parents: &Query<&ChildOf>,
    children_query: &Query<&Children>,
) -> Option<Entity>
where
    D: QueryData,
    F: QueryFilter,
{
    if let Some(found) = find_ancestor_or_self(entity, query, parents) {
        return Some(found);
    }
    children_query
        .get(entity)
        .ok()
        .and_then(|children| children.iter().find(|&e| query.contains(e)))
}

/// Visits descendants of `root` (excluding `root`) depth-first without recursion limits.
pub(crate) fn visit_descendants<F>(root: Entity, children_query: &Query<&Children>, mut visitor: F)
where
    F: FnMut(Entity) -> DescendantVisit,
{
    let Ok(root_children) = children_query.get(root) else {
        return;
    };
    let mut stack: Vec<Entity> = root_children.iter().rev().collect();
    while let Some(entity) = stack.pop() {
        match visitor(entity) {
            DescendantVisit::Continue => {
                if let Ok(children) = children_query.get(entity) {
                    stack.extend(children.iter().rev());
                }
            }
            DescendantVisit::SkipChildren => {}
            DescendantVisit::Stop => break,
        }
    }
}

/// Updates an entity's border color while keeping an active [`UFocusVisual`] synchronized,
/// so losing keyboard focus restores the updated color instead of a stale captured one.
pub(crate) fn sync_focus_visual_border(
    focus_visual: Option<Mut<UFocusVisual>>,
    border: &mut UBorder,
    color: Color,
) {
    if let Some(mut fv) = focus_visual {
        if fv.is_captured() {
            fv.set_original_border_color(color);
            return;
        }
    }
    border.color = color;
}

/// Snaps `raw` onto a step grid anchored at `origin` when `step > 0.0`.
pub(crate) fn snap_stepped(raw: f32, step: f32, origin: f32) -> f32 {
    if step > 0.0 && step.is_finite() {
        origin + ((raw - origin) / step).round() * step
    } else {
        raw
    }
}

/// Returns a dimmed copy of `color` suitable for disabled widget states.
pub(crate) fn dim_color(color: Color, alpha_factor: f32) -> Color {
    color.with_alpha(color.alpha() * alpha_factor)
}

/// Shifts a color brighter (`delta > 0`) or darker (`delta < 0`) in sRGB space.
pub(crate) fn shift_brightness(color: Color, delta: f32) -> Color {
    let s = color.to_srgba();
    Color::srgba(
        (s.red + delta).clamp(0.0, 1.0),
        (s.green + delta).clamp(0.0, 1.0),
        (s.blue + delta).clamp(0.0, 1.0),
        s.alpha,
    )
}

/// Builds [`UInteractionColors`], locking hover and pressed states to `normal` when `disabled`.
pub(crate) fn interaction_colors(
    normal: Color,
    hovered: Color,
    pressed: Color,
    disabled: bool,
) -> UInteractionColors {
    if disabled {
        UInteractionColors {
            normal,
            hovered: normal,
            pressed: normal,
        }
    } else {
        UInteractionColors {
            normal,
            hovered,
            pressed,
        }
    }
}

/// Computes viewport coordinates `(left, top, trigger_width)` for a popup anchored
/// above or below `trigger_entity`, clamped within the active window bounds.
pub(crate) fn compute_overlay_anchor(
    trigger_entity: Entity,
    estimated_width: f32,
    estimated_height: f32,
    windows: &Query<&Window>,
    cameras: &Query<(&Camera, &GlobalTransform)>,
    transforms: &Query<(&GlobalTransform, &ComputedSize)>,
) -> (f32, f32, f32) {
    let window = windows.iter().next();
    let (win_w, win_h) = window.map(|w| (w.width(), w.height())).unwrap_or((1280.0, 720.0));

    let (trigger_w, trigger_h, screen_x, screen_y) = if let Ok((gt, computed)) = transforms.get(trigger_entity) {
        let mut screen_pos = Vec2::new(
            (win_w * 0.5) + gt.translation().x,
            (win_h * 0.5) - gt.translation().y,
        );
        for (cam, cam_gt) in cameras.iter() {
            if let Ok(viewport_pos) = cam.world_to_viewport(cam_gt, gt.translation()) {
                screen_pos = viewport_pos;
                break;
            }
        }
        (computed.width.max(120.0), computed.height.max(30.0), screen_pos.x, screen_pos.y)
    } else {
        (estimated_width, 36.0, win_w * 0.5, win_h * 0.5)
    };

    let half_w = trigger_w * 0.5;
    let half_h = trigger_h * 0.5;
    let trigger_left = screen_x - half_w;
    let trigger_top = screen_y - half_h;
    let trigger_bottom = screen_y + half_h;

    let menu_width = trigger_w.max(estimated_width);
    let space_below = win_h - trigger_bottom;
    let space_above = trigger_top;
    let place_above = space_below < (estimated_height + 8.0) && space_above > space_below;
    let menu_top = if place_above {
        (trigger_top - estimated_height - 4.0).max(4.0)
    } else {
        (trigger_bottom + 4.0).min((win_h - estimated_height - 4.0).max(4.0))
    };
    let menu_left = trigger_left.clamp(4.0, (win_w - menu_width - 4.0).max(4.0));

    (menu_left, menu_top, menu_width)
}
