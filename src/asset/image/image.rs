use crate::asset::asset::{AssetNameTrait, AssetTrait};

pub enum PieceImageName {
    Cat,
}

impl AssetNameTrait for PieceImageName {
    fn asset_path(&self) -> String {
        match self {
            Self::Cat => "img/Cat.png".to_string(),
        }
    }
}

pub struct PieceImageAsset {}
impl AssetTrait<PieceImageName> for PieceImageAsset {}
