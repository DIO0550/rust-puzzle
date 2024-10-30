use bevy::{
    asset::AssetServer,
    math::{Affine3A, Vec2, Vec3, Vec3A},
    prelude::{
        BuildChildren, Commands, GlobalTransform, ImageBundle, NodeBundle, Query, Res, Transform,
        With,
    },
    sprite::{Sprite, SpriteBundle},
    ui::{FlexDirection, PositionType, Style, UiImage, Val, ZIndex},
    utils::default,
    window::{PrimaryWindow, Window},
};

use crate::{
    asset::{
        asset::AssetTrait,
        image::image::{ImageAsset, ImageName},
    },
    BOX_SIZE_HEIHT,
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
                y: (-BOX_SIZE_HEIHT + 100.0),
                z: (-100.0),
            },
            ..default()
        },

        ..default()
    };
    commands.spawn(bundle);
}
