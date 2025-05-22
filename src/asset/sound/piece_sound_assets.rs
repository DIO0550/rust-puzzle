use bevy::{
    asset::{AssetId, AssetServer, Handle},
    audio::AudioSource,
    ecs::system::{Res, Resource},
};

use crate::asset::asset::{AssetIdCollection, AssetLoadTrait, AssetNameTrait};

use super::sound_assets::{FromAssetServer, SoundAssets};

#[derive(Resource)]
pub struct PieceSoundAssets {
    pub piece_fall: Handle<AudioSource>,
    pub piece_union: Handle<AudioSource>,
}

impl FromAssetServer for PieceSoundAssets {
    fn new(asset_server: &Res<AssetServer>) -> Self {
        let piece_fall = Handle::<AudioSource>::from_path(asset_server, "sound/piece-fall.mp3");
        let piece_union = Handle::<AudioSource>::from_path(asset_server, "sound/piece-union.mp3");

        Self {
            piece_fall,
            piece_union,
        }
    }
}

impl AssetIdCollection<AudioSource> for PieceSoundAssets {
    fn assets_ids(&self) -> Vec<AssetId<AudioSource>> {
        vec![self.piece_fall.id(), self.piece_union.id()]
    }
}

impl AssetLoadTrait<AudioSource> for PieceSoundAssets {}

impl SoundAssets<PieceSoundName> for PieceSoundAssets {
    fn handle_sounds_from(&self, asset_name: &PieceSoundName) -> Handle<AudioSource> {
        match asset_name {
            PieceSoundName::Fall => self.piece_fall.clone(),
            PieceSoundName::Union => self.piece_union.clone(),
        }
    }
}

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
