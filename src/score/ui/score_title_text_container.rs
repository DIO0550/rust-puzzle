use bevy::{
    ecs::{
        entity::Entity,
        system::{Commands, Res},
    },
    hierarchy::BuildChildren,
    render::color::Color,
    text::TextStyle,
    ui::{
        node_bundles::{NodeBundle, TextBundle},
        Style, UiRect, Val,
    },
    utils::default,
};

use crate::asset::font::font_assets::FontAssets;

struct ScoreTitleText;

impl ScoreTitleText {
    fn text_style(font_assets: &Res<FontAssets>) -> TextStyle {
        TextStyle {
            font: font_assets.hachi_maru_pop_regular.clone(),
            font_size: 50.,
            color: Color::WHITE,
            ..default()
        }
    }

    fn value() -> String {
        "Score".to_string()
    }

    fn spawn(font_assets: &Res<FontAssets>) -> TextBundle {
        TextBundle::from_section(Self::value(), Self::text_style(font_assets))
    }
}

pub struct ScoreTitleTextContainer;

impl ScoreTitleTextContainer {
    fn style() -> Style {
        Style {
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
                .spawn((NodeBundle {
                    style: Self::style(),
                    ..default()
                },))
                .with_children(|parent| {
                    // スコアのタイトルテキストを追加
                    parent.spawn(ScoreTitleText::spawn(font_assets));
                });
        });
    }
}
