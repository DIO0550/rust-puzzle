use crate::asset::font::font_assets::FontAssets;
use crate::consts::color_theme::ColorTheme;
use crate::score::high_score::ui::item_style::ItemStyle;
use bevy::color::Color;
use bevy::prelude::*;
use bevy::ui::{AlignItems, BorderRadius, Display, JustifyContent, Node, Val};

pub enum HighScoreItemRank {
    First,
    Second,
    Third,
    Other,
}

impl HighScoreItemRank {
    pub fn to_str(&self) -> &str {
        match self {
            HighScoreItemRank::First => "1st",
            HighScoreItemRank::Second => "2nd",
            HighScoreItemRank::Third => "3rd",
            HighScoreItemRank::Other => "Other",
        }
    }

    pub fn to_bg_color(&self) -> BackgroundColor {
        match self {
            HighScoreItemRank::First => BackgroundColor(ColorTheme::GOLD),
            HighScoreItemRank::Second => BackgroundColor(ColorTheme::SILVER),
            HighScoreItemRank::Third => BackgroundColor(ColorTheme::BRONZE),
            HighScoreItemRank::Other => BackgroundColor(ColorTheme::GRAY),
        }
    }

    pub fn to_border_color(&self) -> BorderColor {
        match self {
            HighScoreItemRank::First => BorderColor(ColorTheme::ORANGE),
            HighScoreItemRank::Second => BorderColor(ColorTheme::DARK_GRAY),
            HighScoreItemRank::Third => BorderColor(ColorTheme::BROWN),
            HighScoreItemRank::Other => BorderColor(ColorTheme::DARK_GRAY),
        }
    }

    pub fn to_text_color(&self) -> TextColor {
        match self {
            HighScoreItemRank::First => TextColor(Color::BLACK),
            HighScoreItemRank::Second => TextColor(Color::BLACK),
            HighScoreItemRank::Third => TextColor(Color::BLACK),
            HighScoreItemRank::Other => TextColor(Color::WHITE),
        }
    }
}

pub struct HighScoreItemBuilder {
    pub rank: HighScoreItemRank,
}

impl HighScoreItemBuilder {
    pub fn new(rank: HighScoreItemRank) -> Self {
        Self { rank }
    }

    fn node(&self) -> Node {
        Node {
            width: Val::Percent(90.0),
            height: Val::Px(50.0),
            padding: ItemStyle::PADDING,
            border: UiRect::all(Val::Px(ItemStyle::BORDER_WIDTH)),
            ..default()
        }
    }

    pub fn build(self, commands: &mut Commands, font_assets: &Res<FontAssets>) -> Entity {
        let wrapper = Node {
            width: Val::Percent(100.0),
            height: Val::Percent(100.0),
            display: Display::Flex,
            justify_content: JustifyContent::Center,
            align_items: AlignItems::Center,
            column_gap: ItemStyle::SPACING,
            ..default()
        };

        let bg_color = self.rank.to_bg_color();
        let border_color = self.rank.to_border_color();
        let text_color = self.rank.to_text_color();
        let mut entity = Entity::PLACEHOLDER;

        let text = self.rank.to_str();
        let text_font = TextFont {
            font: font_assets.hachi_maru_pop_regular.clone(),
            font_size: 40.0,
            ..default()
        };

        commands.spawn(wrapper).with_children(|parent| {
            entity = parent
                .spawn((self.node(), border_color, bg_color))
                .with_children(|child| {
                    child.spawn((Text::new(text), text_font, text_color));
                })
                .id();
        });

        entity
    }
}

pub struct HighScoreItem;
