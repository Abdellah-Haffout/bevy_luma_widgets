//! Dropdown selection menu widget.
//!
//! Features:
//! - Trigger box displaying the current selection or placeholder, with chevron indicator.
//! - Floating popover overlay window rendered in an independent [`URootUi::screen`] at `UZIndex::Global(9999)`.
//! - Never pushes or shifts sibling layout elements (zero layout shift).
//! - Immune to parent container clipping (`UClip` / `overflow: hidden`).
//! - Fullscreen dismiss barrier / backdrop to seamlessly close on outside clicks.
//! - Keyboard navigation (closes on `Escape`).
//! - Auto-flipping: opens upward if near the bottom edge of the screen.

use bevy::prelude::*;
use bevy::scene::{ResolveContext, ResolvedScene, ResolveSceneError};
use bevy_luma::prelude::*;

use crate::helpers::{
    find_ancestor_or_self, interaction_colors, is_primary_click, visit_descendants, DescendantVisit,
};
use crate::navigation::SceneList;
use crate::theme::{SelectStyle, WidgetColors};

/// Single option entry in a Select menu.
#[derive(Clone, Debug, PartialEq, Eq, Reflect)]
pub struct SelectOption {
    pub label: String,
    pub value: String,
}

impl SelectOption {
    pub fn new(label: impl Into<String>, value: impl Into<String>) -> Self {
        Self {
            label: label.into(),
            value: value.into(),
        }
    }
}

/// Marker and state component for a Select dropdown.
#[derive(Component, Clone, Debug, Reflect)]
pub struct LumaSelect {
    pub is_open: bool,
    pub selected_index: Option<usize>,
    pub disabled: bool,
    pub placeholder: String,
    pub options: Vec<SelectOption>,
    pub style: SelectStyle,
    pub has_explicit_style: bool,
    #[reflect(ignore)]
    pub overlay_entity: Option<Entity>,
}

impl Default for LumaSelect {
    fn default() -> Self {
        Self {
            is_open: false,
            selected_index: None,
            disabled: false,
            placeholder: "Select an option...".to_string(),
            options: Vec::new(),
            style: SelectStyle::default(),
            has_explicit_style: false,
            overlay_entity: None,
        }
    }
}

/// Marker for the clickable trigger header of a Select dropdown.
#[derive(Component, Clone, Debug, Default, Reflect)]
pub struct LumaSelectTrigger;

/// Marker for the display text element inside a Select dropdown.
#[derive(Component, Clone, Debug, Default, Reflect)]
pub struct LumaSelectText;

/// Marker for the directional chevron icon inside a Select dropdown.
#[derive(Component, Clone, Debug, Default, Reflect)]
pub struct LumaSelectChevron;

/// Marker for the expandable options menu container inside the floating overlay.
#[derive(Component, Clone, Debug, Default, Reflect)]
pub struct LumaSelectMenu;

/// Root marker attached to the independent floating overlay window.
#[derive(Component, Clone, Debug, Reflect)]
pub struct LumaSelectOverlay {
    pub select_entity: Entity,
}

impl Default for LumaSelectOverlay {
    fn default() -> Self {
        Self {
            select_entity: Entity::PLACEHOLDER,
        }
    }
}

/// Fullscreen transparent barrier backdrop capturing outside clicks to dismiss the menu.
#[derive(Component, Clone, Debug, Reflect)]
pub struct LumaSelectBackdrop {
    pub select_entity: Entity,
}

impl Default for LumaSelectBackdrop {
    fn default() -> Self {
        Self {
            select_entity: Entity::PLACEHOLDER,
        }
    }
}

/// Component attached to each option item row inside a Select dropdown menu.
#[derive(Component, Clone, Debug, Reflect)]
pub struct LumaSelectOptionItem {
    pub select_entity: Entity,
    pub index: usize,
    pub value: String,
    pub label: String,
}

impl Default for LumaSelectOptionItem {
    fn default() -> Self {
        Self {
            select_entity: Entity::PLACEHOLDER,
            index: 0,
            value: String::new(),
            label: String::new(),
        }
    }
}

/// Event triggered when a select option is picked.
#[derive(Event, Clone, Debug)]
pub struct SelectChanged {
    pub entity: Entity,
    pub selected_index: usize,
    pub value: String,
}

