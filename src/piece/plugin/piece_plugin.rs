use bevy::{
    app::*,
    ecs::schedule::{common_conditions::in_state, IntoSystemConfigs},
};

use crate::{game::system::game_state::GameState, piece::system::piece_system::*};

pub struct PiecePlugin;
impl Plugin for PiecePlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(
            Startup,
            spawn_piece_system.run_if(in_state(GameState::InGame)),
        )
        .add_systems(
            Update,
            (piece_collision_events, move_piece, release_piece).run_if(in_state(GameState::InGame)),
        );
    }
}
