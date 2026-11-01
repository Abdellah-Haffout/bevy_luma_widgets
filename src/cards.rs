//! Specialized compound card components complementing the core primitive Card.

use bevy::prelude::*;
use bevy::scene::{ResolveContext, ResolvedScene, ResolveSceneError};
use bevy_luma::prelude::*;

use crate::navigation::SceneList;
use crate::prelude::*;
use crate::button::Button;

// ============================================================================
// 3.2 Product Card
// ============================================================================

/// Product showcase card with image, price, discount tag, rating, and CTA.
#[derive(Clone, Debug, Reflect)]
pub struct ProductCard {
    pub title: String,
    pub price: String,
    pub original_price: Option<String>,
    pub discount: Option<String>,
    pub rating: u8,
    pub cta_label: String,
    pub font: Handle<Font>,
    pub icon_font: Handle<Font>,
}

impl Default for ProductCard {
    fn default() -> Self {
        Self {
            title: "Wireless Noise-Canceling Headphones".into(),
            price: "$199.99".into(),
            original_price: Some("$249.99".into()),
            discount: Some("-20%".into()),
            rating: 5,
            cta_label: "Add to Cart".into(),
            font: Handle::default(),
            icon_font: Handle::default(),
        }
    }
}

impl ProductCard {
    pub fn new(title: impl Into<String>, price: impl Into<String>) -> Self {
        Self {
            title: title.into(),
            price: price.into(),
            ..default()
        }
    }

    pub fn original_price(mut self, original: impl Into<String>) -> Self {
        self.original_price = Some(original.into());
        self
    }

    pub fn discount(mut self, discount: impl Into<String>) -> Self {
        self.discount = Some(discount.into());
        self
    }

    pub fn rating(mut self, rating: u8) -> Self {
        self.rating = rating;
        self
    }

