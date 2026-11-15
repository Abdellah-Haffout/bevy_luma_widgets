//! Navigation and structural widgets for Bevy Luma.

use bevy::prelude::*;
use bevy::scene::{ResolveContext, ResolvedScene, ResolveSceneError};
use bevy_luma::prelude::*;

use crate::avatar::LumaAvatar;
use crate::button::{
    apply_button_variant_visuals, Button, ButtonClicked, LumaButton, LumaButtonIcon,
    LumaButtonLabel,
};
use crate::helpers::{
    compute_overlay_anchor, find_ancestor, find_ancestor_or_self, interaction_colors,
    is_primary_click, visit_descendants, ConditionalScene, DescendantVisit,
};
use crate::prelude::*;

/// Helper wrapper allowing dynamic vectors of scenes to resolve sequentially into a BSN container.
pub struct SceneList<T: Scene>(pub Vec<T>);

impl<T: Scene> SceneList<T> {
    pub fn new(items: Vec<T>) -> Self {
        Self(items)
    }
}

impl<T: Scene> bevy::scene::SceneList for SceneList<T> {
    fn resolve_list(
        self,
        context: &mut ResolveContext,
        scenes: &mut Vec<ResolvedScene>,
    ) -> Result<(), ResolveSceneError> {
        self.0.resolve_list(context, scenes)
    }

    fn register_dependencies(&self, dependencies: &mut bevy::scene::SceneDependencies) {
        self.0.register_dependencies(dependencies);
    }
}

// ============================================================================
// 2.1 Navbar / Top App Bar
// ============================================================================

/// Top navigation bar with brand, page links, and action slots.
#[derive(Clone, Debug, Reflect)]
pub struct Navbar {
    pub brand: String,
    pub items: Vec<String>,
    pub active_index: usize,
    pub font: Handle<Font>,
    pub icon_font: Handle<Font>,
}

impl Default for Navbar {
    fn default() -> Self {
        Self {
            brand: "Bevy App".into(),
            items: vec!["Home".into(), "Features".into(), "Docs".into(), "Pricing".into()],
            active_index: 0,
            font: Handle::default(),
            icon_font: Handle::default(),
        }
    }
}

impl Navbar {
    pub fn new(brand: impl Into<String>) -> Self {
        Self {
            brand: brand.into(),
            ..default()
        }
    }

    pub fn items(mut self, items: Vec<String>) -> Self {
        self.items = items;
        self
    }

    pub fn active(mut self, index: usize) -> Self {
        self.active_index = index;
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
}

impl Scene for Navbar {
    fn resolve(
        self,
        context: &mut ResolveContext,
        scene: &mut ResolvedScene,
    ) -> Result<(), ResolveSceneError> {
        let brand = self.brand;
        let font = self.font;
        let icon_font = self.icon_font;
        let active_index = self.active_index;

        let nav_links: Vec<_> = self.items.into_iter().enumerate().map(|(idx, item)| {
            let is_active = idx == active_index;
            let text_color = if is_active {
                Color::srgb(0.96, 0.97, 0.99)
            } else {
                Color::srgb(0.65, 0.70, 0.80)
            };
            let weight_color = if is_active {
                Color::srgb(0.35, 0.65, 1.0)
            } else {
                Color::NONE
            };
            let font_c = font.clone();
            let item_label = item.clone();
            let link_colors = interaction_colors(
                Color::NONE,
                Color::srgba(1.0, 1.0, 1.0, 0.06),
                Color::srgba(1.0, 1.0, 1.0, 0.12),
                false,
            );
            bsn! {
                LumaNavbarItem {
                    index: idx,
                    label: item_label,
                }
                UNode {
                    padding: USides::axes(12.0, 8.0),
                    border_radius: UCornerRadius::all(6.0),
                }
                UBorder {
                    color: weight_color,
                    width: 2.0,
                }
                UInteraction::default()
                link_colors
                UFocusable::default()
                ULayout {
                    display: UDisplay::Flex,
                    align_items: UAlignItems::Center,
                }
                Children [
                    UText {
                        text: item,
                        font_size: 14.0,
                        color: text_color,
                        font: font_c,
                    }
                ]
            }
        }).collect();

        let nav_links_scene = SceneList::new(nav_links);
        let brand_font = font.clone();
        let signin_font = font.clone();
        let luma_navbar = LumaNavbar { active_index };
        let s = bsn! {
            luma_navbar
            UNode {
                width: UVal::Percent(1.0),
                height: UVal::Px(56.0),
                padding: USides::axes(20.0, 0.0),
                background_color: Color::srgba(0.08, 0.09, 0.13, 0.95),
            }
            UBorder {
                color: Color::srgba(1.0, 1.0, 1.0, 0.08),
                width: 1.0,
            }
            ULayout {
                display: UDisplay::Flex,
                flex_direction: UFlexDirection::Row,
                justify_content: UJustifyContent::SpaceBetween,
                align_items: UAlignItems::Center,
            }
            Children [
                // Brand / Logo
                UNode::default()
                ULayout {
                    display: UDisplay::Flex,
                    flex_direction: UFlexDirection::Row,
                    align_items: UAlignItems::Center,
                    gap: 8.0,
                }
                Children [
                    @{Button::icon_button(Icon::STAR).size(ButtonSize::Small).icon_font(icon_font)}
                    --
                    UText {
                        text: brand,
                        font_size: 16.0,
                        color: Color::srgb(0.96, 0.97, 0.99),
                        font: brand_font,
                    }
                ]
                --
                // Navigation links
                UNode::default()
                ULayout {
                    display: UDisplay::Flex,
                    flex_direction: UFlexDirection::Row,
                    gap: 4.0,
                    align_items: UAlignItems::Center,
                }
                Children [
                    {nav_links_scene}
                ]
                --
                // Right actions / Profile
                UNode::default()
                ULayout {
                    display: UDisplay::Flex,
                    flex_direction: UFlexDirection::Row,
                    gap: 8.0,
                    align_items: UAlignItems::Center,
                }
                Children [
                    LumaNavbarSignIn
                    UNode::default()
                    Children [
                        @{Button::new("Sign In").outline().size(ButtonSize::Small).font(signin_font)}
                    ]
                    --
                    @{Avatar::new("AD").size(AvatarSize::Small).font(font)}
                ]
            ]
        };
        s.resolve(context, scene)
    }
}

// ============================================================================
// 2.2 Bottom Navigation Bar
// ============================================================================

/// Mobile bottom navigation bar with icon & label destinations.
#[derive(Clone, Debug, Reflect)]
pub struct BottomNav {
    pub items: Vec<(String, String)>, // (icon, label)
    pub active_index: usize,
    pub font: Handle<Font>,
    pub icon_font: Handle<Font>,
}

impl Default for BottomNav {
    fn default() -> Self {
        Self {
            items: vec![
                (Icon::STAR.to_string(), "Home".into()),
                (Icon::SEARCH.to_string(), "Search".into()),
                (Icon::BELL.to_string(), "Alerts".into()),
                (Icon::USER.to_string(), "Profile".into()),
            ],
            active_index: 0,
            font: Handle::default(),
            icon_font: Handle::default(),
        }
    }
}

impl BottomNav {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn items(mut self, items: Vec<(String, String)>) -> Self {
        self.items = items;
        self
    }

    pub fn active(mut self, index: usize) -> Self {
        self.active_index = index;
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
}

impl Scene for BottomNav {
    fn resolve(
        self,
        context: &mut ResolveContext,
        scene: &mut ResolvedScene,
    ) -> Result<(), ResolveSceneError> {
        let active_index = self.active_index;
        let font = self.font;
        let icon_font = self.icon_font;

        let item_colors = interaction_colors(
            Color::NONE,
            Color::srgba(1.0, 1.0, 1.0, 0.06),
            Color::srgba(1.0, 1.0, 1.0, 0.12),
            false,
        );
        let nav_items: Vec<_> = self.items.into_iter().enumerate().map(|(idx, (icon, label))| {
            let is_active = idx == active_index;
            let color = if is_active {
                Color::srgb(0.35, 0.65, 1.0)
            } else {
                Color::srgb(0.55, 0.60, 0.70)
            };
            let f = font.clone();
            let ifont = icon_font.clone();
            let label_clone = label.clone();
            let ic = item_colors.clone();
            bsn! {
                LumaBottomNavItem {
                    index: idx,
                    label: label_clone,
                }
                UNode {
                    padding: USides::axes(10.0, 4.0),
                    border_radius: UCornerRadius::all(8.0),
                }
                UInteraction::default()
                ic
                UFocusable::default()
                ULayout {
                    display: UDisplay::Flex,
                    flex_direction: UFlexDirection::Column,
                    align_items: UAlignItems::Center,
                    justify_content: UJustifyContent::Center,
                    gap: 4.0,
                }
                Children [
                    UText {
                        text: icon,
                        font_size: 18.0,
                        color,
                        font: ifont,
                    }
                    --
                    UText {
                        text: label,
                        font_size: 11.0,
                        color,
                        font: f,
                    }
                ]
            }
        }).collect();

        let nav_items_scene = SceneList::new(nav_items);
        let luma_bnav = LumaBottomNav { active_index };
        let s = bsn! {
            luma_bnav
            UNode {
                width: UVal::Percent(1.0),
                height: UVal::Px(58.0),
                background_color: Color::srgba(0.08, 0.09, 0.13, 0.98),
            }
            UBorder {
                color: Color::srgba(1.0, 1.0, 1.0, 0.08),
                width: 1.0,
            }
            ULayout {
                display: UDisplay::Flex,
                flex_direction: UFlexDirection::Row,
                justify_content: UJustifyContent::SpaceAround,
                align_items: UAlignItems::Center,
            }
            Children [
                {nav_items_scene}
            ]
        };
        s.resolve(context, scene)
    }
}

// ============================================================================
// 2.3 Sidebar / Navigation Drawer
// ============================================================================

/// Vertical navigation sidebar with brand header and menu items.
#[derive(Clone, Debug, Reflect)]
pub struct Sidebar {
    pub brand: String,
    pub items: Vec<(String, String)>, // (icon, label)
    pub active_index: usize,
    pub collapsed: bool,
    pub font: Handle<Font>,
    pub icon_font: Handle<Font>,
}

impl Default for Sidebar {
    fn default() -> Self {
        Self {
            brand: "Dashboard".into(),
            items: vec![
                (Icon::STAR.to_string(), "Overview".into()),
                (Icon::FOLDER.to_string(), "Projects".into()),
                (Icon::USER.to_string(), "Team".into()),
                (Icon::SETTINGS.to_string(), "Settings".into()),
            ],
            active_index: 0,
            collapsed: false,
            font: Handle::default(),
            icon_font: Handle::default(),
        }
    }
}

impl Sidebar {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn brand(mut self, brand: impl Into<String>) -> Self {
        self.brand = brand.into();
        self
    }

    pub fn items(mut self, items: Vec<(String, String)>) -> Self {
        self.items = items;
        self
    }

    pub fn active(mut self, index: usize) -> Self {
        self.active_index = index;
        self
    }

    pub fn collapsed(mut self, collapsed: bool) -> Self {
        self.collapsed = collapsed;
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
}

impl Scene for Sidebar {
    fn resolve(
        self,
        context: &mut ResolveContext,
        scene: &mut ResolvedScene,
    ) -> Result<(), ResolveSceneError> {
        let active_index = self.active_index;
        let font = self.font;
        let icon_font = self.icon_font;
        let brand = self.brand;

        let collapsed = self.collapsed;
        let active_bg = Color::srgba(0.35, 0.65, 1.0, 0.15);
        let luma_sidebar = LumaSidebar {
            active_index,
            collapsed,
        };
        let menu_items: Vec<_> = self.items.into_iter().enumerate().map(|(idx, (icon, label))| {
            let is_active = idx == active_index;
            let bg = if is_active { active_bg } else { Color::NONE };
            let text_color = if is_active { Color::srgb(0.35, 0.65, 1.0) } else { Color::srgb(0.70, 0.75, 0.85) };
            let item_colors = interaction_colors(
                bg,
                Color::srgba(1.0, 1.0, 1.0, 0.06),
                Color::srgba(1.0, 1.0, 1.0, 0.12),
                false,
            );
            let f = font.clone();
            let ifont = icon_font.clone();
            let item = LumaSidebarItem {
                index: idx,
                label: label.clone(),
            };
            let pad = if collapsed { USides::all(0.0) } else { USides::axes(12.0, 0.0) };
            let justify = if collapsed { UJustifyContent::Center } else { UJustifyContent::Start };
            let gap = if collapsed { 0.0_f32 } else { 12.0_f32 };
            let label_scene = ConditionalScene((!collapsed).then(move || {
                bsn! {
                    UText {
                        text: label,
                        font_size: 14.0,
                        color: text_color,
                        font: f,
                    }
                }
            }));
            bsn! {
                item
                UNode {
                    height: UVal::Px(40.0),
                    padding: pad,
                    background_color: bg,
                    border_radius: UCornerRadius::all(6.0),
                }
                UInteraction::default()
                item_colors
                UFocusable::default()
                ULayout {
                    display: UDisplay::Flex,
                    flex_direction: UFlexDirection::Row,
                    justify_content: justify,
                    align_items: UAlignItems::Center,
                    gap: gap,
                }
                Children [
                    UText {
                        text: icon,
                        font_size: 16.0,
                        color: text_color,
                        font: ifont,
                    }
                    --
                    @{label_scene}
                ]
            }
        }).collect();

        let menu_items_scene = SceneList::new(menu_items);
        let header_font = font.clone();
        let sidebar_width = if collapsed { 64.0_f32 } else { 240.0_f32 };
        let header_scene = ConditionalScene((!collapsed).then(|| {
            bsn! {
                UNode {
                    height: UVal::Px(40.0),
                    padding: USides::axes(8.0, 0.0),
                }
                ULayout {
                    display: UDisplay::Flex,
                    align_items: UAlignItems::Center,
                }
                Children [
                    UText {
                        text: brand,
                        font_size: 16.0,
                        color: Color::srgb(0.96, 0.97, 0.99),
                        font: header_font,
                    }
                ]
            }
        }));
        let s = bsn! {
            luma_sidebar
            UNode {
                width: UVal::Px(sidebar_width),
                height: UVal::Percent(1.0),
                padding: USides::all(16.0),
                background_color: Color::srgba(0.07, 0.08, 0.12, 0.98),
            }
            UBorder {
                color: Color::srgba(1.0, 1.0, 1.0, 0.08),
                width: 1.0,
            }
            ULayout {
                display: UDisplay::Flex,
                flex_direction: UFlexDirection::Column,
                justify_content: UJustifyContent::SpaceBetween,
            }
            Children [
                UNode::default()
                ULayout {
                    display: UDisplay::Flex,
                    flex_direction: UFlexDirection::Column,
                    gap: 8.0,
                }
                Children [
                    @{header_scene}
                    --
                    {menu_items_scene}
                ]
                --
                UNode {
                    padding: USides::all(8.0),
                }
                UBorder {
                    color: Color::srgba(1.0, 1.0, 1.0, 0.08),
                    width: 1.0,
                }
                ULayout {
                    display: UDisplay::Flex,
                    flex_direction: UFlexDirection::Row,
                    align_items: UAlignItems::Center,
                    gap: 8.0,
                }
                Children [
                    @{Avatar::new("US").size(AvatarSize::Small).font(font)}
                ]
            ]
        };
        s.resolve(context, scene)
    }
}

// ============================================================================
// 2.4 Breadcrumbs Wrapper
// ============================================================================

/// Declarative Breadcrumbs trail for multiple path items.
#[derive(Clone, Debug, Reflect)]
pub struct Breadcrumbs {
    pub items: Vec<String>,
    pub font: Handle<Font>,
    pub icon_font: Handle<Font>,
}

impl Default for Breadcrumbs {
    fn default() -> Self {
        Self {
            items: vec!["Home".into(), "Projects".into(), "Luma UI".into()],
            font: Handle::default(),
            icon_font: Handle::default(),
        }
    }
}

impl Breadcrumbs {
    pub fn new(items: Vec<impl Into<String>>) -> Self {
        Self {
            items: items.into_iter().map(|s| s.into()).collect(),
            ..default()
        }
    }

    pub fn font(mut self, font: Handle<Font>) -> Self {
        self.font = font;
        self
    }

    pub fn icon_font(mut self, icon_font: Handle<Font>) -> Self {
        self.icon_font = icon_font;
        self
    }
}

impl Scene for Breadcrumbs {
    fn resolve(
        self,
        context: &mut ResolveContext,
        scene: &mut ResolvedScene,
    ) -> Result<(), ResolveSceneError> {
        let total = self.items.len();
        let font = self.font;
        let icon_font = self.icon_font;

        let mut crumb_nodes = Vec::new();
        for (idx, item) in self.items.into_iter().enumerate() {
            let is_last = idx + 1 == total;
            let f = font.clone();
            let crumb = BreadcrumbItem::new(format!("crumb_{idx}"), item)
                .index(idx)
                .current(is_last)
                .font(f);
            let sep = ConditionalScene((!is_last).then(|| BreadcrumbSeparator::new(icon_font.clone())));
            crumb_nodes.push(bsn! {
                UNode::default()
                ULayout {
                    display: UDisplay::Flex,
                    flex_direction: UFlexDirection::Row,
                    align_items: UAlignItems::Center,
                    gap: 4.0,
                }
                Children [
                    @{crumb}
                    --
                    @{sep}
                ]
            });
        }

        let crumb_nodes_scene = SceneList::new(crumb_nodes);
        let s = bsn! {
            LumaBreadcrumb
            UNode::default()
            ULayout {
                display: UDisplay::Flex,
                flex_direction: UFlexDirection::Row,
                align_items: UAlignItems::Center,
                gap: 4.0,
            }
            Children [
                {crumb_nodes_scene}
            ]
        };
        s.resolve(context, scene)
    }
}

// ============================================================================
// 2.6 Pagination
// ============================================================================

/// Pagination row with previous, page number pills, and next buttons.
#[derive(Clone, Debug, Reflect)]
pub struct Pagination {
    pub current_page: usize,
    pub total_pages: usize,
    pub font: Handle<Font>,
    pub icon_font: Handle<Font>,
}

impl Default for Pagination {
    fn default() -> Self {
        Self {
            current_page: 1,
            total_pages: 5,
            font: Handle::default(),
            icon_font: Handle::default(),
        }
    }
}

impl Pagination {
    pub fn new(current: usize, total: usize) -> Self {
        Self {
            current_page: current,
            total_pages: total,
            ..default()
        }
    }

    pub fn font(mut self, font: Handle<Font>) -> Self {
        self.font = font;
        self
    }

    pub fn icon_font(mut self, icon_font: Handle<Font>) -> Self {
        self.icon_font = icon_font;
        self
    }
}

impl Scene for Pagination {
    fn resolve(
        self,
        context: &mut ResolveContext,
        scene: &mut ResolvedScene,
    ) -> Result<(), ResolveSceneError> {
        let total_pages = self.total_pages.max(1);
        let current = self.current_page.clamp(1, total_pages);
        let font = self.font;
        let icon_font = self.icon_font;
        let prev_icon_font = icon_font.clone();
        let luma_pag = LumaPagination {
            current_page: current,
            total_pages,
        };

        let page_buttons: Vec<_> = (1..=total_pages).map(|p| {
            let is_active = p == current;
            let f = font.clone();
            let btn = if is_active {
                Button::new(p.to_string()).size(ButtonSize::Small).font(f)
            } else {
                Button::new(p.to_string()).outline().size(ButtonSize::Small).font(f)
            };
            bsn! {
                LumaPaginationBtn {
                    action: PaginationAction::Page(p),
                }
                UNode::default()
                Children [ @{btn} ]
            }
        }).collect();

        let page_buttons_scene = SceneList::new(page_buttons);
        let s = bsn! {
            luma_pag
            UNode::default()
            ULayout {
                display: UDisplay::Flex,
                flex_direction: UFlexDirection::Row,
                align_items: UAlignItems::Center,
                gap: 6.0,
            }
            Children [
                LumaPaginationBtn { action: PaginationAction::Prev }
                UNode::default()
                Children [
                    @{Button::icon_button(Icon::CHEVRON_LEFT).outline().size(ButtonSize::Small).icon_font(prev_icon_font)}
                ]
                --
                UNode::default()
                ULayout {
                    display: UDisplay::Flex,
                    flex_direction: UFlexDirection::Row,
                    gap: 4.0,
                }
                Children [
                    {page_buttons_scene}
                ]
                --
                LumaPaginationBtn { action: PaginationAction::Next }
                UNode::default()
                Children [
                    @{Button::icon_button(Icon::CHEVRON_RIGHT).outline().size(ButtonSize::Small).icon_font(icon_font)}
                ]
            ]
        };
        s.resolve(context, scene)
    }
}

// ============================================================================
// 2.7 StepTracker / Progress Stepper
// ============================================================================

/// Multi-step workflow progression tracker.
#[derive(Clone, Debug, Reflect)]
pub struct StepTracker {
    pub steps: Vec<String>,
    pub current_step: usize,
    pub font: Handle<Font>,
    pub icon_font: Handle<Font>,
}

impl Default for StepTracker {
    fn default() -> Self {
        Self {
            steps: vec!["Details".into(), "Payment".into(), "Review".into()],
            current_step: 1,
            font: Handle::default(),
            icon_font: Handle::default(),
        }
    }
}

impl StepTracker {
    pub fn new(steps: Vec<impl Into<String>>) -> Self {
        Self {
            steps: steps.into_iter().map(|s| s.into()).collect(),
            ..default()
        }
    }

