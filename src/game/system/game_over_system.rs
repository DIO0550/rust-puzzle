use bevy::ecs::{
    entity::Entity,
    query::With,
    system::{Query, Res},
};
use bevy_rapier2d::{
    geometry::{Collider, Sensor},
    plugin::RapierContext,
};

use crate::game::component::game_over_sensor::GameOverSeonsor;

/**
 * センサーとの交差イベント
 */
pub fn game_over_sensor_intersection_events(
    rapier_context: Res<RapierContext>,
    mut query: Query<Entity, (With<GameOverSeonsor>, With<Sensor>)>,
) {
    // let Ok(entity) = query.get_single_mut() else {
    //     println!("Query<Entity, With<GameOverSeonsor>> no single mut");
    //     return;
    // };

    // for (collider1, collider2, intersecting) in rapier_context.intersection_pairs_with(entity) {
    //     if intersecting {
    //         println!(
    //             "The entities {:?} and {:?} have intersecting colliders!",
    //             collider1, collider2
    //         );
    //     }

    //     println!("hoge");
    // }
}
