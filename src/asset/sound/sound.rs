use bevy::audio::AudioSource;

use crate::asset::asset::{AssetNameTrait, AssetTrait};

pub enum PieceSoundName {
    Fall,
    Union,
}

impl AssetNameTrait for PieceSoundName {
    fn asset_path(&self) -> String {
        match self {
            Self::Fall => "sound/piece-fall.mp3".to_string(),
            Self::Union => "sound/piece-union.mp3".to_string(),
        }
    }
}

pub struct PieceSoundAsset {}
impl AssetTrait<PieceSoundName, AudioSource> for PieceSoundAsset {}
