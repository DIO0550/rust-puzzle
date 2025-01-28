use bevy::{
    asset::AssetServer,
    ecs::system::{Commands, Res},
};

use crate::asset::{font::font_assets::FontAssets, image::piece_image_assets::PieceImageAssets};

use super::{font_loader_system::load_font_assets, image_loader_system::load_image_assets};

pub fn load_assets(mut commands: Commands, asset_server: Res<AssetServer>) {
    load_font_assets(&mut commands, &asset_server);
    load_image_assets(&mut commands, &asset_server);
}

pub fn check_assets_ready(
    asset_server: Res<AssetServer>,
    image_assets: Res<PieceImageAssets>,
    font_assets: Res<FontAssets>,
) {
    // TODO
}