    pub fn current(mut self, step: usize) -> Self {
        self.current_step = step;
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
}

impl Scene for StepTracker {
    fn resolve(
        self,
        context: &mut ResolveContext,
        scene: &mut ResolvedScene,
    ) -> Result<(), ResolveSceneError> {
        let total = self.steps.len();
        let current = if total == 0 { 0 } else { self.current_step.min(total - 1) };
        let font = self.font;
        let icon_font = self.icon_font;
        let luma_stepper = LumaStepTracker {
            current_step: current,
            total_steps: total,
        };

        let step_nodes: Vec<_> = self.steps.into_iter().enumerate().map(|(idx, title)| {
            let is_done = idx < current;
            let is_active = idx == current;
            let circle_bg = if is_done || is_active {
                Color::srgb(0.35, 0.65, 1.0)
            } else {
                Color::srgba(0.2, 0.22, 0.3, 0.6)
            };
            let text_color = if is_active {
                Color::srgb(0.96, 0.97, 0.99)
            } else {
                Color::srgb(0.6, 0.65, 0.75)
            };
            let f = font.clone();
            let ifont = icon_font.clone();
            let step_num = (idx + 1).to_string();
            let check_text = Icon::CHECK.to_string();
            let digit_display = if is_done { UDisplay::None } else { UDisplay::Flex };
            let check_display = if is_done { UDisplay::Flex } else { UDisplay::None };

            let line_color = if idx < current {
                Color::srgb(0.35, 0.65, 1.0)
            } else {
                Color::srgba(0.2, 0.22, 0.3, 0.6)
            };
            let line_w = if idx + 1 < total { UVal::Px(32.0) } else { UVal::Px(0.0) };
            let title_clone = title.clone();
            let row_colors = interaction_colors(
                Color::NONE,
                Color::srgba(1.0, 1.0, 1.0, 0.04),
                Color::srgba(1.0, 1.0, 1.0, 0.08),
                false,
            );

            bsn! {
                LumaStepItem {
                    index: idx,
                    title: title_clone,
                }
                UNode {
                    padding: USides::axes(4.0, 2.0),
                    border_radius: UCornerRadius::all(6.0),
                }
                UInteraction::default()
                row_colors
                UFocusable::default()
                ULayout {
                    display: UDisplay::Flex,
                    flex_direction: UFlexDirection::Row,
                    align_items: UAlignItems::Center,
                    gap: 8.0,
                }
                Children [
                    // Circle node
                    LumaStepCircle
                    UNode {
                        width: UVal::Px(28.0),
                        height: UVal::Px(28.0),
                        border_radius: UCornerRadius::all(14.0),
                        background_color: circle_bg,
                    }
                    ULayout {
                        display: UDisplay::Flex,
                        justify_content: UJustifyContent::Center,
                        align_items: UAlignItems::Center,
                    }
                    Children [
                        LumaStepCircleText
                        UText {
                            text: step_num,
                            font_size: 12.0,
                            color: Color::WHITE,
                            font: { f.clone() },
                        }
                        ULayout {
                            display: digit_display,
                        }
                        --
                        LumaStepCircleCheck
                        UText {
                            text: check_text,
                            font_size: 12.0,
                            color: Color::WHITE,
                            font: ifont,
                        }
                        ULayout {
                            display: check_display,
                        }
                    ]
                    --
                    // Title
                    LumaStepTitle
                    UText {
                        text: title,
                        font_size: 13.0,
                        color: text_color,
                        font: f,
                    }
                    --
                    // Connecting line if not last
                    LumaStepLine
                    UNode {
                        width: line_w,
                        height: UVal::Px(2.0),
                        background_color: line_color,
                        margin: USides::axes(4.0, 0.0),
                    }
                ]
            }
        }).collect();

        let step_nodes_scene = SceneList::new(step_nodes);
        let s = bsn! {
            luma_stepper
            UNode::default()
            ULayout {
                display: UDisplay::Flex,
                flex_direction: UFlexDirection::Row,
                align_items: UAlignItems::Center,
                gap: 8.0,
            }
            Children [
                {step_nodes_scene}
            ]
        };
        s.resolve(context, scene)
    }
}

// ============================================================================
// 2.8 Hamburger Menu Icon
// ============================================================================

/// Hamburger menu button toggling between 3-bars and X.
#[derive(Clone, Debug, Reflect)]
pub struct HamburgerMenu {
    pub is_open: bool,
    pub icon_font: Handle<Font>,
}

impl Default for HamburgerMenu {
    fn default() -> Self {
        Self {
            is_open: false,
            icon_font: Handle::default(),
        }
    }
}

impl HamburgerMenu {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn open(mut self, is_open: bool) -> Self {
        self.is_open = is_open;
        self
    }

    pub fn icon_font(mut self, icon_font: Handle<Font>) -> Self {
        self.icon_font = icon_font;
        self
    }
}

impl Scene for HamburgerMenu {
    fn resolve(
        self,
        context: &mut ResolveContext,
        scene: &mut ResolvedScene,
    ) -> Result<(), ResolveSceneError> {
        let is_open = self.is_open;
        let icon = if is_open { Icon::X } else { Icon::MENU };
        let btn = Button::icon_button(icon)
            .ghost()
            .icon_font(self.icon_font);
        let s = bsn! {
            LumaHamburgerMenu { is_open }
            UNode::default()
            Children [ @{btn} ]
        };
        s.resolve(context, scene)
    }
}

// ============================================================================
// 2.9 Mega Menu
// ============================================================================

/// Multi-column categorized mega menu popover panel.
#[derive(Clone, Debug, Reflect)]
pub struct MegaMenu {
    pub title: String,
    pub categories: Vec<(String, Vec<String>)>,
    pub font: Handle<Font>,
}

impl Default for MegaMenu {
    fn default() -> Self {
        Self {
            title: "Products & Solutions".into(),
            categories: vec![
                ("Design System".into(), vec!["Components".into(), "Icons".into(), "Tokens".into()]),
                ("Framework".into(), vec!["SDF Pipeline".into(), "BSN Macro".into(), "ECS Layout".into()]),
                ("Ecosystem".into(), vec!["Community".into(), "Plugins".into(), "Showcase".into()]),
            ],
            font: Handle::default(),
        }
    }
}

impl MegaMenu {
    pub fn new(title: impl Into<String>) -> Self {
        Self {
            title: title.into(),
            ..default()
        }
    }

    pub fn categories(mut self, cats: Vec<(String, Vec<String>)>) -> Self {
        self.categories = cats;
        self
    }

    pub fn font(mut self, font: Handle<Font>) -> Self {
        self.font = font;
        self
    }
}

impl Scene for MegaMenu {
    fn resolve(
        self,
        context: &mut ResolveContext,
        scene: &mut ResolvedScene,
    ) -> Result<(), ResolveSceneError> {
        let font = self.font;
        let cols: Vec<_> = self.categories.into_iter().map(|(cat_title, links)| {
            let f1 = font.clone();
            let group_name = cat_title.clone();
            let links_nodes: Vec<_> = links.into_iter().map(|lnk| {
                let f2 = font.clone();
                let link_item = LumaNavLinkItem {
                    group: group_name.clone(),
                    label: lnk.clone(),
                };
                let ic = interaction_colors(
                    Color::NONE,
                    Color::srgba(1.0, 1.0, 1.0, 0.06),
                    Color::srgba(1.0, 1.0, 1.0, 0.12),
                    false,
                );
                bsn! {
                    link_item
                    UNode {
                        padding: USides::axes(6.0, 4.0),
                        border_radius: UCornerRadius::all(4.0),
                    }
                    UInteraction::default()
                    ic
                    UFocusable::default()
                    Children [
                        UText {
                            text: lnk,
                            font_size: 13.0,
                            color: Color::srgb(0.70, 0.75, 0.85),
                            font: f2,
                        }
                    ]
                }
            }).collect();

            let links_scene = SceneList::new(links_nodes);
            bsn! {
                UNode {
                    width: UVal::Px(160.0),
                }
                ULayout {
                    display: UDisplay::Flex,
                    flex_direction: UFlexDirection::Column,
                    gap: 6.0,
                }
                Children [
                    UText {
                        text: cat_title,
                        font_size: 14.0,
                        color: Color::srgb(0.96, 0.97, 0.99),
                        font: f1,
                    }
                    --
                    UNode::default()
                    ULayout {
                        display: UDisplay::Flex,
                        flex_direction: UFlexDirection::Column,
                        gap: 4.0,
                    }
                    Children [
                        {links_scene}
                    ]
                ]
            }
        }).collect();

        let cols_scene = SceneList::new(cols);
        let s = bsn! {
            UNode {
                padding: USides::all(20.0),
                background_color: Color::srgba(0.09, 0.10, 0.15, 0.98),
                border_radius: UCornerRadius::all(8.0),
            }
            UBorder {
                color: Color::srgba(1.0, 1.0, 1.0, 0.12),
                width: 1.0,
                radius: UCornerRadius::all(8.0),
            }
            ULayout {
                display: UDisplay::Flex,
                flex_direction: UFlexDirection::Row,
                gap: 24.0,
            }
            Children [
                {cols_scene}
            ]
        };
        s.resolve(context, scene)
    }
}

// ============================================================================
// 2.10 Footer
// ============================================================================

/// Site footer with multi-column links and copyright row.
#[derive(Clone, Debug, Reflect)]
pub struct Footer {
    pub copyright: String,
    pub columns: Vec<(String, Vec<String>)>,
    pub font: Handle<Font>,
}

impl Default for Footer {
    fn default() -> Self {
        Self {
            copyright: "© 2026 Bevy Luma UI. All rights reserved.".into(),
            columns: vec![
                ("Product".into(), vec!["Overview".into(), "Features".into(), "Releases".into()]),
                ("Resources".into(), vec!["Docs".into(), "Learn".into(), "Blog".into()]),
                ("Company".into(), vec!["About".into(), "Careers".into(), "Contact".into()]),
            ],
            font: Handle::default(),
        }
    }
}

impl Footer {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn copyright(mut self, cr: impl Into<String>) -> Self {
        self.copyright = cr.into();
        self
    }

    pub fn columns(mut self, cols: Vec<(String, Vec<String>)>) -> Self {
        self.columns = cols;
        self
    }

    pub fn font(mut self, font: Handle<Font>) -> Self {
        self.font = font;
        self
    }
}

impl Scene for Footer {
    fn resolve(
        self,
        context: &mut ResolveContext,
        scene: &mut ResolvedScene,
    ) -> Result<(), ResolveSceneError> {
        let font = self.font;
        let copyright = self.copyright;

        let col_nodes: Vec<_> = self.columns.into_iter().map(|(col_title, items)| {
            let f = font.clone();
            let group_name = col_title.clone();
            let item_nodes: Vec<_> = items.into_iter().map(|item| {
                let f2 = font.clone();
                let link_item = LumaNavLinkItem {
                    group: group_name.clone(),
                    label: item.clone(),
                };
                let ic = interaction_colors(
                    Color::NONE,
                    Color::srgba(1.0, 1.0, 1.0, 0.06),
                    Color::srgba(1.0, 1.0, 1.0, 0.12),
                    false,
                );
                bsn! {
                    link_item
                    UNode {
                        padding: USides::axes(6.0, 2.0),
                        border_radius: UCornerRadius::all(4.0),
                    }
                    UInteraction::default()
                    ic
                    UFocusable::default()
                    Children [
                        UText {
                            text: item,
                            font_size: 13.0,
                            color: Color::srgb(0.60, 0.65, 0.75),
                            font: f2,
                        }
                    ]
                }
            }).collect();

            let item_nodes_scene = SceneList::new(item_nodes);
            bsn! {
                UNode::default()
                ULayout {
                    display: UDisplay::Flex,
                    flex_direction: UFlexDirection::Column,
                    gap: 8.0,
                }
                Children [
                    UText {
                        text: col_title,
                        font_size: 14.0,
                        color: Color::srgb(0.96, 0.97, 0.99),
                        font: f,
                    }
                    --
                    UNode::default()
                    ULayout {
                        display: UDisplay::Flex,
                        flex_direction: UFlexDirection::Column,
                        gap: 6.0,
                    }
                    Children [
                        {item_nodes_scene}
                    ]
                ]
            }
        }).collect();

        let col_nodes_scene = SceneList::new(col_nodes);
        let s = bsn! {
            UNode {
                width: UVal::Percent(1.0),
                padding: USides::all(32.0),
                background_color: Color::srgba(0.06, 0.07, 0.10, 0.98),
            }
            UBorder {
                color: Color::srgba(1.0, 1.0, 1.0, 0.08),
                width: 1.0,
            }
            ULayout {
                display: UDisplay::Flex,
                flex_direction: UFlexDirection::Column,
                gap: 24.0,
            }
            Children [
                UNode::default()
                ULayout {
                    display: UDisplay::Flex,
                    flex_direction: UFlexDirection::Row,
                    justify_content: UJustifyContent::SpaceBetween,
                }
                Children [
                    {col_nodes_scene}
                ]
                --
                @{Divider::horizontal()}
                --
                UText {
                    text: copyright,
                    font_size: 12.0,
                    color: Color::srgb(0.45, 0.50, 0.60),
                    font,
                }
            ]
        };
        s.resolve(context, scene)
    }
}

// ============================================================================
// 2.11 Context Menu
// ============================================================================

/// Right-click contextual popover menu.
#[derive(Clone, Debug, Reflect)]
pub struct ContextMenu {
    pub items: Vec<String>,
    pub font: Handle<Font>,
    pub icon_font: Handle<Font>,
}

impl Default for ContextMenu {
    fn default() -> Self {
        Self {
            items: vec!["Cut".into(), "Copy".into(), "Paste".into(), "Delete".into()],
            font: Handle::default(),
            icon_font: Handle::default(),
        }
    }
}

impl ContextMenu {
    pub fn new(items: Vec<impl Into<String>>) -> Self {
        Self {
            items: items.into_iter().map(|s| s.into()).collect(),
            ..default()
        }
    }

    pub fn font(mut self, font: Handle<Font>) -> Self {
        self.font = font;
        self
    }

    pub fn icon_font(mut self, icon_font: Handle<Font>) -> Self {
        self.icon_font = icon_font;
        self
    }
}

impl Scene for ContextMenu {
    fn resolve(
        self,
        context: &mut ResolveContext,
        scene: &mut ResolvedScene,
    ) -> Result<(), ResolveSceneError> {
        let font = self.font;
        let rows: Vec<_> = self.items.into_iter().enumerate().map(|(idx, item)| {
            let f = font.clone();
            let menu_item = LumaContextMenuItem {
                index: idx,
                label: item.clone(),
            };
            let ic = interaction_colors(
                Color::NONE,
                Color::srgba(1.0, 1.0, 1.0, 0.08),
                Color::srgba(1.0, 1.0, 1.0, 0.14),
                false,
            );
            bsn! {
                menu_item
                UNode {
                    padding: USides::axes(12.0, 6.0),
                    border_radius: UCornerRadius::all(4.0),
                }
                UInteraction::default()
                ic
                UFocusable::default()
                Children [
                    UText {
                        text: item,
                        font_size: 13.0,
                        color: Color::srgb(0.85, 0.88, 0.95),
                        font: f,
                    }
                ]
            }
        }).collect();

        let rows_scene = SceneList::new(rows);
        let s = bsn! {
            LumaContextMenu
            UNode {
                width: UVal::Px(160.0),
                padding: USides::all(6.0),
                background_color: Color::srgb(0.10, 0.12, 0.17),
                border_radius: UCornerRadius::all(8.0),
            }
            UBorder {
                color: Color::srgb(0.20, 0.24, 0.32),
                width: 1.0,
                radius: UCornerRadius::all(8.0),
            }
            ULayout {
                display: UDisplay::Flex,
                flex_direction: UFlexDirection::Column,
                gap: 2.0,
            }
            Children [
                {rows_scene}
            ]
        };
        s.resolve(context, scene)
    }
}

// ============================================================================
// 2.12 Command Palette
// ============================================================================

/// State and marker component attached to a Command Palette root entity.
#[derive(Component, Clone, Debug, Default, Reflect)]
pub struct LumaCommandPalette {
    pub commands: Vec<String>,
    pub filtered_indices: Vec<usize>,
    pub selected_index: usize,
    pub is_overlay: bool,
    pub is_open: bool,
}

/// Marker component for individual command row items within the palette.
#[derive(Component, Clone, Debug, Default, Reflect)]
pub struct LumaCommandPaletteItem {
    pub command_index: usize,
}

/// Marker component for the container holding command rows.
#[derive(Component, Clone, Debug, Default, Reflect)]
pub struct LumaCommandPaletteList;

/// Marker component for the Command Palette full-screen backdrop.
#[derive(Component, Clone, Debug, Default, Reflect)]
pub struct LumaCommandPaletteBackdrop;

/// Marker component for the "No results found" placeholder row.
#[derive(Component, Clone, Debug, Default, Reflect)]
pub struct LumaCommandPaletteEmpty;

/// Event fired when a command is selected / executed from the Command Palette.
#[derive(Event, Clone, Debug, Reflect)]
pub struct CommandSelected {
    pub palette_entity: Entity,
    pub index: usize,
    pub command: String,
}

/// Fast keyboard command launcher (Ctrl + K) overlay or inline dialog.
#[derive(Clone, Debug, Reflect)]
pub struct CommandPalette {
    pub placeholder: String,
    pub commands: Vec<String>,
    pub is_overlay: bool,
    pub is_open: bool,
    pub font: Handle<Font>,
    pub icon_font: Handle<Font>,
}

impl Default for CommandPalette {
    fn default() -> Self {
        Self {
            placeholder: "Type a command or search...".into(),
            commands: vec![
                "Open Workspace Settings".into(),
                "Toggle Dark / Light Theme".into(),
                "Go to File...".into(),
                "Open Documentation".into(),
            ],
            is_overlay: false,
            is_open: true,
            font: Handle::default(),
            icon_font: Handle::default(),
        }
    }
}

impl CommandPalette {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn commands(mut self, cmds: Vec<impl Into<String>>) -> Self {
        self.commands = cmds.into_iter().map(|s| s.into()).collect();
        self
    }

    pub fn overlay(mut self, is_overlay: bool) -> Self {
        self.is_overlay = is_overlay;
        self
    }

