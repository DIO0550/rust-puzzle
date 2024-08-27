use std::time::Duration;

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
    app::{App, Plugin, PreUpdate, Update},
    ecs::schedule::{IntoSystemConfigs, OnEnter},
    prelude::{in_state, run_once},
    time::common_conditions::on_timer,
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
                    .run_if(run_once()),
            );
        // .add_systems(
        //     Update,
        //     setup_high_score_text.run_if(in_state(GameState::InGame)),
        // );
    }
}
