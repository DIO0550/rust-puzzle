use std::f64::consts::PI;

use bevy::{prelude::*, render::color};
use bevy_rapier2d::{
    dynamics::{Ccd, GravityScale, RigidBody, Sleeping, Velocity},
    geometry::{Collider, Restitution},
    plugin::{NoUserData, RapierPhysicsPlugin},
    render::RapierDebugRenderPlugin,
};
use resource::material::Materials;
mod coordinate;
mod piece;
mod resource;
use rand::prelude::*;

const UNIT_WIDTH: f32 = 5.0;
const UNIT_HEIGHT: f32 = 5.0;

const X_LENGTH: f32 = 100.0;
const Y_LENGTH: f32 = 130.0;

const SCREEN_WIDTH: f32 = UNIT_WIDTH * X_LENGTH;
const SCREEN_HEIGHT: f32 = UNIT_HEIGHT * Y_LENGTH;

const BACKGROUND_COLOR: Color = Color::rgb(0.9, 0.9, 0.9);
const CUE_SIZE: Vec2 = Vec2::new(50.0, 50.0);
const CUE_COLOR: Color = Color::rgb(0.4, 0.4, 0.4);

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
        .insert_resource(ClearColor(BACKGROUND_COLOR))
        .add_systems(Startup, setup)
        .add_systems(Update, position_transform)
        .add_systems(Update, spawn_piece)
        .add_plugins(RapierPhysicsPlugin::<NoUserData>::pixels_per_meter(100.0))
        .add_plugins(RapierDebugRenderPlugin::default())
        .add_systems(Startup, setup_physics)
        .add_systems(Update, print_ball_altitude)
        .run();
}

fn setup(mut commands: Commands) {
    commands.spawn(Camera2dBundle::default());

    commands
        .spawn(SpriteBundle {
            sprite: Sprite {
                color: CUE_COLOR,
                custom_size: Some(CUE_SIZE),
                ..default()
            },
            ..default()
        })
        .insert(coordinate::position::Position { x: 1, y: 5 });

    commands.insert_resource(resource::material::Materials {
        colors: vec![
            Color::rgb_u8(64, 230, 100).into(),
            Color::rgb_u8(220, 64, 90).into(),
            Color::rgb_u8(70, 150, 210).into(),
            Color::rgb_u8(220, 230, 70).into(),
            Color::rgb_u8(35, 220, 241).into(),
            Color::rgb_u8(240, 140, 70).into(),
        ],
    });
}

fn position_transform(
    mut position_query: Query<(&coordinate::position::Position, &mut Transform, &mut Sprite)>,
) {
    let origin_x = UNIT_WIDTH as i32 / 2 - SCREEN_WIDTH as i32 / 2;
    let origin_y = UNIT_HEIGHT as i32 / 2 - SCREEN_HEIGHT as i32 / 2;
    position_query
        .iter_mut()
        .for_each(|(pos, mut transform, mut sprite)| {
            transform.translation = Vec3::new(
                (origin_x + pos.x as i32 * UNIT_WIDTH as i32) as f32,
                (origin_y + pos.y as i32 * UNIT_HEIGHT as i32) as f32,
                0.0,
            );
            sprite.custom_size = Some(Vec2::new(UNIT_WIDTH as f32, UNIT_HEIGHT as f32))
        });
}

fn spawn_piece(mut commands: Commands, materials: Res<Materials>) {
    let mut rng = rand::thread_rng();
    let mut color_index: usize = rng.gen();
    color_index %= materials.colors.len();
    commands
        .spawn(SpriteBundle {
            sprite: Sprite {
                color: materials.colors[color_index].color.clone(),
                ..Default::default()
            },
            ..Default::default()
        })
        .insert(coordinate::position::Position { x: 0, y: 0 });
}

fn setup_physics(mut commands: Commands) {
    /* Create the ground. */
    commands.spawn(Collider::compound(vec![
        (
            Vec2 {
                x: -100.0,
                y: 100.0,
            },
            0.0,
            Collider::cuboid(10.0, 100.0),
        ),
        (Vec2 { x: 0.0, y: 0.0 }, 0.0, Collider::cuboid(100.0, 10.0)),
        (
            Vec2 { x: 100.0, y: 100.0 },
            0.0,
            Collider::cuboid(10.0, 100.0),
        ),
    ]));
    //.insert(TransformBundle::from(Transform::from_xyz(0.0, -100.0, 0.0)));

    commands
        .spawn(Collider::ball(UNIT_HEIGHT * 2.0))
        .insert(RigidBody::Fixed)
        .insert(TransformBundle::from(Transform::from_xyz(0.0, 200.0, 0.0)))
        .insert(Velocity {
            linvel: Vec2::new(1.0, 2.0),
            angvel: 0.2,
        })
        .insert(GravityScale(0.5))
        .insert(Sleeping::disabled())
        .insert(Ccd::enabled());

    /* Create the bouncing ball. */
    commands
        .spawn(RigidBody::Dynamic)
        .insert(Collider::ball(UNIT_HEIGHT * 2.0))
        .insert(Restitution::coefficient(0.7))
        .insert(TransformBundle::from(Transform::from_xyz(0.0, 400.0, 0.0)));
}

fn print_ball_altitude(positions: Query<&Transform, With<RigidBody>>) {
    for transform in positions.iter() {
        println!("Ball altitude: {}", transform.translation.y);
    }
}
