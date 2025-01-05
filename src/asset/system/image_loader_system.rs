use bevy::asset::AssetServer;
use bevy::prelude::{Commands, Res};

use crate::asset::image::image_assets::ImageAssets;
use crate::asset::image::piece_iamge_assets::PieceIamgeAssets;

pub fn load_image_assets(mut commands: Commands, asset_server: &Res<AssetServer>) {
    let image_assets = ImageAssets::new(asset_server);
    let piece_image_assets = PieceIamgeAssets::new(asset_server);

    commands.insert_resource(image_assets);
    commands.insert_resource(piece_image_assets);
}
