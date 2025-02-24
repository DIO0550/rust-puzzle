use bevy::prelude::*;

use crate::{
    asset::asset::{AssetIdCollection, AssetLoadTrait, AssetTrait},
    piece::component::animal_piece::piece_type::PieceType,
};

use super::{
    image::PieceImageAsset,
    image_assets::{FromAssetServer, ImageAssets},
};

#[derive(Resource)]
pub struct PieceImageAssets {
    pub rat: Handle<Image>,
    pub cat: Handle<Image>,
    pub dog: Handle<Image>,
    pub giraffe: Handle<Image>,
    pub horse: Handle<Image>,
    pub penguin: Handle<Image>,
    pub panda: Handle<Image>,
    pub elephant: Handle<Image>,
}

impl AssetIdCollection<Image> for PieceImageAssets {
    fn assets_ids(&self) -> Vec<AssetId<Image>> {
        vec![
            self.rat.id(),
            self.cat.id(),
            self.dog.id(),
            self.giraffe.id(),
            self.horse.id(),
            self.penguin.id(),
            self.panda.id(),
            self.elephant.id(),
        ]
    }
}

impl AssetLoadTrait<Image> for PieceImageAssets {}

impl FromAssetServer for PieceImageAssets {
    fn new(asset_server: &Res<AssetServer>) -> Self {
        let rat = PieceImageAsset::asset(asset_server, &PieceType::Rat);
        let cat = PieceImageAsset::asset(asset_server, &PieceType::Cat);
        let dog = PieceImageAsset::asset(asset_server, &PieceType::Dog);
        let giraffe = PieceImageAsset::asset(asset_server, &PieceType::Giraffe);
        let horse = PieceImageAsset::asset(asset_server, &PieceType::Horse);
        let penguin = PieceImageAsset::asset(asset_server, &PieceType::Penguin);
        let panda = PieceImageAsset::asset(asset_server, &PieceType::Panda);
        let elephant = PieceImageAsset::asset(asset_server, &PieceType::Elephant);

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

impl ImageAssets<PieceType> for PieceImageAssets {
    fn handle_image_from(&self, asset_name: &PieceType) -> Handle<Image> {
        let image = match asset_name {
            PieceType::Rat => self.rat.clone(),
            PieceType::Cat => self.cat.clone(),
            PieceType::Dog => self.dog.clone(),
            PieceType::Giraffe => self.giraffe.clone(),
            PieceType::Horse => self.horse.clone(),
            PieceType::Penguin => self.penguin.clone(),
            PieceType::Panda => self.panda.clone(),
            PieceType::Elephant => self.elephant.clone(),
        };

        return image;
    }
}
