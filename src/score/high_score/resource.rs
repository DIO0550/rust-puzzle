use bevy::ecs::system::Resource;
use chrono::Utc;
use serde::{Deserialize, Serialize};
use serde_json::{from_value, json, Value};

use crate::file::json_file::JsonFile;
use crate::file::storage::{Storage, StorageError};
use crate::file_name::traits::{FileNameParseError, FromFileName};
use crate::score::resource::Score;

pub const MAX_HIGH_SCORE_COUNT: usize = 5;
const HIGH_SCORE_DATE_FORMAT: &str = "%Y年%m月%d日";
const MONTH_HIGH_SCORE_FILE_NAME_FORMAT: &str = "%Y%m_high_score.json";

#[derive(Resource, Clone, Serialize, Deserialize)]
pub struct HighScore {
    pub date: String,
    pub score: u32,
}

// impl HighScore {
//     pub fn new(score: u32) -> Self {
//         let date = Utc::now().format(HIGH_SCORE_DATE_FORMAT).to_string();
//         HighScore { date, score }
//     }
// }

impl From<Score> for HighScore {
    fn from(score: Score) -> Self {
        let date = Utc::now().format(HIGH_SCORE_DATE_FORMAT).to_string();
        HighScore {
            date,
            score: score.0,
        }
    }
}

#[derive(Resource, Clone, Serialize, Deserialize)]
pub struct HighScores(pub Vec<HighScore>);

impl HighScores {
    pub fn push<T: Into<HighScore>>(&mut self, item: T) {
        let value = item.into();
        self.0.sort_by(|a, b| b.score.cmp(&a.score));
        if self.0.len() < MAX_HIGH_SCORE_COUNT {
            self.0.push(value);
            self.0.sort_by(|a, b| b.score.cmp(&a.score));

            return;
        }

        let minimum_score = &self.0[MAX_HIGH_SCORE_COUNT - 1];
        if minimum_score.score > value.score {
            return;
        }

        self.0[MAX_HIGH_SCORE_COUNT - 1] = value;
        self.0.sort_by(|a, b| b.score.cmp(&a.score));
    }

    pub fn to_value(&self) -> Value {
        json!({
            "high_scores": self,
        })
    }

    pub fn current_month_filename() -> String {
        Utc::now()
            .format(MONTH_HIGH_SCORE_FILE_NAME_FORMAT)
            .to_string()
    }

    pub fn load_current_month() -> Self {
        let file_name = Self::current_month_filename();
        let Some(mut json_data) = JsonFile::load(&file_name) else {
            return HighScores(vec![]);
        };

        let Some(high_scores_value) = json_data.get_mut("high_scores") else {
            return HighScores(vec![]);
        };
        let Ok(high_scores) = from_value::<HighScores>(high_scores_value.take()) else {
            return HighScores(vec![]);
        };

        return high_scores;
    }
}

impl FromFileName for HighScores {
    fn from_file_name(file_name: &str) -> Result<Self, FileNameParseError> {
        let Some(mut json_data) = JsonFile::load(file_name) else {
            return Err(FileNameParseError::FileNotFound(file_name.to_string()));
        };

        let Some(high_scores_value) = json_data.get_mut("high_scores") else {
            return Err(FileNameParseError::InvalidFormat(json_data.to_string()));
        };
        let Ok(high_scores) = from_value::<HighScores>(high_scores_value.take()) else {
            return Err(FileNameParseError::InvalidComponent {
                component: "high_scores",
                details: "Data structure is invalid or corrupted".to_string(),
            });
        };

        Ok(high_scores)
    }
}

impl Storage for HighScores {
    fn save(&self, file_name: &str) -> Result<(), StorageError> {
        let json_container = self.to_value();
        JsonFile::save(file_name, json_container);
        Ok(())
    }
}
