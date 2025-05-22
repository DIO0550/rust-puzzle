use bevy::{
    asset::AssetServer,
    ecs::{
        schedule::NextState,
        system::{Commands, Res, ResMut},
    },
};

use crate::{
    asset::{
        asset::AssetLoadTrait,
        font::font_assets::FontAssets,
        image::{game_image_assets::GameImageAssets, piece_image_assets::PieceImageAssets},
        state::asset_load_state::AssetLoadState,
    },
    game::state::game_page_state::GamePageState,
};

use super::{font_loader_system::load_font_assets, image_loader_system::load_image_assets};

pub fn load_assets(mut commands: Commands, asset_server: Res<AssetServer>) {
    load_font_assets(&mut commands, &asset_server);
    load_image_assets(&mut commands, &asset_server);
}

pub fn check_assets_ready(
    asset_server: Res<AssetServer>,
    piece_images_assets: Option<Res<PieceImageAssets>>,
    images_assets: Option<Res<GameImageAssets>>,
    fonts_assets: Option<Res<FontAssets>>,
    mut assets_load_state: ResMut<NextState<AssetLoadState>>,
    mut game_page_state: ResMut<NextState<GamePageState>>,
) {
    let all_loaded = [
        piece_images_assets
            .map(|a| a.is_loaded(&asset_server))
            .unwrap_or(false),
        images_assets
            .map(|a| a.is_loaded(&asset_server))
            .unwrap_or(false),
        fonts_assets
            .map(|a| a.is_loaded(&asset_server))
            .unwrap_or(false),
    ]
    .iter()
    .all(|loaded| *loaded);

    if !all_loaded {
        return;
    }

    assets_load_state.set(AssetLoadState::Loaded);
    game_page_state.set(GamePageState::Title);
}