/// Declarative Select dropdown widget struct.
#[derive(Clone, Debug, Reflect)]
pub struct Select {
    pub placeholder: String,
    pub options: Vec<SelectOption>,
    pub selected_index: Option<usize>,
    pub disabled: bool,
    pub font: Handle<Font>,
    pub icon_font: Handle<Font>,
    pub style: Option<SelectStyle>,
}

impl Default for Select {
    fn default() -> Self {
        Self {
            placeholder: "Select an option...".to_string(),
            options: Vec::new(),
            selected_index: None,
            disabled: false,
            font: Handle::default(),
            icon_font: Handle::default(),
            style: None,
        }
    }
}

impl Select {
    pub fn new(placeholder: impl Into<String>, options: Vec<SelectOption>) -> Self {
        Self {
            placeholder: placeholder.into(),
            options,
            ..default()
        }
    }

    pub fn selected(mut self, index: usize) -> Self {
        self.selected_index = Some(index);
        self
    }

    pub fn disabled(mut self, disabled: bool) -> Self {
        self.disabled = disabled;
        self
    }

    pub fn font(mut self, font: Handle<Font>) -> Self {
        self.font = font;
        self
    }

    pub fn icon_font(mut self, icon_font: Handle<Font>) -> Self {
        self.icon_font = icon_font;
        self
    }

    pub fn style(mut self, style: SelectStyle) -> Self {
        self.style = Some(style);
        self
    }
}

impl Scene for Select {
    fn resolve(
        self,
        context: &mut ResolveContext,
        scene: &mut ResolvedScene,
    ) -> Result<(), ResolveSceneError> {
        let has_explicit_style = self.style.is_some();
        let style = self.style.unwrap_or_default();
        let disabled = self.disabled;
        let palette = WidgetColors::default();

        let display_text = if let Some(idx) = self.selected_index {
            self.options
                .get(idx)
                .map(|o| o.label.clone())
                .unwrap_or_else(|| self.placeholder.clone())
        } else {
            self.placeholder.clone()
        };

        let is_placeholder = self.selected_index.is_none();
        let text_color = if disabled {
            palette.disabled_text()
        } else if is_placeholder {
            palette.text_dim
        } else {
            style.text_color
        };

        let min_height = style.min_height;
        let trigger_bg = if disabled {
            palette.disabled_bg()
        } else {
            style.bg_color
        };
        let trigger_border = if disabled {
            palette.disabled_border()
        } else {
            style.border_color
        };
        let radius = style.radius;
        let padding = style.padding;
        let border_width = style.border_width;
        let border_focus = style.border_focus;
        let font_size = style.font_size;
        let icon_color = if disabled {
            palette.disabled_text()
        } else {
            style.text_color
        };
        let i_colors = interaction_colors(trigger_bg, style.bg_hover, style.bg_hover, disabled);

        let selected_index = self.selected_index;
        let font = self.font;
        let icon_font = self.icon_font.clone();
        let placeholder = self.placeholder.clone();
        let options = self.options.clone();
        let select_style = style.clone();

        let s = bsn! {
            LumaSelect {
                is_open: false,
                selected_index,
                disabled,
                placeholder,
                options,
                style: select_style,
                has_explicit_style,
                overlay_entity: None,
            }
            UNode {
                width: UVal::Percent(1.0),
                min_height,
                background_color: trigger_bg,
                border_radius: UCornerRadius::all(radius),
                padding,
            }
            LumaSelectTrigger
            UBorder {
                color: trigger_border,
                width: border_width,
                radius: UCornerRadius::all(radius),
            }
            UInteraction::default()
            i_colors
            UFocusable {
                enabled: { !disabled },
                ..UFocusable::new()
            }
            UFocusVisual::border(border_focus, 2.0)
            ULayout {
                display: UDisplay::Flex,
                flex_direction: UFlexDirection::Row,
                align_items: UAlignItems::Center,
                justify_content: UJustifyContent::SpaceBetween,
            }
            Children [
                UNode::default()
                Children [
                    LumaSelectText
                    UText {
                        text: display_text,
                        font_size,
                        font,
                        color: text_color,
                    }
                ]
                --
                UNode::default()
                Children [
                    LumaSelectChevron
                    UText {
                        text: { Icon::CHEVRON_DOWN.to_string() },
                        font_size: 14.0,
                        font: icon_font,
                        color: icon_color,
                    }
                ]
            ]
        };

        s.resolve(context, scene)
    }
}

