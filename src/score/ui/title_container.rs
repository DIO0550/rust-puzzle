use bevy::{
    ecs::{
        entity::Entity,
        system::{Commands, Res},
    },
    prelude::*,
};

use crate::asset::font::font_assets::FontAssets;

struct ScoreTitleText;

impl ScoreTitleText {
    fn value() -> String {
        "Score".to_string()
    }

    fn spawn(font_assets: &Res<FontAssets>) -> (Text, TextFont, TextColor) {
        (
            Text::new(Self::value()),
            TextFont {
                font: font_assets.hachi_maru_pop_regular.clone(),
                font_size: 50.,
                ..default()
            },
            TextColor(Color::WHITE),
        )
    }
}

pub struct ScoreTitleTextContainer;

impl ScoreTitleTextContainer {
    fn style() -> Node {
        Node {
            margin: UiRect {
                left: Val::Px(0.0),
                right: Val::Px(0.0),
                top: Val::Px(75.0),
                bottom: Val::Px(0.0),
            },
            ..default()
        }
    }

    pub fn spawn_as_child(
        commands: &mut Commands,
        parent_entity: Entity,
        font_assets: &Res<FontAssets>,
    ) {
        commands.entity(parent_entity).with_children(|parent| {
            // スコアのタイトルテキストを追加
            parent
                .spawn((Node {
                    ..Self::style()
                },))
                .with_children(|parent| {
                    // スコアのタイトルテキストを追加
                    let (text, font, color) = ScoreTitleText::spawn(font_assets);
                    parent.spawn((text, font, color));
                });
        });
    }
}
