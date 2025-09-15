use bevy::{
    ecs::{
        component::Component,
        entity::Entity,
        system::{Commands, Res},
    },
    prelude::*,
};

use crate::asset::font::font_assets::FontAssets;

#[derive(Component)]
pub struct ScoreValueText;
impl ScoreValueText {
    fn value() -> String {
        "".to_string()
    }

    fn spawn(font_assets: &Res<FontAssets>) -> (Text, TextFont, TextColor, Self) {
        return (
            Text::new(Self::value()),
            TextFont {
                font: font_assets.hachi_maru_pop_regular.clone(),
                font_size: 50.,
                ..default()
            },
            TextColor(Color::WHITE),
            Self,
        );
    }
}

pub struct ScoreValueTextContainer;
impl ScoreValueTextContainer {
    fn style() -> Node {
        Node {
            margin: UiRect {
                left: Val::Px(0.0),
                right: Val::Px(0.0),
                top: Val::Px(10.0),
                bottom: Val::Px(0.0),
            },
            flex_direction: FlexDirection::Row,
            justify_content: JustifyContent::Center,
            align_items: AlignItems::Center,

            ..default()
        }
    }

    pub fn spawn_as_child(
        commands: &mut Commands,
        parent_entity: Entity,
        font_assets: &Res<FontAssets>,
    ) {
        commands.entity(parent_entity).with_children(|parent| {
            parent
                .spawn((Node { ..Self::style() },))
                .with_children(|parent| {
                    // スコアのタイトルテキストを追加
                    let (text, font, color, component) = ScoreValueText::spawn(font_assets);
                    parent.spawn((text, font, color, component));
                });
        });
    }
}
