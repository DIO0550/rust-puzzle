use bevy::prelude::*;

use crate::asset::asset::AssetTrait;

use super::image::{ImageAsset, ImageName};

#[derive(Resource)]
pub struct FontAssets {
    pub hachi_maru_pop_regular: Handle<Font>,
}
