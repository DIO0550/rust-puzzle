use bevy::{
    ecs::{
        entity::Entity,
        event::EventReader,
        query::{With, Without},
        schedule::NextState,
        system::{Commands, Query, Res, ResMut},
    },
    transform::components::Transform,
};
use bevy_rapier2d::{
    geometry::Sensor,
    pipeline::CollisionEvent,
    plugin::{RapierConfiguration, RapierContext},
};

use crate::{
    game::state::GameState,
    game_over::sensor::GameOverSensor,
    piece::{
        component::{
            active_piece::ActivePiece,
            animal_piece::{
                animal_piece::AnimalPiece, animal_piece_component::AnimalPieceComponent,
            },
            falling::Falling,
        },
        ext::commands_ext::PieceCommandsExt,
        sound::PieceSoundPlayer,
        spawner::PieceSpawner,
    },
    score::resource::Score,
};

/**
 * 衝突イベント
 */
pub fn handle_piece_collisions(
    mut commands: Commands,
    mut collision_events: EventReader<CollisionEvent>,
    piece_query: Query<(&AnimalPieceComponent, &Transform)>,
    falling_query: Query<&Falling>,
    sensor_query: Query<&Sensor>,
    mut score_res: ResMut<Score>,
    mut piece_sound_player: PieceSoundPlayer,
    mut piece_spawner: PieceSpawner,
) {
    for collision_event in collision_events.read() {
        let (entity1, entity2) = match collision_event {
            CollisionEvent::Started(entity1, entity2, _) => (entity1, entity2),
            CollisionEvent::Stopped(entity1, entity2, _) => (entity1, entity2),
        };

        if sensor_query.contains(*entity1) || sensor_query.contains(*entity2) {
            continue;
        }

        // 落下状態を解除（contains で効率化）
        if falling_query.contains(*entity1) {
            commands.entity(*entity1).remove::<Falling>();
        }
        if falling_query.contains(*entity2) {
            commands.entity(*entity2).remove::<Falling>();
        }

        let Ok((piece_comp1, transform1)) = piece_query.get(*entity1) else {
            continue;
        };

        let Ok((piece_comp2, transform2)) = piece_query.get(*entity2) else {
            continue;
        };

        if piece_comp1.get_piece_type() != piece_comp2.get_piece_type() {
            continue;
        }

        commands.entity(*entity1).despawn();
        commands.entity(*entity2).despawn();

        piece_sound_player.play_union_sound();

        score_res.0 += piece_comp1.animal_piece.get_score().to_u32();

        let Some(piece) = piece_comp1.evolve() else {
            continue;
        };

        let position_x = (transform1.translation.x + transform2.translation.x) / 2.0;
        let position_y = (transform1.translation.y + transform2.translation.y) / 2.0;

        let entity =
            piece_spawner.spawn_inactive_piece_with_position(position_x, position_y, piece.clone());
        commands.convert_to_physical(entity, &piece);
    }
}

/**
 *  ゲームオーバーセンサーとの交差イベント
 */
pub fn handle_game_over_sensor_collisions(
    _: Commands,
    rapier_context: Res<RapierContext>,
    mut config: ResMut<RapierConfiguration>,
    target_piece_query: Query<&AnimalPieceComponent, (Without<ActivePiece>, Without<Falling>)>,
    mut sensor_query: Query<Entity, (With<GameOverSensor>, With<Sensor>)>,
    mut game_state: ResMut<NextState<GameState>>,
) {
    let Ok(entity) = sensor_query.get_single_mut() else {
        return;
    };

    for (collider1, collider2, intersecting) in rapier_context.intersection_pairs_with(entity) {
        if !intersecting {
            continue;
        }

        if !target_piece_query.contains(collider1) && !target_piece_query.contains(collider2) {
            continue;
        }

        config.physics_pipeline_active = false;
        game_state.set(GameState::GameOver);

        return;
    }
}
