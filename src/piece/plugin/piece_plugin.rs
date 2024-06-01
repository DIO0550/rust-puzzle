use bevy::{
    app::*,
    ecs::schedule::{common_conditions::in_state, IntoSystemConfigs},
};

use crate::{
    game::system::game_state::GameState,
    piece::{resource::next_piece::NextPiece, system::piece_system::*, ui::next_piece_ui::*},
};

pub struct PiecePlugin;
impl Plugin for PiecePlugin {
    fn build(&self, app: &mut App) {
        app.insert_resource(NextPiece::new())
            .add_systems(
                Startup,
                (spawn_piece, setup_display_next_piece).run_if(in_state(GameState::InGame)),
            )
            .add_systems(
                Update,
                (
                    spawn_piece,
                    move_piece,
                    release_piece,
                    piece_collision_events,
                    game_over_sensor_intersection_events,
                )
                    .chain()
                    .run_if(in_state(GameState::InGame)),
            )
            .add_systems(
                Update,
                (update_display_next_piece).run_if(in_state(GameState::InGame)),
            );
    }
}
