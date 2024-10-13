use bevy::{
    asset::AssetServer,
    prelude::{Commands, Res},
};

use crate::asset::{
    asset::AssetTrait,
    resource::piece_sound::PieceFallSound,
    sound::sound::{PieceSoundAsset, PieceSoundName},
};

pub fn setup_piece_sound(mut commands: Commands, asset_server: Res<AssetServer>) {
    let sound = PieceSoundAsset::asset(&asset_server, &PieceSoundName::Fall);
    commands.insert_resource(PieceFallSound(sound));
}
