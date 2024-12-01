use bevy::{
    asset::AssetServer,
    math::{Vec2, Vec3},
    prelude::{Commands, Query, Res, Transform, With},
    sprite::{Sprite, SpriteBundle},
    utils::default,
    window::{PrimaryWindow, Window},
};

use crate::{
    asset::{
        asset::AssetTrait,
        image::image::{ImageAsset, ImageName},
    },
    BOX_SIZE_HEIHT, BOX_SIZE_WIDTH, BOX_THICKNESS,
};

pub fn desk_background(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    window_query: Query<&Window, With<PrimaryWindow>>,
) {
    let window = window_query.get_single().unwrap();
    let width = window.width() * 0.7;
    let height = width * 0.6;
    let image = ImageAsset::asset(&asset_server, &ImageName::DeskBg);
    let bundle = SpriteBundle {
        sprite: Sprite {
            custom_size: Some(Vec2 {
                x: width,
                y: height,
            }),
            ..default()
        },
        texture: image,
        transform: Transform {
            translation: Vec3 {
                x: (0.0),
                y: -height / 2.0 - BOX_SIZE_HEIHT / 2.0 - BOX_THICKNESS,
                z: (-100.0),
            },
            ..default()
        },

        ..default()
    };
    commands.spawn(bundle);
}

pub fn desk_book_background(mut commands: Commands, asset_server: Res<AssetServer>) {
    let width = 200.0;
    let height = 100.0;
    let image = ImageAsset::asset(&asset_server, &ImageName::DeskBookBg);
    let bundle = SpriteBundle {
        sprite: Sprite {
            custom_size: Some(Vec2 {
                x: width,
                y: height,
            }),
            ..default()
        },
        texture: image,
        transform: Transform {
            translation: Vec3 {
                x: (-BOX_SIZE_WIDTH / 2.0 - width / 2.0),
                y: height / 2.0 - BOX_SIZE_HEIHT / 2.0 - BOX_THICKNESS,
                z: (-100.0),
            },
            ..default()
        },

        ..default()
    };
    commands.spawn(bundle);
}
