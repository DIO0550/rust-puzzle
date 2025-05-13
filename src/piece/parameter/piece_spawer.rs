use bevy::{
    ecs::{
        entity::Entity,
        system::{Commands, Res, ResMut, SystemParam},
    },
    transform::{components::Transform, TransformBundle},
};
use bevy_rapier2d::prelude::ActiveCollisionTypes;

use crate::{
    asset::image::{image_assets::ImageAssets, piece_image_assets::PieceImageAssets},
    consts::consts::{BOX_SIZE_HEIHT, PIECE_POSITION_Y_MARGIN},
    parameter::material_mesh::MeshMaterial,
    piece::component::{
        active_piece::ActivePiece,
        animal_piece::{
            animal_piece::AnimalPiece, animal_piece_component::AnimalPieceComponentGenerator,
        },
    },
    resource::drop_postion::DropPositionController,
};

#[derive(SystemParam)]
pub struct PieceSpawner<'w, 's> {
    commands: Commands<'w, 's>,
    mesh_material: MeshMaterial<'w>,
    animal_piece_generator: AnimalPieceComponentGenerator<'w>,
    piece_image_assets: Res<'w, PieceImageAssets>,
    grab_position_manager: DropPositionController<'w>,
}

impl<'w, 's> PieceSpawner<'w, 's> {
    pub fn spawn(&mut self) -> Entity {
        let animal_piece_component = self.animal_piece_generator.generate();

        let size = animal_piece_component.get_size().to_f32();
        let image = self
            .piece_image_assets
            .handle_image_from(&animal_piece_component.get_piece_type());

        self.grab_position_manager
            .set_grab_position(animal_piece_component.animal_piece.as_ref());

        let new_grab_postion = self.grab_position_manager.grab_position.x;
        let material_mesh = self.mesh_material.create_circle_material_mesh(size, image);

        return self
            .commands
            .spawn(ActivePiece)
            .insert(animal_piece_component)
            .insert(material_mesh)
            .insert(ActiveCollisionTypes::all())
            .insert(TransformBundle::from(Transform::from_xyz(
                new_grab_postion,
                BOX_SIZE_HEIHT / 2.0 + PIECE_POSITION_Y_MARGIN,
                0.0,
            )))
            .id();
    }
}
