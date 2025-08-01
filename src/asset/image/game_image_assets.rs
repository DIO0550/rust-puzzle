use bevy::prelude::*;

use crate::asset::asset::{AssetIdCollection, AssetLoadTrait, AssetTrait};

use super::{
    image::{ImageAsset, ImageName},
    image_assets::{FromAssetServer, ImageAssets},
};

#[derive(Resource)]
pub struct GameImageAssets {
    pub cat_hand: Handle<Image>,
    pub cat_silhouette: Handle<Image>,
    pub cat_mug: Handle<Image>,
    pub cat_mug_ear: Handle<Image>,
    pub cat_mug_handle: Handle<Image>,
    pub high_score_frame: Handle<Image>,
    pub desk_bg: Handle<Image>,
    pub desk_book_bg: Handle<Image>,
    pub piece_evolve: Handle<Image>,
}

impl AssetIdCollection<Image> for GameImageAssets {
    fn assets_ids(&self) -> Vec<AssetId<Image>> {
        vec![
            self.cat_hand.id(),
            self.cat_silhouette.id(),
            self.cat_mug.id(),
            self.cat_mug_ear.id(),
            self.cat_mug_handle.id(),
            self.high_score_frame.id(),
            self.desk_bg.id(),
            self.desk_book_bg.id(),
            self.piece_evolve.id(),
        ]
    }
}

impl AssetLoadTrait<Image> for GameImageAssets {}

impl FromAssetServer for GameImageAssets {
    fn new(asset_server: &Res<AssetServer>) -> Self {
        let cat_hand = ImageAsset::asset(asset_server, &ImageName::CatHand);
        let cat_silhouette = ImageAsset::asset(asset_server, &ImageName::CatSilhouette);
        let cat_mug = ImageAsset::asset(asset_server, &ImageName::CatMug);
        let cat_mug_ear = ImageAsset::asset(asset_server, &ImageName::CatMugEar);
        let cat_mug_handle = ImageAsset::asset(asset_server, &ImageName::CatMugHandle);
        let high_score_frame = ImageAsset::asset(asset_server, &ImageName::HighScoreFrame);
        let desk_bg = ImageAsset::asset(asset_server, &ImageName::DeskBg);
        let desk_book_bg = ImageAsset::asset(asset_server, &ImageName::DeskBookBg);
        let piece_evolve = ImageAsset::asset(asset_server, &ImageName::PieceEvolve);

        Self {
            cat_hand,
            cat_silhouette,
            cat_mug,
            cat_mug_ear,
            cat_mug_handle,
            high_score_frame,
            desk_bg,
            desk_book_bg,
            piece_evolve,
        }
    }
}

impl ImageAssets<ImageName> for GameImageAssets {
    fn handle_image_from(&self, asset_name: &ImageName) -> Handle<Image> {
        let image = match asset_name {
            ImageName::CatHand => self.cat_hand.clone(),
            ImageName::CatSilhouette => self.cat_silhouette.clone(),
            ImageName::CatMug => self.cat_mug.clone(),
            ImageName::CatMugEar => self.cat_mug_ear.clone(),
            ImageName::CatMugHandle => self.cat_mug_handle.clone(),
            ImageName::HighScoreFrame => self.high_score_frame.clone(),
            ImageName::DeskBg => self.desk_bg.clone(),
            ImageName::DeskBookBg => self.desk_book_bg.clone(),
            ImageName::PieceEvolve => self.piece_evolve.clone(),
        };

        return image;
    }
}
