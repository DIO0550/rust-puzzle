use crate::asset::asset::{AssetNameTrait, AssetTrait};

pub enum PieceSoundName {
    Fall,
}

impl AssetNameTrait for PieceSoundName {
    fn asset_path(&self) -> String {
        match self {
            Self::Fall => "sound/piece-fall.mp3".to_string(),
        }
    }
}

pub struct PieceSoundAsset {}
impl AssetTrait<PieceSoundName> for PieceSoundAsset {}
