use bevy::ecs::{
    entity::Entity,
    query::{Or, With},
    schedule::NextState,
    system::{Query, Res, ResMut},
};
use bevy_rapier2d::{
    geometry::Sensor,
    plugin::{RapierConfiguration, RapierContext},
};

use crate::{
    game::component::game_over_sensor::GameOverSeonsor,
    piece::component::{
        animal_piece::animal_piece_component::AnimalPieceComponent, falling::Falling, grab::Grab,
    },
};

use super::game_state::GameState;

/**
 * センサーとの交差イベント
 */
pub fn game_over_sensor_intersection_events(
    rapier_context: Res<RapierContext>,
    mut config: ResMut<RapierConfiguration>,
    exclude_piece_query: Query<&AnimalPieceComponent, Or<(With<Grab>, With<Falling>)>>,
    mut query: Query<Entity, (With<GameOverSeonsor>, With<Sensor>)>,
    mut app_state: ResMut<NextState<GameState>>,
) {
    let Ok(entity) = query.get_single_mut() else {
        println!("non single mut");
        return;
    };

    for (collider1, collider2, intersecting) in rapier_context.intersection_pairs_with(entity) {
        if intersecting {
            println!(
                "The entities {:?} and {:?} have intersecting colliders!",
                collider1, collider2
            );

            if !exclude_piece_query.get(collider1).is_ok()
                && !exclude_piece_query.get(collider2).is_ok()
            {
                config.physics_pipeline_active = false;
                app_state.set(GameState::GameOver);
            }
        }
    }
}
