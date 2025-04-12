use std::path::Display;

use bevy::render::texture::Image;

use crate::{
    asset::asset::{AssetNameTrait, AssetTrait},
    piece::component::animal_piece::piece_type::PieceType,
};

impl AssetNameTrait for PieceType {
    fn asset_path(&self) -> String {
        match self {
            Self::Rat => "img/Rat.png".to_string(),
            Self::Cat => "img/Cat.png".to_string(),
            Self::Dog => "img/Dog.png".to_string(),
            Self::Giraffe => "img/giraffe.png".to_string(),
            Self::Horse => "img/Horse.png".to_string(),
            Self::Penguin => "img/Penguin.png".to_string(),
            Self::Panda => "img/Panda.png".to_string(),
            Self::Elephant => "img/Elephant.png".to_string(),
        }
    }
}

pub struct PieceImageAsset {}
impl AssetTrait<PieceType, Image> for PieceImageAsset {}

pub enum ImageName {
    CatHand,
    CatSilhouette,
    CatMug,
    CatMugEar,
    CatMugHandle,
    HighScoreFrame,
    DeskBg,
    DeskBookBg,
    PieceEvolve,
}
impl AssetNameTrait for ImageName {
    fn asset_path(&self) -> String {
        match self {
            Self::CatHand => "img/cat-hand.png".to_string(),
            Self::CatSilhouette => "img/cat-silhouette.png".to_string(),
            Self::CatMug => "img/cat-mug.png".to_string(),
            Self::CatMugEar => "img/cat-mug-ear.png".to_string(),
            Self::CatMugHandle => "img/cat-mug-handle.png".to_string(),
            Self::HighScoreFrame => "img/high-score-frame.png".to_string(),
            Self::DeskBg => "img/desk-bg.png".to_string(),
            Self::DeskBookBg => "img/desk-book-bg.png".to_string(),
            Self::PieceEvolve => "img/piece-evolve.png".to_string(),
        }
    }
}
pub struct ImageAsset {}
impl AssetTrait<ImageName, Image> for ImageAsset {}
