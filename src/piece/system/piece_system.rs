use bevy::{
    ecs::{
        entity::Entity,
        event::EventReader,
        query::{Or, With},
        schedule::{NextState, State},
        system::{Commands, Query, Res, ResMut},
    },
    input::{keyboard::KeyCode, Input},
    time::Time,
    transform::components::Transform,
};
use bevy_rapier2d::{
    geometry::Sensor,
    pipeline::CollisionEvent,
    plugin::{RapierConfiguration, RapierContext},
};

use crate::{
    consts::consts::*,
    game::{component::game_over_sensor::GameOverSensor, state::game_state::GameState},
    parameter::{input::PlayerInput, time::TimeParams},
    piece::{
        self,
        component::{
            active_piece::ActivePiece, animal_piece::animal_piece_component::AnimalPieceComponent,
            falling::Falling,
        },
        parameter::{
            piece_faller::PieceFaller, piece_physics_converter::PiecePhysicsConverter,
            piece_sound_player::PieceSoundPlayer, piece_spawer::PieceSpawner,
        },
        resource::spawn_piece_state::SpawnPieceState,
    },
    resource::drop_postion::DropPosition,
    score::resource::score::Score,
};

#[derive(Debug, Clone, Copy)]
pub struct PieceMovement {
    direction: f32,
    speed: f32,
    delta_time: f32,
}

impl PieceMovement {
    pub fn new(input: &PlayerInput, speed: f32, delta_time: f32) -> Self {
        PieceMovement {
            direction: match (input.is_key_pressed_left(), input.is_key_pressed_right()) {
                (true, false) => -1.0,
                (false, true) => 1.0,
                _ => 0.0,
            },
            speed,
            delta_time,
        }
    }

    pub fn calculate_new_position(&self, current_x: f32) -> f32 {
        current_x + self.direction * self.speed * self.delta_time
    }

    pub fn is_moving(&self) -> bool {
        self.direction != 0.0
    }
}

/**
 * ピース生成
 */
pub fn spawn_piece(
    mut commands: Commands,
    mut query: Query<&AnimalPieceComponent, Or<(With<ActivePiece>, With<Falling>)>>,
    spawn_piece_state: Res<SpawnPieceState>,
    app_state: ResMut<State<GameState>>,
    piece_spawer: &mut PieceSpawner,
) {
    let Err(_) = query.get_single_mut() else {
        return;
    };

    if *app_state.get() != GameState::InGame {
        return;
    }

    if *spawn_piece_state == SpawnPieceState::Wait {
        return;
    }

    piece_spawer.spawn();
    commands.insert_resource(SpawnPieceState::Wait)
}

/**
 * ピース移動
 */
pub fn move_piece(
    mut commands: Commands,
    input: PlayerInput,
    mut query: Query<(&mut Transform, &AnimalPieceComponent), With<ActivePiece>>,
    time: TimeParams,
) {
    let Ok((mut transform, animal_piece_component)) = query.get_single_mut() else {
        return;
    };

    let piece_movement = PieceMovement::new(&input, PIECE_SPEED, time.delta_seconds());
    if !piece_movement.is_moving() {
        return;
    }

    let new_position = piece_movement.calculate_new_position(transform.translation.x);
    let new_drop_position =
        DropPosition::new(new_position, animal_piece_component.animal_piece.as_ref());
    transform.translation.x = new_drop_position.x;
    commands.insert_resource(new_drop_position);
}

/**
 * ピースを離す
 */
pub fn release_piece(
    _: Commands,
    mut piece_physics_converter: PiecePhysicsConverter,
    mut piece_faller: PieceFaller,
    keyboard_input: Res<Input<KeyCode>>,
    mut query: Query<(Entity, &AnimalPieceComponent), With<ActivePiece>>,
) {
    if !keyboard_input.just_released(KeyCode::Space) {
        return;
    }

    let Ok((entity, piece)) = query.get_single_mut() else {
        return;
    };

    piece_physics_converter.convert_to_physical(entity, piece);
    piece_faller.make_falling(entity);
}

/**
 * 衝突イベント
 */
pub fn piece_collision_events(
    mut commands: Commands,
    mut collision_events: EventReader<CollisionEvent>,
    piece_query: Query<(&AnimalPieceComponent, &Transform)>,
    falling_query: Query<&Falling>,
    sensor_query: Query<&Sensor>,
    score_res: Res<Score>,
    mut piece_sound_player: PieceSoundPlayer,
    mut piece_spawer: PieceSpawner,
    mut piece_physics_converter: PiecePhysicsConverter,
) {
    for collision_event in collision_events.read() {
        let entities = match collision_event {
            CollisionEvent::Started(entity1, entity2, _) => (entity1, entity2),
            CollisionEvent::Stopped(entity1, entity2, _) => (entity1, entity2),
        };

        if sensor_query.get(*entities.0).is_ok() || sensor_query.get(*entities.1).is_ok() {
            continue;
        }

        if falling_query.get(*entities.0).is_ok() {
            commands.entity(*entities.0).remove::<Falling>();
        };

        if falling_query.get(*entities.1).is_ok() {
            commands.entity(*entities.1).remove::<Falling>();
        };

        let Ok((entity1, transform1)) = piece_query.get(*entities.0) else {
            continue;
        };

        let Ok((entity2, transform2)) = piece_query.get(*entities.1) else {
            continue;
        };

        if entity1.animal_piece.get_piece_type() != entity2.animal_piece.get_piece_type() {
            continue;
        }

        commands.entity(*entities.0).despawn();
        commands.entity(*entities.1).despawn();

        piece_sound_player.play_union_sound();

        let Some(piece) = entity1.evolve() else {
            continue;
        };
        let new_score = score_res.0 + entity1.animal_piece.get_score().to_u32();
        commands.insert_resource(Score(new_score));

        let position_x = (transform1.translation.x + transform2.translation.x) / 2.0;
        let position_y = (transform1.translation.y + transform2.translation.y) / 2.0;

        let entity = piece_spawer.spawn_inactive_piece_with_position(position_x, position_y);
        piece_physics_converter.convert_to_physical(entity, &piece);
    }
}

/**
 *  ゲームオーバーセンサーとの交差イベント
 */
pub fn game_over_sensor_intersection_events(
    mut commands: Commands,
    rapier_context: Res<RapierContext>,
    mut config: ResMut<RapierConfiguration>,
    mut exclude_piece_query: Query<&AnimalPieceComponent, Or<(With<ActivePiece>, With<Falling>)>>,
    piece_query: Query<&AnimalPieceComponent>,
    mut query: Query<Entity, (With<GameOverSensor>, With<Sensor>)>,
    mut app_state: ResMut<NextState<GameState>>,
) {
    let Ok(entity) = query.get_single_mut() else {
        return;
    };

    for (collider1, collider2, intersecting) in rapier_context.intersection_pairs_with(entity) {
        if !intersecting {
            continue;
        }
        if exclude_piece_query.get(collider1).is_ok() || exclude_piece_query.get(collider2).is_ok()
        {
            continue;
        }

        // Piece以外なら除外
        if !piece_query.get(collider1).is_ok() && !piece_query.get(collider2).is_ok() {
            continue;
        }

        config.physics_pipeline_active = false;
        app_state.set(GameState::GameOver);

        return;
    }

    let Err(_) = exclude_piece_query.get_single_mut() else {
        return;
    };

    commands.insert_resource(SpawnPieceState::ShouldSpawn);
}
