//! Vertical icon sidebar strip for [`DockAreaStyle::IconSidebar`] panels.

use bevy::ecs::system::SystemParam;
use bevy::picking::pointer::PointerButton;
use bevy::prelude::*;
use bevy_luma::prelude::*;

use crate::helpers::find_ancestor_or_self;

use super::area::{
    ActiveDockWindow, DockArea, DockAreaStyle, DockSidebarContainer, DockSidebarIcon,
    DockTabContent, DockWindow, IconFontHandle,
};
use super::reconcile::LeafBinding;
use super::style::{DockStyle, resolve_fonts, set_interaction_colors_if_changed};
use super::tabs::{activate_window_in_area, area_windows, close_window_in_area};
use super::tree::DockTree;

#[derive(Component, Clone, Copy, Debug, Default, Reflect)]
#[reflect(Component)]
struct DockSidebarAccent;

#[derive(Component, Clone, Debug, Default, PartialEq, Eq)]
struct SidebarSignature {
    items: Vec<(Entity, String, Option<String>)>,
}

#[derive(SystemParam)]
struct SidebarDirtyQueries<'w, 's> {
    just_area_q: Query<'w, 's, (), With<DockArea>>,
    changed_contents: Query<
        'w,
        's,
        &'static ChildOf,
        (
            With<DockTabContent>,
            Or<(Added<DockTabContent>, Changed<Children>)>,
        ),
    >,
    changed_windows: Query<
        'w,
        's,
        &'static ChildOf,
        (With<DockWindow>, Or<(Added<DockWindow>, Changed<DockWindow>)>),
    >,
    added_sidebars: Query<'w, 's, &'static ChildOf, Added<DockSidebarContainer>>,
    parents_q: Query<'w, 's, &'static ChildOf>,
}

