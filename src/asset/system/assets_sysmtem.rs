use bevy::{
    asset::AssetServer,
    ecs::system::{Commands, Res},
};

use crate::asset::{
    asset::AssetLoadTrait,
    font::font_assets::FontAssets,
    image::{image_assets::ImageAssets, piece_image_assets::PieceImageAssets},
};

use super::{font_loader_system::load_font_assets, image_loader_system::load_image_assets};

pub fn load_assets(mut commands: Commands, asset_server: Res<AssetServer>) {
    load_font_assets(&mut commands, &asset_server);
    load_image_assets(&mut commands, &asset_server);
}

pub fn check_assets_ready(
    asset_server: Res<AssetServer>,
    piece_image_assets: Res<PieceImageAssets>,
    image_assets: Res<ImageAssets>,
    font_assets: Res<FontAssets>,
) {
    if (piece_image_assets.is_loaded(&asset_server)
        && image_assets.is_loaded(&asset_server)
        && font_assets.is_loaded(&asset_server))
    {
        // TODO: ここでアセットのロードが完了したことを通知する
    }
}
