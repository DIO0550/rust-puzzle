use bevy::{
    app::{App, Plugin},
    ecs::schedule::OnEnter,
};

use crate::{
    game::state::game_state::GameState,
    piece::next_piece::{
        resource::next_piece::NextPiece, system::next_piece_system::setup_next_piece,
    },
};

pub struct NextPiecePlugin;
impl Plugin for NextPiecePlugin {
    fn build(&self, app: &mut App) {
        app.insert_resource(NextPiece::new())
            .add_systems(OnEnter(GameState::InGame), setup_next_piece);
    }
}
