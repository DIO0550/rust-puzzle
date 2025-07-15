use bevy::prelude::{Commands, Res, ResMut};
use chrono::Utc;
use serde_json::{from_value, json};

use crate::{
    file::file_reader::{FileReader, FileReaderTrait},
    score::{
        high_score::resource::{HighScore, HighScores},
        resource::Score,
    },
};

const HIGH_SCORE_FILE_NAME: &str = "high_score.json";
const MONTH_HIGH_SCORE_FILE_NAME_FORMAT: &str = "%Y%m_high_score.json";
const HIGH_SCORE_DATE_FORMAT: &str = "%Y年%m月%d日";

/**
 * 現在の月毎のハイスコアのファイル名
 */
pub fn now_month_high_score_file_name() -> String {
    return Utc::now()
        .format(MONTH_HIGH_SCORE_FILE_NAME_FORMAT)
        .to_string();
}

/**
 * ハイスコア保存
 */
pub fn save_high_score(
    _: Commands,
    puzzle_score_res: Res<Score>,
    mut high_scores_res: ResMut<HighScores>,
) {
    let score = puzzle_score_res.0;
    let new_high_score: HighScore = HighScore {
        score: score,
        date: Utc::now().format(HIGH_SCORE_DATE_FORMAT).to_string(),
    };

    let high_scores = high_scores_res.as_mut();
    high_scores.push(new_high_score);

    let json_container = json!({
        "high_scores": high_scores,
    });

    FileReader::save_data(&HIGH_SCORE_FILE_NAME, json_container);
}

/**
 * 現在の月毎のハイスコア保存
 */
pub fn save_now_month_high_score(_: Commands, puzzle_score_res: Res<Score>) {
    let score = puzzle_score_res.0;
    let new_high_score: HighScore = HighScore {
        score: score,
        date: Utc::now().format(HIGH_SCORE_DATE_FORMAT).to_string(),
    };

    let mut high_scores = load_now_month_high_score_file();
    high_scores.push(new_high_score);

    let json_container = json!({
        "high_scores": high_scores,
    });
    let file_name = now_month_high_score_file_name();

    FileReader::save_data(&file_name, json_container);
}

/**
 * ハイスコアロード
 */
pub fn load_high_score(mut commnads: Commands) {
    let load_value = FileReader::load_data(&HIGH_SCORE_FILE_NAME);

    let Some(mut data_value) = load_value else {
        commnads.insert_resource(HighScores(vec![]));

        return;
    };

    let Some(high_scores_value) = data_value.get_mut("high_scores") else {
        commnads.insert_resource(HighScores(vec![]));

        return;
    };

    let Ok(high_scores) = from_value::<HighScores>(high_scores_value.take()) else {
        commnads.insert_resource(HighScores(vec![]));

        return;
    };

    commnads.insert_resource(high_scores);
}

/**
 * 現在の月毎のハイスコアファイルロード
 */
pub fn load_now_month_high_score_file() -> HighScores {
    let load_value = FileReader::load_data(&now_month_high_score_file_name());

    let Some(mut data_value) = load_value else {
        return HighScores(vec![]);
    };

    let Some(high_scores_value) = data_value.get_mut("high_scores") else {
        return HighScores(vec![]);
    };

    let Ok(high_scores) = from_value::<HighScores>(high_scores_value.take()) else {
        return HighScores(vec![]);
    };

    return high_scores;
}
