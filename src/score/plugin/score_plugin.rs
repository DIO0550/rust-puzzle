use bevy::prelude::*;

use crate::{
    game::system::game_state::GameState,
    score::{
        resource::score::Score,
        system::score_system::{setup_score, update_score},
    },
};

pub struct ScorePlugin;
impl Plugin for ScorePlugin {
    fn build(&self, app: &mut App) {
        app.insert_resource(Score(0))
            .add_systems(Startup, setup_score.run_if(in_state(GameState::InGame)))
            .add_systems(Update, update_score.run_if(in_state(GameState::InGame)));
    }
}
