use crate::asset::asset::{AssetNameTrait, AssetTrait};

pub enum PieceImageName {
    Rat,
    Cat,
    Dog,
    Giraffe,
    Horse,
    Penguin,
    Panda,
    Elephant,
}

impl AssetNameTrait for PieceImageName {
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
impl AssetTrait<PieceImageName> for PieceImageAsset {}
