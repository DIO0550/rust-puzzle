use bevy::{
    asset::{AssetServer, Handle},
    ecs::system::Res,
    render::texture::Image,
};

use crate::asset::{
    asset::AssetTrait,
    image::image::{PieceImageAsset, PieceImageName},
};

use super::piece_type::PieceType;

pub struct PieceImage {}

impl PieceImage {
    pub fn from_piece_type(
        asset_server: &Res<AssetServer>,
        piece_type: &PieceType,
    ) -> Handle<Image> {
        let image = match piece_type {
            PieceType::Cat => PieceImageAsset::asset(asset_server, &PieceImageName::Cat),
            PieceType::Dog => PieceImageAsset::asset(asset_server, &PieceImageName::Dog),
            PieceType::Elephant => PieceImageAsset::asset(asset_server, &PieceImageName::Elephant),
            PieceType::Giraffe => PieceImageAsset::asset(asset_server, &PieceImageName::Giraffe),
            PieceType::Horse => PieceImageAsset::asset(asset_server, &PieceImageName::Horse),
            PieceType::Panda => PieceImageAsset::asset(asset_server, &PieceImageName::Panda),
            PieceType::Penguin => PieceImageAsset::asset(asset_server, &PieceImageName::Penguin),
            PieceType::Rat => PieceImageAsset::asset(asset_server, &PieceImageName::Rat),
        };

        return image;
    }
}
