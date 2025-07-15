use bevy::{
    ecs::{
        component::Component,
        entity::Entity,
        system::{Commands, Res},
    },
    hierarchy::BuildChildren,
    render::color::Color,
    text::TextStyle,
    ui::{
        node_bundles::{NodeBundle, TextBundle},
        AlignItems, FlexDirection, JustifyContent, Style, UiRect, Val,
    },
    utils::default,
};

use crate::asset::font::font_assets::FontAssets;

#[derive(Component)]
pub struct ScoreValueText;
impl ScoreValueText {
    fn text_style(font_assets: &Res<FontAssets>) -> TextStyle {
        TextStyle {
            font: font_assets.hachi_maru_pop_regular.clone(),
            font_size: 50.,
            color: Color::WHITE,
            ..default()
        }
    }

    fn value() -> String {
        "".to_string()
    }

    fn spawn(font_assets: &Res<FontAssets>) -> (TextBundle, Self) {
        return (
            TextBundle::from_section(Self::value(), Self::text_style(font_assets)),
            Self,
        );
    }
}

pub struct ScoreValueTextContainer;
impl ScoreValueTextContainer {
    fn style() -> Style {
        Style {
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
                .spawn((NodeBundle {
                    style: Self::style(),
                    ..default()
                },))
                .with_children(|parent| {
                    // スコアのタイトルテキストを追加
                    parent.spawn(ScoreValueText::spawn(font_assets));
                });
        });
    }
}