    pub fn open(mut self, is_open: bool) -> Self {
        self.is_open = is_open;
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
}

impl Scene for CommandPalette {
    fn resolve(
        self,
        context: &mut ResolveContext,
        scene: &mut ResolvedScene,
    ) -> Result<(), ResolveSceneError> {
        let font = self.font;
        let icon_font = self.icon_font;
        let total_cmds = self.commands.len();
        let filtered_indices: Vec<usize> = (0..total_cmds).collect();

        let cmd_rows: Vec<_> = self.commands.iter().enumerate().map(|(idx, cmd)| {
            let f = font.clone();
            let ifont = icon_font.clone();
            let cmd_text = cmd.clone();
            let is_selected = idx == 0;
            let bg_color = if is_selected {
                Color::srgba(0.20, 0.45, 0.85, 0.25)
            } else {
                Color::srgba(1.0, 1.0, 1.0, 0.03)
            };

            bsn! {
                LumaCommandPaletteItem {
                    command_index: idx,
                }
                UNode {
                    width: UVal::Percent(1.0),
                    padding: USides::axes(12.0, 8.0),
                    border_radius: UCornerRadius::all(6.0),
                    background_color: bg_color,
                }
                UInteraction::default()
                ULayout {
                    display: UDisplay::Flex,
                    flex_direction: UFlexDirection::Row,
                    align_items: UAlignItems::Center,
                    gap: 10.0,
                }
                Children [
                    UText {
                        text: { Icon::SEARCH.to_string() },
                        font_size: 14.0,
                        color: Color::srgb(0.5, 0.55, 0.65),
                        font: ifont,
                    }
                    --
                    UText {
                        text: cmd_text,
                        font_size: 13.0,
                        color: Color::srgb(0.9, 0.92, 0.98),
                        font: f,
                    }
                ]
            }
        }).collect();

        let cmd_rows_scene = SceneList::new(cmd_rows);
        let input_font = font.clone();
        let input_ifont = icon_font.clone();
        let empty_font = font.clone();

        let empty_row = bsn! {
            LumaCommandPaletteEmpty
            UNode {
                width: UVal::Percent(1.0),
                padding: USides::axes(12.0, 12.0),
                border_radius: UCornerRadius::all(6.0),
                background_color: Color::srgba(1.0, 1.0, 1.0, 0.02),
            }
            ULayout {
                display: UDisplay::None,
                justify_content: UJustifyContent::Center,
                align_items: UAlignItems::Center,
            }
            Children [
                UText {
                    text: { "No commands found".to_string() },
                    font_size: 13.0,
                    color: Color::srgb(0.55, 0.60, 0.70),
                    font: empty_font,
                }
            ]
        };

        let commands_for_palette = self.commands;
        let is_open_val = self.is_open;
        let is_overlay = self.is_overlay;

        let dialog_box = bsn! {
            LumaCommandPalette {
                commands: commands_for_palette,
                filtered_indices,
                selected_index: 0,
                is_overlay,
                is_open: is_open_val,
            }
            UNode {
                width: UVal::Px(480.0),
                padding: USides::all(16.0),
                background_color: Color::srgba(0.09, 0.10, 0.15, 0.98),
                border_radius: UCornerRadius::all(10.0),
            }
            UBorder {
                color: Color::srgba(1.0, 1.0, 1.0, 0.15),
                width: 1.0,
                radius: UCornerRadius::all(10.0),
            }
            UShadow::drop(Color::srgba(0.0, 0.0, 0.0, 0.5), Vec2::new(0.0, 8.0), 16.0)
            ULayout {
                display: UDisplay::Flex,
                flex_direction: UFlexDirection::Column,
                gap: 12.0,
            }
            Children [
                @{TextInput::new(self.placeholder).leading_icon(Icon::SEARCH).font(input_font).icon_font(input_ifont)}
                --
                LumaCommandPaletteList
                UNode::default()
                ULayout {
                    display: UDisplay::Flex,
                    flex_direction: UFlexDirection::Column,
                    gap: 4.0,
                }
                Children [
                    {cmd_rows_scene}
                    --
                    @{empty_row}
                ]
            ]
        };

        if is_overlay {
            let display_mode = if is_open_val { UDisplay::Flex } else { UDisplay::None };
            let overlay_root = bsn! {
                LumaCommandPaletteBackdrop
                UNode {
                    width: UVal::Percent(1.0),
                    height: UVal::Percent(1.0),
                    background_color: Color::srgba(0.0, 0.0, 0.0, 0.60),
                    padding: USides::top(100.0),
                }
                USelf {
                    position_type: UPositionType::Absolute,
                    left: UVal::Px(0.0),
                    right: UVal::Px(0.0),
                    top: UVal::Px(0.0),
                    bottom: UVal::Px(0.0),
                    ..USelf::default()
                }
                UZIndex::Global(100)
                ULayout {
                    display: display_mode,
                    align_items: UAlignItems::Center,
                    justify_content: UJustifyContent::Start,
                    flex_direction: UFlexDirection::Column,
                }
                Children [
                    @{dialog_box}
                ]
            };
            overlay_root.resolve(context, scene)
        } else {
            dialog_box.resolve(context, scene)
        }
    }
}

fn on_command_palette_text_changed(
    trigger: On<UTextInputChanged>,
    parents_query: Query<&ChildOf>,
    mut palette_query: Query<&mut LumaCommandPalette>,
    mut item_query: Query<(Entity, &LumaCommandPaletteItem, &mut ULayout, &mut UNode)>,
    mut empty_query: Query<(Entity, &mut ULayout), (With<LumaCommandPaletteEmpty>, Without<LumaCommandPaletteItem>)>,
) {
    let Some(palette_entity) = parents_query.iter_ancestors(trigger.entity).find(|&e| palette_query.contains(e)) else {
        return;
    };
    let Ok(mut palette) = palette_query.get_mut(palette_entity) else {
        return;
    };

    let query = trigger.value.to_lowercase().trim().to_string();
    palette.filtered_indices = palette
        .commands
        .iter()
        .enumerate()
        .filter(|(_, cmd)| query.is_empty() || cmd.to_lowercase().contains(&query))
        .map(|(i, _)| i)
        .collect();

    palette.selected_index = 0;
    let selected_actual = palette.filtered_indices.first().copied();

    for (item_entity, item, mut layout, mut node) in item_query.iter_mut() {
        if parents_query.iter_ancestors(item_entity).any(|a| a == palette_entity) {
            let is_match = palette.filtered_indices.contains(&item.command_index);
            layout.display = if is_match { UDisplay::Flex } else { UDisplay::None };

            let is_sel = selected_actual == Some(item.command_index);
            node.background_color = if is_sel {
                Color::srgba(0.20, 0.45, 0.85, 0.25)
            } else {
                Color::srgba(1.0, 1.0, 1.0, 0.03)
            };
        }
    }

    for (empty_entity, mut layout) in empty_query.iter_mut() {
        if parents_query.iter_ancestors(empty_entity).any(|a| a == palette_entity) {
            layout.display = if palette.filtered_indices.is_empty() {
                UDisplay::Flex
            } else {
                UDisplay::None
            };
        }
    }
}

fn on_command_palette_row_over(
    trigger: On<PointerOver>,
    parents_query: Query<&ChildOf>,
    mut palette_query: Query<&mut LumaCommandPalette>,
    mut all_items: Query<(Entity, &LumaCommandPaletteItem, &mut UNode)>,
) {
    let Some(item_entity) = find_ancestor_or_self(trigger.entity, &all_items, &parents_query) else { return; };
    let Ok((_, hovered_item, _)) = all_items.get(item_entity) else { return; };
    let Some(palette_entity) = parents_query.iter_ancestors(item_entity).find(|&e| palette_query.contains(e)) else { return; };
    let Ok(mut palette) = palette_query.get_mut(palette_entity) else { return; };

    if let Some(pos) = palette.filtered_indices.iter().position(|&idx| idx == hovered_item.command_index) {
        palette.selected_index = pos;
        let selected_actual = hovered_item.command_index;

        for (item_entity, item, mut node) in all_items.iter_mut() {
            if parents_query.iter_ancestors(item_entity).any(|a| a == palette_entity) {
                node.background_color = if item.command_index == selected_actual {
                    Color::srgba(0.20, 0.45, 0.85, 0.25)
                } else {
                    Color::srgba(1.0, 1.0, 1.0, 0.03)
                };
            }
        }
    }
}

fn on_command_palette_row_click(
    trigger: On<PointerClick>,
    parents_query: Query<&ChildOf>,
    item_query: Query<&LumaCommandPaletteItem>,
    mut palette_query: Query<(Entity, &mut LumaCommandPalette)>,
    mut backdrop_query: Query<&mut ULayout, With<LumaCommandPaletteBackdrop>>,
    mut commands: Commands,
) {
    if !is_primary_click(&trigger) {
        return;
    }
    let Some(item_entity) = find_ancestor_or_self(trigger.entity, &item_query, &parents_query) else { return; };
    let Ok(clicked_item) = item_query.get(item_entity) else { return; };
    let Some(palette_entity) = parents_query.iter_ancestors(item_entity).find(|&e| palette_query.contains(e)) else { return; };
    let Ok((_, mut palette)) = palette_query.get_mut(palette_entity) else { return; };

    if let Some(cmd_text) = palette.commands.get(clicked_item.command_index).cloned() {
        commands.trigger(CommandSelected {
            palette_entity,
            index: clicked_item.command_index,
            command: cmd_text,
        });

        if palette.is_overlay {
            palette.is_open = false;
            for ancestor in parents_query.iter_ancestors(palette_entity) {
                if let Ok(mut bd_layout) = backdrop_query.get_mut(ancestor) {
                    bd_layout.display = UDisplay::None;
                }
            }
        }
    }
}

fn on_command_palette_backdrop_click(
    trigger: On<PointerClick>,
    backdrop_query: Query<&LumaCommandPaletteBackdrop>,
    children_query: Query<&Children>,
    mut palette_query: Query<&mut LumaCommandPalette>,
    mut layout_query: Query<&mut ULayout>,
) {
    if !is_primary_click(&trigger) {
        return;
    }
    if !backdrop_query.contains(trigger.entity) {
        return;
    }
    if let Ok(mut bd_layout) = layout_query.get_mut(trigger.entity) {
        bd_layout.display = UDisplay::None;
    }
    if let Ok(children) = children_query.get(trigger.entity) {
        for child in children.iter() {
            if let Ok(mut palette) = palette_query.get_mut(child) {
                palette.is_open = false;
            }
        }
    }
}

fn command_palette_keyboard_system(
    keyboard: Option<Res<ButtonInput<KeyCode>>>,
    parents_query: Query<&ChildOf>,
    focused_query: Query<Entity, With<UFocused>>,
    mut palette_query: Query<(Entity, &mut LumaCommandPalette)>,
    mut backdrop_query: Query<&mut ULayout, With<LumaCommandPaletteBackdrop>>,
    mut item_query: Query<(Entity, &LumaCommandPaletteItem, &mut UNode)>,
    mut commands: Commands,
) {
    let Some(keyboard) = keyboard else {
        return;
    };

    let ctrl = keyboard.pressed(KeyCode::ControlLeft)
        || keyboard.pressed(KeyCode::ControlRight)
        || keyboard.pressed(KeyCode::SuperLeft)
        || keyboard.pressed(KeyCode::SuperRight);

    if ctrl && keyboard.just_pressed(KeyCode::KeyK) {
        for (palette_entity, mut palette) in palette_query.iter_mut() {
            if palette.is_overlay {
                palette.is_open = !palette.is_open;
                let display = if palette.is_open { UDisplay::Flex } else { UDisplay::None };
                for ancestor in parents_query.iter_ancestors(palette_entity) {
                    if let Ok(mut bd_layout) = backdrop_query.get_mut(ancestor) {
                        bd_layout.display = display;
                    }
                }
            }
        }
        return;
    }

    let focused_entities: Vec<Entity> = focused_query.iter().collect();

    for (palette_entity, mut palette) in palette_query.iter_mut() {
        if !palette.is_open {
            continue;
        }

        let is_focused = focused_entities.iter().any(|&f| {
            f == palette_entity || parents_query.iter_ancestors(f).any(|a| a == palette_entity)
        });

        if !is_focused && !palette.is_overlay {
            continue;
        }

        if keyboard.just_pressed(KeyCode::Escape) {
            if palette.is_overlay {
                palette.is_open = false;
                for ancestor in parents_query.iter_ancestors(palette_entity) {
                    if let Ok(mut bd_layout) = backdrop_query.get_mut(ancestor) {
                        bd_layout.display = UDisplay::None;
                    }
                }
            }
            continue;
        }

        if palette.filtered_indices.is_empty() {
            continue;
        }

        let mut changed = false;
        if keyboard.just_pressed(KeyCode::ArrowDown) {
            if palette.selected_index + 1 < palette.filtered_indices.len() {
                palette.selected_index += 1;
                changed = true;
            }
        } else if keyboard.just_pressed(KeyCode::ArrowUp) {
            if palette.selected_index > 0 {
                palette.selected_index -= 1;
                changed = true;
            }
        } else if keyboard.just_pressed(KeyCode::Enter) || keyboard.just_pressed(KeyCode::NumpadEnter) {
            let actual_idx = palette.filtered_indices[palette.selected_index];
            if let Some(cmd_text) = palette.commands.get(actual_idx).cloned() {
                commands.trigger(CommandSelected {
                    palette_entity,
                    index: actual_idx,
                    command: cmd_text,
                });

                if palette.is_overlay {
                    palette.is_open = false;
                    for ancestor in parents_query.iter_ancestors(palette_entity) {
                        if let Ok(mut bd_layout) = backdrop_query.get_mut(ancestor) {
                            bd_layout.display = UDisplay::None;
                        }
                    }
                }
            }
        }

        if changed {
            let selected_actual = palette.filtered_indices[palette.selected_index];
            for (item_entity, item, mut node) in item_query.iter_mut() {
                if parents_query.iter_ancestors(item_entity).any(|a| a == palette_entity) {
                    node.background_color = if item.command_index == selected_actual {
                        Color::srgba(0.20, 0.45, 0.85, 0.25)
                    } else {
                        Color::srgba(1.0, 1.0, 1.0, 0.03)
                    };
                }
            }
        }
    }
}

pub struct LumaNavigationPlugin;

impl Plugin for LumaNavigationPlugin {
    fn build(&self, app: &mut App) {
        app.register_type::<LumaNavbar>()
            .register_type::<LumaNavbarItem>()
            .register_type::<LumaNavbarSignIn>()
            .register_type::<LumaBottomNav>()
            .register_type::<LumaBottomNavItem>()
            .register_type::<LumaSidebar>()
            .register_type::<LumaSidebarItem>()
            .register_type::<LumaPagination>()
            .register_type::<PaginationAction>()
            .register_type::<LumaPaginationBtn>()
            .register_type::<LumaStepTracker>()
            .register_type::<LumaStepItem>()
            .register_type::<LumaStepCircle>()
            .register_type::<LumaStepCircleText>()
            .register_type::<LumaStepCircleCheck>()
            .register_type::<LumaStepTitle>()
            .register_type::<LumaStepLine>()
            .register_type::<LumaHamburgerMenu>()
            .register_type::<LumaNavLinkItem>()
            .register_type::<LumaContextMenu>()
            .register_type::<LumaContextMenuItem>()
            .register_type::<LumaCommandPalette>()
            .register_type::<LumaCommandPaletteItem>()
            .register_type::<LumaCommandPaletteList>()
            .register_type::<LumaCommandPaletteBackdrop>()
            .register_type::<LumaCommandPaletteEmpty>()
            .register_type::<LumaCustomScrollbar>()
            .register_type::<LumaCustomScrollbarThumb>()
            .register_type::<LumaBackToTop>()
            .register_type::<LumaActionSheet>()
            .register_type::<LumaActionSheetAction>()
            .register_type::<LumaActionSheetCancel>()
            .register_type::<LumaBottomSheet>()
            .register_type::<LumaBottomSheetHandle>()
            .register_type::<LumaSplitter>()
            .register_type::<LumaTreeView>()
            .register_type::<LumaTreeNode>()
            .register_type::<LumaTreeNodeIcon>()
            .register_type::<LumaCarouselControls>()
            .register_type::<CarouselAction>()
            .register_type::<LumaCarouselBtn>()
            .register_type::<LumaCarouselDot>()
            .register_type::<LumaTableOfContents>()
            .register_type::<LumaTocItem>()
            .register_type::<LumaFilterSortBar>()
            .register_type::<LumaFilterChipItem>()
            .register_type::<LumaFilterSortBtn>()
            .register_type::<LumaFilterViewBtn>()
            .register_type::<LumaFloatingToolbar>()
            .register_type::<FloatingToolbarActionKind>()
            .register_type::<LumaFloatingToolbarBtn>()
            .register_type::<LumaWorkspaceSwitcher>()
            .register_type::<LumaWorkspaceLabel>()
            .register_type::<LumaLocaleSelector>()
            .register_type::<LumaLocaleLabel>()
            .register_type::<LumaNavDropdownBackdrop>()
            .register_type::<LumaNavDropdownItem>()
            .register_type::<LumaThemeToggle>()
            .register_type::<CommandSelected>()
            .register_type::<NavbarItemSelected>()
            .register_type::<NavbarSignInClicked>()
            .register_type::<BottomNavChanged>()
            .register_type::<SidebarItemSelected>()
            .register_type::<PageChanged>()
            .register_type::<StepSelected>()
            .register_type::<HamburgerToggled>()
            .register_type::<NavLinkClicked>()
            .register_type::<ContextMenuItemSelected>()
            .register_type::<ScrollbarChanged>()
            .register_type::<BackToTopClicked>()
            .register_type::<ActionSheetSelected>()
            .register_type::<ActionSheetCancelled>()
            .register_type::<BottomSheetToggled>()
            .register_type::<SplitterDragged>()
            .register_type::<TreeNodeSelected>()
            .register_type::<TreeNodeToggled>()
            .register_type::<CarouselChanged>()
            .register_type::<TocSectionSelected>()
            .register_type::<FilterSortChanged>()
            .register_type::<FloatingToolbarAction>()
            .register_type::<WorkspaceChanged>()
            .register_type::<LocaleChanged>()
            .register_type::<ThemeToggled>()
            .add_observer(on_command_palette_text_changed)
            .add_observer(on_command_palette_row_over)
            .add_observer(on_command_palette_row_click)
            .add_observer(on_command_palette_backdrop_click)
            .add_observer(on_navbar_item_click)
            .add_observer(on_navbar_item_focus)
            .add_observer(on_bottom_nav_item_click)
            .add_observer(on_bottom_nav_item_focus)
            .add_observer(on_sidebar_item_click)
            .add_observer(on_sidebar_item_focus)
            .add_observer(on_pagination_btn_click)
            .add_observer(on_step_item_click)
            .add_observer(on_step_item_focus)
            .add_observer(on_nav_simple_button_click)
            .add_observer(on_nav_link_item_click)
            .add_observer(on_nav_link_item_focus)
            .add_observer(on_context_menu_item_click)
            .add_observer(on_context_menu_item_focus)
            .add_observer(on_scrollbar_pointer_click)
            .add_observer(on_scrollbar_pointer_drag)
            .add_observer(on_bottom_sheet_handle_click)
            .add_observer(on_bottom_sheet_handle_focus)
            .add_observer(on_splitter_pointer_drag)
            .add_observer(on_tree_node_click)
            .add_observer(on_tree_node_focus)
            .add_observer(on_carousel_btn_click)
            .add_observer(on_carousel_dot_click)
            .add_observer(on_carousel_dot_focus)
            .add_observer(on_toc_item_click)
            .add_observer(on_toc_item_focus)
            .add_observer(on_filter_sort_btn_click)
            .add_observer(on_filter_chip_click)
            .add_observer(on_filter_chip_focus)
            .add_observer(on_nav_dropdown_trigger_click)
            .add_observer(on_nav_dropdown_trigger_focus)
            .add_observer(on_nav_dropdown_item_click)
            .add_observer(on_nav_dropdown_backdrop_click)
            .add_systems(
                Update,
                (
                    command_palette_keyboard_system,
                    nav_scrollbar_keyboard_system,
                    nav_dropdown_cleanup_system,
                ),
            );
    }
}

// ============================================================================
// 2.13 Custom Scrollbar
// ============================================================================

/// Styled SDF scrollbar track & thumb.
#[derive(Clone, Debug, Reflect)]
pub struct CustomScrollbar {
    pub progress: f32, // 0.0 to 1.0
    pub is_horizontal: bool,
}

impl Default for CustomScrollbar {
    fn default() -> Self {
        Self {
            progress: 0.25,
            is_horizontal: false,
        }
    }
}

impl CustomScrollbar {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn progress(mut self, progress: f32) -> Self {
        self.progress = progress.clamp(0.0, 1.0);
        self
    }

    pub fn horizontal(mut self, is_horizontal: bool) -> Self {
        self.is_horizontal = is_horizontal;
        self
    }
}

impl Scene for CustomScrollbar {
    fn resolve(
        self,
        context: &mut ResolveContext,
        scene: &mut ResolvedScene,
    ) -> Result<(), ResolveSceneError> {
        let progress = self.progress.clamp(0.0, 1.0);
        let is_horizontal = self.is_horizontal;
        let thumb_margin = progress * 100.0;
        let sb = LumaCustomScrollbar { progress, is_horizontal };
        let ic = interaction_colors(
            Color::srgba(1.0, 1.0, 1.0, 0.05),
            Color::srgba(1.0, 1.0, 1.0, 0.10),
            Color::srgba(1.0, 1.0, 1.0, 0.16),
            false,
        );
        if is_horizontal {
            let s = bsn! {
                sb
                UNode {
                    width: UVal::Percent(1.0),
                    height: UVal::Px(8.0),
                    background_color: Color::srgba(1.0, 1.0, 1.0, 0.05),
                    border_radius: UCornerRadius::all(4.0),
                }
                UInteraction::default()
                ic
                UFocusable::default()
                Children [
                    LumaCustomScrollbarThumb
                    UNode {
                        width: UVal::Px(40.0),
                        height: UVal::Px(8.0),
                        margin: USides { top: 0.0, right: 0.0, bottom: 0.0, left: thumb_margin },
                        background_color: Color::srgba(0.4, 0.45, 0.6, 0.7),
                        border_radius: UCornerRadius::all(4.0),
                    }
                ]
            };
            s.resolve(context, scene)
        } else {
            let s = bsn! {
                sb
                UNode {
                    width: UVal::Px(8.0),
                    height: UVal::Percent(1.0),
                    background_color: Color::srgba(1.0, 1.0, 1.0, 0.05),
                    border_radius: UCornerRadius::all(4.0),
                }
                UInteraction::default()
                ic
                UFocusable::default()
                Children [
                    LumaCustomScrollbarThumb
                    UNode {
                        width: UVal::Px(8.0),
                        height: UVal::Px(40.0),
                        margin: USides { top: thumb_margin, right: 0.0, bottom: 0.0, left: 0.0 },
                        background_color: Color::srgba(0.4, 0.45, 0.6, 0.7),
                        border_radius: UCornerRadius::all(4.0),
                    }
                ]
            };
            s.resolve(context, scene)
        }
    }
}

// ============================================================================
// 2.14 Back to Top Button
// ============================================================================

/// Floating circle button to scroll back to top of page.
#[derive(Clone, Debug, Reflect)]
pub struct BackToTopButton {
    pub icon_font: Handle<Font>,
}

impl Default for BackToTopButton {
    fn default() -> Self {
        Self {
            icon_font: Handle::default(),
        }
    }
}

impl BackToTopButton {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn icon_font(mut self, icon_font: Handle<Font>) -> Self {
        self.icon_font = icon_font;
        self
    }
}

impl Scene for BackToTopButton {
    fn resolve(
        self,
        context: &mut ResolveContext,
        scene: &mut ResolvedScene,
    ) -> Result<(), ResolveSceneError> {
        let btn = Button::fab_button(Icon::CHEVRON_UP)
            .icon_font(self.icon_font);
        let s = bsn! {
            LumaBackToTop
            UNode::default()
            Children [ @{btn} ]
        };
        s.resolve(context, scene)
    }
}

// ============================================================================
// 2.15 Action Sheet
// ============================================================================

/// Bottom mobile modal sheet for contextual action menus.
#[derive(Clone, Debug, Reflect)]
pub struct ActionSheet {
    pub title: String,
    pub actions: Vec<String>,
    pub cancel_label: String,
    pub font: Handle<Font>,
}

impl Default for ActionSheet {
    fn default() -> Self {
        Self {
            title: "Select an Action".into(),
            actions: vec!["Share Link".into(), "Save to Library".into(), "Report".into()],
            cancel_label: "Cancel".into(),
            font: Handle::default(),
        }
    }
}

impl ActionSheet {
    pub fn new(title: impl Into<String>) -> Self {
        Self {
            title: title.into(),
            ..default()
        }
    }

    pub fn actions(mut self, actions: Vec<impl Into<String>>) -> Self {
        self.actions = actions.into_iter().map(|s| s.into()).collect();
        self
    }

    pub fn cancel(mut self, cancel: impl Into<String>) -> Self {
        self.cancel_label = cancel.into();
        self
    }

