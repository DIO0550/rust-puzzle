use bevy::{
    app::{App, Plugin},
    ecs::schedule::{OnEnter, OnExit},
};

use crate::{
    game::{state::game_page_state::GamePageState, system::despawn::despawn_component},
    piece::{
        next_piece::resource::next_piece::NextPiece,
        piece_evolve::{
            system::piece_evolve_system::setup_evolve_piece,
            ui::piece_evolve_container::PieceEvolveContainer,
        },
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
