use crate::{
    game::system::game_state::GameState,
    high_score::{
        resource::high_scores::HighScores,
        system::{
            high_score_system::{load_high_score, save_high_score},
            high_score_text_system::setup_high_score_text,
        },
    },
};
use bevy::{
    app::{App, Plugin, Update},
    ecs::schedule::{IntoSystemConfigs, OnEnter},
    prelude::{in_state, resource_changed, run_once},
};

pub struct HighScorePlugin;
impl Plugin for HighScorePlugin {
    fn build(&self, app: &mut App) {
        app.insert_resource(HighScores(vec![]))
            .add_systems(OnEnter(GameState::GameOver), (save_high_score).chain())
            .add_systems(OnEnter(GameState::InGame), (load_high_score))
            .add_systems(
                Update,
                (setup_high_score_text)
                    .run_if(in_state(GameState::InGame))
                    .run_if(resource_changed::<HighScores>()),
            );
    }
}
