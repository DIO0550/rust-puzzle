use bevy::{
    asset::{AssetServer, Assets},
    ecs::{
        entity::Entity,
        query::With,
        system::{Commands, Query, Res, ResMut},
    },
    input::{keyboard::KeyCode, ButtonInput},
    math::primitives::Circle,
    prelude::default,
    render::mesh::Mesh,
    sprite::{ColorMaterial, MaterialMesh2dBundle},
    time::Time,
    transform::{components::Transform, TransformBundle},
};
use bevy_rapier2d::{
    dynamics::{GravityScale, RigidBody, Sleeping},
    geometry::{ActiveCollisionTypes, ActiveEvents, Collider, ColliderMassProperties},
};

use crate::{
    consts::consts::*,
    piece::component::{
        animal_piece::{animal_piece_component::AnimalPieceComponent, piece_image::PieceImage},
        falling::Falling,
        grab::Grab,
    },
    resource::grab_postion::GrabPostion,
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
        // TODO: 後でピースの画像に直す
        .insert(MaterialMesh2dBundle {
            mesh: meshes.add(Circle::new(size * 2.0 * UNIT_WIDTH)).into(),
            material: materials.add(image), //materials.add(ColorMaterial::from(color)),
            ..default()
        })
        .insert(ActiveCollisionTypes::all())
        .insert(TransformBundle::from(Transform::from_xyz(
            resource.x,
            BOX_SIZE_HEIHT * 2.0 / 3.0,
            0.0,
        )));
}

/**
 * ピース移動
 */
pub fn move_piece(
    mut commands: Commands,
    keyboard_input: Res<ButtonInput<KeyCode>>,
    mut query: Query<&mut Transform, (With<AnimalPieceComponent>, With<Grab>)>,
    time: Res<Time>,
) {
    let Ok(mut transform) = query.get_single_mut() else {
        return;
    };

    let mut direction = 0.0;

    if keyboard_input.pressed(KeyCode::ArrowLeft) {
        direction -= 1.0;
    }

    if keyboard_input.pressed(KeyCode::ArrowRight) {
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
    keyboard_input: Res<ButtonInput<KeyCode>>,
    mut query: Query<(Entity, &AnimalPieceComponent), With<Grab>>,
) {
    let Ok((entity, piece)) = query.get_single_mut() else {
        // println!("no single mut");
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
