//! Floating `+` add-window popup menu anchored to [`DockTabAddButton`].

use bevy::picking::pointer::PointerButton;
use bevy::prelude::*;
use bevy_luma::prelude::*;

use crate::helpers::{compute_overlay_anchor, find_ancestor_or_self};

use super::area::{
    ActiveDockWindow, DockArea, DockTabAddButton, DockTabContent, DockWindow, IconFontHandle,
};
use super::reconcile::LeafBinding;
use super::registry::WindowRegistry;
use super::style::{DockStyle, resolve_fonts};
use super::tabs::{area_windows, find_area_content_entity};
use super::tree::DockTree;

/// Marker on the root [`URootUi::screen()`] overlay spawning the `+` add-window popup.
#[derive(Component, Clone, Copy, Debug, PartialEq, Eq, Reflect)]
#[reflect(Component)]
pub struct AddWindowPopup {
    pub area_entity: Entity,
}

#[derive(Component, Clone, Copy, Debug, Default, Reflect)]
#[reflect(Component)]
struct AddWindowPopupBackdrop;

#[derive(Component, Clone, Debug, Reflect)]
#[reflect(Component)]
struct AddWindowPopupItem {
    area_entity: Entity,
    window_id: String,
    window_name: String,
    icon: Option<String>,
}

fn despawn_existing_popups(commands: &mut Commands, popups: &Query<(Entity, &AddWindowPopup)>) {
    for (ent, _) in popups.iter() {
        if let Ok(mut cmd) = commands.get_entity(ent) {
            cmd.try_despawn();
        }
    }
}

fn on_add_button_click(
    mut trigger: On<PointerClick>,
    mut commands: Commands,
    style: Option<Res<DockStyle>>,
    theme: Option<Res<Theme>>,
    icon_font_res: Option<Res<IconFontHandle>>,
    registry: Res<WindowRegistry>,
    parents: Query<&ChildOf>,
    children_q: Query<&Children>,
    content_q: Query<(), With<DockTabContent>>,
    window_q: Query<&DockWindow>,
    add_btns: Query<&DockTabAddButton>,
    existing_popups: Query<(Entity, &AddWindowPopup)>,
    windows: Query<&Window>,
    cameras: Query<(&Camera, &GlobalTransform)>,
    transforms: Query<(&GlobalTransform, &ComputedSize)>,
) {
    if trigger.event().button != PointerButton::Primary {
        return;
    }
    let Some(btn_ent) = find_ancestor_or_self(trigger.entity, &add_btns, &parents) else {
        return;
    };
    let Ok(add_btn) = add_btns.get(btn_ent) else {
        return;
    };
    trigger.propagate(false);

    // Clicking `+` while its popup is already open toggles it closed cleanly.
    let already_open_for_area = existing_popups
        .iter()
        .any(|(_, p)| p.area_entity == add_btn.area_entity);
    despawn_existing_popups(&mut commands, &existing_popups);
    if already_open_for_area {
        return;
    }

    let style = style.as_deref().cloned().unwrap_or_default();
    let (text_font, icon_font) = resolve_fonts(theme.as_deref(), icon_font_res.as_deref());
    let descriptors = registry.descriptors();

    let open_in_area: Vec<String> =
        area_windows(add_btn.area_entity, &children_q, &content_q, &window_q)
            .into_iter()
            .map(|(_, w)| w.window_id)
            .collect();

    let item_count = descriptors.len().max(1);
    let estimated_height = (item_count as f32 * 34.0 + 16.0).min(style.popup_max_height);
    let (menu_left, menu_top, _) = compute_overlay_anchor(
        btn_ent,
        style.popup_width,
        estimated_height,
        &windows,
        &cameras,
        &transforms,
    );

    let overlay_root = commands
        .spawn((
            URootUi::screen(),
            UZIndex::Global(9999),
            AddWindowPopup {
                area_entity: add_btn.area_entity,
            },
            AddWindowPopupBackdrop,
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
            UInteraction::default(),
        ))
        .id();

    let card = commands
        .spawn((
            ChildOf(overlay_root),
            UNode {
                width: UVal::Px(style.popup_width),
                height: UVal::Auto,
                max_height: style.popup_max_height,
                padding: USides::all(5.0),
                background_color: style.popup_bg,
                border_radius: UCornerRadius::all(style.popup_radius),
                ..default()
            },
            USelf {
                position_type: UPositionType::Absolute,
                left: UVal::Px(menu_left),
                top: UVal::Px(menu_top),
                ..default()
            },
            UBorder {
                color: style.popup_border,
                width: 1.0,
                radius: UCornerRadius::all(style.popup_radius),
                offset: 0.0,
            },
            UShadow::drop(Color::srgba(0.0, 0.0, 0.0, 0.65), Vec2::new(0.0, 8.0), 16.0),
            ULayout {
                display: UDisplay::Flex,
                flex_direction: UFlexDirection::Column,
                gap: 2.0,
                ..default()
            },
            UClip { enabled: true },
        ))
        .id();

    if descriptors.is_empty() {
        commands.spawn((
            ChildOf(card),
            UNode {
                width: UVal::Percent(1.0),
                height: UVal::Px(32.0),
                padding: USides::axes(10.0, 6.0),
                ..default()
            },
            ULayout {
                display: UDisplay::Flex,
                flex_direction: UFlexDirection::Row,
                align_items: UAlignItems::Center,
                ..default()
            },
            UText {
                text: "No windows registered".to_string(),
                font_size: 12.5,
                font: text_font,
                color: style.tab_text_inactive,
                ..default()
            },
        ));
        return;
    }

    for desc in descriptors {
        let already_open = open_in_area.iter().any(|id| id == &desc.id);
        let item_ent = commands
            .spawn((
                ChildOf(card),
                AddWindowPopupItem {
                    area_entity: add_btn.area_entity,
                    window_id: desc.id.clone(),
                    window_name: desc.name.clone(),
                    icon: desc.icon.clone(),
                },
                UNode {
                    width: UVal::Percent(1.0),
                    height: UVal::Px(32.0),
                    padding: USides::axes(8.0, 4.0),
                    border_radius: UCornerRadius::all(5.0),
                    background_color: Color::NONE,
                    ..default()
                },
                ULayout {
                    display: UDisplay::Flex,
                    flex_direction: UFlexDirection::Row,
                    align_items: UAlignItems::Center,
                    justify_content: UJustifyContent::SpaceBetween,
                    gap: 8.0,
                    ..default()
                },
                UInteraction::default(),
                UInteractionColors {
                    normal: Color::NONE,
                    hovered: style.popup_item_hover,
                    pressed: style.tab_bg_active,
                },
            ))
            .id();

        let left_row = commands
            .spawn((
                ChildOf(item_ent),
                UNode::default(),
                ULayout {
                    display: UDisplay::Flex,
                    flex_direction: UFlexDirection::Row,
                    align_items: UAlignItems::Center,
                    gap: 8.0,
                    ..default()
                },
            ))
            .id();

        commands.spawn((
            ChildOf(left_row),
            UNode::default(),
            UText {
                text: desc
                    .icon
                    .clone()
                    .unwrap_or_else(|| Icon::PANELS_TOP_LEFT.to_string()),
                font_size: 13.5,
                font: icon_font.clone(),
                color: if already_open {
                    style.accent_color
                } else {
                    style.icon_color
                },
                ..default()
            },
        ));

        commands.spawn((
            ChildOf(left_row),
            UNode::default(),
            UText {
                text: desc.name.clone(),
                font_size: 13.0,
                font: text_font.clone(),
                color: style.tab_text_active,
                ..default()
            },
        ));

        if already_open {
            commands.spawn((
                ChildOf(item_ent),
                UNode::default(),
                UText {
                    text: Icon::CHECK.to_string(),
                    font_size: 12.0,
                    font: icon_font.clone(),
                    color: style.accent_color,
                    ..default()
                },
            ));
        }
    }
}

