use bevy::{prelude::*, render::mesh::shape::Circle, sprite::MaterialMesh2dBundle};
use bevy_rapier2d::{
    dynamics::{GravityScale, RigidBody, Sleeping, Velocity},
    geometry::{ActiveCollisionTypes, ActiveEvents, Collider, ColliderMassProperties, Sensor},
    pipeline::{CollisionEvent, ContactForceEvent},
    plugin::{NoUserData, RapierPhysicsPlugin},
    render::RapierDebugRenderPlugin,
};

use consts::consts::*;
use game::{
    component::game_over_sensor::GameOverSeonsor,
    system::{game_over_system::*, game_state::GameState},
};
use piece::{
    component::{
        animal_piece::{animal_piece_component::AnimalPieceComponent, piece_image::PieceImage},
        falling::Falling,
    },
    plugin::piece_plugin::PiecePlugin,
    system::piece_system::{move_piece, release_piece, spawn_piece},
};
use resource::{grab_postion::GrabPostion, puzzle_score::PuzzleScore};
use score::{plugin::score_plugin::ScorePlugin, resource::score::Score};

mod asset;
mod consts;
mod game;
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
        .add_state::<GameState>()
        .add_plugins(DefaultPlugins.set(WindowPlugin {
            primary_window,
            ..default()
        }))
        .add_plugins(RapierPhysicsPlugin::<NoUserData>::pixels_per_meter(300.0))
        .add_plugins(RapierDebugRenderPlugin::default())
        .add_plugins(ScorePlugin)
        .add_plugins(PiecePlugin)
        .insert_resource(ClearColor(BACKGROUND_COLOR))
        .insert_resource(GrabPostion { x: 0.0 })
        .insert_resource(PuzzleScore(0))
        .add_systems(Startup, setup)
        .add_systems(Startup, setup_physics)
        .add_systems(FixedUpdate, game_over_sensor_intersection_events)
        .run();
}

fn setup(mut commands: Commands) {
    commands.spawn(Camera2dBundle::default());
}

fn setup_physics(mut commands: Commands) {
    // 入れ物生成
    commands
        .spawn(Collider::compound(vec![
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
        ]))
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
