use crate::{
    asset::{
        asset::AssetTrait,
        font::font::{FontAsset, FontName},
    },
    score::{component::current_score_text::CurrentScoreText, resource::score::Score},
};
use ::bevy::prelude::*;

/**
 * 現在のスコアのテキスト
 */
fn current_score_text(children_builder: &mut ChildBuilder, asset_server: &Res<AssetServer>) {
    children_builder
        .spawn(NodeBundle {
            style: Style {
                flex_direction: FlexDirection::Row,
                justify_content: JustifyContent::Center,
                align_items: AlignItems::Center,
                ..default()
            },
            ..default()
        })
        .with_children(|parent| {
            parent.spawn((
                CurrentScoreText,
                TextBundle::from_sections([TextSection::new(
                    "",
                    TextStyle {
                        font: FontAsset::asset(&asset_server, &FontName::HachiMaruPopReg),
                        font_size: 50.,
                        color: Color::BLACK,
                        ..default()
                    },
                )]),
            ));
        });
}

/**
 * スコアのタイトルテキスト
 */
fn score_title_text(children_builder: &mut ChildBuilder, asset_server: &Res<AssetServer>) {
    children_builder
        .spawn(NodeBundle {
            style: Style { ..default() },
            ..default()
        })
        .with_children(|parent| {
            parent.spawn((TextBundle::from_sections([TextSection::new(
                "SCORE",
                TextStyle {
                    font: FontAsset::asset(&asset_server, &FontName::HachiMaruPopReg),
                    font_size: 50.,
                    color: Color::BLACK,
                    ..default()
                },
            )]),));
        });
}

pub fn setup_score(mut commands: Commands, asset_server: Res<AssetServer>) {
    commands
        .spawn((NodeBundle {
            style: Style {
                position_type: PositionType::Absolute,
                left: Val::Px(50.),
                top: Val::Px(50.),
                flex_direction: FlexDirection::Column,

                ..default()
            },
            ..default()
        },))
        .with_children(|parent| {
            score_title_text(parent, &asset_server);
        })
        .with_children(|parent| {
            current_score_text(parent, &asset_server);
        });
}

pub fn update_current_score_text(
    puzzle_score_res: Res<Score>,
    mut query: Query<&mut Text, With<CurrentScoreText>>,
) {
    let mut text = query.single_mut();
    text.sections[0].value = puzzle_score_res.0.to_string();
}
