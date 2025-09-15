use bevy::prelude::*;
use bevy_rapier2d::{geometry::Sensor, pipeline::CollisionEvent};

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
    mut collision_events: EventReader<CollisionEvent>,
    target_piece_query: Query<&AnimalPieceComponent, (Without<ActivePiece>, Without<Falling>)>,
    sensor_query: Query<Entity, (With<GameOverSensor>, With<Sensor>)>,
    mut game_state: ResMut<NextState<GameState>>,
) {
    let Ok(sensor_entity) = sensor_query.single() else {
        return;
    };

    for collision_event in collision_events.read() {
        match collision_event {
            CollisionEvent::Started(entity1, entity2, _flags) => {
                let collider = if *entity1 == sensor_entity {
                    *entity2
                } else if *entity2 == sensor_entity {
                    *entity1
                } else {
                    continue;
                };

                if target_piece_query.contains(collider) {
                    game_state.set(GameState::GameOver);
                    return;
                }
            }
            _ => {}
        }
    }
}
