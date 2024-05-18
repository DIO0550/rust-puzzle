use bevy::{
    asset::AssetServer,
    ecs::system::Res,
    prelude::default,
    ui::{node_bundles::ImageBundle, Style, UiImage, Val},
};

use crate::piece::component::animal_piece::{piece_image::PieceImage, piece_type::PieceType};

pub struct PieceUI {
    piece_type: PieceType,
}

impl PieceUI {
    pub fn new(piece_type: PieceType) -> Self {
        Self {
            piece_type: piece_type,
        }
    }

    pub fn piece_type(&self) -> &PieceType {
        &self.piece_type
    }
}

pub trait PieceImageUITrait {
    fn image_bundle(&self, asset_server: &Res<AssetServer>, image_size: &f32) -> ImageBundle;
}

impl PieceImageUITrait for PieceUI {
    fn image_bundle(&self, asset_server: &Res<AssetServer>, image_size: &f32) -> ImageBundle {
        let image = PieceImage::from_piece_type(asset_server, self.piece_type());
        let bundle = ImageBundle {
            style: Style {
                width: Val::Px(*image_size),
                height: Val::Px(*image_size),
                ..default()
            },
            image: UiImage::new(image),
            ..default()
        };

        return bundle;
    }
}