/// Helper constructor to spawn a standard select dropdown scene.
pub fn luma_select(placeholder: impl Into<String>, options: Vec<SelectOption>) -> Select {
    Select::new(placeholder, options)
}

/// Dedicated scene for the floating dropdown overlay window.
struct SelectOverlayScene {
    select_entity: Entity,
    menu_left: f32,
    menu_top: f32,
    menu_width: f32,
    menu_height: f32,
    item_height: f32,
    item_gap: f32,
    style: SelectStyle,
    options: Vec<SelectOption>,
    selected_index: Option<usize>,
    font: Handle<Font>,
}

impl Scene for SelectOverlayScene {
    fn resolve(
        self,
        context: &mut ResolveContext,
        scene: &mut ResolvedScene,
    ) -> Result<(), ResolveSceneError> {
        let select_entity = self.select_entity;
        let style = self.style;
        let menu_left = self.menu_left;
        let menu_top = self.menu_top;
        let menu_width = self.menu_width;
        let menu_height = self.menu_height;
        let item_height = self.item_height;
        let item_gap = self.item_gap;
        let radius = style.radius;
        let menu_bg = style.menu_bg;
        let menu_border = style.menu_border;
        let selected_index = self.selected_index;

        let option_rows: Vec<_> = self
            .options
            .into_iter()
            .enumerate()
            .map(|(idx, opt)| {
                let is_sel = selected_index == Some(idx);
                let item_bg = if is_sel {
                    Color::srgba(0.12, 0.84, 0.60, 0.18)
                } else {
                    Color::NONE
                };
                let item_color = if is_sel {
                    style.border_focus
                } else {
                    style.text_color
                };
                let opt_label = opt.label;
                let opt_label_clone = opt_label.clone();
                let opt_value = opt.value;
                let f = self.font.clone();
                let opt_font_size = style.font_size - 1.0;

                bsn! {
                    UNode {
                        width: UVal::Percent(1.0),
                        height: UVal::Px(item_height),
                        padding: USides::axes(12.0, 8.0),
                        border_radius: UCornerRadius::all(6.0),
                        background_color: item_bg,
                    }
                    LumaSelectOptionItem {
                        select_entity,
                        index: idx,
                        value: opt_value,
                        label: opt_label_clone,
                    }
                    UInteraction::default()
                    UInteractionColors {
                        normal: item_bg,
                        hovered: Color::srgba(1.0, 1.0, 1.0, 0.08),
                        pressed: Color::srgba(1.0, 1.0, 1.0, 0.14),
                    }
                    ULayout {
                        display: UDisplay::Flex,
                        flex_direction: UFlexDirection::Row,
                        align_items: UAlignItems::Center,
                        justify_content: UJustifyContent::SpaceBetween,
                    }
                    Children [
                        UText {
                            text: opt_label,
                            font_size: opt_font_size,
                            font: f,
                            color: item_color,
                        }
                    ]
                }
            })
            .collect();

        let options_scene = SceneList::new(option_rows);

        let s = bsn! {
            URootUi::screen()
            UZIndex::Global(9999)
            LumaSelectOverlay {
                select_entity,
            }
            // Fullscreen invisible barrier backdrop to capture outside clicks
            UNode {
                width: UVal::Percent(1.0),
                height: UVal::Percent(1.0),
                background_color: Color::NONE,
            }
            LumaSelectBackdrop {
                select_entity,
            }
            UInteraction::default()
            ULayout {
                display: UDisplay::Flex,
            }
            Children [
                // Floating Menu Card
                UNode {
                    width: UVal::Px(menu_width),
                    height: UVal::Auto,
                    max_height: menu_height,
                    background_color: menu_bg,
                    border_radius: UCornerRadius::all(radius),
                    padding: USides::all(4.0),
                }
                USelf {
                    position_type: UPositionType::Absolute,
                    left: UVal::Px(menu_left),
                    top: UVal::Px(menu_top),
                    ..USelf::default()
                }
                UBorder {
                    color: menu_border,
                    width: 1.0,
                    radius: UCornerRadius::all(radius),
                }
                UShadow::drop(Color::srgba(0.0, 0.0, 0.0, 0.65), Vec2::new(0.0, 8.0), 16.0)
                LumaSelectMenu
                ULayout {
                    display: UDisplay::Flex,
                    flex_direction: UFlexDirection::Column,
                    gap: item_gap,
                }
                Children [
                    {options_scene}
                ]
            ]
        };

        s.resolve(context, scene)
    }
}

