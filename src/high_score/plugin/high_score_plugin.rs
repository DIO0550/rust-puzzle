use crate::{
    game::system::game_state::GameState,
    high_score::{
        resource::high_scores::HighScores,
        system::high_score_system::{load_high_score, save_high_score},
    },
};
use bevy::{
    app::{App, Plugin},
    ecs::schedule::{IntoSystemConfigs, OnEnter},
};

pub struct HighScorePlugin;
impl Plugin for HighScorePlugin {
    fn build(&self, app: &mut App) {
        app.insert_resource(HighScores(vec![]))
            .add_systems(OnEnter(GameState::GameOver), (save_high_score).chain())
            .add_systems(OnEnter(GameState::InGame), (load_high_score).chain());
    }
}