fn sync_sidebar_icons(
    mut commands: Commands,
    style: Option<Res<DockStyle>>,
    theme: Option<Res<Theme>>,
    icon_font_res: Option<Res<IconFontHandle>>,
    areas: Query<(Entity, Ref<DockArea>, Ref<ActiveDockWindow>)>,
    dirty: SidebarDirtyQueries,
    mut dirty_areas: Local<Vec<Entity>>,
    children_q: Query<&Children>,
    content_q: Query<(), With<DockTabContent>>,
    window_q: Query<&DockWindow>,
    sidebars_q: Query<(Entity, Option<&SidebarSignature>), With<DockSidebarContainer>>,
    mut icon_btns_q: Query<(&DockSidebarIcon, &mut UNode, &mut UInteractionColors)>,
    mut accents_q: Query<(&mut UNode, &mut Visibility), (With<DockSidebarAccent>, Without<DockSidebarIcon>)>,
) {
    let visual_res_changed = style.as_ref().is_some_and(|s| s.is_changed())
        || theme.as_ref().is_some_and(|t| t.is_changed())
        || icon_font_res.as_ref().is_some_and(|f| f.is_changed());

    dirty_areas.clear();
    if !visual_res_changed {
        for co in dirty.changed_contents.iter() {
            if !dirty_areas.contains(&co.parent()) {
                dirty_areas.push(co.parent());
            }
        }
        for co in dirty.changed_windows.iter() {
            if let Ok(content_co) = dirty.parents_q.get(co.parent()) {
                let area_ent = content_co.parent();
                if !dirty_areas.contains(&area_ent) {
                    dirty_areas.push(area_ent);
                }
            }
        }
        for co in dirty.added_sidebars.iter() {
            if let Some(area_ent) =
                find_ancestor_or_self(co.parent(), &dirty.just_area_q, &dirty.parents_q)
            {
                if !dirty_areas.contains(&area_ent) {
                    dirty_areas.push(area_ent);
                }
            }
        }
    }

    let mut default_style_storage: Option<DockStyle> = None;
    let mut cached_fonts: Option<(Handle<Font>, Handle<Font>)> = None;

    for (area_ent, area, active) in areas.iter() {
        if area.style != DockAreaStyle::IconSidebar {
            continue;
        }
        let is_dirty = visual_res_changed
            || area.is_added()
            || area.is_changed()
            || active.is_added()
            || active.is_changed()
            || dirty_areas.contains(&area_ent);
        if !is_dirty {
            continue;
        }

        let Ok(area_children) = children_q.get(area_ent) else {
            continue;
        };
        let Some(sidebar_ent) = area_children.iter().find(|&c| sidebars_q.contains(c)) else {
            continue;
        };

        let style = match style.as_deref() {
            Some(s) => s,
            None => default_style_storage.get_or_insert_with(DockStyle::default),
        };
        let (text_font, icon_font) = cached_fonts
            .get_or_insert_with(|| resolve_fonts(theme.as_deref(), icon_font_res.as_deref()));

        let windows_in_area = area_windows(area_ent, &children_q, &content_q, &window_q);
        let sig_matches = !visual_res_changed
            && sidebars_q.get(sidebar_ent).ok().and_then(|(_, s)| s).is_some_and(|sig| {
                sig.items.len() == windows_in_area.len()
                    && sig.items.iter().zip(windows_in_area.iter()).all(
                        |((s_ent, s_name, s_icon), (w_ent, w))| {
                            *s_ent == *w_ent && *s_name == w.name && *s_icon == w.icon
                        },
                    )
            });

        if !sig_matches {
            let next_sig = SidebarSignature {
                items: windows_in_area
                    .iter()
                    .map(|(e, w)| (*e, w.name.clone(), w.icon.clone()))
                    .collect(),
            };
            commands.entity(sidebar_ent).try_insert(next_sig);
            if let Ok(existing) = children_q.get(sidebar_ent) {
                for &c in existing {
                    if let Ok(mut cmd) = commands.get_entity(c) {
                        cmd.try_despawn();
                    }
                }
            }
            for (win_ent, win) in &windows_in_area {
                let is_active = Some(*win_ent) == active.0;
                let bg = if is_active {
                    style.sidebar_icon_active_bg
                } else {
                    Color::NONE
                };
                let (glyph, font) = match &win.icon {
                    Some(ic) => (ic.clone(), icon_font.clone()),
                    None => (
                        win.name
                            .chars()
                            .next()
                            .unwrap_or('?')
                            .to_uppercase()
                            .to_string(),
                        text_font.clone(),
                    ),
                };

                let btn = commands
                    .spawn((
                        ChildOf(sidebar_ent),
                        DockSidebarIcon {
                            window_entity: *win_ent,
                            area_entity: area_ent,
                        },
                        UNode {
                            width: UVal::Px(style.sidebar_icon_size),
                            height: UVal::Px(style.sidebar_icon_size),
                            border_radius: UCornerRadius::all(6.0),
                            background_color: bg,
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
                            normal: bg,
                            hovered: style.sidebar_icon_hover_bg,
                            pressed: style.sidebar_icon_active_bg,
                        },
                    ))
                    .id();

                commands.spawn((
                    ChildOf(btn),
                    DockSidebarAccent,
                    UNode {
                        width: UVal::Px(style.sidebar_accent_width),
                        height: UVal::Percent(0.55),
                        border_radius: UCornerRadius::all(2.0),
                        background_color: style.accent_color,
                        ..default()
                    },
                    USelf {
                        position_type: UPositionType::Absolute,
                        left: UVal::Px(0.0),
                        ..default()
                    },
                    if is_active {
                        Visibility::Inherited
                    } else {
                        Visibility::Hidden
                    },
                ));

                commands.spawn((
                    ChildOf(btn),
                    UNode::default(),
                    UText {
                        text: glyph,
                        font_size: 15.0,
                        font,
                        color: if is_active {
                            style.tab_text_active
                        } else {
                            style.icon_color
                        },
                        ..default()
                    },
                ));
            }
        } else if let Ok(sb_children) = children_q.get(sidebar_ent) {
            for &btn_ent in sb_children {
                let Ok((icon_comp, mut node, mut icolors)) = icon_btns_q.get_mut(btn_ent) else {
                    continue;
                };
                let is_active = Some(icon_comp.window_entity) == active.0;
                let bg = if is_active {
                    style.sidebar_icon_active_bg
                } else {
                    Color::NONE
                };
                if node.background_color != bg {
                    node.background_color = bg;
                }
                let next_icolors = UInteractionColors {
                    normal: bg,
                    hovered: style.sidebar_icon_hover_bg,
                    pressed: style.sidebar_icon_active_bg,
                };
                set_interaction_colors_if_changed(&mut icolors, next_icolors);
                if let Ok(btn_children) = children_q.get(btn_ent) {
                    for &c in btn_children {
                        if let Ok((mut anode, mut avis)) = accents_q.get_mut(c) {
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
                    }
                }
            }
        }
    }
}

fn on_sidebar_icon_click(
    mut trigger: On<PointerClick>,
    mut commands: Commands,
    parents: Query<&ChildOf>,
    icons_q: Query<&DockSidebarIcon>,
    mut active_q: Query<&mut ActiveDockWindow>,
    leaf_bindings: Query<&LeafBinding>,
    children_q: Query<&Children>,
    content_q: Query<(), With<DockTabContent>>,
    window_q: Query<&DockWindow>,
    mut dock_tree: Option<ResMut<DockTree>>,
) {
    let Some(icon_ent) = find_ancestor_or_self(trigger.entity, &icons_q, &parents) else {
        return;
    };
    let Ok(icon) = icons_q.get(icon_ent) else {
        return;
    };

    match trigger.event().button {
        PointerButton::Primary => {
            trigger.propagate(false);
            activate_window_in_area(
                &mut commands,
                icon.area_entity,
                icon.window_entity,
                &mut active_q,
                &leaf_bindings,
                &children_q,
                &content_q,
                &window_q,
                dock_tree.as_deref_mut(),
            );
        }
        PointerButton::Secondary | PointerButton::Middle => {
            trigger.propagate(false);
            close_window_in_area(
                &mut commands,
                icon.area_entity,
                icon.window_entity,
                &mut active_q,
                &leaf_bindings,
                &children_q,
                &content_q,
                &window_q,
                dock_tree.as_deref_mut(),
            );
        }
    }
}

/// Plugin registering icon sidebar synchronization and click/close observers.
pub struct DockSidebarPlugin;

impl Plugin for DockSidebarPlugin {
    fn build(&self, app: &mut App) {
        super::configure_panel_pipeline(app);
        app.add_systems(
            Update,
            sync_sidebar_icons.in_set(super::PanelSystemSet::SyncVisuals),
        )
        .add_observer(on_sidebar_icon_click);
    }
}
