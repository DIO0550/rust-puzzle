use bevy::{
    asset::{AssetServer, Handle},
    ecs::system::Res,
    render::texture::Image,
};

use crate::asset::{
    asset::AssetTrait,
    image::image::{PieceImageAsset, PieceImageName},
};

use super::animal_piece::PieceType;

pub struct PieceImage {}

impl PieceImage {
    pub fn from_piece_type(
        asset_server: &Res<AssetServer>,
        piece_type: &PieceType,
    ) -> Handle<Image> {
        let image = match piece_type {
            PieceType::Cat => PieceImageAsset::asset(asset_server, &PieceImageName::Cat),
            PieceType::Dog => PieceImageAsset::asset(asset_server, &PieceImageName::Dog),
            // 画像準備できたら修正
            PieceType::Elephant => PieceImageAsset::asset(asset_server, &PieceImageName::Cat),
            // 画像準備できたら修正
            PieceType::Giraffe => PieceImageAsset::asset(asset_server, &PieceImageName::Cat),
            // 画像準備できたら修正
            PieceType::Horse => PieceImageAsset::asset(asset_server, &PieceImageName::Cat),
            // 画像準備できたら修正
            PieceType::Panda => PieceImageAsset::asset(asset_server, &PieceImageName::Cat),
            PieceType::Penguin => PieceImageAsset::asset(asset_server, &PieceImageName::Penguin),
            // 画像準備できたら修正
            PieceType::Rat => PieceImageAsset::asset(asset_server, &PieceImageName::Cat),
        };

        return image;
    }
}
