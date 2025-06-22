use bevy::asset::AssetServer;
use bevy::prelude::{Commands, Res};

use crate::asset::sound::piece_sound_assets::PieceSoundAssets;
use crate::asset::sound::sound_assets::FromAssetServer;

pub fn load_sound_assets(commands: &mut Commands, asset_server: &Res<AssetServer>) {
    let piece_sound_assets = PieceSoundAssets::new(asset_server);

    commands.insert_resource(piece_sound_assets);
}
