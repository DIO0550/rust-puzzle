use crate::{
    asset::{
        asset::AssetTrait,
        font::font::{FontAsset, FontName},
        image::image::ImageName,
    },
    game::ui::image_ui::{ImageUI, ImageUITrait},
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
                        color: Color::WHITE,
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
            style: Style {
                margin: UiRect {
                    left: Val::Px(0.0),
                    right: Val::Px(0.0),
                    top: Val::Px(75.0),
                    bottom: Val::Px(0.0),
                },
                ..default()
            },
            ..default()
        })
        .with_children(|parent| {
            parent.spawn((TextBundle::from_sections([TextSection::new(
                "SCORE",
                TextStyle {
                    font: FontAsset::asset(&asset_server, &FontName::HachiMaruPopReg),
                    font_size: 50.,
                    color: Color::WHITE,
                    ..default()
                },
            )]),));
        });
}

pub fn setup_score(mut commands: Commands, asset_server: Res<AssetServer>) {
    let image_size = 250.0;
    let mut piece_image_bundle =
        ImageUI::image_bundle(ImageName::CatHand, &asset_server, &image_size);
    piece_image_bundle.style = Style {
        position_type: PositionType::Absolute,
        left: Val::Px(50.),
        top: Val::Px(50.),
        height: Val::Px(image_size),
        width: Val::Px(image_size),
        flex_direction: FlexDirection::Column,
        align_items: AlignItems::Center,
        ..piece_image_bundle.style
    };
    // ImageUi::new(next_piece_res.0).image_bundle(&asset_server, &PIECE_IMAGE_SIZE);
    commands
        .spawn(piece_image_bundle)
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