fn on_popup_item_click(
    mut trigger: On<PointerClick>,
    mut commands: Commands,
    parents: Query<&ChildOf>,
    children_q: Query<&Children>,
    content_q: Query<(), With<DockTabContent>>,
    items_q: Query<&AddWindowPopupItem>,
    leaf_bindings: Query<&LeafBinding>,
    registry: Res<WindowRegistry>,
    mut dock_tree: Option<ResMut<DockTree>>,
    mut active_q: Query<&mut ActiveDockWindow, With<DockArea>>,
    existing_popups: Query<(Entity, &AddWindowPopup)>,
) {
    if trigger.event().button != PointerButton::Primary {
        return;
    }
    let Some(item_ent) = find_ancestor_or_self(trigger.entity, &items_q, &parents) else {
        return;
    };
    let Ok(item) = items_q.get(item_ent) else {
        return;
    };
    trigger.propagate(false);

    if let (Some(tree), Ok(binding)) = (dock_tree.as_deref_mut(), leaf_bindings.get(item.area_entity))
    {
        tree.add_tab(binding.0, &item.window_id, &item.window_name);
    } else if let Some(content_ent) =
        find_area_content_entity(item.area_entity, &children_q, &content_q)
    {
        // Standalone DockArea outside DockTree.
        let mut win = DockWindow::new(&item.window_id, &item.window_name);
        if let Some(ic) = &item.icon {
            win = win.with_icon(ic);
        }
        let win_ent = commands
            .spawn((
                ChildOf(content_ent),
                win,
                UNode {
                    width: UVal::Percent(1.0),
                    height: UVal::Percent(1.0),
                    background_color: Color::NONE,
                    ..default()
                },
                ULayout {
                    display: UDisplay::Flex,
                    flex_direction: UFlexDirection::Column,
                    align_items: UAlignItems::Stretch,
                    ..default()
                },
                UClip { enabled: true },
            ))
            .id();
        registry.build_into(&item.window_id, &mut commands, win_ent);
        if let Ok(mut active) = active_q.get_mut(item.area_entity) {
            active.0 = Some(win_ent);
        }
    }

    despawn_existing_popups(&mut commands, &existing_popups);
}

fn on_popup_backdrop_click(
    trigger: On<PointerClick>,
    mut commands: Commands,
    backdrop_q: Query<(), With<AddWindowPopupBackdrop>>,
    existing_popups: Query<(Entity, &AddWindowPopup)>,
) {
    if backdrop_q.contains(trigger.entity) && trigger.entity == trigger.original_event_target() {
        despawn_existing_popups(&mut commands, &existing_popups);
    }
}

fn dismiss_popup_on_escape(
    mut commands: Commands,
    keyboard: Option<Res<ButtonInput<KeyCode>>>,
    existing_popups: Query<(Entity, &AddWindowPopup)>,
) {
    if !existing_popups.is_empty() && keyboard.is_some_and(|kb| kb.just_pressed(KeyCode::Escape)) {
        despawn_existing_popups(&mut commands, &existing_popups);
    }
}

/// Plugin registering `+` add-window popup opening, selection, backdrop dismiss, and Escape dismiss.
pub struct AddWindowPopupPlugin;

impl Plugin for AddWindowPopupPlugin {
    fn build(&self, app: &mut App) {
        app.register_type::<AddWindowPopup>()
            .add_systems(Update, dismiss_popup_on_escape)
            .add_observer(on_add_button_click)
            .add_observer(on_popup_item_click)
            .add_observer(on_popup_backdrop_click);
    }
}
