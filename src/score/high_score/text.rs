// TODO：ここのコード汚いので、リファクタリングする

use crate::{
    asset::{
        asset::AssetTrait,
        font::font::{FontAsset, FontName},
    },
    game::image_bundle_builder::ImageSize,
    score::high_score::resource::{HighScores, MAX_HIGH_SCORE_COUNT},
};
use bevy::{ecs::component::Component, log, prelude::*};

#[derive(Component)]
pub struct HighScoreText;

#[derive(Component)]
pub struct HighScoreTextScore;

#[derive(Component)]
pub struct HighScoreTextDate;

const DEFAULT_SCORE_TEXT: &str = "-";

pub fn setup_high_score_text(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    high_scores_res: Res<HighScores>,
) {
    println!("Setting up high score text.");

    let image_size = ImageSize::new(340.0, 300.0);

    commands
        .spawn((
            Node {
                position_type: PositionType::Absolute,
                left: Val::Px(50.),
                bottom: Val::Px(50.),
                height: Val::Px(*image_size.get_height()),
                width: Val::Px(*image_size.get_width()),
                flex_direction: FlexDirection::Column,
                align_items: AlignItems::Center,
                ..default()
            },
            BackgroundColor(Color::srgba(0.1, 0.1, 0.1, 0.8)), // 半透明の背景
        ))
        .with_children(|parent| {
            for index in 0..MAX_HIGH_SCORE_COUNT {
                let high_score = high_scores_res.0.get(index);

                // high_score_text equivalent - directly inline
                parent
                    .spawn((
                        Node {
                            margin: UiRect {
                                left: Val::Px(0.0),
                                right: Val::Px(0.0),
                                top: Val::Px(10.0),
                                bottom: Val::Px(0.0),
                            },
                            flex_direction: FlexDirection::Column,
                            justify_content: JustifyContent::SpaceBetween,
                            align_items: AlignItems::Center,
                            ..default()
                        },
                        HighScoreText,
                    ))
                    .with_children(|parent| {
                        // high_score_text_date equivalent - directly inline
                        parent.spawn((
                            HighScoreTextDate,
                            Text::new(
                                high_score
                                    .map(|hs| hs.date.as_str())
                                    .unwrap_or(DEFAULT_SCORE_TEXT),
                            ),
                            TextFont {
                                font: FontAsset::asset(&asset_server, &FontName::HachiMaruPopReg),
                                font_size: 25.,
                                ..default()
                            },
                            TextColor(Color::WHITE),
                            Node {
                                position_type: PositionType::Relative,
                                ..Default::default()
                            },
                        ));

                        parent.spawn((
                            HighScoreTextScore,
                            Text::new(
                                high_score
                                    .map(|hs| hs.score.to_string())
                                    .unwrap_or(DEFAULT_SCORE_TEXT.to_string()),
                            ),
                            TextFont {
                                font: FontAsset::asset(&asset_server, &FontName::HachiMaruPopReg),
                                font_size: 25.,
                                ..default()
                            },
                            TextColor(Color::WHITE),
                            Node {
                                position_type: PositionType::Relative,
                                ..Default::default()
                            },
                        ));
                    });
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
    if high_scores_res.0.is_empty() {
        println!("No high scores available to update.");
        log::warn!("No high scores available to update.");
        return;
    }

    println!(
        "Updating high score text with {} entries.",
        high_scores_res.0.len()
    );
    log::info!(
        "Updating high score text with {} entries.",
        high_scores_res.0.len()
    );
    let mut score_iterator = high_score_text_score_query.iter_mut();
    let mut date_iterator = high_score_text_date_query.iter_mut();

    for high_score in high_scores_res.0.iter() {
        println!(
            "Updating high score: {} - {}",
            high_score.score, high_score.date
        );
        let score_text_option = score_iterator.next();
        match score_text_option {
            Some(mut score_text) => {
                **score_text = high_score.score.to_string();
            }

            None => {
                // do nothing
            }
        }

        let date_text_option = date_iterator.next();
        match date_text_option {
            Some(mut date_text) => {
                **date_text = high_score.date.clone();
            }

            None => {
                // do nothing
            }
        }
    }
}