fn set_select_chevron(
    children: &Children,
    icon: &'static str,
    chevron_query: &mut Query<&mut UText, (With<LumaSelectChevron>, Without<LumaSelectText>)>,
    children_query: &Query<&Children>,
) {
    for &child in children {
        visit_descendants(child, children_query, |e| {
            if let Ok(mut text) = chevron_query.get_mut(e) {
                text.text = icon.to_string();
            }
            DescendantVisit::Continue
        });
    }
}

fn close_select_menu(
    commands: &mut Commands,
    select: &mut LumaSelect,
    children: &Children,
    chevron_query: &mut Query<&mut UText, (With<LumaSelectChevron>, Without<LumaSelectText>)>,
    children_query: &Query<&Children>,
) {
    select.is_open = false;
    if let Some(overlay) = select.overlay_entity.take() {
        if let Ok(mut cmd) = commands.get_entity(overlay) {
            cmd.try_despawn();
        }
    }
    set_select_chevron(children, Icon::CHEVRON_DOWN, chevron_query, children_query);
}

#[allow(clippy::too_many_arguments)]
fn toggle_select(
    trigger_entity: Entity,
    commands: &mut Commands,
    theme: Option<&Theme>,
    windows: &Query<&Window>,
    cameras: &Query<(&Camera, &GlobalTransform)>,
    parents_query: &Query<&ChildOf>,
    trigger_transform_query: &Query<(&GlobalTransform, &ComputedSize)>,
    select_query: &mut Query<(Entity, &mut LumaSelect, &Children)>,
    chevron_query: &mut Query<&mut UText, (With<LumaSelectChevron>, Without<LumaSelectText>)>,
    text_query: &Query<&UText, (With<LumaSelectText>, Without<LumaSelectChevron>)>,
    children_query: &Query<&Children>,
) {
    let Some(select_entity) = find_ancestor_or_self(trigger_entity, select_query, parents_query)
    else {
        return;
    };

    let Ok((_, mut select, children)) = select_query.get_mut(select_entity) else {
        return;
    };
    if select.disabled {
        return;
    }

    if select.is_open {
        close_select_menu(commands, &mut select, children, chevron_query, children_query);
        return;
    }

    // Close any other open select menus first
    for (other_entity, mut other_select, other_children) in select_query.iter_mut() {
        if other_entity != select_entity && other_select.is_open {
            close_select_menu(
                commands,
                &mut other_select,
                other_children,
                chevron_query,
                children_query,
            );
        }
    }

    // Re-fetch after the loop
    let Ok((_, mut select, children)) = select_query.get_mut(select_entity) else {
        return;
    };

    let window = windows.iter().next();
    let (win_w, win_h) = window.map(|w| (w.width(), w.height())).unwrap_or((1280.0, 720.0));

    let (trigger_w, trigger_h, screen_x, screen_y) = if let Ok((gt, computed)) =
        trigger_transform_query.get(select_entity)
    {
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
        (computed.width, computed.height, screen_pos.x, screen_pos.y)
    } else {
        (200.0, 34.0, win_w * 0.5, win_h * 0.5)
    };

    let half_w = trigger_w * 0.5;
    let half_h = trigger_h * 0.5;
    let trigger_left = screen_x - half_w;
    let trigger_top = screen_y - half_h;
    let trigger_bottom = screen_y + half_h;

    let option_count = select.options.len();
    if option_count == 0 {
        return;
    }
    let item_height = 34.0;
    let item_gap = 2.0;
    let menu_padding = 8.0;
    let menu_height = menu_padding
        + (option_count as f32) * item_height
        + ((option_count.saturating_sub(1)) as f32) * item_gap;
    let menu_width = trigger_w.max(120.0);

    let space_below = win_h - trigger_bottom;
    let space_above = trigger_top;
    let place_above = space_below < (menu_height + 8.0) && space_above > space_below;
    let menu_top = if place_above {
        (trigger_top - menu_height - 4.0).max(4.0)
    } else {
        (trigger_bottom + 4.0).min((win_h - menu_height - 4.0).max(4.0))
    };
    let menu_left = trigger_left.clamp(4.0, (win_w - menu_width - 4.0).max(4.0));

    let mut select_font = Handle::default();
    for &child in children {
        visit_descendants(child, children_query, |e| {
            if let Ok(text) = text_query.get(e) {
                if text.font != Handle::default() {
                    select_font = text.font.clone();
                    return DescendantVisit::Stop;
                }
            }
            DescendantVisit::Continue
        });
        if select_font != Handle::default() {
            break;
        }
    }
    let font = if select_font != Handle::default() {
        select_font
    } else {
        theme
            .map(|t| t.text.font.inter_regular.clone())
            .unwrap_or_default()
    };

    let overlay_scene = SelectOverlayScene {
        select_entity,
        menu_left,
        menu_top,
        menu_width,
        menu_height,
        item_height,
        item_gap,
        style: select.style.clone(),
        options: select.options.clone(),
        selected_index: select.selected_index,
        font,
    };

    let overlay_id = commands.spawn_scene(overlay_scene).id();
    select.overlay_entity = Some(overlay_id);
    select.is_open = true;

    set_select_chevron(children, Icon::CHEVRON_UP, chevron_query, children_query);
}

