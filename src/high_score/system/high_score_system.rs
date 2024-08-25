use bevy::prelude::{Commands, Res, ResMut};
use chrono::Utc;
use serde_json::{from_value, json};

use crate::{
    file::file_reader::{FileReader, FileReaderTrait},
    high_score::resource::{high_score::HighScore, high_scores::HighScores},
    score::resource::score::Score,
};

const HIGH_SCORE_FILE_NAME: &str = "high_score.json";

pub fn save_high_score(
    _: Commands,
    puzzle_score_res: Res<Score>,
    mut high_scores_res: ResMut<HighScores>,
) {
    let score = puzzle_score_res.0;
    let new_high_score: HighScore = HighScore {
        score: score,
        date: Utc::now().to_string(),
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
    let Some(mut high_scores_value) = load_value else {
        commnads.insert_resource(HighScores(vec![]));

        return;
    };

    let high_scores: HighScores =
        from_value(high_scores_value.get_mut("high_scores").unwrap().take()).unwrap();

    commnads.insert_resource(high_scores);
}
