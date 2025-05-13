use bevy::{
    ecs::system::{Res, SystemParam},
    pbr::StandardMaterial,
    sprite::{ColorMaterial, Material2d, MaterialMesh2dBundle},
};

use crate::{
    asset::image::{image_assets::ImageAssets, piece_image_assets::PieceImageAssets},
    parameter::material_mesh::MeshMaterial,
    piece::component::animal_piece::{
        animal_piece::AnimalPiece, animal_piece_component::AnimalPieceComponentGenerator,
    },
};

#[derive(SystemParam)]
pub struct PieceRenderer<'w> {
    mesh_material: MeshMaterial<'w>,
    animal_piece_generator: AnimalPieceComponentGenerator<'w>,
    piece_image_assets: Res<'w, PieceImageAssets>,
}

impl<'w> PieceRenderer<'w> {
    pub fn create_visual(&mut self) -> MaterialMesh2dBundle<ColorMaterial> {
        let animal_piece_component = self.animal_piece_generator.generate();
        let size = animal_piece_component.get_size().to_f32();
        let image = self
            .piece_image_assets
            .handle_image_from(&animal_piece_component.get_piece_type());

        let material_mesh = self.mesh_material.create_circle_material_mesh(size, image);

        return material_mesh;
    }
}
