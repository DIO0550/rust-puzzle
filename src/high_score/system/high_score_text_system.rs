use crate::{
    asset::{
        asset::AssetTrait,
        font::font::{FontAsset, FontName},
        image::image::ImageName,
    },
    game::ui::image_ui::{ImageUI, ImageUITrait},
    high_score::{
        component::high_score_text::{HighScoreTextDate, HighScoreTextScore},
        resource::{high_score::HighScore, high_scores::HighScores},
    },
};
use ::bevy::prelude::*;

fn high_score_text(
    children_builder: &mut ChildBuilder,
    asset_server: &Res<AssetServer>,
    high_score: &HighScore,
) {
    println!("{}", high_score.score);
    children_builder
        .spawn(NodeBundle {
            style: Style {
                margin: UiRect {
                    left: Val::Px(0.0),
                    right: Val::Px(0.0),
                    top: Val::Px(10.0),
                    bottom: Val::Px(0.0),
                },
                flex_direction: FlexDirection::Column,
                justify_content: JustifyContent::Center,
                align_items: AlignItems::Center,
                ..default()
            },
            ..default()
        })
        .with_children(|parent| {
            parent.spawn((
                HighScoreTextScore,
                TextBundle::from_sections([TextSection::new(
                    &high_score.score.to_string(),
                    TextStyle {
                        font: FontAsset::asset(&asset_server, &FontName::HachiMaruPopReg),
                        font_size: 25.,
                        color: Color::BLACK,
                        ..default()
                    },
                )]),
            ));

            parent.spawn((
                HighScoreTextDate,
                TextBundle::from_sections([TextSection::new(
                    &high_score.date,
                    TextStyle {
                        font: FontAsset::asset(&asset_server, &FontName::HachiMaruPopReg),
                        font_size: 25.,
                        color: Color::BLACK,
                        ..default()
                    },
                )]),
            ));
        });
}

pub fn setup_high_score_text(
    mut commands: Commands,
    high_scores_res: Res<HighScores>,
    asset_server: Res<AssetServer>,
) {
    if !high_scores_res.is_changed() {
        return;
    }

    let image_size = 350.0;
    let mut piece_image_bundle =
        ImageUI::image_bundle(ImageName::CatHand, &asset_server, &image_size);
    piece_image_bundle.style = Style {
        position_type: PositionType::Absolute,
        left: Val::Px(50.),
        bottom: Val::Px(50.),
        height: Val::Px(image_size),
        width: Val::Px(image_size),
        flex_direction: FlexDirection::Column,
        align_items: AlignItems::Center,
        ..piece_image_bundle.style
    };

    commands.spawn(piece_image_bundle).with_children(|parent| {
        high_scores_res
            .0
            .iter()
            .for_each(|high_score| high_score_text(parent, &asset_server, &high_score));
    });
}
