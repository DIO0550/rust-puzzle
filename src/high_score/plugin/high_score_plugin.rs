use crate::{
    game::system::{
        despawn::despawn_component, game_page_state::GamePageState, game_state::GameState,
    },
    high_score::{
        component::high_score_text::HighScoreText,
        resource::high_scores::HighScores,
        system::{
            high_score_system::{load_high_score, save_high_score, save_now_month_high_score},
            high_score_text_system::{setup_high_score_text, update_high_score},
        },
    },
};
use bevy::{
    app::{App, Plugin, Update},
    ecs::schedule::{IntoSystemConfigs, OnEnter},
    prelude::{in_state, resource_changed, OnExit},
};

pub struct HighScorePlugin;
impl Plugin for HighScorePlugin {
    fn build(&self, app: &mut App) {
        app.insert_resource(HighScores(vec![]))
            .add_systems(
                OnEnter(GameState::GameOver),
                (save_high_score, save_now_month_high_score).chain(),
            )
            .add_systems(OnEnter(GameState::InGame), load_high_score)
            .add_systems(OnEnter(GamePageState::Game), setup_high_score_text)
            .add_systems(
                OnExit(GamePageState::Game),
                despawn_component::<HighScoreText>,
            )
            .add_systems(
                Update,
                (update_high_score)
                    .run_if(in_state(GameState::InGame))
                    .run_if(resource_changed::<HighScores>()),
            );
    }
}
