use bevy::ecs::system::Resource;
use serde::{Deserialize, Serialize};

pub const MAX_HIGH_SCORE_COUNT: usize = 5;

#[derive(Resource, Serialize, Deserialize)]
pub struct HighScore {
    pub date: String,
    pub score: u32,
}

#[derive(Resource, Serialize, Deserialize)]
pub struct HighScores(pub Vec<HighScore>);

impl HighScores {
    pub fn push(&mut self, value: HighScore) {
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
}
