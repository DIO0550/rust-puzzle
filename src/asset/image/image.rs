use crate::asset::asset::{AssetNameTrait, AssetTrait};

pub enum PieceImageName {
    Cat,
    Dog,
    Penguin,
}

impl AssetNameTrait for PieceImageName {
    fn asset_path(&self) -> String {
        match self {
            Self::Cat => "img/Cat.png".to_string(),
            Self::Dog => "img/Dog.png".to_string(),
            Self::Penguin => "img/Penguin.png".to_string(),
        }
    }
}

pub struct PieceImageAsset {}
impl AssetTrait<PieceImageName> for PieceImageAsset {}
