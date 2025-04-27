use bevy::ecs::system::{Commands, Res};

use crate::{
    asset::image::game_image_assets::GameImageAssets,
    field::game::material::{
        cat_mug_ear_sprite::CatMugEarSprite, cat_mug_handle_sprite::CatMugHandleSprite,
        cat_mug_sprite::CatMugSprite,
    },
};

pub fn setup_cat_mug(mut commands: Commands, game_image_assets: Res<GameImageAssets>) {
    CatMugEarSprite::spawn(&mut commands, &game_image_assets);
    CatMugHandleSprite::spawn(&mut commands, &game_image_assets);
    CatMugSprite::spawn(&mut commands, &game_image_assets);
}
