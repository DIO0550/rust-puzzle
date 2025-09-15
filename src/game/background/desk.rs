use bevy::{
    asset::AssetServer,
    math::{Vec2, Vec3},
    prelude::{Commands, Query, Res, Sprite, Transform, With},
    window::{PrimaryWindow, Window},
};

use crate::{
    asset::{
        asset::AssetTrait,
        image::{
            game_image_assets::GameImageAssets,
            image::{ImageAsset, ImageName},
        },
    },
    BOX_SIZE_HEIHT, BOX_SIZE_WIDTH, BOX_THICKNESS,
};

pub fn desk_background(
    mut commands: Commands,
    image_assets: Res<GameImageAssets>,
    window_query: Query<&Window, With<PrimaryWindow>>,
) {
    let Ok(window) = window_query.get_single() else {
        return;
    };
    let width = window.resolution.width() * 0.7;
    let height = width * 0.6;
    let image = image_assets.desk_bg.clone();

    commands.spawn((
        Sprite {
            custom_size: Some(Vec2::new(width, height)),
            image,
            ..Default::default()
        },
        // Sprite::from_image(image),
        Transform::from_translation(Vec3::new(
            0.0,
            -height / 2.0 - BOX_SIZE_HEIHT / 2.0 - BOX_THICKNESS,
            -100.0,
        )),
    ));
}

pub fn desk_book_background(mut commands: Commands, asset_server: Res<AssetServer>) {
    let width = 200.0;
    let height = 100.0;
    let image = ImageAsset::asset(&asset_server, &ImageName::DeskBookBg);

    commands.spawn((
        Sprite {
            custom_size: Some(Vec2::new(width, height)),
            image,
            ..Default::default()
        },
        // Sprite::from_image(image),
        Transform::from_translation(Vec3::new(
            -BOX_SIZE_WIDTH / 2.0 - width / 2.0,
            height / 2.0 - BOX_SIZE_HEIHT / 2.0 - BOX_THICKNESS,
            -100.0,
        )),
    ));
}
