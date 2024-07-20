use bevy::prelude::*;

use crate::{
    game::system::game_state::GameState,
    score::{
        resource::score::Score,
        system::score_system::{setup_score, update_current_score_text},
    },
};

pub struct ScorePlugin;
impl Plugin for ScorePlugin {
    fn build(&self, app: &mut App) {
        app.insert_resource(Score(0))
            .add_systems(Startup, setup_score.run_if(in_state(GameState::InGame)))
            .add_systems(
                Update,
                update_current_score_text.run_if(in_state(GameState::InGame)),
            );
    }
}