    pub fn font(mut self, font: Handle<Font>) -> Self {
        self.font = font;
        self
    }
}

impl Scene for ActionSheet {
    fn resolve(
        self,
        context: &mut ResolveContext,
        scene: &mut ResolvedScene,
    ) -> Result<(), ResolveSceneError> {
        let font = self.font;
        let title = self.title;
        let action_btns: Vec<_> = self.actions.into_iter().enumerate().map(|(idx, act)| {
            let f = font.clone();
            let marker = LumaActionSheetAction {
                index: idx,
                label: act.clone(),
            };
            let btn = Button::new(act).secondary().size(ButtonSize::Large).font(f);
            bsn! {
                marker
                UNode::default()
                Children [ @{btn} ]
            }
        }).collect();

        let action_btns_scene = SceneList::new(action_btns);
        let cancel_font = font.clone();
        let title_font = font.clone();
        let s = bsn! {
            LumaActionSheet
            UNode {
                width: UVal::Percent(1.0),
                padding: USides::all(16.0),
                background_color: Color::srgba(0.08, 0.09, 0.13, 0.98),
                border_radius: UCornerRadius::top(16.0),
            }
            UBorder {
                color: Color::srgba(1.0, 1.0, 1.0, 0.1),
                width: 1.0,
            }
            ULayout {
                display: UDisplay::Flex,
                flex_direction: UFlexDirection::Column,
                gap: 8.0,
            }
            Children [
                UText {
                    text: title,
                    font_size: 14.0,
                    color: Color::srgb(0.65, 0.7, 0.8),
                    font: title_font,
                }
                --
                UNode::default()
                ULayout {
                    display: UDisplay::Flex,
                    flex_direction: UFlexDirection::Column,
                    gap: 6.0,
                }
                Children [
                    {action_btns_scene}
                ]
                --
                @{Divider::horizontal()}
                --
                LumaActionSheetCancel
                UNode::default()
                Children [
                    @{Button::new(self.cancel_label).outline().size(ButtonSize::Large).font(cancel_font)}
                ]
            ]
        };
        s.resolve(context, scene)
    }
}

// ============================================================================
// 2.16 Bottom Sheet
// ============================================================================

/// Draggable mobile bottom drawer sheet panel.
#[derive(Clone, Debug, Reflect)]
pub struct BottomSheet {
    pub title: String,
    pub font: Handle<Font>,
}

impl Default for BottomSheet {
    fn default() -> Self {
        Self {
            title: "Details Panel".into(),
            font: Handle::default(),
        }
    }
}

impl BottomSheet {
    pub fn new(title: impl Into<String>) -> Self {
        Self {
            title: title.into(),
            ..default()
        }
    }

    pub fn font(mut self, font: Handle<Font>) -> Self {
        self.font = font;
        self
    }
}

impl Scene for BottomSheet {
    fn resolve(
        self,
        context: &mut ResolveContext,
        scene: &mut ResolvedScene,
    ) -> Result<(), ResolveSceneError> {
        let title = self.title;
        let font = self.font;
        let handle_colors = interaction_colors(
            Color::srgba(1.0, 1.0, 1.0, 0.25),
            Color::srgba(1.0, 1.0, 1.0, 0.45),
            Color::srgba(1.0, 1.0, 1.0, 0.65),
            false,
        );
        let s = bsn! {
            LumaBottomSheet { expanded: true }
            UNode {
                width: UVal::Percent(1.0),
                height: UVal::Px(220.0),
                padding: USides::all(16.0),
                background_color: Color::srgba(0.08, 0.09, 0.13, 0.98),
                border_radius: UCornerRadius::top(16.0),
            }
            UBorder {
                color: Color::srgba(1.0, 1.0, 1.0, 0.12),
                width: 1.0,
            }
            ULayout {
                display: UDisplay::Flex,
                flex_direction: UFlexDirection::Column,
                align_items: UAlignItems::Center,
                gap: 12.0,
            }
            Children [
                // Grab handle
                LumaBottomSheetHandle
                UNode {
                    width: UVal::Px(36.0),
                    height: UVal::Px(4.0),
                    background_color: Color::srgba(1.0, 1.0, 1.0, 0.25),
                    border_radius: UCornerRadius::all(2.0),
                }
                UInteraction::default()
                handle_colors
                UFocusable::default()
                --
                UText {
                    text: title,
                    font_size: 15.0,
                    color: Color::srgb(0.96, 0.97, 0.99),
                    font,
                }
            ]
        };
        s.resolve(context, scene)
    }
}

// ============================================================================
// 2.17 Splitter / Resizable Handle
// ============================================================================

/// Resizable dividing pane splitter bar.
#[derive(Clone, Debug, Reflect)]
pub struct Splitter {
    pub is_horizontal: bool,
}

impl Default for Splitter {
    fn default() -> Self {
        Self {
            is_horizontal: false,
        }
    }
}

impl Splitter {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn horizontal(mut self, is_horizontal: bool) -> Self {
        self.is_horizontal = is_horizontal;
        self
    }
}

impl Scene for Splitter {
    fn resolve(
        self,
        context: &mut ResolveContext,
        scene: &mut ResolvedScene,
    ) -> Result<(), ResolveSceneError> {
        let is_horizontal = self.is_horizontal;
        let sb = LumaSplitter { is_horizontal, offset: 0.0 };
        let ic = interaction_colors(
            Color::srgba(1.0, 1.0, 1.0, 0.08),
            Color::srgba(0.35, 0.65, 1.0, 0.28),
            Color::srgba(0.35, 0.65, 1.0, 0.45),
            false,
        );
        if is_horizontal {
            let s = bsn! {
                sb
                UNode {
                    width: UVal::Percent(1.0),
                    height: UVal::Px(6.0),
                    background_color: Color::srgba(1.0, 1.0, 1.0, 0.08),
                }
                UInteraction::default()
                ic
                UFocusable::default()
                ULayout {
                    display: UDisplay::Flex,
                    justify_content: UJustifyContent::Center,
                    align_items: UAlignItems::Center,
                }
                Children [
                    UNode {
                        width: UVal::Px(32.0),
                        height: UVal::Px(3.0),
                        background_color: Color::srgba(1.0, 1.0, 1.0, 0.3),
                        border_radius: UCornerRadius::all(1.5),
                    }
                ]
            };
            s.resolve(context, scene)
        } else {
            let s = bsn! {
                sb
                UNode {
                    width: UVal::Px(6.0),
                    height: UVal::Percent(1.0),
                    background_color: Color::srgba(1.0, 1.0, 1.0, 0.08),
                }
                UInteraction::default()
                ic
                UFocusable::default()
                ULayout {
                    display: UDisplay::Flex,
                    justify_content: UJustifyContent::Center,
                    align_items: UAlignItems::Center,
                }
                Children [
                    UNode {
                        width: UVal::Px(3.0),
                        height: UVal::Px(32.0),
                        background_color: Color::srgba(1.0, 1.0, 1.0, 0.3),
                        border_radius: UCornerRadius::all(1.5),
                    }
                ]
            };
            s.resolve(context, scene)
        }
    }
}

// ============================================================================
// 2.18 Tree View
// ============================================================================

/// Hierarchical tree directory view with expandable nodes.
#[derive(Clone, Debug, Reflect)]
pub struct TreeView {
    pub items: Vec<(usize, String, bool)>, // (depth, label, is_folder)
    pub font: Handle<Font>,
    pub icon_font: Handle<Font>,
}

impl Default for TreeView {
    fn default() -> Self {
        Self {
            items: vec![
                (0, "src".into(), true),
                (1, "main.rs".into(), false),
                (1, "lib.rs".into(), false),
                (0, "assets".into(), true),
                (1, "textures".into(), true),
            ],
            font: Handle::default(),
            icon_font: Handle::default(),
        }
    }
}

impl TreeView {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn items(mut self, items: Vec<(usize, String, bool)>) -> Self {
        self.items = items;
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
}

impl Scene for TreeView {
    fn resolve(
        self,
        context: &mut ResolveContext,
        scene: &mut ResolvedScene,
    ) -> Result<(), ResolveSceneError> {
        let font = self.font;
        let icon_font = self.icon_font;
        let tree_state = LumaTreeView {
            selected_index: None,
        };

        let rows: Vec<_> = self.items.into_iter().enumerate().map(|(idx, (depth, label, is_folder))| {
            let indent = depth as f32 * 16.0;
            let icon = if is_folder { Icon::FOLDER } else { Icon::SETTINGS };
            let f = font.clone();
            let ifont = icon_font.clone();
            let pad_left = indent + 8.0;
            let node_marker = LumaTreeNode {
                index: idx,
                depth,
                label: label.clone(),
                is_folder,
                expanded: true,
            };
            let ic = interaction_colors(
                Color::NONE,
                Color::srgba(1.0, 1.0, 1.0, 0.06),
                Color::srgba(1.0, 1.0, 1.0, 0.12),
                false,
            );
            bsn! {
                node_marker
                UNode {
                    padding: USides { top: 4.0, right: 8.0, bottom: 4.0, left: pad_left },
                    border_radius: UCornerRadius::all(4.0),
                }
                UInteraction::default()
                ic
                UFocusable::default()
                ULayout {
                    display: UDisplay::Flex,
                    flex_direction: UFlexDirection::Row,
                    align_items: UAlignItems::Center,
                    gap: 8.0,
                }
                Children [
                    LumaTreeNodeIcon
                    UText {
                        text: icon,
                        font_size: 14.0,
                        color: Color::srgb(0.55, 0.60, 0.70),
                        font: ifont,
                    }
                    --
                    UText {
                        text: label,
                        font_size: 13.0,
                        color: Color::srgb(0.85, 0.88, 0.95),
                        font: f,
                    }
                ]
            }
        }).collect();

        let rows_scene = SceneList::new(rows);
        let s = bsn! {
            tree_state
            UNode::default()
            ULayout {
                display: UDisplay::Flex,
                flex_direction: UFlexDirection::Column,
                gap: 2.0,
            }
            Children [
                {rows_scene}
            ]
        };
        s.resolve(context, scene)
    }
}

// ============================================================================
// 2.19 Carousel Controls
// ============================================================================

/// Previous/Next navigation controls with pagination indicator dots.
#[derive(Clone, Debug, Reflect)]
pub struct CarouselControls {
    pub total: usize,
    pub current: usize,
    pub icon_font: Handle<Font>,
}

impl Default for CarouselControls {
    fn default() -> Self {
        Self {
            total: 4,
            current: 0,
            icon_font: Handle::default(),
        }
    }
}

impl CarouselControls {
    pub fn new(total: usize, current: usize) -> Self {
        Self {
            total,
            current,
            ..default()
        }
    }

    pub fn icon_font(mut self, icon_font: Handle<Font>) -> Self {
        self.icon_font = icon_font;
        self
    }
}

impl Scene for CarouselControls {
    fn resolve(
        self,
        context: &mut ResolveContext,
        scene: &mut ResolvedScene,
    ) -> Result<(), ResolveSceneError> {
        let total = self.total;
        let current = if total == 0 { 0 } else { self.current.min(total - 1) };
        let carousel_state = LumaCarouselControls { current, total };
        let dots: Vec<_> = (0..total).map(|idx| {
            let is_active = idx == current;
            let (w, bg) = if is_active {
                (20.0_f32, Color::srgb(0.35, 0.65, 1.0))
            } else {
                (6.0_f32, Color::srgba(1.0, 1.0, 1.0, 0.3))
            };
            let dot_marker = LumaCarouselDot { index: idx };
            let dot_ic = interaction_colors(
                bg,
                Color::srgb(0.50, 0.75, 1.0),
                Color::srgb(0.35, 0.65, 1.0),
                false,
            );
            bsn! {
                dot_marker
                UNode {
                    width: UVal::Px(w),
                    height: UVal::Px(6.0),
                    background_color: bg,
                    border_radius: UCornerRadius::all(3.0),
                }
                UInteraction::default()
                dot_ic
                UFocusable::default()
            }
        }).collect();

        let dots_scene = SceneList::new(dots);
        let prev_ifont = self.icon_font.clone();
        let s = bsn! {
            carousel_state
            UNode::default()
            ULayout {
                display: UDisplay::Flex,
                flex_direction: UFlexDirection::Row,
                align_items: UAlignItems::Center,
                gap: 12.0,
            }
            Children [
                LumaCarouselBtn { action: CarouselAction::Prev }
                UNode::default()
                Children [
                    @{Button::icon_button(Icon::CHEVRON_LEFT).size(ButtonSize::Small).icon_font(prev_ifont)}
                ]
                --
                UNode::default()
                ULayout {
                    display: UDisplay::Flex,
                    flex_direction: UFlexDirection::Row,
                    align_items: UAlignItems::Center,
                    gap: 6.0,
                }
                Children [
                    {dots_scene}
                ]
                --
                LumaCarouselBtn { action: CarouselAction::Next }
                UNode::default()
                Children [
                    @{Button::icon_button(Icon::CHEVRON_RIGHT).size(ButtonSize::Small).icon_font(self.icon_font)}
                ]
            ]
        };
        s.resolve(context, scene)
    }
}

// ============================================================================
// 2.20 Table of Contents / Spy Scroll
// ============================================================================

/// Side index highlighting the current active document section.
#[derive(Clone, Debug, Reflect)]
pub struct TableOfContents {
    pub sections: Vec<String>,
    pub active_index: usize,
    pub font: Handle<Font>,
}

impl Default for TableOfContents {
    fn default() -> Self {
        Self {
            sections: vec![
                "Introduction".into(),
                "Architecture".into(),
                "Installation".into(),
                "Usage Examples".into(),
            ],
            active_index: 0,
            font: Handle::default(),
        }
    }
}

impl TableOfContents {
    pub fn new(sections: Vec<impl Into<String>>) -> Self {
        Self {
            sections: sections.into_iter().map(|s| s.into()).collect(),
            ..default()
        }
    }

    pub fn active(mut self, index: usize) -> Self {
        self.active_index = index;
        self
    }

    pub fn font(mut self, font: Handle<Font>) -> Self {
        self.font = font;
        self
    }
}

impl Scene for TableOfContents {
    fn resolve(
        self,
        context: &mut ResolveContext,
        scene: &mut ResolvedScene,
    ) -> Result<(), ResolveSceneError> {
        let active = self.active_index;
        let font = self.font;
        let toc_state = LumaTableOfContents { active_index: active };

        let rows: Vec<_> = self.sections.into_iter().enumerate().map(|(idx, sec)| {
            let is_active = idx == active;
            let (bar_color, text_color) = if is_active {
                (Color::srgb(0.35, 0.65, 1.0), Color::srgb(0.96, 0.97, 0.99))
            } else {
                (Color::NONE, Color::srgb(0.60, 0.65, 0.75))
            };
            let f = font.clone();
            let item_marker = LumaTocItem {
                index: idx,
                section: sec.clone(),
            };
            let ic = interaction_colors(
                Color::NONE,
                Color::srgba(1.0, 1.0, 1.0, 0.05),
                Color::srgba(1.0, 1.0, 1.0, 0.10),
                false,
            );
            bsn! {
                item_marker
                UNode {
                    padding: USides::axes(10.0, 4.0),
                }
                UBorder {
                    color: bar_color,
                    width: 2.0,
                }
                UInteraction::default()
                ic
                UFocusable::default()
                ULayout {
                    display: UDisplay::Flex,
                    flex_direction: UFlexDirection::Row,
                    align_items: UAlignItems::Center,
                }
                Children [
                    UText {
                        text: sec,
                        font_size: 13.0,
                        color: text_color,
                        font: f,
                    }
                ]
            }
        }).collect();

        let rows_scene = SceneList::new(rows);
        let s = bsn! {
            toc_state
            UNode {
                width: UVal::Px(200.0),
            }
            ULayout {
                display: UDisplay::Flex,
                flex_direction: UFlexDirection::Column,
                gap: 4.0,
            }
            Children [
                {rows_scene}
            ]
        };
        s.resolve(context, scene)
    }
}

// ============================================================================
// 2.21 Filter & Sort Bar
// ============================================================================

/// Composite toolbar for filtering, sorting, and view style toggling.
#[derive(Clone, Debug, Reflect)]
pub struct FilterSortBar {
    pub active_filter: String,
    pub sort_label: String,
    pub is_grid_view: bool,
    pub font: Handle<Font>,
    pub icon_font: Handle<Font>,
}

impl Default for FilterSortBar {
    fn default() -> Self {
        Self {
            active_filter: "All Items".into(),
            sort_label: "Newest First".into(),
            is_grid_view: true,
            font: Handle::default(),
            icon_font: Handle::default(),
        }
    }
}

impl FilterSortBar {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn filter(mut self, filter: impl Into<String>) -> Self {
        self.active_filter = filter.into();
        self
    }

    pub fn sort(mut self, sort: impl Into<String>) -> Self {
        self.sort_label = sort.into();
        self
    }

    pub fn grid_view(mut self, grid: bool) -> Self {
        self.is_grid_view = grid;
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
}

impl Scene for FilterSortBar {
    fn resolve(
        self,
        context: &mut ResolveContext,
        scene: &mut ResolvedScene,
    ) -> Result<(), ResolveSceneError> {
        let is_grid_view = self.is_grid_view;
        let view_icon = if is_grid_view { Icon::FOLDER } else { Icon::MENU };
        let bar_state = LumaFilterSortBar {
            active_filter: self.active_filter.clone(),
            sort_label: self.sort_label.clone(),
            ascending: false,
            is_grid_view,
        };
        let chip_labels = [
            self.active_filter,
            "Featured".to_string(),
            "Archived".to_string(),
        ];
        let chips: Vec<_> = chip_labels
            .into_iter()
            .enumerate()
            .map(|(idx, lbl)| {
                let is_active = idx == 0;
                let border_color = if is_active {
                    Color::srgb(0.35, 0.65, 1.0)
                } else {
                    Color::NONE
                };
                let border_width = if is_active { 1.5_f32 } else { 0.0_f32 };
                let marker = LumaFilterChipItem {
                    index: idx,
                    label: lbl.clone(),
                };
                let f = self.font.clone();
                bsn! {
                    marker
                    UNode {
                        border_radius: UCornerRadius::all(999.0),
                    }
                    UBorder {
                        color: border_color,
                        width: border_width,
                        radius: UCornerRadius::all(999.0),
                    }
                    UInteraction::default()
                    UFocusable::default()
                    Children [
                        @{Chip::new(lbl).font(f)}
                    ]
                }
            })
            .collect();
        let chips_scene = SceneList::new(chips);
        let s = bsn! {
            bar_state
            UNode {
                width: UVal::Percent(1.0),
                padding: USides::all(8.0),
                background_color: Color::srgba(1.0, 1.0, 1.0, 0.03),
                border_radius: UCornerRadius::all(8.0),
            }
            ULayout {
                display: UDisplay::Flex,
                flex_direction: UFlexDirection::Row,
                justify_content: UJustifyContent::SpaceBetween,
                align_items: UAlignItems::Center,
            }
            Children [
                UNode::default()
                ULayout {
                    display: UDisplay::Flex,
                    flex_direction: UFlexDirection::Row,
                    gap: 8.0,
                    align_items: UAlignItems::Center,
                }
                Children [
                    {chips_scene}
                ]
                --
                UNode::default()
                ULayout {
                    display: UDisplay::Flex,
                    flex_direction: UFlexDirection::Row,
                    gap: 8.0,
                    align_items: UAlignItems::Center,
                }
                Children [
                    LumaFilterSortBtn
                    UNode::default()
                    Children [
                        @{Button::new(self.sort_label).outline().size(ButtonSize::Small).font(self.font)}
                    ]
                    --
                    LumaFilterViewBtn
                    UNode::default()
                    Children [
                        @{Button::icon_button(view_icon).size(ButtonSize::Small).icon_font(self.icon_font)}
                    ]
                ]
            ]
        };
        s.resolve(context, scene)
    }
}

// ============================================================================
// 2.22 Floating Toolbar
// ============================================================================

/// Contextual selection pill toolbar.
#[derive(Clone, Debug, Reflect)]
pub struct FloatingToolbar {
    pub icon_font: Handle<Font>,
}

impl Default for FloatingToolbar {
    fn default() -> Self {
        Self {
            icon_font: Handle::default(),
        }
    }
}

impl FloatingToolbar {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn icon_font(mut self, icon_font: Handle<Font>) -> Self {
        self.icon_font = icon_font;
        self
    }
}

impl Scene for FloatingToolbar {
    fn resolve(
        self,
        context: &mut ResolveContext,
        scene: &mut ResolvedScene,
    ) -> Result<(), ResolveSceneError> {
        let ifont1 = self.icon_font.clone();
        let ifont2 = self.icon_font.clone();
        let ifont3 = self.icon_font.clone();
        let ifont4 = self.icon_font;
        let s = bsn! {
            LumaFloatingToolbar
            UNode {
                padding: USides::all(4.0),
                background_color: Color::srgba(0.1, 0.12, 0.18, 0.98),
                border_radius: UCornerRadius::all(20.0),
            }
            UBorder {
                color: Color::srgba(1.0, 1.0, 1.0, 0.15),
                width: 1.0,
                radius: UCornerRadius::all(20.0),
            }
            ULayout {
                display: UDisplay::Flex,
                flex_direction: UFlexDirection::Row,
                gap: 4.0,
            }
            Children [
                LumaFloatingToolbarBtn { action: FloatingToolbarActionKind::Favorite }
                UNode::default()
                Children [
                    @{Button::icon_button(Icon::STAR).ghost().size(ButtonSize::Small).icon_font(ifont1)}
                ]
                --
                LumaFloatingToolbarBtn { action: FloatingToolbarActionKind::CopyLink }
                UNode::default()
                Children [
                    @{Button::icon_button(Icon::LINK).ghost().size(ButtonSize::Small).icon_font(ifont2)}
                ]
                --
                LumaFloatingToolbarBtn { action: FloatingToolbarActionKind::Approve }
                UNode::default()
                Children [
                    @{Button::icon_button(Icon::CHECK).ghost().size(ButtonSize::Small).icon_font(ifont3)}
                ]
                --
                LumaFloatingToolbarBtn { action: FloatingToolbarActionKind::Dismiss }
                UNode::default()
                Children [
                    @{Button::icon_button(Icon::X).ghost().size(ButtonSize::Small).icon_font(ifont4)}
                ]
            ]
        };
        s.resolve(context, scene)
    }
}

// ============================================================================
// 2.23 Workspace Switcher
// ============================================================================

/// Dropdown menu to switch workspaces or organizations.
#[derive(Clone, Debug, Reflect)]
pub struct WorkspaceSwitcher {
    pub current_workspace: String,
    pub font: Handle<Font>,
    pub icon_font: Handle<Font>,
}

impl Default for WorkspaceSwitcher {
    fn default() -> Self {
        Self {
            current_workspace: "Acme Corp".into(),
            font: Handle::default(),
            icon_font: Handle::default(),
        }
    }
}

impl WorkspaceSwitcher {
    pub fn new(workspace: impl Into<String>) -> Self {
        Self {
            current_workspace: workspace.into(),
            ..default()
        }
    }