fn on_select_trigger_click(
    trigger: On<PointerClick>,
    mut commands: Commands,
    theme: Option<Res<Theme>>,
    trigger_query: Query<&LumaSelectTrigger>,
    parents_query: Query<&ChildOf>,
    windows: Query<&Window>,
    cameras: Query<(&Camera, &GlobalTransform)>,
    trigger_transform_query: Query<(&GlobalTransform, &ComputedSize)>,
    mut select_query: Query<(Entity, &mut LumaSelect, &Children)>,
    mut chevron_query: Query<&mut UText, (With<LumaSelectChevron>, Without<LumaSelectText>)>,
    text_query: Query<&UText, (With<LumaSelectText>, Without<LumaSelectChevron>)>,
    children_query: Query<&Children>,
) {
    if !is_primary_click(&trigger) {
        return;
    }

    if find_ancestor_or_self(trigger.entity, &trigger_query, &parents_query).is_none() {
        return;
    }

    toggle_select(
        trigger.entity,
        &mut commands,
        theme.as_deref(),
        &windows,
        &cameras,
        &parents_query,
        &trigger_transform_query,
        &mut select_query,
        &mut chevron_query,
        &text_query,
        &children_query,
    );
}

fn on_select_trigger_activate(
    trigger: On<UFocusActivate>,
    mut commands: Commands,
    theme: Option<Res<Theme>>,
    parents_query: Query<&ChildOf>,
    windows: Query<&Window>,
    cameras: Query<(&Camera, &GlobalTransform)>,
    trigger_transform_query: Query<(&GlobalTransform, &ComputedSize)>,
    mut select_query: Query<(Entity, &mut LumaSelect, &Children)>,
    mut chevron_query: Query<&mut UText, (With<LumaSelectChevron>, Without<LumaSelectText>)>,
    text_query: Query<&UText, (With<LumaSelectText>, Without<LumaSelectChevron>)>,
    children_query: Query<&Children>,
) {
    toggle_select(
        trigger.entity,
        &mut commands,
        theme.as_deref(),
        &windows,
        &cameras,
        &parents_query,
        &trigger_transform_query,
        &mut select_query,
        &mut chevron_query,
        &text_query,
        &children_query,
    );
}

