use bevy::ecs::system::{Commands, Res};

use crate::asset::image::game_image_assets::GameImageAssets;
use crate::field::game::cat_mug::ear_sprite::CatMugEarSprite;
use crate::field::game::cat_mug::handle_sprite::CatMugHandleSprite;
use crate::field::game::cat_mug::sprite::CatMugSprite;

pub mod ear_sprite;
pub mod handle_sprite;
pub mod sprite;

pub fn setup_cat_mug(mut commands: Commands, game_image_assets: Res<GameImageAssets>) {
    CatMugEarSprite::spawn(&mut commands, &game_image_assets);
    CatMugHandleSprite::spawn(&mut commands, &game_image_assets);
    CatMugSprite::spawn(&mut commands, &game_image_assets);
}
