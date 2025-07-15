use bevy::prelude::*;

use crate::{
    game::{despawn::despawn_component, screen_state::ScreenState, state::GameState},
    score::{
        resource::Score,
        ui::{container::ScoreTextContainer, setup::setup_score, value_container::ScoreValueText},
    },
    ui::text::updateable_text_controller::update_text,
};

pub struct ScorePlugin;
impl Plugin for ScorePlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<Score>()
            .add_systems(
                OnEnter(ScreenState::Game),
                setup_score.run_if(in_state(GameState::InGame)),
            )
            .add_systems(
                Update,
                update_text::<ScoreValueText, Score>
                    .run_if(in_state(ScreenState::Game).and_then(in_state(GameState::InGame))),
            )
            .add_systems(
                OnExit(ScreenState::Game),
                despawn_component::<ScoreTextContainer>,
            );
    }
}
