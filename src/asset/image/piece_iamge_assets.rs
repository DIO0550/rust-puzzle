use bevy::prelude::*;

use crate::asset::asset::AssetTrait;

use super::image::{PieceImageAsset, PieceImageName};

#[derive(Resource)]
pub struct PieceIamgeAssets {
    pub rat: Handle<Image>,
    pub cat: Handle<Image>,
    pub dog: Handle<Image>,
    pub giraffe: Handle<Image>,
    pub horse: Handle<Image>,
    pub penguin: Handle<Image>,
    pub panda: Handle<Image>,
    pub elephant: Handle<Image>,
}

impl PieceIamgeAssets {
    pub fn new(asset_server: &Res<AssetServer>) -> Self {
        let rat = PieceImageAsset::asset(asset_server, &PieceImageName::Rat);
        let cat = PieceImageAsset::asset(asset_server, &PieceImageName::Cat);
        let dog = PieceImageAsset::asset(asset_server, &PieceImageName::Dog);
        let giraffe = PieceImageAsset::asset(asset_server, &PieceImageName::Giraffe);
        let horse = PieceImageAsset::asset(asset_server, &PieceImageName::Horse);
        let penguin = PieceImageAsset::asset(asset_server, &PieceImageName::Penguin);
        let panda = PieceImageAsset::asset(asset_server, &PieceImageName::Panda);
        let elephant = PieceImageAsset::asset(asset_server, &PieceImageName::Elephant);

        Self {
            rat,
            cat,
            dog,
            giraffe,
            horse,
            penguin,
            panda,
            elephant,
        }
    }
}
