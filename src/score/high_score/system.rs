use bevy::{
    log,
    prelude::{Commands, Res, ResMut},
};

use crate::{
    date::year_month::YearMonth,
    file::storage::Storage,
    file_name::traits::FromFileName,
    score::{
        high_score::{monthly::MonthHighScores, resource::HighScores},
        resource::Score,
    },
};

const HIGH_SCORE_PROPERTY_NAME: &str = "high_scores";
const HIGH_SCORE_FILE_NAME: &str = "high_score.json";

/**
 * ハイスコア保存
 */
pub fn save_high_score(
    _: Commands,
    puzzle_score_res: Res<Score>,
    mut high_scores_res: ResMut<HighScores>,
) {
    high_scores_res.push(*puzzle_score_res);
    match high_scores_res.save(&HIGH_SCORE_FILE_NAME) {
        Ok(_) => {
            log::info!("High scores saved successfully.");
        }
        Err(e) => {
            log::error!("Failed to save high scores: {:?}", e);
        }
    }
}

/**
 * 現在の月毎のハイスコア保存
 */
pub fn save_current_month_high_score(_: Commands, puzzle_score_res: Res<Score>) {
    let mut high_scores = MonthHighScores::load_current_month();
    high_scores.push(*puzzle_score_res);
    match high_scores.save(high_scores.file_name().as_str()) {
        Ok(_) => {
            log::info!("Current month high scores saved successfully.");
        }
        Err(e) => {
            log::error!("Failed to save current month high scores: {:?}", e);
        }
    }
}

/**
 * ハイスコアロード
 */
pub fn load_high_score(mut commands: Commands) {
    let high_scores = HighScores::from_file_name(HIGH_SCORE_FILE_NAME);

    let Ok(high_scores) = high_scores else {
        commands.insert_resource(HighScores(vec![]));
        return;
    };

    commands.insert_resource(high_scores);
}

/**
 * 現在の月毎のハイスコアファイルロード
 */
pub fn load_current_month_high_score_file() -> MonthHighScores {
    let load_value = MonthHighScores::from_file_name(YearMonth::default().file_name().as_str());
    load_value.unwrap_or_default()
}