    pub fn cta(mut self, cta: impl Into<String>) -> Self {
        self.cta_label = cta.into();
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

impl Scene for ProductCard {
    fn resolve(
        self,
        context: &mut ResolveContext,
        scene: &mut ResolvedScene,
    ) -> Result<(), ResolveSceneError> {
        let title = self.title;
        let price = self.price;
        let rating = self.rating;
        let cta_label = self.cta_label;
        let font = self.font;
        let icon_font = self.icon_font;
        let rating_ifont = icon_font.clone();
        let title_font = font.clone();
        let price_font = font.clone();
        let btn_font = font.clone();
        let review_count = "(128)".to_string();

        let s = bsn! {
            UNode {
                width: UVal::Px(280.0),
                padding: USides::all(16.0),
                background_color: Color::srgb(0.10, 0.12, 0.17),
                border_radius: UCornerRadius::all(12.0),
            }
            UBorder {
                color: Color::srgb(0.18, 0.22, 0.30),
                width: 1.0,
                radius: UCornerRadius::all(12.0),
            }
            ULayout {
                display: UDisplay::Flex,
                flex_direction: UFlexDirection::Column,
                gap: 12.0,
            }
            Children [
                // Product preview media placeholder
                UNode {
                    width: UVal::Percent(1.0),
                    height: UVal::Px(160.0),
                    background_color: Color::srgba(1.0, 1.0, 1.0, 0.04),
                    border_radius: UCornerRadius::all(8.0),
                }
                ULayout {
                    display: UDisplay::Flex,
                    justify_content: UJustifyContent::Center,
                    align_items: UAlignItems::Center,
                }
                Children [
                    UText {
                        text: { Icon::SHOPPING_BAG.to_string() },
                        font_size: 36.0,
                        color: Color::srgb(0.40, 0.45, 0.55),
                        font: { icon_font.clone() },
                    }
                ]
                --
                // Title
                UText {
                    text: title,
                    font_size: 15.0,
                    color: Color::srgb(0.96, 0.97, 0.99),
                    font: title_font,
                }
                --
                // Rating row
                UNode::default()
                ULayout {
                    display: UDisplay::Flex,
                    flex_direction: UFlexDirection::Row,
                    align_items: UAlignItems::Center,
                    gap: 4.0,
                }
                Children [
                    @{Rating::new(rating).icon_font(rating_ifont)}
                    --
                    UText {
                        text: review_count,
                        font_size: 12.0,
                        color: Color::srgb(0.55, 0.60, 0.70),
                        font: { font.clone() },
                    }
                ]
                --
                // Price & Discount row
                UNode::default()
                ULayout {
                    display: UDisplay::Flex,
                    flex_direction: UFlexDirection::Row,
                    justify_content: UJustifyContent::SpaceBetween,
                    align_items: UAlignItems::Center,
                }
                Children [
                    UText {
                        text: price,
                        font_size: 18.0,
                        color: Color::srgb(0.35, 0.65, 1.0),
                        font: price_font,
                    }
                    --
                    @{Badge::success("20% OFF")}
                ]
                --
                // CTA Button
                @{Button::new(cta_label).size(ButtonSize::Medium).font(btn_font)}
            ]
        };
        s.resolve(context, scene)
    }
}

// ============================================================================
// 3.3 User Profile Card
// ============================================================================

/// Social user profile card with avatar, stats, bio, and follow action.
#[derive(Clone, Debug, Reflect)]
pub struct UserProfileCard {
    pub name: String,
    pub handle: String,
    pub bio: String,
    pub followers: String,
    pub following: String,
    pub font: Handle<Font>,
    pub icon_font: Handle<Font>,
}

impl Default for UserProfileCard {
    fn default() -> Self {
        Self {
            name: "Alex Vance".into(),
            handle: "@alexvance".into(),
            bio: "Game UI & Graphics Engineer building with Bevy Engine.".into(),
            followers: "14.2K".into(),
            following: "482".into(),
            font: Handle::default(),
            icon_font: Handle::default(),
        }
    }
}

impl UserProfileCard {
    pub fn new(name: impl Into<String>, handle: impl Into<String>) -> Self {
        Self {
            name: name.into(),
            handle: handle.into(),
            ..default()
        }
    }

    pub fn bio(mut self, bio: impl Into<String>) -> Self {
        self.bio = bio.into();
        self
    }

    pub fn followers(mut self, followers: impl Into<String>) -> Self {
        self.followers = followers.into();
        self
    }

    pub fn following(mut self, following: impl Into<String>) -> Self {
        self.following = following.into();
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

impl Scene for UserProfileCard {
    fn resolve(
        self,
        context: &mut ResolveContext,
        scene: &mut ResolvedScene,
    ) -> Result<(), ResolveSceneError> {
        let name = self.name;
        let handle = self.handle;
        let bio = self.bio;
        let followers = self.followers;
        let following = self.following;
        let font = self.font;
        let name_font = font.clone();
        let handle_font = font.clone();
        let bio_font = font.clone();
        let stat_font1 = font.clone();
        let stat_font2 = font.clone();
        let btn_font = font.clone();

        let s = bsn! {
            UNode {
                width: UVal::Px(300.0),
                padding: USides::all(20.0),
                background_color: Color::srgb(0.10, 0.12, 0.17),
                border_radius: UCornerRadius::all(12.0),
            }
            UBorder {
                color: Color::srgb(0.20, 0.24, 0.32),
                width: 1.0,
                radius: UCornerRadius::all(12.0),
            }
            ULayout {
                display: UDisplay::Flex,
                flex_direction: UFlexDirection::Column,
                align_items: UAlignItems::Center,
                gap: 12.0,
            }
            Children [
                @{Avatar::new("AV").size(AvatarSize::Large).font(font)}
                --
                UText {
                    text: name,
                    font_size: 16.0,
                    color: Color::srgb(0.96, 0.97, 0.99),
                    font: name_font,
                }
                --
                UText {
                    text: handle,
                    font_size: 13.0,
                    color: Color::srgb(0.35, 0.65, 1.0),
                    font: handle_font,
                }
                --
                UText {
                    text: bio,
                    font_size: 12.0,
                    color: Color::srgb(0.65, 0.70, 0.80),
                    font: bio_font,
                }
                --
                // Followers / Following row
                UNode {
                    width: UVal::Percent(1.0),
                    padding: USides::axes(0.0, 8.0),
                }
                ULayout {
                    display: UDisplay::Flex,
                    flex_direction: UFlexDirection::Row,
                    justify_content: UJustifyContent::SpaceAround,
                }
                Children [
                    UText {
                        text: format!("{followers} Followers"),
                        font_size: 12.0,
                        color: Color::srgb(0.85, 0.88, 0.95),
                        font: stat_font1,
                    }
                    --
                    UText {
                        text: format!("{following} Following"),
                        font_size: 12.0,
                        color: Color::srgb(0.85, 0.88, 0.95),
                        font: stat_font2,
                    }
                ]
                --
                @{Button::new("Follow").size(ButtonSize::Small).font(btn_font)}
            ]
        };
        s.resolve(context, scene)
    }
}

// ============================================================================
// 3.4 Pricing Card
// ============================================================================

/// Subscription plan pricing card with features list and highlight badge.
#[derive(Clone, Debug, Reflect)]
pub struct PricingCard {
    pub tier: String,
    pub price: String,
    pub period: String,
    pub features: Vec<String>,
    pub is_featured: bool,
    pub cta_label: String,
    pub font: Handle<Font>,
    pub icon_font: Handle<Font>,
}

impl Default for PricingCard {
    fn default() -> Self {
        Self {
            tier: "Pro Plan".into(),
            price: "$29".into(),
            period: "/month".into(),
            features: vec![
                "Unlimited UI Scenes".into(),
                "Custom Shader Nodes".into(),
                "24/7 Priority Support".into(),
                "Commercial License".into(),
            ],
            is_featured: true,
            cta_label: "Upgrade to Pro".into(),
            font: Handle::default(),
            icon_font: Handle::default(),
        }
    }
}

impl PricingCard {
    pub fn new(tier: impl Into<String>, price: impl Into<String>) -> Self {
        Self {
            tier: tier.into(),
            price: price.into(),
            ..default()
        }
    }

    pub fn period(mut self, period: impl Into<String>) -> Self {
        self.period = period.into();
        self
    }

    pub fn features(mut self, features: Vec<impl Into<String>>) -> Self {
        self.features = features.into_iter().map(|s| s.into()).collect();
        self
    }

    pub fn featured(mut self, featured: bool) -> Self {
        self.is_featured = featured;
        self
    }

    pub fn cta(mut self, cta: impl Into<String>) -> Self {
        self.cta_label = cta.into();
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

impl Scene for PricingCard {
    fn resolve(
        self,
        context: &mut ResolveContext,
        scene: &mut ResolvedScene,
    ) -> Result<(), ResolveSceneError> {
        let border_color = if self.is_featured {
            Color::srgb(0.35, 0.65, 1.0)
        } else {
            Color::srgb(0.20, 0.24, 0.32)
        };
        let font = self.font;
        let icon_font = self.icon_font;
        let tier = self.tier;
        let price = self.price;
        let period = self.period;
        let cta_label = self.cta_label;

        let feature_rows: Vec<_> = self.features.into_iter().map(|feat| {
            let f = font.clone();
            let ifont = icon_font.clone();
            bsn! {
                UNode::default()
                ULayout {
                    display: UDisplay::Flex,
                    flex_direction: UFlexDirection::Row,
                    align_items: UAlignItems::Center,
                    gap: 8.0,
                }
                Children [
                    UText {
                        text: { Icon::CHECK.to_string() },
                        font_size: 13.0,
                        color: Color::srgb(0.18, 0.80, 0.44),
                        font: ifont,
                    }
                    --
                    UText {
                        text: feat,
                        font_size: 13.0,
                        color: Color::srgb(0.85, 0.88, 0.95),
                        font: f,
                    }
                ]
            }
        }).collect();

        let feature_rows_scene = SceneList::new(feature_rows);
        let tier_font = font.clone();
        let price_font = font.clone();
        let period_font = font.clone();
        let btn_font = font.clone();

        let s = bsn! {
            UNode {
                width: UVal::Px(280.0),
                padding: USides::all(24.0),
                background_color: Color::srgb(0.09, 0.11, 0.16),
                border_radius: UCornerRadius::all(12.0),
            }
            UBorder {
                color: border_color,
                width: 1.5,
                radius: UCornerRadius::all(12.0),
            }
            ULayout {
                display: UDisplay::Flex,
                flex_direction: UFlexDirection::Column,
                gap: 16.0,
            }
            Children [
                UText {
                    text: tier,
                    font_size: 16.0,
                    color: Color::srgb(0.96, 0.97, 0.99),
                    font: tier_font,
                }
                --
                UNode::default()
                ULayout {
                    display: UDisplay::Flex,
                    flex_direction: UFlexDirection::Row,
                    align_items: UAlignItems::End,
                    gap: 4.0,
                }
                Children [
                    UText {
                        text: price,
                        font_size: 28.0,
                        color: Color::srgb(0.96, 0.97, 0.99),
                        font: price_font,
                    }
                    --
                    UText {
                        text: period,
                        font_size: 13.0,
                        color: Color::srgb(0.55, 0.60, 0.70),
                        font: period_font,
                    }
                ]
                --
                @{Divider::horizontal()}
                --
                UNode::default()
                ULayout {
                    display: UDisplay::Flex,
                    flex_direction: UFlexDirection::Column,
                    gap: 8.0,
                }
                Children [
                    {feature_rows_scene}
                ]
                --
                @{Button::new(cta_label).size(ButtonSize::Medium).font(btn_font)}
            ]
        };
        s.resolve(context, scene)
    }
}

// ============================================================================
// 3.5 Article / Blog Card
// ============================================================================

/// Article summary card with cover preview, category badge, and read time.
#[derive(Clone, Debug, Reflect)]
pub struct ArticleCard {
    pub title: String,
    pub excerpt: String,
    pub category: String,
    pub date: String,
    pub read_time: String,
    pub font: Handle<Font>,
    pub icon_font: Handle<Font>,
}

impl Default for ArticleCard {
    fn default() -> Self {
        Self {
            title: "GPU UI Rendering with Signed Distance Fields".into(),
            excerpt: "Learn how SDF primitives unlock infinite scaling, crisp corners, and ultra-smooth shadows.".into(),
            category: "Graphics".into(),
            date: "Oct 15, 2026".into(),
            read_time: "5 min read".into(),
            font: Handle::default(),
            icon_font: Handle::default(),
        }
    }
}

impl ArticleCard {
    pub fn new(title: impl Into<String>, excerpt: impl Into<String>) -> Self {
        Self {
            title: title.into(),
            excerpt: excerpt.into(),
            ..default()
        }
    }

    pub fn category(mut self, cat: impl Into<String>) -> Self {
        self.category = cat.into();
        self
    }

    pub fn date(mut self, date: impl Into<String>) -> Self {
        self.date = date.into();
        self
    }

    pub fn read_time(mut self, rt: impl Into<String>) -> Self {
        self.read_time = rt.into();
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

impl Scene for ArticleCard {
    fn resolve(
        self,
        context: &mut ResolveContext,
        scene: &mut ResolvedScene,
    ) -> Result<(), ResolveSceneError> {
        let title = self.title;
        let excerpt = self.excerpt;
        let category = self.category;
        let date_str = format!("{} • {}", self.date, self.read_time);
        let font = self.font;
        let title_font = font.clone();
        let excerpt_font = font.clone();
        let meta_font = font.clone();

        let s = bsn! {
            UNode {
                width: UVal::Px(320.0),
                padding: USides::all(16.0),
                background_color: Color::srgb(0.10, 0.12, 0.17),
                border_radius: UCornerRadius::all(10.0),
            }
            UBorder {
                color: Color::srgb(0.20, 0.24, 0.32),
                width: 1.0,
                radius: UCornerRadius::all(10.0),
            }
            ULayout {
                display: UDisplay::Flex,
                flex_direction: UFlexDirection::Column,
                gap: 10.0,
            }
            Children [
                @{Badge::info(category)}
                --
                UText {
                    text: title,
                    font_size: 15.0,
                    color: Color::srgb(0.96, 0.97, 0.99),
                    font: title_font,
                }
                --
                UText {
                    text: excerpt,
                    font_size: 12.0,
                    color: Color::srgb(0.65, 0.70, 0.80),
                    font: excerpt_font,
                }
                --
                UText {
                    text: date_str,
                    font_size: 11.0,
                    color: Color::srgb(0.45, 0.50, 0.60),
                    font: meta_font,
                }
            ]
        };
        s.resolve(context, scene)
    }
}

// ============================================================================
// 3.6 Testimonial Card
// ============================================================================

/// Customer feedback card with quote, rating stars, and author initials.
#[derive(Clone, Debug, Reflect)]
pub struct TestimonialCard {
    pub quote: String,
    pub author: String,
    pub company: String,
    pub rating: u8,
    pub font: Handle<Font>,
    pub icon_font: Handle<Font>,
}

impl Default for TestimonialCard {
    fn default() -> Self {
        Self {
            quote: "\"Bevy Luma Widgets transformed our game UI workflow completely!\"".into(),
            author: "Sarah Chen".into(),
            company: "Lead UI Artist, IndieCraft".into(),
            rating: 5,
            font: Handle::default(),
            icon_font: Handle::default(),
        }
    }
}

impl TestimonialCard {
    pub fn new(quote: impl Into<String>, author: impl Into<String>) -> Self {
        Self {
            quote: quote.into(),
            author: author.into(),
            ..default()
        }
    }

    pub fn company(mut self, comp: impl Into<String>) -> Self {
        self.company = comp.into();
        self
    }

    pub fn rating(mut self, rating: u8) -> Self {
        self.rating = rating;
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

impl Scene for TestimonialCard {
    fn resolve(
        self,
        context: &mut ResolveContext,
        scene: &mut ResolvedScene,
    ) -> Result<(), ResolveSceneError> {
        let quote = self.quote;
        let author = self.author;
        let company = self.company;
        let rating = self.rating;
        let icon_font = self.icon_font;
        let font = self.font;
        let quote_font = font.clone();
        let author_font = font.clone();
        let company_font = font.clone();
        let initial = author.chars().next().unwrap_or('U').to_string();

        let s = bsn! {
            UNode {
                width: UVal::Px(300.0),
                padding: USides::all(20.0),
                background_color: Color::srgb(0.09, 0.11, 0.15),
                border_radius: UCornerRadius::all(10.0),
            }
            UBorder {
                color: Color::srgb(0.18, 0.22, 0.30),
                width: 1.0,
                radius: UCornerRadius::all(10.0),
            }
            ULayout {
                display: UDisplay::Flex,
                flex_direction: UFlexDirection::Column,
                gap: 12.0,
            }
            Children [
                @{Rating::new(rating).icon_font(icon_font)}
                --
                UText {
                    text: quote,
                    font_size: 13.0,
                    color: Color::srgb(0.85, 0.88, 0.95),
                    font: quote_font,
                }
                --
                UNode::default()
                ULayout {
                    display: UDisplay::Flex,
                    flex_direction: UFlexDirection::Row,
                    align_items: UAlignItems::Center,
                    gap: 10.0,
                }
                Children [
                    @{Avatar::new(initial).size(AvatarSize::Small).font(font.clone())}
                    --
                    UNode::default()
                    ULayout {
                        display: UDisplay::Flex,
                        flex_direction: UFlexDirection::Column,
                        gap: 2.0,
                    }
                    Children [
                        UText {
                            text: author,
                            font_size: 13.0,
                            color: Color::srgb(0.96, 0.97, 0.99),
                            font: author_font,
                        }
                        --
                        UText {
                            text: company,
                            font_size: 11.0,
                            color: Color::srgb(0.55, 0.60, 0.70),
                            font: company_font,
                        }
                    ]
                ]
            ]
        };
        s.resolve(context, scene)
    }
}

// ============================================================================
// 3.17 Social Feed Post
// ============================================================================

/// Social timeline card with author, text content, and like/comment buttons.
#[derive(Clone, Debug, Reflect)]
pub struct SocialFeedPost {
    pub author: String,
    pub handle: String,
    pub content: String,
    pub time_ago: String,
    pub likes: usize,
    pub comments: usize,
    pub font: Handle<Font>,
    pub icon_font: Handle<Font>,
}

impl Default for SocialFeedPost {
    fn default() -> Self {
        Self {
            author: "Jordan Lee".into(),
            handle: "@jordanlee".into(),
            content: "Just shipped 100 fully declarative UI components for Bevy 0.19! Loving the ergonomics of BSN and SDF.".into(),
            time_ago: "2h ago".into(),
            likes: 42,
            comments: 7,
            font: Handle::default(),
            icon_font: Handle::default(),
        }
    }
}

impl SocialFeedPost {
    pub fn new(author: impl Into<String>, content: impl Into<String>) -> Self {
        Self {
            author: author.into(),
            content: content.into(),
            ..default()
        }
    }

    pub fn handle(mut self, handle: impl Into<String>) -> Self {
        self.handle = handle.into();
        self
    }

    pub fn time_ago(mut self, time: impl Into<String>) -> Self {
        self.time_ago = time.into();
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

impl Scene for SocialFeedPost {
    fn resolve(
        self,
        context: &mut ResolveContext,
        scene: &mut ResolvedScene,
    ) -> Result<(), ResolveSceneError> {
        let author = self.author;
        let handle = self.handle;
        let content = self.content;
        let time_ago = self.time_ago;
        let font = self.font;
        let icon_font = self.icon_font;
        let author_font = font.clone();
        let handle_font = font.clone();
        let time_font = font.clone();
        let content_font = font.clone();
        let initial = author.chars().next().unwrap_or('J').to_string();
        let likes_str = format!("{} Likes", self.likes);
        let comments_str = format!("{} Comments", self.comments);

        let s = bsn! {
            UNode {
                width: UVal::Px(360.0),
                padding: USides::all(16.0),
                background_color: Color::srgb(0.09, 0.10, 0.15),
                border_radius: UCornerRadius::all(10.0),
            }
            UBorder {
                color: Color::srgb(0.18, 0.22, 0.30),
                width: 1.0,
                radius: UCornerRadius::all(10.0),
            }
            ULayout {
                display: UDisplay::Flex,
                flex_direction: UFlexDirection::Column,
                gap: 12.0,
            }
            Children [
                // Header row
                UNode::default()
                ULayout {
                    display: UDisplay::Flex,
                    flex_direction: UFlexDirection::Row,
                    align_items: UAlignItems::Center,
                    gap: 10.0,
                }
                Children [
                    @{Avatar::new(initial).size(AvatarSize::Small).font(font.clone())}
                    --
                    UNode::default()
                    ULayout {
                        display: UDisplay::Flex,
                        flex_direction: UFlexDirection::Column,
                        gap: 2.0,
                    }
                    Children [
                        UText {
                            text: author,
                            font_size: 14.0,
                            color: Color::srgb(0.96, 0.97, 0.99),
                            font: author_font,
                        }
                        --
                        UText {
                            text: handle,
                            font_size: 12.0,
                            color: Color::srgb(0.50, 0.55, 0.65),
                            font: handle_font,
                        }
                    ]
                    --
                    UText {
                        text: time_ago,
                        font_size: 11.0,
                        color: Color::srgb(0.40, 0.45, 0.55),
                        font: time_font,
                    }
                ]
                --
                // Content
                UText {
                    text: content,
                    font_size: 13.0,
                    color: Color::srgb(0.85, 0.88, 0.95),
                    font: content_font,
                }
                --
                @{Divider::horizontal()}
                --
                // Action counts
                UNode::default()
                ULayout {
                    display: UDisplay::Flex,
                    flex_direction: UFlexDirection::Row,
                    gap: 16.0,
                }
                Children [
                    @{Button::new(likes_str).ghost().size(ButtonSize::Small).icon(Icon::STAR).font(font.clone()).icon_font(icon_font.clone())}
                    --
                    @{Button::new(comments_str).ghost().size(ButtonSize::Small).font(font).icon_font(icon_font)}
                ]
            ]
        };
        s.resolve(context, scene)
    }
}

// ============================================================================
// 3.24 Code Block
// ============================================================================

/// Code syntax container with language tag and copy button.
#[derive(Clone, Debug, Reflect)]
pub struct CodeBlock {
    pub language: String,
    pub code: String,
    pub font: Handle<Font>,
    pub icon_font: Handle<Font>,
}

impl Default for CodeBlock {
    fn default() -> Self {
        Self {
            language: "rust".into(),
            code: "app.add_plugins(LumaWidgetsPlugin);".into(),
            font: Handle::default(),
            icon_font: Handle::default(),
        }
    }
}

impl CodeBlock {
    pub fn new(code: impl Into<String>) -> Self {
        Self {
            code: code.into(),
            ..default()
        }
    }

    pub fn language(mut self, lang: impl Into<String>) -> Self {
        self.language = lang.into();
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

impl Scene for CodeBlock {
    fn resolve(
        self,
        context: &mut ResolveContext,
        scene: &mut ResolvedScene,
    ) -> Result<(), ResolveSceneError> {
        let lang = self.language;
        let code = self.code;
        let font = self.font;
        let icon_font = self.icon_font;
        let lang_font = font.clone();
        let code_font = font.clone();

        let s = bsn! {
            UNode {
                width: UVal::Percent(1.0),
                padding: USides::all(14.0),
                background_color: Color::srgb(0.06, 0.07, 0.10),
                border_radius: UCornerRadius::all(8.0),
            }
            UBorder {
                color: Color::srgb(0.18, 0.22, 0.30),
                width: 1.0,
                radius: UCornerRadius::all(8.0),
            }
            ULayout {
                display: UDisplay::Flex,
                flex_direction: UFlexDirection::Column,
                gap: 8.0,
            }
            Children [
                // Top header bar
                UNode::default()
                ULayout {
                    display: UDisplay::Flex,
                    flex_direction: UFlexDirection::Row,
                    justify_content: UJustifyContent::SpaceBetween,
                    align_items: UAlignItems::Center,
                }
                Children [
                    UText {
                        text: lang,
                        font_size: 11.0,
                        color: Color::srgb(0.55, 0.60, 0.70),
                        font: lang_font,
                    }
                    --
                    @{Button::icon_button(Icon::COPY).ghost().size(ButtonSize::Small).icon_font(icon_font)}
                ]
                --
                // Code text
                UText {
                    text: code,
                    font_size: 13.0,
                    color: Color::srgb(0.70, 0.85, 0.95),
                    font: code_font,
                }
            ]
        };
        s.resolve(context, scene)
    }
}
