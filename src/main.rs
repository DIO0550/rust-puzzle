use asset::{
    asset::AssetTrait,
    image::image::{ImageAsset, ImageName},
    plugin::piece_sound_plugin::PieceSoundPlugin,
};
use bevy::{
    math::{Affine3A, Vec3A},
    prelude::*,
    sprite::{MaterialMesh2dBundle, Mesh2dHandle},
};
use bevy_rapier2d::{
    geometry::{ActiveEvents, Collider, Sensor},
    plugin::{NoUserData, RapierPhysicsPlugin},
    render::RapierDebugRenderPlugin,
};

use consts::consts::*;
use game::{
    component::game_over_sensor::GameOverSeonsor, plugin::game_plugin::GamePlugin,
    ui::evolve_ui::evolve_describe,
};
use game_over::plugin::game_over_plugin::GameOverPlugin;
use high_score::plugin::high_score_plugin::HighScorePlugin;
use piece::plugin::piece_plugin::PiecePlugin;
use resource::grab_postion::GrabPostion;
use score::plugin::score_plugin::ScorePlugin;
use shape::Quad;

mod asset;
mod consts;
mod file;
mod game;
mod game_over;
mod high_score;
mod piece;
mod resource;
mod score;

fn main() {
    let window = Window {
        title: "puzzle".to_string(),
        resolution: (SCREEN_WIDTH, SCREEN_HEIGHT).into(),
        resizable: false,
        ..default()
    };
    let primary_window = Some(window);
    App::new()
        .add_plugins(DefaultPlugins.set(WindowPlugin {
            primary_window,
            ..default()
        }))
        .add_plugins(GamePlugin)
        .add_plugins(RapierPhysicsPlugin::<NoUserData>::pixels_per_meter(300.0))
        .add_plugins(RapierDebugRenderPlugin::default())
        .add_plugins(ScorePlugin)
        .add_plugins(GameOverPlugin)
        .add_plugins(HighScorePlugin)
        .add_plugins(PieceSoundPlugin)
        .add_plugins(PiecePlugin)
        .insert_resource(ClearColor(BACKGROUND_COLOR))
        .insert_resource(GrabPostion { x: 0.0 })
        .add_systems(Startup, setup)
        .add_systems(Startup, setup_physics)
        .run();
}

fn setup(mut commands: Commands) {
    commands.spawn(Camera2dBundle::default());
}

fn setup_physics(mut commands: Commands, asset_server: Res<AssetServer>) {
    let cat_mug_image = ImageAsset::asset(&asset_server, &ImageName::CatMug);
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

    let cat_mug_ear = ImageAsset::asset(&asset_server, &ImageName::CatMugEar);
    let cat_mug_ear_bundle = SpriteBundle {
        sprite: Sprite {
            custom_size: Some(Vec2 {
                x: (BOX_SIZE_WIDTH + BOX_THICKNESS) * 3.0 / 4.0,
                y: BOX_SIZE_HEIHT / 5.0,
            }),
            ..default()
        },
        texture: cat_mug_ear,
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

    // ゲームオーバー用のセンサー生成
    commands
        .spawn(Collider::cuboid(BOX_SIZE_WIDTH / 2.0, BOX_THICKNESS))
        .insert(TransformBundle::from(Transform::from_xyz(
            0.0,
            BOX_SIZE_HEIHT / 2.0,
            0.0,
        )))
        .insert(GameOverSeonsor)
        .insert(Sensor);
}
