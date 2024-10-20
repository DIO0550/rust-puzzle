use bevy::{
    asset::{AssetServer, Assets},
    audio::{AudioBundle, PlaybackSettings},
    ecs::{
        entity::Entity,
        event::EventReader,
        query::{Or, With},
        schedule::{NextState, State},
        system::{Commands, Query, Res, ResMut},
    },
    input::{keyboard::KeyCode, Input},
    math::Vec2,
    prelude::default,
    render::mesh::{shape::Circle, Mesh},
    sprite::{ColorMaterial, MaterialMesh2dBundle},
    time::Time,
    transform::{components::Transform, TransformBundle},
};
use bevy_rapier2d::{
    dynamics::{GravityScale, RigidBody, Sleeping, Velocity},
    geometry::{ActiveCollisionTypes, ActiveEvents, Collider, ColliderMassProperties, Sensor},
    pipeline::CollisionEvent,
    plugin::{RapierConfiguration, RapierContext},
};

use crate::{
    asset::resource::piece_sound::{PieceFallSound, PieceUnionSound},
    consts::consts::*,
    game::{component::game_over_sensor::GameOverSeonsor, system::game_state::GameState},
    piece::{
        component::{
            animal_piece::{animal_piece_component::AnimalPieceComponent, piece_image::PieceImage},
            falling::Falling,
            grab::Grab,
        },
        resource::{next_piece::NextPiece, spawn_piece_state::SpawnPieceState},
    },
    resource::grab_postion::GrabPostion,
    score::resource::score::Score,
};

/**
 * ピース生成
 */
pub fn spawn_piece(
    mut commands: Commands,
    mut query: Query<&AnimalPieceComponent, Or<(With<Grab>, With<Falling>)>>,
    grab_position_resource: Res<GrabPostion>,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<ColorMaterial>>,
    asset_server: Res<AssetServer>,
    next_piece_res: Res<NextPiece>,
    spawn_piece_state: Res<SpawnPieceState>,
    app_state: ResMut<State<GameState>>,
) {
    let Err(_) = query.get_single_mut() else {
        return;
    };

    if *app_state.get() != GameState::InGame {
        print!("Not InGame");
        return;
    }

    if *spawn_piece_state == SpawnPieceState::Wait {
        println!("wait");
        return;
    }

    println!("Spawn");

    let piece = AnimalPieceComponent::from(next_piece_res.0);
    let size = piece.animal_piece.get_size().to_f32();
    let image = PieceImage::from_piece_type(&asset_server, &piece.animal_piece.get_piece_type());

    let new_grab_postion = GrabPostion::new(grab_position_resource.x, &*piece.animal_piece);

    commands.insert_resource(NextPiece::new());

    commands
        .spawn(Grab)
        .insert(piece)
        .insert(MaterialMesh2dBundle {
            mesh: bevy::sprite::Mesh2dHandle(
                meshes.add(Circle::new(size * 2.0 * UNIT_WIDTH).into()),
            ),
            material: materials.add(image.into()),
            ..default()
        })
        .insert(ActiveCollisionTypes::all())
        .insert(TransformBundle::from(Transform::from_xyz(
            new_grab_postion.x,
            BOX_SIZE_HEIHT * 2.0 / 3.0,
            0.0,
        )));

    commands.insert_resource(new_grab_postion);
    commands.insert_resource(SpawnPieceState::Wait)
}

/**
 * ピース移動
 */
pub fn move_piece(
    mut commands: Commands,
    keyboard_input: Res<Input<KeyCode>>,
    mut query: Query<(&mut Transform, &AnimalPieceComponent), With<Grab>>,
    time: Res<Time>,
) {
    let Ok((mut transform, animal_piece_component)) = query.get_single_mut() else {
        return;
    };

    let mut direction = 0.0;

    if keyboard_input.pressed(KeyCode::Left) {
        direction -= 1.0;
    }

    if keyboard_input.pressed(KeyCode::Right) {
        direction += 1.0;
    }

    let new_paddle_position =
        transform.translation.x + direction * PIECE_SPEED * time.delta_seconds();
    let new_grab_position =
        GrabPostion::new(new_paddle_position, &*animal_piece_component.animal_piece);
    transform.translation.x = new_grab_position.x;
    commands.insert_resource(new_grab_position)
}

/**
 * ピースを離す
 */
pub fn release_piece(
    mut commands: Commands,
    keyboard_input: Res<Input<KeyCode>>,
    mut query: Query<(Entity, &AnimalPieceComponent), With<Grab>>,
    piece_fall_sound_res: Res<PieceFallSound>,
) {
    let Ok((entity, piece)) = query.get_single_mut() else {
        return;
    };

    if !keyboard_input.just_released(KeyCode::Space) {
        return;
    }
    commands.entity(entity).remove::<Grab>();
    commands
        .entity(entity)
        .insert(RigidBody::Dynamic)
        .insert(Collider::ball(
            piece.animal_piece.get_size().to_f32() * 2.0 * UNIT_WIDTH,
        ))
        .insert(ActiveEvents::COLLISION_EVENTS)
        .insert(ColliderMassProperties::Mass(50.0))
        .insert(GravityScale(10.0))
        .insert(Sleeping::disabled())
        .insert(Falling);

    commands.spawn(AudioBundle {
        source: piece_fall_sound_res.0.clone(),
        settings: PlaybackSettings::DESPAWN,
    });
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
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<ColorMaterial>>,
    score_res: Res<Score>,
    asset_server: Res<AssetServer>,
    piece_union_sound_res: Res<PieceUnionSound>,
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

        commands.spawn(AudioBundle {
            source: piece_union_sound_res.0.clone(),
            settings: PlaybackSettings::DESPAWN,
        });

        let Some(piece) = entity1.evolve() else {
            continue;
        };
        let new_score = score_res.0 + entity1.animal_piece.get_score().to_u32();
        commands.insert_resource(Score(new_score));

        let size = piece.animal_piece.get_size().to_f32();
        let image = PieceImage::from_piece_type(&asset_server, piece.animal_piece.get_piece_type());
        let position_x = (transform1.translation.x + transform2.translation.x) / 2.0;
        let position_y = (transform1.translation.y + transform2.translation.y) / 2.0;

        commands
            .spawn(piece)
            .insert(MaterialMesh2dBundle {
                mesh: bevy::sprite::Mesh2dHandle(
                    meshes.add(Circle::new(size * 2.0 * UNIT_WIDTH).into()),
                ),
                material: materials.add(image.into()),
                ..default()
            })
            .insert(TransformBundle::from(Transform::from_xyz(
                position_x, position_y, 0.0,
            )))
            .insert(RigidBody::Dynamic)
            .insert(Collider::ball(size * 2.0 * UNIT_WIDTH))
            .insert(ColliderMassProperties::Mass(50.0))
            .insert(GravityScale(10.0))
            .insert(ActiveEvents::COLLISION_EVENTS)
            .insert(Velocity {
                linvel: Vec2::new(0.0, 0.0),
                angvel: 0.0,
            })
            .insert(Sleeping::disabled());
    }
}

/**
 *  ゲームオーバーセンサーとの交差イベント
 */
pub fn game_over_sensor_intersection_events(
    mut commands: Commands,
    rapier_context: Res<RapierContext>,
    mut config: ResMut<RapierConfiguration>,
    mut exclude_piece_query: Query<&AnimalPieceComponent, Or<(With<Grab>, With<Falling>)>>,
    piece_query: Query<&AnimalPieceComponent>,
    mut query: Query<Entity, (With<GameOverSeonsor>, With<Sensor>)>,
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
