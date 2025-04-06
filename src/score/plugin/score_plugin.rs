use bevy::{ecs::system::Despawn, prelude::*};

use crate::{
    game::{state::game_state::GameState, system::despawn::despawn_component},
    score::{
        resource::score::Score,
        system::score_system::setup_score,
        ui::{
            score_text_container::ScoreTextContainer, score_value_text_container::ScoreValueText,
        },
    },
    ui::text::updateable_text_controller::update_text,
};

pub struct ScorePlugin;
impl Plugin for ScorePlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<Score>()
            .add_systems(
                OnEnter(GameState::InGame),
                setup_score.run_if(in_state(GameState::InGame)),
            )
            .add_systems(
                Update,
                update_text::<ScoreValueText, Score>.run_if(in_state(GameState::InGame)),
            )
            .add_systems(
                OnExit(GameState::InGame),
                despawn_component::<ScoreTextContainer>,
            );
    }
}
