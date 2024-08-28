use bevy::prelude::{Commands, Res, ResMut};
use chrono::Utc;
use serde_json::{from_value, json};

use crate::{
    file::file_reader::{FileReader, FileReaderTrait},
    high_score::resource::{high_score::HighScore, high_scores::HighScores},
    score::resource::score::Score,
};

const HIGH_SCORE_FILE_NAME: &str = "high_score.json";
const HIGH_SCORE_DATE_FORMAT: &str = "%Y年%m月%d日";

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
