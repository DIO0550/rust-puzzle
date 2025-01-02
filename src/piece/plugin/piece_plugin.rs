use bevy::{
    app::*,
    ecs::schedule::{common_conditions::in_state, IntoSystemConfigs, OnEnter},
};

use crate::{
    game::{state::game_state::GameState, system::despawn::despawn_component},
    piece::{
        component::animal_piece::animal_piece_component::AnimalPieceComponent,
        resource::{next_piece::NextPiece, spawn_piece_state::SpawnPieceState},
        system::piece_system::*,
        ui::next_piece_ui::*,
    },
};

pub struct PiecePlugin;
impl Plugin for PiecePlugin {
    fn build(&self, app: &mut App) {
        app.insert_resource(NextPiece::new())
            .insert_resource(SpawnPieceState::ShouldSpawn)
            .add_systems(
                OnEnter(GameState::InGame),
                despawn_component::<AnimalPieceComponent>,
            )
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
