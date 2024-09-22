use crate::{
    asset::{
        asset::AssetTrait,
        font::font::{FontAsset, FontName},
        image::image::ImageName,
    },
    game::ui::image_ui::{ImageSize, ImageUI, ImageUITrait},
    high_score::{
        component::high_score_text::{HighScoreText, HighScoreTextDate, HighScoreTextScore},
        resource::high_scores::{HighScores, MAX_HIGH_SCORE_COUNT},
    },
};
use ::bevy::prelude::*;

const DEFAULT_SCORE_TEXT: &str = "-";

fn high_score_text_score(children_builder: &mut ChildBuilder, asset_server: &Res<AssetServer>) {
    children_builder.spawn((
        HighScoreTextScore,
        TextBundle::from_sections([TextSection::new(
            DEFAULT_SCORE_TEXT,
            TextStyle {
                font: FontAsset::asset(&asset_server, &FontName::HachiMaruPopReg),
                font_size: 25.,
                color: Color::WHITE,
                ..default()
            },
        )]),
    ));
}

fn high_score_text_date(children_builder: &mut ChildBuilder, asset_server: &Res<AssetServer>) {
    children_builder.spawn((
        HighScoreTextDate,
        TextBundle::from_sections([TextSection::new(
            DEFAULT_SCORE_TEXT,
            TextStyle {
                font: FontAsset::asset(&asset_server, &FontName::HachiMaruPopReg),
                font_size: 25.,
                color: Color::WHITE,
                ..default()
            },
        )]),
    ));
}

fn high_score_text(children_builder: &mut ChildBuilder, asset_server: &Res<AssetServer>) {
    children_builder
        .spawn((
            NodeBundle {
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
            },
            HighScoreText,
        ))
        .with_children(|parent: &mut ChildBuilder<'_, '_, '_>| {
            high_score_text_score(parent, asset_server);
            high_score_text_date(parent, asset_server);
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

    let image_size = ImageSize::new(340.0, 300.0);

    let mut piece_image_bundle =
        ImageUI::image_bundle(ImageName::HighScoreFrame, &asset_server, &image_size);
    piece_image_bundle.style = Style {
        position_type: PositionType::Absolute,
        left: Val::Px(50.),
        bottom: Val::Px(50.),
        height: Val::Px(*image_size.get_height()),
        width: Val::Px(*image_size.get_width()),
        flex_direction: FlexDirection::Column,
        align_items: AlignItems::Center,
        ..piece_image_bundle.style
    };

    commands.spawn(piece_image_bundle).with_children(|parent| {
        for _ in 0..MAX_HIGH_SCORE_COUNT {
            high_score_text(parent, &asset_server)
        }
    });
}

/**
 * ハイスコアの更新
 */
pub fn update_high_score(
    _: Commands,
    mut high_score_text_score_query: Query<
        &mut Text,
        (With<HighScoreTextScore>, Without<HighScoreTextDate>),
    >,
    mut high_score_text_date_query: Query<
        &mut Text,
        (With<HighScoreTextDate>, Without<HighScoreTextScore>),
    >,
    high_scores_res: Res<HighScores>,
) {
    let mut score_iterator = high_score_text_score_query.iter_mut();
    let mut date_iterator = high_score_text_date_query.iter_mut();

    for high_score in high_scores_res.0.iter() {
        let score_text_option = score_iterator.next();
        match score_text_option {
            Some(mut score_text) => {
                score_text.sections[0].value = high_score.score.to_string();
            }

            None => {
                // do nothing
            }
        }

        let date_text_option = date_iterator.next();
        match date_text_option {
            Some(mut date_text) => {
                date_text.sections[0].value = high_score.date.clone();
            }

            None => {
                // do nothing
            }
        }
    }
}
