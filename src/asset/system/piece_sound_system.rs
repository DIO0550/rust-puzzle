use bevy::{
    asset::AssetServer,
    prelude::{Commands, Res},
};

use crate::asset::{
    asset::AssetTrait,
    resource::piece_sound::{PieceFallSound, PieceUnionSound},
    sound::sound::{PieceSoundAsset, PieceSoundName},
};

pub fn setup_piece_sound(mut commands: Commands, asset_server: Res<AssetServer>) {
    // 落下音
    let fall_sound = PieceSoundAsset::asset(&asset_server, &PieceSoundName::Fall);
    commands.insert_resource(PieceFallSound(fall_sound));

    // ピース結合
    let union_sound = PieceSoundAsset::asset(&asset_server, &PieceSoundName::Union);
    commands.insert_resource(PieceUnionSound(union_sound));
}
