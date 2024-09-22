use asset::{
    asset::AssetTrait,
    image::image::{ImageAsset, ImageName},
};
use bevy::{
    prelude::*,
    sprite::{MaterialMesh2dBundle, Mesh2dHandle},
};
use bevy_rapier2d::{
    dynamics::{GravityScale, RigidBody, Sleeping, Velocity},
    geometry::{ActiveCollisionTypes, ActiveEvents, Collider, ColliderMassProperties, Sensor},
    pipeline::{CollisionEvent, ContactForceEvent},
    plugin::{NoUserData, RapierPhysicsPlugin},
    rapier::prelude::ColliderBuilder,
    render::RapierDebugRenderPlugin,
};

use consts::consts::*;
use game::{
    component::game_over_sensor::GameOverSeonsor, plugin::game_plugin::GamePlugin,
    system::game_state::GameState, ui::evolve_ui::evolve_describe,
};
use game_over::plugin::game_over_plugin::GameOverPlugin;
use high_score::plugin::high_score_plugin::HighScorePlugin;
use piece::plugin::piece_plugin::PiecePlugin;
use resource::grab_postion::GrabPostion;
use score::plugin::score_plugin::ScorePlugin;
use shape::{Box, Circle, Quad};

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
        .add_plugins(RapierPhysicsPlugin::<NoUserData>::pixels_per_meter(300.0))
        .add_plugins(RapierDebugRenderPlugin::default())
        .add_plugins(ScorePlugin)
        .add_plugins(PiecePlugin)
        .add_plugins(GamePlugin)
        .add_plugins(GameOverPlugin)
        .add_plugins(HighScorePlugin)
        .insert_resource(ClearColor(BACKGROUND_COLOR))
        .insert_resource(GrabPostion { x: 0.0 })
        .add_systems(Startup, setup)
        .add_systems(Startup, (setup_physics, setup_cat_mug))
        .add_systems(Startup, evolve_describe)
        .run();
}

fn setup(mut commands: Commands) {
    commands.spawn(Camera2dBundle::default());
}

fn setup_physics(mut commands: Commands) {
    let collider = Collider::compound(vec![
        // 左
        (
            Vec2 {
                x: -BOX_SIZE_WIDTH,
                y: -BOX_SIZE_HEIHT / 2.0 + BOX_MARGIN_BOTTOM,
            },
            0.0,
            Collider::cuboid(BOX_THICKNESS, BOX_SIZE_HEIHT),
        ),
        // 真ん中
        (
            Vec2 {
                x: 0.0,
                y: -BOX_SIZE_HEIHT * 3.0 / 2.0 + BOX_MARGIN_BOTTOM,
            },
            0.0,
            Collider::cuboid(BOX_SIZE_WIDTH + BOX_THICKNESS, BOX_THICKNESS),
        ),
        // 下
        (
            Vec2 {
                x: BOX_SIZE_WIDTH,
                y: -BOX_SIZE_HEIHT / 2.0 + BOX_MARGIN_BOTTOM,
            },
            0.0,
            Collider::cuboid(BOX_THICKNESS, BOX_SIZE_HEIHT),
        ),
    ]);
    // 入れ物生成
    commands
        .spawn(collider)
        .insert(ActiveEvents::COLLISION_EVENTS);

    // ゲームオーバー用のセンサー生成
    commands
        .spawn(Collider::cuboid(
            BOX_SIZE_WIDTH + BOX_THICKNESS,
            BOX_THICKNESS,
        ))
        .insert(TransformBundle::from(Transform::from_xyz(
            0.0,
            BOX_SIZE_HEIHT / 2.0 + BOX_MARGIN_BOTTOM,
            0.0,
        )))
        .insert(GameOverSeonsor)
        .insert(Sensor);
}

fn setup_cat_mug(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<ColorMaterial>>,
    asset_server: Res<AssetServer>,
) {
    let image = ImageAsset::asset(&asset_server, &ImageName::CatMug);
    let material = MaterialMesh2dBundle {
        mesh: Mesh2dHandle(
            meshes.add(
                Quad::new(Vec2 {
                    x: (BOX_SIZE_WIDTH * 3.5 - BOX_THICKNESS * 4.0),
                    y: (BOX_SIZE_HEIHT * 2.5 - 5.0),
                })
                .into(),
            ),
        ),
        transform: Transform {
            translation: Vec3 {
                x: (55.0),
                y: (-BOX_MARGIN_BOTTOM * 2.0 - 17.0),
                z: (-1.00),
            },
            ..default()
        },
        material: materials.add(image.into()),
        ..default()
    };

    commands.spawn(material);
}
