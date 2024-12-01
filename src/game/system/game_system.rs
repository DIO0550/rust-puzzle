use bevy::{
    asset::AssetServer,
    math::{Vec2, Vec3},
    prelude::{default, Commands, Res, ResMut, Transform, TransformBundle},
    sprite::{Sprite, SpriteBundle},
};
use bevy_rapier2d::{
    plugin::RapierConfiguration,
    prelude::{ActiveEvents, Collider, Sensor},
};

use crate::{
    asset::{
        asset::AssetTrait,
        image::image::{ImageAsset, ImageName},
    },
    game::component::game_over_sensor::GameOverSensor,
    score::resource::score::Score,
    BOX_SIZE_HEIHT, BOX_SIZE_WIDTH, BOX_THICKNESS,
};

pub fn restart(mut commands: Commands, mut config: ResMut<RapierConfiguration>) {
    commands.insert_resource(Score(0));
    config.physics_pipeline_active = true;
}

pub fn setup_cat_mug(mut commands: Commands, asset_server: Res<AssetServer>) {
    let cat_mug_image: bevy::prelude::Handle<bevy::prelude::Image> =
        ImageAsset::asset(&asset_server, &ImageName::CatMug);
    let cat_mug_bundle = SpriteBundle {
        sprite: Sprite {
            custom_size: Some(Vec2 {
                x: BOX_SIZE_WIDTH + BOX_THICKNESS * 2.0,
                y: BOX_SIZE_HEIHT + BOX_THICKNESS * 2.0,
            }),
            ..default()
        },
        texture: cat_mug_image,
        transform: Transform {
            translation: Vec3 {
                x: (0.0),
                y: (0.0),
                z: (-1.0),
            },
            ..default()
        },

        ..default()
    };

    let cat_mug_ear_image = ImageAsset::asset(&asset_server, &ImageName::CatMugEar);
    let cat_mug_ear_bundle = SpriteBundle {
        sprite: Sprite {
            custom_size: Some(Vec2 {
                x: (BOX_SIZE_WIDTH + BOX_THICKNESS) * 3.0 / 4.0,
                y: BOX_SIZE_HEIHT / 5.0,
            }),
            ..default()
        },
        texture: cat_mug_ear_image,
        transform: Transform {
            translation: Vec3 {
                x: (0.0),
                y: (BOX_SIZE_HEIHT / 2.0 + BOX_SIZE_HEIHT / 10.0 - BOX_THICKNESS * 2.0),
                z: (-2.0),
            },
            ..default()
        },

        ..default()
    };

    let cat_mug_handle_image = ImageAsset::asset(&asset_server, &ImageName::CatMugHandle);
    let cat_mug_handle_image_width = BOX_SIZE_WIDTH / 4.0;
    let cat_mug_handle_bundle = SpriteBundle {
        sprite: Sprite {
            custom_size: Some(Vec2 {
                x: cat_mug_handle_image_width,
                y: BOX_SIZE_HEIHT,
            }),
            ..default()
        },
        texture: cat_mug_handle_image,
        transform: Transform {
            translation: Vec3 {
                x: (BOX_SIZE_WIDTH / 2.0 + (cat_mug_handle_image_width / 2.0) - BOX_THICKNESS),
                y: (0.0),
                z: (5.0),
            },
            ..default()
        },

        ..default()
    };

    let collider = Collider::compound(vec![
        // 左
        (
            Vec2 {
                x: -BOX_SIZE_WIDTH / 2.0,
                y: 0.0,
            },
            0.0,
            Collider::cuboid(BOX_THICKNESS, BOX_SIZE_HEIHT / 2.0),
        ),
        // 真ん中
        (
            Vec2 {
                x: 0.0,
                y: -BOX_SIZE_HEIHT / 2.0,
            },
            0.0,
            Collider::cuboid(BOX_SIZE_WIDTH / 2.0 + BOX_THICKNESS, BOX_THICKNESS),
        ),
        // 右
        (
            Vec2 {
                x: BOX_SIZE_WIDTH / 2.0,
                y: 0.0,
            },
            0.0,
            Collider::cuboid(BOX_THICKNESS, BOX_SIZE_HEIHT / 2.0),
        ),
    ]);
    // 入れ物生成
    commands
        .spawn(collider)
        .insert(ActiveEvents::COLLISION_EVENTS)
        .insert(cat_mug_bundle);
    commands.spawn(cat_mug_ear_bundle);
    commands.spawn(cat_mug_handle_bundle);
}

pub fn setup_gameover_sensor(mut commands: Commands) {
    // ゲームオーバー用のセンサー生成
    commands
        .spawn(Collider::cuboid(BOX_SIZE_WIDTH / 2.0, BOX_THICKNESS))
        .insert(TransformBundle::from(Transform::from_xyz(
            0.0,
            BOX_SIZE_HEIHT / 2.0,
            0.0,
        )))
        .insert(GameOverSensor)
        .insert(Sensor);
}
