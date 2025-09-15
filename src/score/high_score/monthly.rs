use bevy::prelude::Resource;
use serde::{Deserialize, Serialize};
use serde_json::{from_value, json, Value};

use crate::date::year_month::YearMonth;
use crate::file::json_file::JsonFile;
use crate::file::storage::Storage;
use crate::file_name::traits::{FileNameParseError, FromFileName};
use crate::score::resource::Score;

#[derive(Resource, Serialize, Deserialize)]
pub struct MonthHighScores {
    pub date: YearMonth,
    pub high_scores: Vec<Score>,
}

impl MonthHighScores {
    pub fn push<T: Into<Score>>(&mut self, item: T) {
        let value = item.into();
        self.high_scores.push(value);
        self.high_scores.sort_by(|a, b| b.cmp(a));
    }

    pub fn try_succ(&self) -> Result<Self, FileNameParseError> {
        let new_year_month = self.date.succ();
        Self::from_file_name(&new_year_month.file_name())
    }

    pub fn try_prev(&self) -> Result<Self, FileNameParseError> {
        let new_year_month = self.date.pred();

        Self::from_file_name(&new_year_month.file_name())
    }

    pub fn file_name(&self) -> String {
        self.date.file_name()
    }

    pub fn to_value(&self) -> Value {
        json!({
            "month_high_scores": self,
        })
    }

    pub fn load_current_month() -> Self {
        let year_month = YearMonth::default();
        Self::from_file_name(&year_month.file_name()).unwrap_or_default()
    }
}

impl Default for MonthHighScores {
    fn default() -> Self {
        let year_month = YearMonth::default();

        Self {
            date: year_month,
            high_scores: vec![],
        }
    }
}

impl FromFileName for MonthHighScores {
    fn from_file_name(file_name: &str) -> Result<Self, FileNameParseError> {
        let Some(mut json_data) = JsonFile::load(file_name) else {
            return Err(FileNameParseError::FileNotFound(file_name.to_string()));
        };

        let Some(month_high_scores_value) = json_data.get_mut("month_high_scores") else {
            return Err(FileNameParseError::InvalidFormat(json_data.to_string()));
        };
        let Ok(month_high_scores) = from_value::<MonthHighScores>(month_high_scores_value.take())
        else {
            return Err(FileNameParseError::InvalidComponent {
                component: "month_high_scores",
                details: "Data structure is invalid or corrupted".to_string(),
            });
        };

        Ok(month_high_scores)
    }
}

impl Storage for MonthHighScores {
    fn save(&self, file_name: &str) -> Result<(), crate::file::storage::StorageError> {
        let json_container = self.to_value();
        JsonFile::save(file_name, json_container);
        Ok(())
    }
}
