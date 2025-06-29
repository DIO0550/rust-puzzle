use bevy::{
    app::{App, Plugin},
    ecs::schedule::{OnEnter, OnExit},
};

use crate::{
    game::{state::game_page_state::GamePageState, system::despawn::despawn_component},
    piece::{
        evolution::{ui::container::PieceEvolveContainer, ui::setup::setup_evolve_piece},
        next::state::NextPiece,
    },
};

pub struct PieceEvolvePlugin;
impl Plugin for PieceEvolvePlugin {
    fn build(&self, app: &mut App) {
        app.insert_resource(NextPiece::new())
            .add_systems(OnEnter(GamePageState::Game), setup_evolve_piece)
            .add_systems(
                OnExit(GamePageState::Game),
                despawn_component::<PieceEvolveContainer>,
            );
    }
}
