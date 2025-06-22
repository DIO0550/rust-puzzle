use bevy::asset::AssetServer;
use bevy::prelude::{Commands, Res};

use crate::asset::font::font_assets::FontAssets;

pub fn load_font_assets(commands: &mut Commands, asset_server: &Res<AssetServer>) {
    let font_assets = FontAssets::new(asset_server);

    commands.insert_resource(font_assets);
}
