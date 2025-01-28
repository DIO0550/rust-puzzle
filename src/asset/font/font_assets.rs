use bevy::prelude::*;

use crate::asset::asset::AssetTrait;

use super::font::{FontAsset, FontName};

#[derive(Resource)]
pub struct FontAssets {
    pub hachi_maru_pop_regular: Handle<Font>,
}

impl FontAssets {
    pub fn new(asset_server: &Res<AssetServer>) -> Self {
        let hachi_maru_pop_regular = FontAsset::asset(asset_server, &FontName::HachiMaruPopReg);

        Self {
            hachi_maru_pop_regular,
        }
    }
}
