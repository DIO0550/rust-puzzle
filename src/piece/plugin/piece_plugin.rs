use bevy::{
    app::*,
    ecs::schedule::{common_conditions::in_state, IntoSystemConfigs, OnEnter},
};

use crate::{
    game::{
        state::{game_page_state::GamePageState, game_state::GameState},
        system::despawn::despawn_component,
    },
    piece::{
        component::animal_piece::animal_piece_component::AnimalPieceComponent,
        resource::spawn_piece_state::SpawnPieceState, system::piece_system::*,
    },
};

pub struct PiecePlugin;
impl Plugin for PiecePlugin {
    fn build(&self, app: &mut App) {
        app.insert_resource(SpawnPieceState::ShouldSpawn)
            .add_systems(
                OnEnter(GameState::InGame),
                despawn_component::<AnimalPieceComponent>,
            )
            .add_systems(
                Startup,
                (spawn_piece)
                    .run_if(in_state(GamePageState::Game))
                    .run_if(in_state(GameState::InGame)),
            )
            .add_systems(
                Update,
                (
                    spawn_piece,
                    move_piece,
                    release_piece,
                    handle_piece_collisions,
                    handle_game_over_sensor_collisions,
                    update_spawn_piece_state,
                    spawn_drop_piece_indicator,
                    despawn_drop_piece_indicator,
                )
                    .chain()
                    .run_if(in_state(GamePageState::Game))
                    .run_if(in_state(GameState::InGame)),
            );
    }
}