    pub fn font(mut self, font: Handle<Font>) -> Self {
        self.font = font;
        self
    }

    pub fn icon_font(mut self, icon_font: Handle<Font>) -> Self {
        self.icon_font = icon_font;
        self
    }
}

impl Scene for WorkspaceSwitcher {
    fn resolve(
        self,
        context: &mut ResolveContext,
        scene: &mut ResolvedScene,
    ) -> Result<(), ResolveSceneError> {
        let initial = self.current_workspace.chars().next().unwrap_or('A').to_string();
        let ws_name = self.current_workspace;
        let font = self.font;
        let icon_font = self.icon_font;
        let ws_state = LumaWorkspaceSwitcher {
            current_workspace: ws_name.clone(),
            options: vec![
                ws_name.clone(),
                "Personal Studio".into(),
                "Open Source".into(),
                "Production Team".into(),
            ],
            is_open: false,
        };
        let ic = interaction_colors(
            Color::srgba(1.0, 1.0, 1.0, 0.05),
            Color::srgba(1.0, 1.0, 1.0, 0.10),
            Color::srgba(1.0, 1.0, 1.0, 0.16),
            false,
        );
        let s = bsn! {
            ws_state
            UNode {
                padding: USides { top: 6.0, right: 12.0, bottom: 6.0, left: 8.0 },
                background_color: Color::srgba(1.0, 1.0, 1.0, 0.05),
                border_radius: UCornerRadius::all(6.0),
            }
            UBorder {
                color: Color::srgba(1.0, 1.0, 1.0, 0.1),
                width: 1.0,
                radius: UCornerRadius::all(6.0),
            }
            UInteraction::default()
            ic
            UFocusable::default()
            ULayout {
                display: UDisplay::Flex,
                flex_direction: UFlexDirection::Row,
                align_items: UAlignItems::Center,
                gap: 8.0,
            }
            Children [
                @{Avatar::new(initial).size(AvatarSize::Small).font(font.clone())}
                --
                LumaWorkspaceLabel
                UText {
                    text: ws_name,
                    font_size: 13.0,
                    color: Color::srgb(0.92, 0.94, 0.98),
                    font,
                }
                --
                UText {
                    text: { Icon::CHEVRON_DOWN.to_string() },
                    font_size: 12.0,
                    color: Color::srgb(0.55, 0.6, 0.7),
                    font: icon_font,
                }
            ]
        };
        s.resolve(context, scene)
    }
}

// ============================================================================
// 2.24 Language & Currency Selector
// ============================================================================

/// Language and currency switcher dropdown trigger.
#[derive(Clone, Debug, Reflect)]
pub struct LocaleSelector {
    pub current_locale: String,
    pub font: Handle<Font>,
    pub icon_font: Handle<Font>,
}

impl Default for LocaleSelector {
    fn default() -> Self {
        Self {
            current_locale: "EN / USD".into(),
            font: Handle::default(),
            icon_font: Handle::default(),
        }
    }
}

impl LocaleSelector {
    pub fn new(locale: impl Into<String>) -> Self {
        Self {
            current_locale: locale.into(),
            ..default()
        }
    }

    pub fn font(mut self, font: Handle<Font>) -> Self {
        self.font = font;
        self
    }

    pub fn icon_font(mut self, icon_font: Handle<Font>) -> Self {
        self.icon_font = icon_font;
        self
    }
}

impl Scene for LocaleSelector {
    fn resolve(
        self,
        context: &mut ResolveContext,
        scene: &mut ResolvedScene,
    ) -> Result<(), ResolveSceneError> {
        let current_locale = self.current_locale;
        let ifont_folder = self.icon_font.clone();
        let font = self.font;
        let icon_font = self.icon_font;
        let locale_state = LumaLocaleSelector {
            current_locale: current_locale.clone(),
            options: vec![
                current_locale.clone(),
                "AR / SAR".into(),
                "FR / EUR".into(),
                "DE / EUR".into(),
                "JP / JPY".into(),
            ],
            is_open: false,
        };
        let ic = interaction_colors(
            Color::srgba(1.0, 1.0, 1.0, 0.04),
            Color::srgba(1.0, 1.0, 1.0, 0.09),
            Color::srgba(1.0, 1.0, 1.0, 0.15),
            false,
        );
        let s = bsn! {
            locale_state
            UNode {
                padding: USides::axes(10.0, 6.0),
                background_color: Color::srgba(1.0, 1.0, 1.0, 0.04),
                border_radius: UCornerRadius::all(6.0),
            }
            UBorder {
                color: Color::srgba(1.0, 1.0, 1.0, 0.1),
                width: 1.0,
                radius: UCornerRadius::all(6.0),
            }
            UInteraction::default()
            ic
            UFocusable::default()
            ULayout {
                display: UDisplay::Flex,
                flex_direction: UFlexDirection::Row,
                align_items: UAlignItems::Center,
                gap: 6.0,
            }
            Children [
                UText {
                    text: { Icon::FOLDER.to_string() },
                    font_size: 14.0,
                    color: Color::srgb(0.6, 0.65, 0.75),
                    font: ifont_folder,
                }
                --
                LumaLocaleLabel
                UText {
                    text: current_locale,
                    font_size: 12.0,
                    color: Color::srgb(0.9, 0.92, 0.98),
                    font,
                }
                --
                UText {
                    text: { Icon::CHEVRON_DOWN.to_string() },
                    font_size: 12.0,
                    color: Color::srgb(0.55, 0.6, 0.7),
                    font: icon_font,
                }
            ]
        };
        s.resolve(context, scene)
    }
}

// ============================================================================
// 2.25 Theme Toggle
// ============================================================================

/// Dark and Light mode switcher toggle button.
#[derive(Clone, Debug, Reflect)]
pub struct ThemeToggle {
    pub is_dark: bool,
    pub icon_font: Handle<Font>,
}

impl Default for ThemeToggle {
    fn default() -> Self {
        Self {
            is_dark: true,
            icon_font: Handle::default(),
        }
    }
}

impl ThemeToggle {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn dark(mut self, is_dark: bool) -> Self {
        self.is_dark = is_dark;
        self
    }