fn on_select_option_click(
    trigger: On<PointerClick>,
    mut commands: Commands,
    option_query: Query<&LumaSelectOptionItem>,
    parents_query: Query<&ChildOf>,
    mut select_query: Query<(Entity, &mut LumaSelect, &Children)>,
    mut chevron_query: Query<&mut UText, (With<LumaSelectChevron>, Without<LumaSelectText>)>,
    mut text_query: Query<&mut UText, (With<LumaSelectText>, Without<LumaSelectChevron>)>,
    children_query: Query<&Children>,
) {
    if !is_primary_click(&trigger) {
        return;
    }

    let Some(opt_entity) = find_ancestor_or_self(trigger.entity, &option_query, &parents_query)
    else {
        return;
    };
    let Ok(opt_item) = option_query.get(opt_entity) else {
        return;
    };

    let select_entity = opt_item.select_entity;
    let opt_index = opt_item.index;
    let opt_value = opt_item.value.clone();
    let opt_label = opt_item.label.clone();

    let Ok((entity, mut select, children)) = select_query.get_mut(select_entity) else {
        return;
    };
    if select.disabled {
        return;
    }

    select.selected_index = Some(opt_index);
    close_select_menu(
        &mut commands,
        &mut select,
        children,
        &mut chevron_query,
        &children_query,
    );

    let text_color = select.style.text_color;
    for &child in children {
        visit_descendants(child, &children_query, |e| {
            if let Ok(mut text) = text_query.get_mut(e) {
                text.text = opt_label.clone();
                text.color = text_color;
            }
            DescendantVisit::Continue
        });
    }

    commands.trigger(SelectChanged {
        entity,
        selected_index: opt_index,
        value: opt_value,
    });
}

fn on_select_backdrop_click(
    trigger: On<PointerClick>,
    mut commands: Commands,
    backdrop_query: Query<&LumaSelectBackdrop>,
    parents_query: Query<&ChildOf>,
    menu_query: Query<&LumaSelectMenu>,
    mut select_query: Query<(&mut LumaSelect, &Children)>,
    mut chevron_query: Query<&mut UText, (With<LumaSelectChevron>, Without<LumaSelectText>)>,
    children_query: Query<&Children>,
) {
    if !is_primary_click(&trigger) {
        return;
    }

    // If the click is inside the menu card (or its children), ignore
    if find_ancestor_or_self(trigger.entity, &menu_query, &parents_query).is_some() {
        return;
    }

    let Some(backdrop_entity) =
        find_ancestor_or_self(trigger.entity, &backdrop_query, &parents_query)
    else {
        return;
    };
    let Ok(backdrop) = backdrop_query.get(backdrop_entity) else {
        return;
    };

    let select_entity = backdrop.select_entity;
    let Ok((mut select, children)) = select_query.get_mut(select_entity) else {
        return;
    };

    close_select_menu(
        &mut commands,
        &mut select,
        children,
        &mut chevron_query,
        &children_query,
    );
}

fn close_select_on_escape(
    mut commands: Commands,
    keyboard: Option<Res<ButtonInput<KeyCode>>>,
    mut select_query: Query<(&mut LumaSelect, &Children)>,
    mut chevron_query: Query<&mut UText, (With<LumaSelectChevron>, Without<LumaSelectText>)>,
    children_query: Query<&Children>,
) {
    let Some(keyboard) = keyboard else {
        return;
    };
    if keyboard.just_pressed(KeyCode::Escape) {
        for (mut select, children) in select_query.iter_mut() {
            if select.is_open {
                close_select_menu(
                    &mut commands,
                    &mut select,
                    children,
                    &mut chevron_query,
                    &children_query,
                );
            }
        }
    }
}

fn cleanup_orphaned_select_overlays(
    mut commands: Commands,
    overlay_query: Query<(Entity, &LumaSelectOverlay)>,
    select_query: Query<&LumaSelect>,
) {
    for (overlay_entity, overlay) in overlay_query.iter() {
        if !select_query.contains(overlay.select_entity) {
            if let Ok(mut cmd) = commands.get_entity(overlay_entity) {
                cmd.try_despawn();
            }
        }
    }
}

/// Plugin registering select dropdown systems and observers.
pub struct LumaSelectPlugin;

impl Plugin for LumaSelectPlugin {
    fn build(&self, app: &mut App) {
        app.register_type::<LumaSelect>()
            .register_type::<LumaSelectTrigger>()
            .register_type::<LumaSelectMenu>()
            .register_type::<LumaSelectText>()
            .register_type::<LumaSelectChevron>()
            .register_type::<LumaSelectOptionItem>()
            .register_type::<LumaSelectOverlay>()
            .register_type::<LumaSelectBackdrop>()
            .add_systems(Update, (close_select_on_escape, cleanup_orphaned_select_overlays))
            .add_observer(on_select_trigger_click)
            .add_observer(on_select_trigger_activate)
            .add_observer(on_select_option_click)
            .add_observer(on_select_backdrop_click);
    }
}
