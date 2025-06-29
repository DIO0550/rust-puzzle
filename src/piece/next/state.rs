use bevy::{
    asset::Handle,
    ecs::system::{Res, ResMut, Resource, SystemParam},
    render::texture::Image,
};

use crate::{
    asset::image::{image_assets::ImageAssets, piece_image_assets::PieceImageAssets},
    piece::component::animal_piece::piece_type::PieceType,
    ui::image::image_handle_resource::ImageHandleResource,
};

#[derive(Resource)]
pub struct NextPiece(pub PieceType);

impl NextPiece {
    pub fn new() -> NextPiece {
        let piece_type = PieceType::new_random();
        let next_piece = NextPiece(piece_type);

        return next_piece;
    }
}

impl ImageHandleResource<PieceImageAssets> for NextPiece {
    fn image_handle_resource(&self, assets: &Res<PieceImageAssets>) -> Handle<Image> {
        return assets.handle_image_from(&self.0);
    }
}

#[derive(SystemParam)]
pub struct NextPieceManager<'w> {
    next_piece: ResMut<'w, NextPiece>,
}

impl<'w> NextPieceManager<'w> {
    pub fn update_next_piece(&mut self) {
        self.next_piece.0 = NextPiece::new().0;
    }

    pub fn get_next_piece(&self) -> &PieceType {
        return &self.next_piece.0;
    }
}
