use bevy::{
    ecs::{
        entity::Entity,
        event::EventReader,
        query::{Or, With, Without},
        schedule::{NextState, State},
        system::{Commands, Query, Res, ResMut},
    },
    gizmos::gizmos::Gizmos,
    math::Vec2,
    render::{color::Color, primitives::Sphere},
    sprite::{Sprite, SpriteBundle},
    transform::components::Transform,
    utils::default,
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
        component::{
            active_piece::{self, ActivePiece},
            animal_piece::{
                animal_piece::AnimalPiece, animal_piece_component::AnimalPieceComponent,
            },
            drop_piece_indicator::DropPieceIndicator,
            falling::{self, Falling},
        },
        ext::piece_commands_ext::PieceCommandsExt,
        parameter::{
            piece_faller::PieceFaller, piece_sound_player::PieceSoundPlayer,
            piece_spawer::PieceSpawner,
        },
        resource::spawn_piece_state::SpawnPieceState,
    },
    resource::drop_postion::{self, DropPosition},
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
    query: Query<&AnimalPieceComponent, Or<(With<ActivePiece>, With<Falling>)>>,
    spawn_piece_state: Res<SpawnPieceState>,
    app_state: ResMut<State<GameState>>,
    mut piece_spawer: PieceSpawner,
) {
    if !query.is_empty() {
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
    mut commnads: Commands,
    mut piece_faller: PieceFaller,
    input: PlayerInput,
    mut query: Query<(Entity, &AnimalPieceComponent), With<ActivePiece>>,
) {
    if !input.is_key_just_released_space() {
        return;
    }

    let Ok((entity, piece)) = query.get_single_mut() else {
        return;
    };

    commnads.convert_to_physical(entity, piece);
    piece_faller.make_falling(entity);
}

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

        println!(
            "Entity1 pos: ({}, {})",
            transform1.translation.x, transform1.translation.y
        );
        println!(
            "Entity2 pos: ({}, {})",
            transform2.translation.x, transform2.translation.y
        );
        println!("New piece pos: ({}, {})", position_x, position_y);

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

pub fn update_spawn_piece_state(
    mut spawn_piece_state: ResMut<SpawnPieceState>,
    query: Query<&AnimalPieceComponent, Or<(With<ActivePiece>, With<Falling>)>>,
) {
    // アクティブまたは落下中のピースがない場合のみ
    if !query.is_empty() {
        return;
    }

    *spawn_piece_state = SpawnPieceState::ShouldSpawn;
}

pub fn despawn_drop_piece_indicator(
    mut commands: Commands,
    falling_query: Query<&Falling>,
    query: Query<Entity, With<DropPieceIndicator>>,
) {
    if falling_query.is_empty() {
        return;
    }

    for entity in query.iter() {
        commands.entity(entity).despawn();
    }
}

pub fn spawn_drop_piece_indicator(
    mut commands: Commands,
    drop_piece_indicator: Query<Entity, With<DropPieceIndicator>>,
    drop_position: Res<DropPosition>,
    active_piece_query: Query<Entity, With<ActivePiece>>,
) {
    if active_piece_query.is_empty() {
        return;
    }

    // スポーン済み
    if !drop_piece_indicator.is_empty() {
        return;
    }

    let position_x = drop_position.x;

    commands.spawn((
        SpriteBundle {
            sprite: Sprite {
                // 色を指定
                color: Color::rgba(1.0, 1.0, 0.0, 0.6),
                // サイズを指定（幅、高さ）
                custom_size: Some(Vec2::new(3.0, 600.0)), // 幅3px、高さ600pxの矩形
                // テクスチャの一部を切り取り（アトラス使用時）
                rect: None,
                // 水平/垂直反転
                flip_x: false,
                flip_y: false,
                // アンカーポイント（中心、左上など）
                anchor: bevy::sprite::Anchor::Center,
                ..default()
            },
            // 位置、回転、スケール
            transform: Transform::from_xyz(position_x, 0.0, 5.0),
            ..default()
        },
        DropPieceIndicator,
    ));
}
