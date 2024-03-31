use bevy::prelude::*;

use crate::score::{
    resource::score::Score,
    system::score_system::{setup_score, update_score},
};

pub struct ScorePlugin;
impl Plugin for ScorePlugin {
    fn build(&self, app: &mut App) {
        app.insert_resource(Score(0))
            .add_systems(Startup, setup_score)
            .add_systems(Update, update_score);
    }
}
