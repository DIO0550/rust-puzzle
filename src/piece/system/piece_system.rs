use bevy::{
    asset::{AssetServer, Assets},
    ecs::{
        entity::Entity,
        event::EventReader,
        query::With,
        system::{Commands, EntityCommands, Query, Res, ResMut},
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
};

use crate::{
    consts::consts::*,
    piece::component::{
        animal_piece::{animal_piece_component::AnimalPieceComponent, piece_image::PieceImage},
        falling::Falling,
        grab::Grab,
    },
    resource::grab_postion::GrabPostion,
    score::resource::score::Score,
};

/**
 * ピース生成
 */
pub fn spawn_piece(
    commands: &mut Commands,
    resource: &mut Res<GrabPostion>,
    meshes: &mut ResMut<Assets<Mesh>>,
    materials: &mut ResMut<Assets<ColorMaterial>>,
    asset_server: &Res<AssetServer>,
) {
    let piece = AnimalPieceComponent::spawn();
    let size = piece.animal_piece.get_size().to_f32();
    let image = PieceImage::from_piece_type(asset_server, &piece.animal_piece.get_piece_type());

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
            resource.x,
            BOX_SIZE_HEIHT * 2.0 / 3.0,
            0.0,
        )));
}

pub fn spawn_piece_system(
    mut commands: Commands,
    mut resource: Res<GrabPostion>,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<ColorMaterial>>,
    asset_server: Res<AssetServer>,
) {
    spawn_piece(
        &mut commands,
        &mut resource,
        &mut meshes,
        &mut materials,
        &asset_server,
    )
}

/**
 * ピース移動
 */
pub fn move_piece(
    mut commands: Commands,
    keyboard_input: Res<Input<KeyCode>>,
    mut query: Query<&mut Transform, (With<AnimalPieceComponent>, With<Grab>)>,
    time: Res<Time>,
) {
    let Ok(mut transform) = query.get_single_mut() else {
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
    transform.translation.x = new_paddle_position;
    commands.insert_resource(GrabPostion {
        x: new_paddle_position,
    })
}

/**
 * ピースを離す
 */
pub fn release_piece(
    mut commands: Commands,
    keyboard_input: Res<Input<KeyCode>>,
    mut query: Query<(Entity, &AnimalPieceComponent), With<Grab>>,
) {
    let Ok((entity, piece)) = query.get_single_mut() else {
        return;
    };

    if keyboard_input.just_released(KeyCode::Space) {
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
    }
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
    mut grab_postion: Res<GrabPostion>,
    score_res: Res<Score>,
    asset_server: Res<AssetServer>,
) {
    for collision_event in collision_events.read() {
        let entities = match collision_event {
            CollisionEvent::Started(entity1, entity2, _) => (entity1, entity2),
            CollisionEvent::Stopped(entity1, entity2, _) => (entity1, entity2),
        };

        if sensor_query.get(*entities.0).is_ok() || sensor_query.get(*entities.1).is_ok() {
            println!("sensor");

            continue;
        }

        if falling_query.get(*entities.0).is_ok() {
            // println!("remove falling");
            commands.entity(*entities.0).remove::<Falling>();
            spawn_piece(
                &mut commands,
                &mut grab_postion,
                &mut meshes,
                &mut materials,
                &asset_server,
            );
        };

        if falling_query.get(*entities.1).is_ok() {
            // println!("remove falling");
            commands.entity(*entities.1).remove::<Falling>();
            spawn_piece(
                &mut commands,
                &mut grab_postion,
                &mut meshes,
                &mut materials,
                &asset_server,
            );
        };

        let Ok((entity1, transform1)) = piece_query.get(*entities.0) else {
            println!("not animal piece entity 0");

            continue;
        };

        let Ok((entity2, transform2)) = piece_query.get(*entities.1) else {
            // println!("not animal piece entity 1");
            continue;
        };

        if entity1.animal_piece.get_piece_type() != entity2.animal_piece.get_piece_type() {
            // println!("not same type!");
            // println!("entity1 : {:?}", entity1.animal_piece.get_piece_type());
            // println!("entity2 : {:?}", entity2.animal_piece.get_piece_type());

            continue;
        }

        commands.entity(*entities.0).despawn();
        commands.entity(*entities.1).despawn();

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