    pub fn icon_font(mut self, icon_font: Handle<Font>) -> Self {
        self.icon_font = icon_font;
        self
    }
}

impl Scene for ThemeToggle {
    fn resolve(
        self,
        context: &mut ResolveContext,
        scene: &mut ResolvedScene,
    ) -> Result<(), ResolveSceneError> {
        let is_dark = self.is_dark;
        let icon = if is_dark { Icon::MOON } else { Icon::SUN };
        let btn = Button::icon_button(icon)
            .ghost()
            .icon_font(self.icon_font);
        let s = bsn! {
            LumaThemeToggle { is_dark }
            UNode::default()
            Children [ @{btn} ]
        };
        s.resolve(context, scene)
    }
}

// ============================================================================
// Runtime Components, Events, Observers & Systems
// ============================================================================

/// Runtime state for [`Navbar`].
#[derive(Component, Clone, Copy, Debug, Default, Reflect)]
pub struct LumaNavbar {
    pub active_index: usize,
}

/// Marker on an individual item in [`Navbar`].
#[derive(Component, Clone, Debug, Default, Reflect)]
pub struct LumaNavbarItem {
    pub index: usize,
    pub label: String,
}

/// Marker wrapping the Sign In button inside [`Navbar`].
#[derive(Component, Clone, Copy, Debug, Default, Reflect)]
pub struct LumaNavbarSignIn;

/// Runtime state for [`BottomNav`].
#[derive(Component, Clone, Copy, Debug, Default, Reflect)]
pub struct LumaBottomNav {
    pub active_index: usize,
}

/// Marker on an individual item in [`BottomNav`].
#[derive(Component, Clone, Debug, Default, Reflect)]
pub struct LumaBottomNavItem {
    pub index: usize,
    pub label: String,
}

/// Runtime state for [`Sidebar`].
#[derive(Component, Clone, Copy, Debug, Default, Reflect)]
pub struct LumaSidebar {
    pub active_index: usize,
    pub collapsed: bool,
}

/// Marker on an individual item in [`Sidebar`].
#[derive(Component, Clone, Debug, Default, Reflect)]
pub struct LumaSidebarItem {
    pub index: usize,
    pub label: String,
}

/// Runtime state for [`Pagination`].
#[derive(Component, Clone, Copy, Debug, Default, Reflect)]
pub struct LumaPagination {
    pub current_page: usize,
    pub total_pages: usize,
}

/// Navigation action for a [`Pagination`] button.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Default, Reflect)]
pub enum PaginationAction {
    #[default]
    Prev,
    Page(usize),
    Next,
}

/// Marker wrapping each button inside [`Pagination`].
#[derive(Component, Clone, Copy, Debug, Default, Reflect)]
pub struct LumaPaginationBtn {
    pub action: PaginationAction,
}

/// Runtime state for [`StepTracker`].
#[derive(Component, Clone, Debug, Default, Reflect)]
pub struct LumaStepTracker {
    pub current_step: usize,
    pub total_steps: usize,
}

/// Marker on each step item in [`StepTracker`].
#[derive(Component, Clone, Debug, Default, Reflect)]
pub struct LumaStepItem {
    pub index: usize,
    pub title: String,
}

/// Marker on step circle background in [`StepTracker`].
#[derive(Component, Clone, Copy, Debug, Default, Reflect)]
pub struct LumaStepCircle;

/// Marker on step circle digit text in [`StepTracker`].
#[derive(Component, Clone, Copy, Debug, Default, Reflect)]
pub struct LumaStepCircleText;

/// Marker on step circle check icon in [`StepTracker`].
#[derive(Component, Clone, Copy, Debug, Default, Reflect)]
pub struct LumaStepCircleCheck;

/// Marker on step title text in [`StepTracker`].
#[derive(Component, Clone, Copy, Debug, Default, Reflect)]
pub struct LumaStepTitle;

/// Marker on connecting line in [`StepTracker`].
#[derive(Component, Clone, Copy, Debug, Default, Reflect)]
pub struct LumaStepLine;

/// Runtime state wrapping [`HamburgerMenu`].
#[derive(Component, Clone, Copy, Debug, Default, Reflect)]
pub struct LumaHamburgerMenu {
    pub is_open: bool,
}

/// Interactive link item inside [`MegaMenu`] or [`Footer`].
#[derive(Component, Clone, Debug, Default, Reflect)]
pub struct LumaNavLinkItem {
    pub group: String,
    pub label: String,
}

/// Root marker for [`ContextMenu`].
#[derive(Component, Clone, Copy, Debug, Default, Reflect)]
pub struct LumaContextMenu;

/// Interactive item row inside [`ContextMenu`].
#[derive(Component, Clone, Debug, Default, Reflect)]
pub struct LumaContextMenuItem {
    pub index: usize,
    pub label: String,
}

/// Runtime state for [`CustomScrollbar`].
#[derive(Component, Clone, Copy, Debug, Default, Reflect)]
pub struct LumaCustomScrollbar {
    pub progress: f32,
    pub is_horizontal: bool,
}

/// Thumb marker inside [`CustomScrollbar`].
#[derive(Component, Clone, Copy, Debug, Default, Reflect)]
pub struct LumaCustomScrollbarThumb;

/// Marker wrapping [`BackToTopButton`].
#[derive(Component, Clone, Copy, Debug, Default, Reflect)]
pub struct LumaBackToTop;

/// Root marker for [`ActionSheet`].
#[derive(Component, Clone, Copy, Debug, Default, Reflect)]
pub struct LumaActionSheet;

/// Marker wrapping an action button inside [`ActionSheet`].
#[derive(Component, Clone, Debug, Default, Reflect)]
pub struct LumaActionSheetAction {
    pub index: usize,
    pub label: String,
}

/// Marker wrapping the cancel button inside [`ActionSheet`].
#[derive(Component, Clone, Copy, Debug, Default, Reflect)]
pub struct LumaActionSheetCancel;

/// Runtime state for [`BottomSheet`].
#[derive(Component, Clone, Copy, Debug, Default, Reflect)]
pub struct LumaBottomSheet {
    pub expanded: bool,
}

/// Interactive drag/click grab handle inside [`BottomSheet`].
#[derive(Component, Clone, Copy, Debug, Default, Reflect)]
pub struct LumaBottomSheetHandle;

/// Runtime state for [`Splitter`].
#[derive(Component, Clone, Copy, Debug, Default, Reflect)]
pub struct LumaSplitter {
    pub is_horizontal: bool,
    pub offset: f32,
}

/// Runtime state for [`TreeView`].
#[derive(Component, Clone, Debug, Default, Reflect)]
pub struct LumaTreeView {
    pub selected_index: Option<usize>,
}

/// Marker and state for each row in [`TreeView`].
#[derive(Component, Clone, Debug, Default, Reflect)]
pub struct LumaTreeNode {
    pub index: usize,
    pub depth: usize,
    pub label: String,
    pub is_folder: bool,
    pub expanded: bool,
}

/// Marker on icon `UText` inside [`TreeView`] nodes.
#[derive(Component, Clone, Copy, Debug, Default, Reflect)]
pub struct LumaTreeNodeIcon;

/// Runtime state for [`CarouselControls`].
#[derive(Component, Clone, Copy, Debug, Default, Reflect)]
pub struct LumaCarouselControls {
    pub current: usize,
    pub total: usize,
}

/// Prev/Next action for [`CarouselControls`] buttons.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Default, Reflect)]
pub enum CarouselAction {
    #[default]
    Prev,
    Next,
}

/// Marker wrapping Prev/Next buttons inside [`CarouselControls`].
#[derive(Component, Clone, Copy, Debug, Default, Reflect)]
pub struct LumaCarouselBtn {
    pub action: CarouselAction,
}

/// Marker on indicator dot nodes inside [`CarouselControls`].
#[derive(Component, Clone, Copy, Debug, Default, Reflect)]
pub struct LumaCarouselDot {
    pub index: usize,
}

/// Runtime state for [`TableOfContents`].
#[derive(Component, Clone, Copy, Debug, Default, Reflect)]
pub struct LumaTableOfContents {
    pub active_index: usize,
}

/// Interactive section row inside [`TableOfContents`].
#[derive(Component, Clone, Debug, Default, Reflect)]
pub struct LumaTocItem {
    pub index: usize,
    pub section: String,
}

/// Runtime state for [`FilterSortBar`].
#[derive(Component, Clone, Debug, Default, Reflect)]
pub struct LumaFilterSortBar {
    pub active_filter: String,
    pub sort_label: String,
    pub ascending: bool,
    pub is_grid_view: bool,
}

/// Marker wrapping filter chips in [`FilterSortBar`].
#[derive(Component, Clone, Debug, Default, Reflect)]
pub struct LumaFilterChipItem {
    pub index: usize,
    pub label: String,
}

/// Marker wrapping the sort button in [`FilterSortBar`].
#[derive(Component, Clone, Copy, Debug, Default, Reflect)]
pub struct LumaFilterSortBtn;

/// Marker wrapping the view toggle button in [`FilterSortBar`].
#[derive(Component, Clone, Copy, Debug, Default, Reflect)]
pub struct LumaFilterViewBtn;

/// Root marker for [`FloatingToolbar`].
#[derive(Component, Clone, Copy, Debug, Default, Reflect)]
pub struct LumaFloatingToolbar;

/// Built-in action identifier for [`FloatingToolbar`] buttons.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Default, Reflect)]
pub enum FloatingToolbarActionKind {
    #[default]
    Favorite,
    CopyLink,
    Approve,
    Dismiss,
}

/// Marker wrapping buttons inside [`FloatingToolbar`].
#[derive(Component, Clone, Copy, Debug, Default, Reflect)]
pub struct LumaFloatingToolbarBtn {
    pub action: FloatingToolbarActionKind,
}

/// Runtime state for [`WorkspaceSwitcher`].
#[derive(Component, Clone, Debug, Default, Reflect)]
pub struct LumaWorkspaceSwitcher {
    pub current_workspace: String,
    pub options: Vec<String>,
    pub is_open: bool,
}

/// Marker on workspace label text inside [`WorkspaceSwitcher`].
#[derive(Component, Clone, Copy, Debug, Default, Reflect)]
pub struct LumaWorkspaceLabel;

/// Runtime state for [`LocaleSelector`].
#[derive(Component, Clone, Debug, Default, Reflect)]
pub struct LumaLocaleSelector {
    pub current_locale: String,
    pub options: Vec<String>,
    pub is_open: bool,
}

/// Marker on locale label text inside [`LocaleSelector`].
#[derive(Component, Clone, Copy, Debug, Default, Reflect)]
pub struct LumaLocaleLabel;

/// Full-screen transparent backdrop for navigation switcher popups.
#[derive(Component, Clone, Copy, Debug, Reflect)]
pub struct LumaNavDropdownBackdrop {
    pub owner: Entity,
}

impl Default for LumaNavDropdownBackdrop {
    fn default() -> Self {
        Self {
            owner: Entity::PLACEHOLDER,
        }
    }
}

/// Selectable item inside a navigation switcher popup overlay.
#[derive(Component, Clone, Debug, Reflect)]
pub struct LumaNavDropdownItem {
    pub owner: Entity,
    pub index: usize,
    pub value: String,
}

impl Default for LumaNavDropdownItem {
    fn default() -> Self {
        Self {
            owner: Entity::PLACEHOLDER,
            index: 0,
            value: String::new(),
        }
    }
}

/// Runtime state wrapping [`ThemeToggle`].
#[derive(Component, Clone, Copy, Debug, Default, Reflect)]
pub struct LumaThemeToggle {
    pub is_dark: bool,
}

// Semantic Navigation Events

/// Event emitted when a [`Navbar`] item is selected.
#[derive(Event, Clone, Debug, Reflect)]
pub struct NavbarItemSelected {
    pub entity: Entity,
    pub index: usize,
    pub label: String,
}

/// Event emitted when the Sign In button in [`Navbar`] is activated.
#[derive(Event, Clone, Copy, Debug, Reflect)]
pub struct NavbarSignInClicked {
    pub entity: Entity,
}

/// Event emitted when a [`BottomNav`] item is selected.
#[derive(Event, Clone, Debug, Reflect)]
pub struct BottomNavChanged {
    pub entity: Entity,
    pub index: usize,
    pub label: String,
}

/// Event emitted when a [`Sidebar`] item is selected.
#[derive(Event, Clone, Debug, Reflect)]
pub struct SidebarItemSelected {
    pub entity: Entity,
    pub index: usize,
    pub label: String,
}

/// Event emitted when [`Pagination`] changes active page.
#[derive(Event, Clone, Copy, Debug, Reflect)]
pub struct PageChanged {
    pub entity: Entity,
    pub page: usize,
}

/// Event emitted when a step in [`StepTracker`] is activated.
#[derive(Event, Clone, Debug, Reflect)]
pub struct StepSelected {
    pub entity: Entity,
    pub index: usize,
    pub title: String,
}

/// Event emitted when [`HamburgerMenu`] toggles open state.
#[derive(Event, Clone, Copy, Debug, Reflect)]
pub struct HamburgerToggled {
    pub entity: Entity,
    pub is_open: bool,
}

/// Event emitted when a link inside [`MegaMenu`] or [`Footer`] is clicked.
#[derive(Event, Clone, Debug, Reflect)]
pub struct NavLinkClicked {
    pub entity: Entity,
    pub group: String,
    pub label: String,
}

/// Event emitted when an item inside [`ContextMenu`] is selected.
#[derive(Event, Clone, Debug, Reflect)]
pub struct ContextMenuItemSelected {
    pub entity: Entity,
    pub index: usize,
    pub label: String,
}

/// Event emitted when [`CustomScrollbar`] progress changes.
#[derive(Event, Clone, Copy, Debug, Reflect)]
pub struct ScrollbarChanged {
    pub entity: Entity,
    pub progress: f32,
}

/// Event emitted when [`BackToTopButton`] is activated.
#[derive(Event, Clone, Copy, Debug, Reflect)]
pub struct BackToTopClicked {
    pub entity: Entity,
}

/// Event emitted when an action in [`ActionSheet`] is selected.
#[derive(Event, Clone, Debug, Reflect)]
pub struct ActionSheetSelected {
    pub entity: Entity,
    pub index: usize,
    pub label: String,
}

/// Event emitted when [`ActionSheet`] cancel button is activated.
#[derive(Event, Clone, Copy, Debug, Reflect)]
pub struct ActionSheetCancelled {
    pub entity: Entity,
}

/// Event emitted when [`BottomSheet`] toggles expanded/collapsed state.
#[derive(Event, Clone, Copy, Debug, Reflect)]
pub struct BottomSheetToggled {
    pub entity: Entity,
    pub expanded: bool,
}

/// Event emitted when [`Splitter`] is dragged.
#[derive(Event, Clone, Copy, Debug, Reflect)]
pub struct SplitterDragged {
    pub entity: Entity,
    pub is_horizontal: bool,
    pub delta: f32,
    pub offset: f32,
}

/// Event emitted when a node in [`TreeView`] is clicked.
#[derive(Event, Clone, Debug, Reflect)]
pub struct TreeNodeSelected {
    pub entity: Entity,
    pub index: usize,
    pub label: String,
    pub is_folder: bool,
}

/// Event emitted when a folder in [`TreeView`] expands or collapses.
#[derive(Event, Clone, Copy, Debug, Reflect)]
pub struct TreeNodeToggled {
    pub entity: Entity,
    pub index: usize,
    pub expanded: bool,
}

/// Event emitted when [`CarouselControls`] active index changes.
#[derive(Event, Clone, Copy, Debug, Reflect)]
pub struct CarouselChanged {
    pub entity: Entity,
    pub index: usize,
}

/// Event emitted when a section in [`TableOfContents`] is clicked.
#[derive(Event, Clone, Debug, Reflect)]
pub struct TocSectionSelected {
    pub entity: Entity,
    pub index: usize,
    pub section: String,
}

/// Event emitted when [`FilterSortBar`] filter, sort direction, or view mode changes.
#[derive(Event, Clone, Debug, Reflect)]
pub struct FilterSortChanged {
    pub entity: Entity,
    pub active_filter: String,
    pub sort_label: String,
    pub ascending: bool,
    pub is_grid_view: bool,
}

/// Event emitted when a [`FloatingToolbar`] action button is activated.
#[derive(Event, Clone, Copy, Debug, Reflect)]
pub struct FloatingToolbarAction {
    pub entity: Entity,
    pub action: FloatingToolbarActionKind,
}

/// Event emitted when [`WorkspaceSwitcher`] selects a workspace.
#[derive(Event, Clone, Debug, Reflect)]
pub struct WorkspaceChanged {
    pub entity: Entity,
    pub workspace: String,
}

/// Event emitted when [`LocaleSelector`] selects a locale.
#[derive(Event, Clone, Debug, Reflect)]
pub struct LocaleChanged {
    pub entity: Entity,
    pub locale: String,
}

/// Event emitted when [`ThemeToggle`] toggles theme polarity.
#[derive(Event, Clone, Copy, Debug, Reflect)]
pub struct ThemeToggled {
    pub entity: Entity,
    pub is_dark: bool,
}

// --- Navbar Observers ---

fn activate_navbar_item(
    item_entity: Entity,
    commands: &mut Commands,
    parents_query: &Query<&ChildOf>,
    children_query: &Query<&Children>,
    navbar_q: &mut Query<&mut LumaNavbar>,
    item_q: &mut Query<(&LumaNavbarItem, &mut UBorder)>,
    text_q: &mut Query<&mut UText>,
) {
    let Ok((item, _)) = item_q.get(item_entity) else {
        return;
    };
    let idx = item.index;
    let label = item.label.clone();
    let Some(nav_entity) = find_ancestor(item_entity, navbar_q, parents_query) else {
        return;
    };
    if let Ok(mut nav) = navbar_q.get_mut(nav_entity) {
        nav.active_index = idx;
    }
    visit_descendants(nav_entity, children_query, |e| {
        if let Ok((it, mut border)) = item_q.get_mut(e) {
            let active = it.index == idx;
            border.color = if active {
                Color::srgb(0.35, 0.65, 1.0)
            } else {
                Color::NONE
            };
            let txt_col = if active {
                Color::srgb(0.96, 0.97, 0.99)
            } else {
                Color::srgb(0.65, 0.70, 0.80)
            };
            visit_descendants(e, children_query, |c| {
                if let Ok(mut t) = text_q.get_mut(c) {
                    t.color = txt_col;
                }
                DescendantVisit::Continue
            });
            return DescendantVisit::SkipChildren;
        }
        DescendantVisit::Continue
    });
    commands.trigger(NavbarItemSelected {
        entity: nav_entity,
        index: idx,
        label,
    });
}

fn on_navbar_item_click(
    trigger: On<PointerClick>,
    mut commands: Commands,
    parents_query: Query<&ChildOf>,
    children_query: Query<&Children>,
    mut navbar_q: Query<&mut LumaNavbar>,
    mut item_q: Query<(&LumaNavbarItem, &mut UBorder)>,
    mut text_q: Query<&mut UText>,
) {
    if !is_primary_click(&trigger) {
        return;
    }
    if let Some(ent) = find_ancestor_or_self(trigger.entity, &item_q, &parents_query) {
        activate_navbar_item(
            ent,
            &mut commands,
            &parents_query,
            &children_query,
            &mut navbar_q,
            &mut item_q,
            &mut text_q,
        );
    }
}

fn on_navbar_item_focus(
    trigger: On<UFocusActivate>,
    mut commands: Commands,
    parents_query: Query<&ChildOf>,
    children_query: Query<&Children>,
    mut navbar_q: Query<&mut LumaNavbar>,
    mut item_q: Query<(&LumaNavbarItem, &mut UBorder)>,
    mut text_q: Query<&mut UText>,
) {
    if let Some(ent) = find_ancestor_or_self(trigger.entity, &item_q, &parents_query) {
        activate_navbar_item(
            ent,
            &mut commands,
            &parents_query,
            &children_query,
            &mut navbar_q,
            &mut item_q,
            &mut text_q,
        );
    }
}

// --- BottomNav Observers ---

fn activate_bottom_nav_item(
    item_entity: Entity,
    commands: &mut Commands,
    parents_query: &Query<&ChildOf>,
    children_query: &Query<&Children>,
    bnav_q: &mut Query<&mut LumaBottomNav>,
    item_q: &Query<&LumaBottomNavItem>,
    text_q: &mut Query<&mut UText>,
) {
    let Ok(item) = item_q.get(item_entity) else {
        return;
    };
    let idx = item.index;
    let label = item.label.clone();
    let Some(bnav_entity) = find_ancestor(item_entity, bnav_q, parents_query) else {
        return;
    };
    if let Ok(mut bnav) = bnav_q.get_mut(bnav_entity) {
        bnav.active_index = idx;
    }
    visit_descendants(bnav_entity, children_query, |e| {
        if let Ok(it) = item_q.get(e) {
            let color = if it.index == idx {
                Color::srgb(0.35, 0.65, 1.0)
            } else {
                Color::srgb(0.55, 0.60, 0.70)
            };
            visit_descendants(e, children_query, |c| {
                if let Ok(mut t) = text_q.get_mut(c) {
                    t.color = color;
                }
                DescendantVisit::Continue
            });
            return DescendantVisit::SkipChildren;
        }
        DescendantVisit::Continue
    });
    commands.trigger(BottomNavChanged {
        entity: bnav_entity,
        index: idx,
        label,
    });
}

fn on_bottom_nav_item_click(
    trigger: On<PointerClick>,
    mut commands: Commands,
    parents_query: Query<&ChildOf>,
    children_query: Query<&Children>,
    mut bnav_q: Query<&mut LumaBottomNav>,
    item_q: Query<&LumaBottomNavItem>,
    mut text_q: Query<&mut UText>,
) {
    if !is_primary_click(&trigger) {
        return;
    }
    if let Some(ent) = find_ancestor_or_self(trigger.entity, &item_q, &parents_query) {
        activate_bottom_nav_item(
            ent,
            &mut commands,
            &parents_query,
            &children_query,
            &mut bnav_q,
            &item_q,
            &mut text_q,
        );
    }
}

fn on_bottom_nav_item_focus(
    trigger: On<UFocusActivate>,
    mut commands: Commands,
    parents_query: Query<&ChildOf>,
    children_query: Query<&Children>,
    mut bnav_q: Query<&mut LumaBottomNav>,
    item_q: Query<&LumaBottomNavItem>,
    mut text_q: Query<&mut UText>,
) {
    if let Some(ent) = find_ancestor_or_self(trigger.entity, &item_q, &parents_query) {
        activate_bottom_nav_item(
            ent,
            &mut commands,
            &parents_query,
            &children_query,
            &mut bnav_q,
            &item_q,
            &mut text_q,
        );
    }
}

// --- Sidebar Observers ---

fn activate_sidebar_item(
    item_entity: Entity,
    commands: &mut Commands,
    parents_query: &Query<&ChildOf>,
    children_query: &Query<&Children>,
    sidebar_q: &mut Query<&mut LumaSidebar>,
    item_q: &mut Query<(&LumaSidebarItem, &mut UNode, &mut UInteractionColors)>,
    text_q: &mut Query<&mut UText>,
) {
    let Ok((item, _, _)) = item_q.get(item_entity) else {
        return;
    };
    let idx = item.index;
    let label = item.label.clone();
    let Some(sb_entity) = find_ancestor(item_entity, sidebar_q, parents_query) else {
        return;
    };
    if let Ok(mut sb) = sidebar_q.get_mut(sb_entity) {
        sb.active_index = idx;
    }
    let active_bg = Color::srgba(0.35, 0.65, 1.0, 0.15);
    visit_descendants(sb_entity, children_query, |e| {
        if let Ok((it, mut node, mut ic)) = item_q.get_mut(e) {
            let active = it.index == idx;
            let bg = if active { active_bg } else { Color::NONE };
            let txt_col = if active {
                Color::srgb(0.35, 0.65, 1.0)
            } else {
                Color::srgb(0.70, 0.75, 0.85)
            };
            node.background_color = bg;
            *ic = interaction_colors(
                bg,
                Color::srgba(1.0, 1.0, 1.0, 0.06),
                Color::srgba(1.0, 1.0, 1.0, 0.12),
                false,
            );
            visit_descendants(e, children_query, |c| {
                if let Ok(mut t) = text_q.get_mut(c) {
                    t.color = txt_col;
                }
                DescendantVisit::Continue
            });
            return DescendantVisit::SkipChildren;
        }
        DescendantVisit::Continue
    });
    commands.trigger(SidebarItemSelected {
        entity: sb_entity,
        index: idx,
        label,
    });
}

fn on_sidebar_item_click(
    trigger: On<PointerClick>,
    mut commands: Commands,
    parents_query: Query<&ChildOf>,
    children_query: Query<&Children>,
    mut sidebar_q: Query<&mut LumaSidebar>,
    mut item_q: Query<(&LumaSidebarItem, &mut UNode, &mut UInteractionColors)>,
    mut text_q: Query<&mut UText>,
) {
    if !is_primary_click(&trigger) {
        return;
    }
    if let Some(ent) = find_ancestor_or_self(trigger.entity, &item_q, &parents_query) {
        activate_sidebar_item(
            ent,
            &mut commands,
            &parents_query,
            &children_query,
            &mut sidebar_q,
            &mut item_q,
            &mut text_q,
        );
    }
}

fn on_sidebar_item_focus(
    trigger: On<UFocusActivate>,
    mut commands: Commands,
    parents_query: Query<&ChildOf>,
    children_query: Query<&Children>,
    mut sidebar_q: Query<&mut LumaSidebar>,
    mut item_q: Query<(&LumaSidebarItem, &mut UNode, &mut UInteractionColors)>,
    mut text_q: Query<&mut UText>,
) {
    if let Some(ent) = find_ancestor_or_self(trigger.entity, &item_q, &parents_query) {
        activate_sidebar_item(
            ent,
            &mut commands,
            &parents_query,
            &children_query,
            &mut sidebar_q,
            &mut item_q,
            &mut text_q,
        );
    }
}

// --- Pagination Observer ---

fn on_pagination_btn_click(
    trigger: On<ButtonClicked>,
    mut commands: Commands,
    parents_query: Query<&ChildOf>,
    children_query: Query<&Children>,
    pag_btn_q: Query<&LumaPaginationBtn>,
    mut pag_q: Query<&mut LumaPagination>,
    mut btn_query: Query<(&mut LumaButton, &mut UNode, &mut UBorder, &mut UInteractionColors)>,
    mut text_query: Query<&mut UText>,
    btn_parts_query: Query<(), Or<(With<LumaButtonLabel>, With<LumaButtonIcon>)>>,
) {
    let Some(pag_btn_ent) = find_ancestor(trigger.0, &pag_btn_q, &parents_query) else {
        return;
    };
    let Ok(pag_btn) = pag_btn_q.get(pag_btn_ent) else {
        return;
    };
    let Some(pag_ent) = find_ancestor(pag_btn_ent, &pag_q, &parents_query) else {
        return;
    };
    let Ok(mut pag) = pag_q.get_mut(pag_ent) else {
        return;
    };

    let total = pag.total_pages.max(1);
    let next_page = match pag_btn.action {
        PaginationAction::Prev => pag.current_page.saturating_sub(1).max(1),
        PaginationAction::Next => (pag.current_page + 1).min(total),
        PaginationAction::Page(p) => p.clamp(1, total),
    };
    pag.current_page = next_page;

    visit_descendants(pag_ent, &children_query, |e| {
        if let Ok(pbtn) = pag_btn_q.get(e) {
            if let PaginationAction::Page(p) = pbtn.action {
                let target_variant = if p == next_page {
                    ButtonVariant::Primary
                } else {
                    ButtonVariant::Outline
                };
                if let Ok(wrap_children) = children_query.get(e) {
                    for &child in wrap_children {
                        if let Ok((mut luma_btn, mut node, mut border, mut ic)) =
                            btn_query.get_mut(child)
                        {
                            apply_button_variant_visuals(
                                target_variant,
                                &mut luma_btn,
                                &mut node,
                                &mut border,
                                &mut ic,
                                children_query.get(child).ok(),
                                &children_query,
                                &mut text_query,
                                &btn_parts_query,
                            );
                        }
                    }
                }
            }
            return DescendantVisit::SkipChildren;
        }
        DescendantVisit::Continue
    });

    commands.trigger(PageChanged {
        entity: pag_ent,
        page: next_page,
    });
}

// --- StepTracker Observers ---

fn activate_step_item(
    item_entity: Entity,
    commands: &mut Commands,
    parents_query: &Query<&ChildOf>,
    children_query: &Query<&Children>,
    stepper_q: &mut Query<&mut LumaStepTracker>,
    step_item_q: &Query<&LumaStepItem>,
    circle_q: &Query<(), With<LumaStepCircle>>,
    line_q: &Query<(), With<LumaStepLine>>,
    circle_text_q: &Query<(), With<LumaStepCircleText>>,
    circle_check_q: &Query<(), With<LumaStepCircleCheck>>,
    title_q: &Query<(), With<LumaStepTitle>>,
    mut node_q: &mut Query<&mut UNode>,
    mut text_q: &mut Query<&mut UText>,
    mut layout_q: &mut Query<&mut ULayout>,
) {
    let Ok(item) = step_item_q.get(item_entity) else {
        return;
    };
    let idx = item.index;
    let title = item.title.clone();
    let Some(stepper_ent) = find_ancestor(item_entity, stepper_q, parents_query) else {
        return;
    };
    let current = {
        let Ok(mut stepper) = stepper_q.get_mut(stepper_ent) else {
            return;
        };
        let max_idx = stepper.total_steps.saturating_sub(1);
        let clamped = idx.min(max_idx);
        stepper.current_step = clamped;
        clamped
    };
    let _ = &mut node_q;
    let _ = &mut text_q;
    let _ = &mut layout_q;

    visit_descendants(stepper_ent, children_query, |e| {
        if let Ok(st_item) = step_item_q.get(e) {
            let s_idx = st_item.index;
            let is_done = s_idx < current;
            let is_active = s_idx == current;
            let circle_bg = if is_done || is_active {
                Color::srgb(0.35, 0.65, 1.0)
            } else {
                Color::srgba(0.2, 0.22, 0.3, 0.6)
            };
            let title_col = if is_active {
                Color::srgb(0.96, 0.97, 0.99)
            } else {
                Color::srgb(0.6, 0.65, 0.75)
            };
            let line_col = if is_done {
                Color::srgb(0.35, 0.65, 1.0)
            } else {
                Color::srgba(0.2, 0.22, 0.3, 0.6)
            };

            visit_descendants(e, children_query, |c| {
                if circle_q.contains(c) {
                    if let Ok(mut n) = node_q.get_mut(c) {
                        n.background_color = circle_bg;
                    }
                } else if line_q.contains(c) {
                    if let Ok(mut n) = node_q.get_mut(c) {
                        n.background_color = line_col;
                    }
                } else if circle_text_q.contains(c) {
                    if let Ok(mut layout) = layout_q.get_mut(c) {
                        layout.display = if is_done { UDisplay::None } else { UDisplay::Flex };
                    }
                } else if circle_check_q.contains(c) {
                    if let Ok(mut layout) = layout_q.get_mut(c) {
                        layout.display = if is_done { UDisplay::Flex } else { UDisplay::None };
                    }
                } else if title_q.contains(c) {
                    if let Ok(mut t) = text_q.get_mut(c) {
                        t.color = title_col;
                    }
                }
                DescendantVisit::Continue
            });
            return DescendantVisit::SkipChildren;
        }
        DescendantVisit::Continue
    });

    commands.trigger(StepSelected {
        entity: stepper_ent,
        index: current,
        title,
    });
}

fn on_step_item_click(
    trigger: On<PointerClick>,
    mut commands: Commands,
    parents_query: Query<&ChildOf>,
    children_query: Query<&Children>,
    mut stepper_q: Query<&mut LumaStepTracker>,
    step_item_q: Query<&LumaStepItem>,
    circle_q: Query<(), With<LumaStepCircle>>,
    line_q: Query<(), With<LumaStepLine>>,
    circle_text_q: Query<(), With<LumaStepCircleText>>,
    circle_check_q: Query<(), With<LumaStepCircleCheck>>,
    title_q: Query<(), With<LumaStepTitle>>,
    mut node_q: Query<&mut UNode>,
    mut text_q: Query<&mut UText>,
    mut layout_q: Query<&mut ULayout>,
) {
    if !is_primary_click(&trigger) {
        return;
    }
    if let Some(ent) = find_ancestor_or_self(trigger.entity, &step_item_q, &parents_query) {
        activate_step_item(
            ent,
            &mut commands,
            &parents_query,
            &children_query,
            &mut stepper_q,
            &step_item_q,
            &circle_q,
            &line_q,
            &circle_text_q,
            &circle_check_q,
            &title_q,
            &mut node_q,
            &mut text_q,
            &mut layout_q,
        );
    }
}

fn on_step_item_focus(
    trigger: On<UFocusActivate>,
    mut commands: Commands,
    parents_query: Query<&ChildOf>,
    children_query: Query<&Children>,
    mut stepper_q: Query<&mut LumaStepTracker>,
    step_item_q: Query<&LumaStepItem>,
    circle_q: Query<(), With<LumaStepCircle>>,
    line_q: Query<(), With<LumaStepLine>>,
    circle_text_q: Query<(), With<LumaStepCircleText>>,
    circle_check_q: Query<(), With<LumaStepCircleCheck>>,
    title_q: Query<(), With<LumaStepTitle>>,
    mut node_q: Query<&mut UNode>,
    mut text_q: Query<&mut UText>,
    mut layout_q: Query<&mut ULayout>,
) {
    if let Some(ent) = find_ancestor_or_self(trigger.entity, &step_item_q, &parents_query) {
        activate_step_item(
            ent,
            &mut commands,
            &parents_query,
            &children_query,
            &mut stepper_q,
            &step_item_q,
            &circle_q,
            &line_q,
            &circle_text_q,
            &circle_check_q,
            &title_q,
            &mut node_q,
            &mut text_q,
            &mut layout_q,
        );
    }
}

// --- Simple Button-Wrapped Navigation Controls ---

fn on_nav_simple_button_click(
    trigger: On<ButtonClicked>,
    mut commands: Commands,
    parents_query: Query<&ChildOf>,
    children_query: Query<&Children>,
    sign_in_q: Query<(), With<LumaNavbarSignIn>>,
    navbar_q: Query<(), With<LumaNavbar>>,
    mut hamburger_q: Query<&mut LumaHamburgerMenu>,
    back_to_top_q: Query<(), With<LumaBackToTop>>,
    sheet_action_q: Query<&LumaActionSheetAction>,
    sheet_cancel_q: Query<(), With<LumaActionSheetCancel>>,
    action_sheet_q: Query<(), With<LumaActionSheet>>,
    toolbar_btn_q: Query<&LumaFloatingToolbarBtn>,
    toolbar_q: Query<(), With<LumaFloatingToolbar>>,
    mut theme_q: Query<&mut LumaThemeToggle>,
    icon_marker: Query<(), With<LumaButtonIcon>>,
    mut text_q: Query<&mut UText>,
) {
    let btn = trigger.0;

    if let Some(sign_wrap) = find_ancestor(btn, &sign_in_q, &parents_query) {
        let nav_ent = find_ancestor(sign_wrap, &navbar_q, &parents_query).unwrap_or(sign_wrap);
        commands.trigger(NavbarSignInClicked { entity: nav_ent });
        return;
    }

    if let Some(ham_ent) = find_ancestor(btn, &hamburger_q, &parents_query) {
        if let Ok(mut ham) = hamburger_q.get_mut(ham_ent) {
            ham.is_open = !ham.is_open;
            let is_open = ham.is_open;
            let new_icon = if is_open { Icon::X } else { Icon::MENU };
            visit_descendants(btn, &children_query, |e| {
                if icon_marker.contains(e) {
                    if let Ok(mut t) = text_q.get_mut(e) {
                        t.text = new_icon.to_string();
                    }
                    return DescendantVisit::Stop;
                }
                DescendantVisit::Continue
            });
            commands.trigger(HamburgerToggled {
                entity: ham_ent,
                is_open,
            });
        }
        return;
    }

    if let Some(top_ent) = find_ancestor(btn, &back_to_top_q, &parents_query) {
        commands.trigger(BackToTopClicked { entity: top_ent });
        return;
    }

    if let Some(act_ent) = find_ancestor(btn, &sheet_action_q, &parents_query) {
        if let Ok(act) = sheet_action_q.get(act_ent) {
            let sheet_ent = find_ancestor(act_ent, &action_sheet_q, &parents_query).unwrap_or(act_ent);
            commands.trigger(ActionSheetSelected {
                entity: sheet_ent,
                index: act.index,
                label: act.label.clone(),
            });
        }
        return;
    }

    if let Some(cancel_ent) = find_ancestor(btn, &sheet_cancel_q, &parents_query) {
        let sheet_ent = find_ancestor(cancel_ent, &action_sheet_q, &parents_query).unwrap_or(cancel_ent);
        commands.trigger(ActionSheetCancelled { entity: sheet_ent });
        return;
    }

    if let Some(tb_btn_ent) = find_ancestor(btn, &toolbar_btn_q, &parents_query) {
        if let Ok(tb_btn) = toolbar_btn_q.get(tb_btn_ent) {
            let tb_ent = find_ancestor(tb_btn_ent, &toolbar_q, &parents_query).unwrap_or(tb_btn_ent);
            commands.trigger(FloatingToolbarAction {
                entity: tb_ent,
                action: tb_btn.action,
            });
        }
        return;
    }

    if let Some(theme_ent) = find_ancestor(btn, &theme_q, &parents_query) {
        if let Ok(mut theme) = theme_q.get_mut(theme_ent) {
            theme.is_dark = !theme.is_dark;
            let is_dark = theme.is_dark;
            let new_icon = if is_dark { Icon::MOON } else { Icon::SUN };
            visit_descendants(btn, &children_query, |e| {
                if icon_marker.contains(e) {
                    if let Ok(mut t) = text_q.get_mut(e) {
                        t.text = new_icon.to_string();
                    }
                    return DescendantVisit::Stop;
                }
                DescendantVisit::Continue
            });
            commands.trigger(ThemeToggled {
                entity: theme_ent,
                is_dark,
            });
        }
    }
}

// --- MegaMenu & Footer Nav Links + ContextMenu ---

fn on_nav_link_item_click(
    trigger: On<PointerClick>,
    mut commands: Commands,
    parents_query: Query<&ChildOf>,
    link_q: Query<&LumaNavLinkItem>,
) {
    if !is_primary_click(&trigger) {
        return;
    }
    if let Some(ent) = find_ancestor_or_self(trigger.entity, &link_q, &parents_query) {
        if let Ok(link) = link_q.get(ent) {
            commands.trigger(NavLinkClicked {
                entity: ent,
                group: link.group.clone(),
                label: link.label.clone(),
            });
        }
    }
}

fn on_nav_link_item_focus(
    trigger: On<UFocusActivate>,
    mut commands: Commands,
    parents_query: Query<&ChildOf>,
    link_q: Query<&LumaNavLinkItem>,
) {
    if let Some(ent) = find_ancestor_or_self(trigger.entity, &link_q, &parents_query) {
        if let Ok(link) = link_q.get(ent) {
            commands.trigger(NavLinkClicked {
                entity: ent,
                group: link.group.clone(),
                label: link.label.clone(),
            });
        }
    }
}

fn on_context_menu_item_click(
    trigger: On<PointerClick>,
    mut commands: Commands,
    parents_query: Query<&ChildOf>,
    item_q: Query<&LumaContextMenuItem>,
    menu_q: Query<(), With<LumaContextMenu>>,
) {
    if !is_primary_click(&trigger) {
        return;
    }
    if let Some(ent) = find_ancestor_or_self(trigger.entity, &item_q, &parents_query) {
        if let Ok(item) = item_q.get(ent) {
            let menu_ent = find_ancestor(ent, &menu_q, &parents_query).unwrap_or(ent);
            commands.trigger(ContextMenuItemSelected {
                entity: menu_ent,
                index: item.index,
                label: item.label.clone(),
            });
        }
    }
}

fn on_context_menu_item_focus(
    trigger: On<UFocusActivate>,
    mut commands: Commands,
    parents_query: Query<&ChildOf>,
    item_q: Query<&LumaContextMenuItem>,
    menu_q: Query<(), With<LumaContextMenu>>,
) {
    if let Some(ent) = find_ancestor_or_self(trigger.entity, &item_q, &parents_query) {
        if let Ok(item) = item_q.get(ent) {
            let menu_ent = find_ancestor(ent, &menu_q, &parents_query).unwrap_or(ent);
            commands.trigger(ContextMenuItemSelected {
                entity: menu_ent,
                index: item.index,
                label: item.label.clone(),
            });
        }
    }
}

// --- CustomScrollbar Handlers ---

pub(crate) fn compute_pointer_bar_ratio(
    pointer_pos: Vec2,
    hit_pos: Option<Vec3>,
    gt: &GlobalTransform,
    computed: &ComputedSize,
    is_horizontal: bool,
    fallback_len: f32,
    cameras: &Query<(&Camera, &GlobalTransform)>,
) -> f32 {
    let len = if is_horizontal {
        computed.width
    } else {
        computed.height
    };
    if len > 1.0 {
        for (cam, cam_gt) in cameras.iter() {
            if let Ok(vp) = cam.world_to_viewport(cam_gt, gt.translation()) {
                return if is_horizontal {
                    ((pointer_pos.x - (vp.x - len * 0.5)) / len).clamp(0.0, 1.0)
                } else {
                    ((pointer_pos.y - (vp.y - len * 0.5)) / len).clamp(0.0, 1.0)
                };
            }
        }
    }
    if let Some(hit) = hit_pos {
        let local = if is_horizontal {
            (hit.x - gt.translation().x) / fallback_len + 0.5
        } else {
            0.5 - (hit.y - gt.translation().y) / fallback_len
        };
        return local.clamp(0.0, 1.0);
    }
    0.5
}

fn apply_scrollbar_progress(
    sb_entity: Entity,
    sb: &mut LumaCustomScrollbar,
    progress: f32,
    commands: &mut Commands,
    children_query: &Query<&Children>,
    thumb_q: &mut Query<&mut UNode, (With<LumaCustomScrollbarThumb>, Without<LumaCustomScrollbar>)>,
) {
    let p = progress.clamp(0.0, 1.0);
    sb.progress = p;
    let margin = p * 100.0;
    visit_descendants(sb_entity, children_query, |e| {
        if let Ok(mut thumb) = thumb_q.get_mut(e) {
            if sb.is_horizontal {
                thumb.margin.left = margin;
            } else {
                thumb.margin.top = margin;
            }
            return DescendantVisit::Stop;
        }
        DescendantVisit::Continue
    });
    commands.trigger(ScrollbarChanged {
        entity: sb_entity,
        progress: p,
    });
}

fn on_scrollbar_pointer_click(
    trigger: On<PointerClick>,
    mut commands: Commands,
    parents_query: Query<&ChildOf>,
    children_query: Query<&Children>,
    cameras: Query<(&Camera, &GlobalTransform)>,
    mut sb_q: Query<(&mut LumaCustomScrollbar, &GlobalTransform, &ComputedSize)>,
    mut thumb_q: Query<&mut UNode, (With<LumaCustomScrollbarThumb>, Without<LumaCustomScrollbar>)>,
) {
    if !is_primary_click(&trigger) {
        return;
    }
    let Some(sb_ent) = find_ancestor_or_self(trigger.entity, &sb_q, &parents_query) else {
        return;
    };
    let Ok((mut sb, gt, computed)) = sb_q.get_mut(sb_ent) else {
        return;
    };
    let ratio = compute_pointer_bar_ratio(
        trigger.pointer.position,
        trigger.hit.position,
        gt,
        computed,
        sb.is_horizontal,
        140.0,
        &cameras,
    );
    apply_scrollbar_progress(sb_ent, &mut sb, ratio, &mut commands, &children_query, &mut thumb_q);
}

fn on_scrollbar_pointer_drag(
    trigger: On<PointerDrag>,
    mut commands: Commands,
    parents_query: Query<&ChildOf>,
    children_query: Query<&Children>,
    cameras: Query<(&Camera, &GlobalTransform)>,
    mut sb_q: Query<(&mut LumaCustomScrollbar, &GlobalTransform, &ComputedSize)>,
    mut thumb_q: Query<&mut UNode, (With<LumaCustomScrollbarThumb>, Without<LumaCustomScrollbar>)>,
) {
    if trigger.event().button != PointerButton::Primary {
        return;
    }
    let Some(sb_ent) = find_ancestor_or_self(trigger.entity, &sb_q, &parents_query) else {
        return;
    };
    let Ok((mut sb, gt, computed)) = sb_q.get_mut(sb_ent) else {
        return;
    };
    let ratio = compute_pointer_bar_ratio(
        trigger.pointer.position,
        None,
        gt,
        computed,
        sb.is_horizontal,
        140.0,
        &cameras,
    );
    apply_scrollbar_progress(sb_ent, &mut sb, ratio, &mut commands, &children_query, &mut thumb_q);
}

fn nav_scrollbar_keyboard_system(
    keyboard: Option<Res<ButtonInput<KeyCode>>>,
    mut commands: Commands,
    children_query: Query<&Children>,
    mut sb_q: Query<(Entity, &mut LumaCustomScrollbar), With<UFocused>>,
    mut thumb_q: Query<&mut UNode, (With<LumaCustomScrollbarThumb>, Without<LumaCustomScrollbar>)>,
) {
    let Some(keyboard) = keyboard else {
        return;
    };
    let fwd = keyboard.just_pressed(KeyCode::ArrowRight) || keyboard.just_pressed(KeyCode::ArrowDown);
    let back = keyboard.just_pressed(KeyCode::ArrowLeft) || keyboard.just_pressed(KeyCode::ArrowUp);
    if !fwd && !back {
        return;
    }
    let delta = if fwd { 0.1 } else { -0.1 };
    for (sb_ent, mut sb) in sb_q.iter_mut() {
        let next = (sb.progress + delta).clamp(0.0, 1.0);
        apply_scrollbar_progress(
            sb_ent,
            &mut sb,
            next,
            &mut commands,
            &children_query,
            &mut thumb_q,
        );
    }
}

// --- BottomSheet & Splitter ---

fn toggle_bottom_sheet(
    handle_ent: Entity,
    commands: &mut Commands,
    parents_query: &Query<&ChildOf>,
    mut sheet_q: &mut Query<(&mut LumaBottomSheet, &mut UNode)>,
) {
    let Some(sheet_ent) = find_ancestor(handle_ent, sheet_q, parents_query) else {
        return;
    };
    let _ = &mut sheet_q;
    if let Ok((mut sheet, mut node)) = sheet_q.get_mut(sheet_ent) {
        sheet.expanded = !sheet.expanded;
        node.height = UVal::Px(if sheet.expanded { 220.0 } else { 68.0 });
        commands.trigger(BottomSheetToggled {
            entity: sheet_ent,
            expanded: sheet.expanded,
        });
    }
}

fn on_bottom_sheet_handle_click(
    trigger: On<PointerClick>,
    mut commands: Commands,
    parents_query: Query<&ChildOf>,
    handle_q: Query<(), With<LumaBottomSheetHandle>>,
    mut sheet_q: Query<(&mut LumaBottomSheet, &mut UNode)>,
) {
    if !is_primary_click(&trigger) {
        return;
    }
    if let Some(ent) = find_ancestor_or_self(trigger.entity, &handle_q, &parents_query) {
        toggle_bottom_sheet(ent, &mut commands, &parents_query, &mut sheet_q);
    }
}

fn on_bottom_sheet_handle_focus(
    trigger: On<UFocusActivate>,
    mut commands: Commands,
    parents_query: Query<&ChildOf>,
    handle_q: Query<(), With<LumaBottomSheetHandle>>,
    mut sheet_q: Query<(&mut LumaBottomSheet, &mut UNode)>,
) {
    if let Some(ent) = find_ancestor_or_self(trigger.entity, &handle_q, &parents_query) {
        toggle_bottom_sheet(ent, &mut commands, &parents_query, &mut sheet_q);
    }
}

fn on_splitter_pointer_drag(
    trigger: On<PointerDrag>,
    mut commands: Commands,
    parents_query: Query<&ChildOf>,
    mut splitter_q: Query<&mut LumaSplitter>,
) {
    if trigger.event().button != PointerButton::Primary {
        return;
    }
    let Some(split_ent) = find_ancestor_or_self(trigger.entity, &splitter_q, &parents_query) else {
        return;
    };
    if let Ok(mut sp) = splitter_q.get_mut(split_ent) {
        let delta = if sp.is_horizontal {
            trigger.delta.y
        } else {
            trigger.delta.x
        };
        sp.offset += delta;
        commands.trigger(SplitterDragged {
            entity: split_ent,
            is_horizontal: sp.is_horizontal,
            delta,
            offset: sp.offset,
        });
    }
}

// --- TreeView Observers ---

fn activate_tree_node(
    node_ent: Entity,
    commands: &mut Commands,
    parents_query: &Query<&ChildOf>,
    children_query: &Query<&Children>,
    tree_q: &mut Query<&mut LumaTreeView>,
    mut node_q: &mut Query<(&mut LumaTreeNode, &mut UNode, &mut UInteractionColors, &mut ULayout)>,
    icon_marker: &Query<(), With<LumaTreeNodeIcon>>,
    text_q: &mut Query<&mut UText>,
) {
    let (target_idx, label, is_folder, mut new_expanded) = {
        let Ok((n, _, _, _)) = node_q.get(node_ent) else {
            return;
        };
        (n.index, n.label.clone(), n.is_folder, n.expanded)
    };
    let Some(tree_ent) = find_ancestor(node_ent, tree_q, parents_query) else {
        return;
    };
    if let Ok(mut tree) = tree_q.get_mut(tree_ent) {
        tree.selected_index = Some(target_idx);
    }

    if is_folder {
        if let Ok((mut n, _, _, _)) = node_q.get_mut(node_ent) {
            n.expanded = !n.expanded;
            new_expanded = n.expanded;
        }
        let icon = if new_expanded {
            Icon::FOLDER
        } else {
            Icon::CHEVRON_RIGHT
        };
        visit_descendants(node_ent, children_query, |e| {
            if icon_marker.contains(e) {
                if let Ok(mut t) = text_q.get_mut(e) {
                    t.text = icon.to_string();
                }
                return DescendantVisit::Stop;
            }
            DescendantVisit::Continue
        });
    }

    let mut collapse_depth: Option<usize> = None;
    let _ = &mut node_q;
    if let Ok(rows) = children_query.get(tree_ent) {
        for &row_ent in rows {
            if let Ok((n, mut unode, mut ic, mut layout)) = node_q.get_mut(row_ent) {
                let is_sel = n.index == target_idx;
                let bg = if is_sel {
                    Color::srgba(0.35, 0.65, 1.0, 0.16)
                } else {
                    Color::NONE
                };
                unode.background_color = bg;
                *ic = interaction_colors(
                    bg,
                    Color::srgba(1.0, 1.0, 1.0, 0.06),
                    Color::srgba(1.0, 1.0, 1.0, 0.12),
                    false,
                );
                if let Some(d) = collapse_depth {
                    if n.depth > d {
                        layout.display = UDisplay::None;
                        continue;
                    }
                    collapse_depth = None;
                }
                layout.display = UDisplay::Flex;
                if n.is_folder && !n.expanded {
                    collapse_depth = Some(n.depth);
                }
            }
        }
    }

    commands.trigger(TreeNodeSelected {
        entity: tree_ent,
        index: target_idx,
        label,
        is_folder,
    });
    if is_folder {
        commands.trigger(TreeNodeToggled {
            entity: tree_ent,
            index: target_idx,
            expanded: new_expanded,
        });
    }
}

fn on_tree_node_click(
    trigger: On<PointerClick>,
    mut commands: Commands,
    parents_query: Query<&ChildOf>,
    children_query: Query<&Children>,
    mut tree_q: Query<&mut LumaTreeView>,
    mut node_q: Query<(&mut LumaTreeNode, &mut UNode, &mut UInteractionColors, &mut ULayout)>,
    icon_marker: Query<(), With<LumaTreeNodeIcon>>,
    mut text_q: Query<&mut UText>,
) {
    if !is_primary_click(&trigger) {
        return;
    }
    if let Some(ent) = find_ancestor_or_self(trigger.entity, &node_q, &parents_query) {
        activate_tree_node(
            ent,
            &mut commands,
            &parents_query,
            &children_query,
            &mut tree_q,
            &mut node_q,
            &icon_marker,
            &mut text_q,
        );
    }
}

fn on_tree_node_focus(
    trigger: On<UFocusActivate>,
    mut commands: Commands,
    parents_query: Query<&ChildOf>,
    children_query: Query<&Children>,
    mut tree_q: Query<&mut LumaTreeView>,
    mut node_q: Query<(&mut LumaTreeNode, &mut UNode, &mut UInteractionColors, &mut ULayout)>,
    icon_marker: Query<(), With<LumaTreeNodeIcon>>,
    mut text_q: Query<&mut UText>,
) {
    if let Some(ent) = find_ancestor_or_self(trigger.entity, &node_q, &parents_query) {
        activate_tree_node(
            ent,
            &mut commands,
            &parents_query,
            &children_query,
            &mut tree_q,
            &mut node_q,
            &icon_marker,
            &mut text_q,
        );
    }
}

// --- CarouselControls Observers ---

fn sync_carousel_dots(
    carousel_ent: Entity,
    current: usize,
    children_query: &Query<&Children>,
    dot_q: &mut Query<(&LumaCarouselDot, &mut UNode, &mut UInteractionColors)>,
) {
    visit_descendants(carousel_ent, children_query, |e| {
        if let Ok((dot, mut node, mut ic)) = dot_q.get_mut(e) {
            let active = dot.index == current;
            let (w, bg) = if active {
                (20.0_f32, Color::srgb(0.35, 0.65, 1.0))
            } else {
                (6.0_f32, Color::srgba(1.0, 1.0, 1.0, 0.3))
            };
            node.width = UVal::Px(w);
            node.background_color = bg;
            *ic = interaction_colors(
                bg,
                Color::srgb(0.50, 0.75, 1.0),
                Color::srgb(0.35, 0.65, 1.0),
                false,
            );
            return DescendantVisit::SkipChildren;
        }
        DescendantVisit::Continue
    });
}

fn on_carousel_btn_click(
    trigger: On<ButtonClicked>,
    mut commands: Commands,
    parents_query: Query<&ChildOf>,
    children_query: Query<&Children>,
    btn_q: Query<&LumaCarouselBtn>,
    mut carousel_q: Query<&mut LumaCarouselControls>,
    mut dot_q: Query<(&LumaCarouselDot, &mut UNode, &mut UInteractionColors)>,
) {
    let Some(cbtn_ent) = find_ancestor(trigger.0, &btn_q, &parents_query) else {
        return;
    };
    let Ok(cbtn) = btn_q.get(cbtn_ent) else {
        return;
    };
    let Some(carousel_ent) = find_ancestor(cbtn_ent, &carousel_q, &parents_query) else {
        return;
    };
    let Ok(mut ctrl) = carousel_q.get_mut(carousel_ent) else {
        return;
    };
    if ctrl.total == 0 {
        return;
    }
    ctrl.current = match cbtn.action {
        CarouselAction::Prev => (ctrl.current + ctrl.total - 1) % ctrl.total,
        CarouselAction::Next => (ctrl.current + 1) % ctrl.total,
    };
    let cur = ctrl.current;
    sync_carousel_dots(carousel_ent, cur, &children_query, &mut dot_q);
    commands.trigger(CarouselChanged {
        entity: carousel_ent,
        index: cur,
    });
}

fn activate_carousel_dot(
    dot_ent: Entity,
    commands: &mut Commands,
    parents_query: &Query<&ChildOf>,
    children_query: &Query<&Children>,
    carousel_q: &mut Query<&mut LumaCarouselControls>,
    dot_q: &mut Query<(&LumaCarouselDot, &mut UNode, &mut UInteractionColors)>,
) {
    let Ok((dot, _, _)) = dot_q.get(dot_ent) else {
        return;
    };
    let target_idx = dot.index;
    let Some(carousel_ent) = find_ancestor(dot_ent, carousel_q, parents_query) else {
        return;
    };
    let Ok(mut ctrl) = carousel_q.get_mut(carousel_ent) else {
        return;
    };
    if ctrl.total == 0 {
        return;
    }
    ctrl.current = target_idx.min(ctrl.total - 1);
    let cur = ctrl.current;
    sync_carousel_dots(carousel_ent, cur, children_query, dot_q);
    commands.trigger(CarouselChanged {
        entity: carousel_ent,
        index: cur,
    });
}

fn on_carousel_dot_click(
    trigger: On<PointerClick>,
    mut commands: Commands,
    parents_query: Query<&ChildOf>,
    children_query: Query<&Children>,
    mut carousel_q: Query<&mut LumaCarouselControls>,
    mut dot_q: Query<(&LumaCarouselDot, &mut UNode, &mut UInteractionColors)>,
) {
    if !is_primary_click(&trigger) {
        return;
    }
    if let Some(ent) = find_ancestor_or_self(trigger.entity, &dot_q, &parents_query) {
        activate_carousel_dot(
            ent,
            &mut commands,
            &parents_query,
            &children_query,
            &mut carousel_q,
            &mut dot_q,
        );
    }
}

fn on_carousel_dot_focus(
    trigger: On<UFocusActivate>,
    mut commands: Commands,
    parents_query: Query<&ChildOf>,
    children_query: Query<&Children>,
    mut carousel_q: Query<&mut LumaCarouselControls>,
    mut dot_q: Query<(&LumaCarouselDot, &mut UNode, &mut UInteractionColors)>,
) {
    if let Some(ent) = find_ancestor_or_self(trigger.entity, &dot_q, &parents_query) {
        activate_carousel_dot(
            ent,
            &mut commands,
            &parents_query,
            &children_query,
            &mut carousel_q,
            &mut dot_q,
        );
    }
}

// --- TableOfContents Observers ---

fn activate_toc_item(
    item_ent: Entity,
    commands: &mut Commands,
    parents_query: &Query<&ChildOf>,
    children_query: &Query<&Children>,
    toc_q: &mut Query<&mut LumaTableOfContents>,
    item_q: &mut Query<(&LumaTocItem, &mut UBorder)>,
    text_q: &mut Query<&mut UText>,
) {
    let Ok((it, _)) = item_q.get(item_ent) else {
        return;
    };
    let idx = it.index;
    let section = it.section.clone();
    let Some(toc_ent) = find_ancestor(item_ent, toc_q, parents_query) else {
        return;
    };
    if let Ok(mut toc) = toc_q.get_mut(toc_ent) {
        toc.active_index = idx;
    }
    visit_descendants(toc_ent, children_query, |e| {
        if let Ok((item, mut border)) = item_q.get_mut(e) {
            let active = item.index == idx;
            border.color = if active {
                Color::srgb(0.35, 0.65, 1.0)
            } else {
                Color::NONE
            };
            let txt_col = if active {
                Color::srgb(0.96, 0.97, 0.99)
            } else {
                Color::srgb(0.60, 0.65, 0.75)
            };
            visit_descendants(e, children_query, |c| {
                if let Ok(mut t) = text_q.get_mut(c) {
                    t.color = txt_col;
                }
                DescendantVisit::Continue
            });
            return DescendantVisit::SkipChildren;
        }
        DescendantVisit::Continue
    });
    commands.trigger(TocSectionSelected {
        entity: toc_ent,
        index: idx,
        section,
    });
}

fn on_toc_item_click(
    trigger: On<PointerClick>,
    mut commands: Commands,
    parents_query: Query<&ChildOf>,
    children_query: Query<&Children>,
    mut toc_q: Query<&mut LumaTableOfContents>,
    mut item_q: Query<(&LumaTocItem, &mut UBorder)>,
    mut text_q: Query<&mut UText>,
) {
    if !is_primary_click(&trigger) {
        return;
    }
    if let Some(ent) = find_ancestor_or_self(trigger.entity, &item_q, &parents_query) {
        activate_toc_item(
            ent,
            &mut commands,
            &parents_query,
            &children_query,
            &mut toc_q,
            &mut item_q,
            &mut text_q,
        );
    }
}

fn on_toc_item_focus(
    trigger: On<UFocusActivate>,
    mut commands: Commands,
    parents_query: Query<&ChildOf>,
    children_query: Query<&Children>,
    mut toc_q: Query<&mut LumaTableOfContents>,
    mut item_q: Query<(&LumaTocItem, &mut UBorder)>,
    mut text_q: Query<&mut UText>,
) {
    if let Some(ent) = find_ancestor_or_self(trigger.entity, &item_q, &parents_query) {
        activate_toc_item(
            ent,
            &mut commands,
            &parents_query,
            &children_query,
            &mut toc_q,
            &mut item_q,
            &mut text_q,
        );
    }
}

// --- FilterSortBar Observers ---

fn emit_filter_sort_changed(commands: &mut Commands, entity: Entity, bar: &LumaFilterSortBar) {
    commands.trigger(FilterSortChanged {
        entity,
        active_filter: bar.active_filter.clone(),
        sort_label: bar.sort_label.clone(),
        ascending: bar.ascending,
        is_grid_view: bar.is_grid_view,
    });
}

fn activate_filter_chip(
    chip_wrap: Entity,
    commands: &mut Commands,
    parents_query: &Query<&ChildOf>,
    children_query: &Query<&Children>,
    bar_q: &mut Query<&mut LumaFilterSortBar>,
    chip_q: &mut Query<(&LumaFilterChipItem, &mut UBorder)>,
) {
    let Ok((clicked, _)) = chip_q.get(chip_wrap) else {
        return;
    };
    let active_idx = clicked.index;
    let active_lbl = clicked.label.clone();
    let Some(bar_ent) = find_ancestor(chip_wrap, bar_q, parents_query) else {
        return;
    };
    let Ok(mut bar) = bar_q.get_mut(bar_ent) else {
        return;
    };
    bar.active_filter = active_lbl;

    visit_descendants(bar_ent, children_query, |e| {
        if let Ok((item, mut border)) = chip_q.get_mut(e) {
            let active = item.index == active_idx;
            border.color = if active {
                Color::srgb(0.35, 0.65, 1.0)
            } else {
                Color::NONE
            };
            border.width = if active { 1.5 } else { 0.0 };
            return DescendantVisit::SkipChildren;
        }
        DescendantVisit::Continue
    });

    emit_filter_sort_changed(commands, bar_ent, &bar);
}

fn on_filter_chip_click(
    trigger: On<PointerClick>,
    mut commands: Commands,
    parents_query: Query<&ChildOf>,
    children_query: Query<&Children>,
    mut bar_q: Query<&mut LumaFilterSortBar>,
    mut chip_q: Query<(&LumaFilterChipItem, &mut UBorder)>,
) {
    if !is_primary_click(&trigger) {
        return;
    }
    if let Some(ent) = find_ancestor_or_self(trigger.entity, &chip_q, &parents_query) {
        activate_filter_chip(
            ent,
            &mut commands,
            &parents_query,
            &children_query,
            &mut bar_q,
            &mut chip_q,
        );
    }
}

fn on_filter_chip_focus(
    trigger: On<UFocusActivate>,
    mut commands: Commands,
    parents_query: Query<&ChildOf>,
    children_query: Query<&Children>,
    mut bar_q: Query<&mut LumaFilterSortBar>,
    mut chip_q: Query<(&LumaFilterChipItem, &mut UBorder)>,
) {
    if let Some(ent) = find_ancestor_or_self(trigger.entity, &chip_q, &parents_query) {
        activate_filter_chip(
            ent,
            &mut commands,
            &parents_query,
            &children_query,
            &mut bar_q,
            &mut chip_q,
        );
    }
}

fn on_filter_sort_btn_click(
    trigger: On<ButtonClicked>,
    mut commands: Commands,
    parents_query: Query<&ChildOf>,
    children_query: Query<&Children>,
    sort_btn_q: Query<(), With<LumaFilterSortBtn>>,
    view_btn_q: Query<(), With<LumaFilterViewBtn>>,
    mut bar_q: Query<&mut LumaFilterSortBar>,
    mut btn_query: Query<(&mut LumaButton, &mut UNode, &mut UBorder, &mut UInteractionColors)>,
    label_marker: Query<(), With<LumaButtonLabel>>,
    icon_marker: Query<(), With<LumaButtonIcon>>,
    btn_parts_query: Query<(), Or<(With<LumaButtonLabel>, With<LumaButtonIcon>)>>,
    mut text_query: Query<&mut UText>,
) {
    let btn = trigger.0;
    if let Some(sort_wrap) = find_ancestor(btn, &sort_btn_q, &parents_query) {
        let Some(bar_ent) = find_ancestor(sort_wrap, &bar_q, &parents_query) else {
            return;
        };
        let Ok(mut bar) = bar_q.get_mut(bar_ent) else {
            return;
        };
        bar.ascending = !bar.ascending;
        if bar.sort_label == "Newest First" {
            bar.sort_label = "Oldest First".into();
        } else if bar.sort_label == "Oldest First" {
            bar.sort_label = "Newest First".into();
        }
        let next_label = bar.sort_label.clone();
        visit_descendants(btn, &children_query, |e| {
            if label_marker.contains(e) {
                if let Ok(mut t) = text_query.get_mut(e) {
                    t.text = next_label.clone();
                }
                return DescendantVisit::Stop;
            }
            DescendantVisit::Continue
        });
        if let Ok((mut lb, mut node, mut border, mut ic)) = btn_query.get_mut(btn) {
            let variant = if bar.ascending {
                ButtonVariant::Secondary
            } else {
                ButtonVariant::Outline
            };
            apply_button_variant_visuals(
                variant,
                &mut lb,
                &mut node,
                &mut border,
                &mut ic,
                children_query.get(btn).ok(),
                &children_query,
                &mut text_query,
                &btn_parts_query,
            );
        }
        emit_filter_sort_changed(&mut commands, bar_ent, &bar);
        return;
    }

    if let Some(view_wrap) = find_ancestor(btn, &view_btn_q, &parents_query) {
        let Some(bar_ent) = find_ancestor(view_wrap, &bar_q, &parents_query) else {
            return;
        };
        let Ok(mut bar) = bar_q.get_mut(bar_ent) else {
            return;
        };
        bar.is_grid_view = !bar.is_grid_view;
        let next_icon = if bar.is_grid_view {
            Icon::FOLDER
        } else {
            Icon::MENU
        };
        visit_descendants(btn, &children_query, |e| {
            if icon_marker.contains(e) {
                if let Ok(mut t) = text_query.get_mut(e) {
                    t.text = next_icon.to_string();
                }
                return DescendantVisit::Stop;
            }
            DescendantVisit::Continue
        });
        emit_filter_sort_changed(&mut commands, bar_ent, &bar);
    }
}

// --- WorkspaceSwitcher & LocaleSelector Unified Popup Overlay ---

fn close_nav_dropdown_for_owner(
    owner: Entity,
    commands: &mut Commands,
    backdrop_q: &Query<(Entity, &LumaNavDropdownBackdrop)>,
) {
    for (bd_ent, bd) in backdrop_q.iter() {
        if bd.owner == owner {
            commands.entity(bd_ent).despawn();
        }
    }
}

fn spawn_nav_dropdown_overlay(
    commands: &mut Commands,
    owner: Entity,
    options: &[String],
    selected: &str,
    font: &Handle<Font>,
    windows: &Query<&Window>,
    cameras: &Query<(&Camera, &GlobalTransform)>,
    transforms: &Query<(&GlobalTransform, &ComputedSize)>,
) {
    let est_h = (options.len().max(1) as f32) * 34.0 + 16.0;
    let (menu_left, menu_top, menu_width) =
        compute_overlay_anchor(owner, 180.0, est_h, windows, cameras, transforms);

    let rows: Vec<_> = options
        .iter()
        .enumerate()
        .map(|(idx, val)| {
            let active = val == selected;
            let bg = if active {
                Color::srgba(0.35, 0.65, 1.0, 0.22)
            } else {
                Color::NONE
            };
            let ic = interaction_colors(
                bg,
                Color::srgba(1.0, 1.0, 1.0, 0.08),
                Color::srgba(1.0, 1.0, 1.0, 0.14),
                false,
            );
            let marker = LumaNavDropdownItem {
                owner,
                index: idx,
                value: val.clone(),
            };
            let f = font.clone();
            let text_col = if active {
                Color::srgb(0.96, 0.97, 0.99)
            } else {
                Color::srgb(0.80, 0.84, 0.92)
            };
            let val_c = val.clone();
            bsn! {
                marker
                UNode {
                    width: UVal::Percent(1.0),
                    padding: USides::axes(10.0, 7.0),
                    background_color: bg,
                    border_radius: UCornerRadius::all(5.0),
                }
                UInteraction::default()
                ic
                UFocusable::default()
                Children [
                    UText {
                        text: val_c,
                        font_size: 13.0,
                        color: text_col,
                        font: f,
                    }
                ]
            }
        })
        .collect();

    let rows_scene = SceneList::new(rows);
    let overlay_scene = bsn! {
        LumaNavDropdownBackdrop { owner }
        URootUi::screen()
        UNode {
            width: UVal::Percent(1.0),
            height: UVal::Percent(1.0),
            background_color: Color::NONE,
        }
        UInteraction::default()
        UZIndex::Global(9999)
        Children [
            UNode {
                width: { UVal::Px(menu_width) },
                padding: USides::all(6.0),
                background_color: Color::srgb(0.10, 0.12, 0.17),
                border_radius: UCornerRadius::all(8.0),
            }
            UBorder {
                color: Color::srgb(0.22, 0.26, 0.36),
                width: 1.0,
                radius: UCornerRadius::all(8.0),
            }
            USelf {
                position_type: UPositionType::Absolute,
                left: { UVal::Px(menu_left) },
                top: { UVal::Px(menu_top) },
                ..USelf::default()
            }
            ULayout {
                display: UDisplay::Flex,
                flex_direction: UFlexDirection::Column,
                gap: 2.0,
            }
            Children [
                {rows_scene}
            ]
        ]
    };
    commands.spawn_scene(overlay_scene);
}

fn find_nav_dropdown_font(
    owner: Entity,
    children_query: &Query<&Children>,
    ws_lbl_marker: &Query<(), With<LumaWorkspaceLabel>>,
    loc_lbl_marker: &Query<(), With<LumaLocaleLabel>>,
    text_q: &Query<&UText>,
) -> Handle<Font> {
    let mut font = Handle::default();
    visit_descendants(owner, children_query, |e| {
        if ws_lbl_marker.contains(e) || loc_lbl_marker.contains(e) {
            if let Ok(t) = text_q.get(e) {
                font = t.font.clone();
                return DescendantVisit::Stop;
            }
        }
        DescendantVisit::Continue
    });
    font
}

fn toggle_nav_dropdown(
    trigger_entity: Entity,
    commands: &mut Commands,
    parents_query: &Query<&ChildOf>,
    children_query: &Query<&Children>,
    backdrop_q: &Query<(Entity, &LumaNavDropdownBackdrop)>,
    windows: &Query<&Window>,
    cameras: &Query<(&Camera, &GlobalTransform)>,
    transforms: &Query<(&GlobalTransform, &ComputedSize)>,
    ws_q: &mut Query<&mut LumaWorkspaceSwitcher>,
    loc_q: &mut Query<&mut LumaLocaleSelector>,
    ws_lbl_marker: &Query<(), With<LumaWorkspaceLabel>>,
    loc_lbl_marker: &Query<(), With<LumaLocaleLabel>>,
    text_q: &Query<&UText>,
) {
    if let Some(ws_ent) = find_ancestor_or_self(trigger_entity, ws_q, parents_query) {
        if let Ok(mut ws) = ws_q.get_mut(ws_ent) {
            close_nav_dropdown_for_owner(ws_ent, commands, backdrop_q);
            ws.is_open = !ws.is_open;
            if ws.is_open {
                let font = find_nav_dropdown_font(
                    ws_ent,
                    children_query,
                    ws_lbl_marker,
                    loc_lbl_marker,
                    text_q,
                );
                spawn_nav_dropdown_overlay(
                    commands,
                    ws_ent,
                    &ws.options,
                    &ws.current_workspace,
                    &font,
                    windows,
                    cameras,
                    transforms,
                );
            }
        }
        return;
    }

    if let Some(loc_ent) = find_ancestor_or_self(trigger_entity, loc_q, parents_query) {
        if let Ok(mut loc) = loc_q.get_mut(loc_ent) {
            close_nav_dropdown_for_owner(loc_ent, commands, backdrop_q);
            loc.is_open = !loc.is_open;
            if loc.is_open {
                let font = find_nav_dropdown_font(
                    loc_ent,
                    children_query,
                    ws_lbl_marker,
                    loc_lbl_marker,
                    text_q,
                );
                spawn_nav_dropdown_overlay(
                    commands,
                    loc_ent,
                    &loc.options,
                    &loc.current_locale,
                    &font,
                    windows,
                    cameras,
                    transforms,
                );
            }
        }
    }
}

fn on_nav_dropdown_trigger_click(
    trigger: On<PointerClick>,
    mut commands: Commands,
    parents_query: Query<&ChildOf>,
    children_query: Query<&Children>,
    backdrop_q: Query<(Entity, &LumaNavDropdownBackdrop)>,
    windows: Query<&Window>,
    cameras: Query<(&Camera, &GlobalTransform)>,
    transforms: Query<(&GlobalTransform, &ComputedSize)>,
    mut ws_q: Query<&mut LumaWorkspaceSwitcher>,
    mut loc_q: Query<&mut LumaLocaleSelector>,
    ws_lbl_marker: Query<(), With<LumaWorkspaceLabel>>,
    loc_lbl_marker: Query<(), With<LumaLocaleLabel>>,
    text_q: Query<&UText>,
) {
    if !is_primary_click(&trigger) {
        return;
    }
    toggle_nav_dropdown(
        trigger.entity,
        &mut commands,
        &parents_query,
        &children_query,
        &backdrop_q,
        &windows,
        &cameras,
        &transforms,
        &mut ws_q,
        &mut loc_q,
        &ws_lbl_marker,
        &loc_lbl_marker,
        &text_q,
    );
}

fn on_nav_dropdown_trigger_focus(
    trigger: On<UFocusActivate>,
    mut commands: Commands,
    parents_query: Query<&ChildOf>,
    children_query: Query<&Children>,
    backdrop_q: Query<(Entity, &LumaNavDropdownBackdrop)>,
    windows: Query<&Window>,
    cameras: Query<(&Camera, &GlobalTransform)>,
    transforms: Query<(&GlobalTransform, &ComputedSize)>,
    mut ws_q: Query<&mut LumaWorkspaceSwitcher>,
    mut loc_q: Query<&mut LumaLocaleSelector>,
    ws_lbl_marker: Query<(), With<LumaWorkspaceLabel>>,
    loc_lbl_marker: Query<(), With<LumaLocaleLabel>>,
    text_q: Query<&UText>,
) {
    toggle_nav_dropdown(
        trigger.entity,
        &mut commands,
        &parents_query,
        &children_query,
        &backdrop_q,
        &windows,
        &cameras,
        &transforms,
        &mut ws_q,
        &mut loc_q,
        &ws_lbl_marker,
        &loc_lbl_marker,
        &text_q,
    );
}

fn on_nav_dropdown_item_click(
    trigger: On<PointerClick>,
    mut commands: Commands,
    parents_query: Query<&ChildOf>,
    children_query: Query<&Children>,
    item_q: Query<&LumaNavDropdownItem>,
    backdrop_q: Query<(Entity, &LumaNavDropdownBackdrop)>,
    mut ws_q: Query<&mut LumaWorkspaceSwitcher>,
    mut loc_q: Query<&mut LumaLocaleSelector>,
    ws_lbl_marker: Query<(), With<LumaWorkspaceLabel>>,
    loc_lbl_marker: Query<(), With<LumaLocaleLabel>>,
    avatar_marker: Query<(), With<LumaAvatar>>,
    mut text_q: Query<&mut UText>,
) {
    if !is_primary_click(&trigger) {
        return;
    }
    let Some(item_ent) = find_ancestor_or_self(trigger.entity, &item_q, &parents_query) else {
        return;
    };
    let Ok(item) = item_q.get(item_ent) else {
        return;
    };
    let owner = item.owner;
    let value = item.value.clone();

    if let Ok(mut ws) = ws_q.get_mut(owner) {
        ws.current_workspace = value.clone();
        ws.is_open = false;
        let initial = value.chars().next().unwrap_or('A').to_string();
        visit_descendants(owner, &children_query, |e| {
            if ws_lbl_marker.contains(e) {
                if let Ok(mut t) = text_q.get_mut(e) {
                    t.text = value.clone();
                }
            } else if avatar_marker.contains(e) {
                visit_descendants(e, &children_query, |c| {
                    if let Ok(mut t) = text_q.get_mut(c) {
                        t.text = initial.clone();
                        return DescendantVisit::Stop;
                    }
                    DescendantVisit::Continue
                });
                return DescendantVisit::SkipChildren;
            }
            DescendantVisit::Continue
        });
        commands.trigger(WorkspaceChanged {
            entity: owner,
            workspace: value,
        });
    } else if let Ok(mut loc) = loc_q.get_mut(owner) {
        loc.current_locale = value.clone();
        loc.is_open = false;
        visit_descendants(owner, &children_query, |e| {
            if loc_lbl_marker.contains(e) {
                if let Ok(mut t) = text_q.get_mut(e) {
                    t.text = value.clone();
                }
                return DescendantVisit::Stop;
            }
            DescendantVisit::Continue
        });
        commands.trigger(LocaleChanged {
            entity: owner,
            locale: value,
        });
    }

    close_nav_dropdown_for_owner(owner, &mut commands, &backdrop_q);
}

fn on_nav_dropdown_backdrop_click(
    trigger: On<PointerClick>,
    mut commands: Commands,
    backdrop_q: Query<(Entity, &LumaNavDropdownBackdrop)>,
    mut ws_q: Query<&mut LumaWorkspaceSwitcher>,
    mut loc_q: Query<&mut LumaLocaleSelector>,
) {
    if !is_primary_click(&trigger) {
        return;
    }
    let Ok((bd_ent, bd)) = backdrop_q.get(trigger.entity) else {
        return;
    };
    if let Ok(mut ws) = ws_q.get_mut(bd.owner) {
        ws.is_open = false;
    }
    if let Ok(mut loc) = loc_q.get_mut(bd.owner) {
        loc.is_open = false;
    }
    commands.entity(bd_ent).despawn();
}

fn nav_dropdown_cleanup_system(
    mut commands: Commands,
    keyboard: Option<Res<ButtonInput<KeyCode>>>,
    backdrop_q: Query<(Entity, &LumaNavDropdownBackdrop)>,
    mut ws_q: Query<&mut LumaWorkspaceSwitcher>,
    mut loc_q: Query<&mut LumaLocaleSelector>,
) {
    let esc_pressed = keyboard.is_some_and(|kb| kb.just_pressed(KeyCode::Escape));
    for (bd_ent, bd) in backdrop_q.iter() {
        let owner_exists = ws_q.contains(bd.owner) || loc_q.contains(bd.owner);
        if esc_pressed || !owner_exists {
            if let Ok(mut ws) = ws_q.get_mut(bd.owner) {
                ws.is_open = false;
            }
            if let Ok(mut loc) = loc_q.get_mut(bd.owner) {
                loc.is_open = false;
            }
            commands.entity(bd_ent).despawn();
        }
    }
}
