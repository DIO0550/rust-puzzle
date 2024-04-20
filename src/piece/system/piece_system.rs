use bevy::{
    asset::{AssetServer, Assets},
    ecs::system::{Commands, Res, ResMut},
    math::primitives::Circle,
    prelude::default,
    render::{color::Color, mesh::Mesh},
    sprite::{ColorMaterial, MaterialMesh2dBundle},
    transform::{components::Transform, TransformBundle},
};
use bevy_rapier2d::geometry::ActiveCollisionTypes;

use crate::{
    asset::{
        asset::AssetTrait,
        image::image::{PieceImageAsset, PieceImageName},
    },
    consts::consts::*,
    piece::component::{
        animal_piece::{animal_piece_component::AnimalPieceComponent, piece_image::PieceImage},
        grab::Grab,
    },
    resource::grab_postion::GrabPostion,
};

pub fn spawn_piece(
    commands: &mut Commands,
    resource: &mut Res<GrabPostion>,
    meshes: &mut ResMut<Assets<Mesh>>,
    materials: &mut ResMut<Assets<ColorMaterial>>,
    asset_server: &Res<AssetServer>,
) {
    let piece = AnimalPieceComponent::spawn();
    let size = piece.animal_piece.get_size().to_f32();
    let color: Color = PieceImage::from_piece_type(&piece.animal_piece.get_piece_type());
    let image = PieceImageAsset::asset(asset_server, &PieceImageName::Cat);

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
