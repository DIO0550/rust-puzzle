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
        collision::*,
        component::animal_piece::animal_piece_component::AnimalPieceComponent,
        drop::{
            despawn_drop_piece_indicator, drop_piece, spawn_drop_piece_indicator,
            update_drop_piece_indicator_position, DropPosition,
        },
        movement::move_piece,
        spawner::{spawn_piece, update_spawn_piece_state, SpawnPieceState},
    },
};

pub struct PiecePlugin;
impl Plugin for PiecePlugin {
    fn build(&self, app: &mut App) {
        app.insert_resource(SpawnPieceState::ShouldSpawn)
            .insert_resource(DropPosition {
                x: 0.0, // 初期位置は0.0
            })
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
                    drop_piece,
                    handle_piece_collisions,
                    handle_game_over_sensor_collisions,
                    update_spawn_piece_state,
                    spawn_drop_piece_indicator,
                    despawn_drop_piece_indicator,
                    update_drop_piece_indicator_position,
                )
                    .chain()
                    .run_if(in_state(GamePageState::Game))
                    .run_if(in_state(GameState::InGame)),
            );
    }
}
