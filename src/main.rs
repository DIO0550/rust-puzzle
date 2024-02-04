use std::f64::consts::PI;

use bevy::{
    ecs::{entity, query, system::Insert},
    prelude::*,
    render::color,
    sprite::MaterialMesh2dBundle,
    transform,
};
use bevy_rapier2d::{
    dynamics::{Ccd, GravityScale, RigidBody, Sleeping, Velocity},
    geometry::{ActiveEvents, Collider, Restitution},
    pipeline::{CollisionEvent, ContactForceEvent},
    plugin::{NoUserData, RapierPhysicsPlugin},
    render::RapierDebugRenderPlugin,
};
use piece::{
    animal_piece::{
        self,
        animal_piece::{AnimalPiece, AnimalPieceComponent, Grab, PieceType},
    },
    piece_factory::{Factory, PieceFactory},
};
mod coordinate;
mod piece;
mod resource;
use rand::prelude::*;

const UNIT_WIDTH: f32 = 3.0;
const UNIT_HEIGHT: f32 = 5.0;

const X_LENGTH: f32 = 300.0;
const Y_LENGTH: f32 = 150.0;
const SCREEN_WIDTH: f32 = 1200.0;
const SCREEN_HEIGHT: f32 = 900.0;

const BOX_SIZE_HEIHT: f32 = SCREEN_HEIGHT / 3.0;
const BOX_SIZE_WIDTH: f32 = SCREEN_WIDTH / 4.0;
const BOX_THICKNESS: f32 = 5.0;
const BOX_MARGIN_BOTTOM: f32 = BOX_SIZE_HEIHT / 10.0;

const BACKGROUND_COLOR: Color = Color::rgb(0.9, 0.9, 0.9);

const PIECE_SPEED: f32 = 500.0;

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
        // .add_systems(Update, position_transform)
        .add_systems(Startup, spawn_piece)
        .add_plugins(RapierPhysicsPlugin::<NoUserData>::pixels_per_meter(100.0))
        .add_plugins(RapierDebugRenderPlugin::default())
        .add_systems(Startup, setup_physics)
        .add_systems(
            FixedUpdate,
            (move_piece).chain(),
            // (hold_out_piece),
        )
        .add_systems(Update, (release_piece, collision_events))
        .run();
}

fn setup(mut commands: Commands) {
    commands.spawn(Camera2dBundle::default());

    // commands
    //     .spawn(SpriteBundle {
    //         sprite: Sprite {
    //             color: CUE_COLOR,
    //             custom_size: Some(CUE_SIZE),
    //             ..default()
    //         },
    //         ..default()
    //     })
    //     .insert(coordinate::position::Position { x: 1, y: 5 });

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

fn spawn_piece(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<ColorMaterial>>,
) {
    let mut rng = rand::thread_rng();
    let rnd: usize = rng.gen();
    let piece_type = PieceType::new(&rnd);
    let piece = AnimalPieceComponent {
        animal_piece: PieceFactory::create_piece(&piece_type),
    };
    let size = piece.animal_piece.get_size().to_f32();

    let color: Color = match piece_type {
        PieceType::Cat => Color::YELLOW,
        PieceType::Dog => Color::RED,
        PieceType::Elephant => Color::GREEN,
        PieceType::Giraffe => Color::AQUAMARINE,
        PieceType::Horse => Color::BEIGE,
        PieceType::Panda => Color::BISQUE,
        PieceType::Penguin => Color::BLACK,
        PieceType::Rat => Color::BLUE,
    };

    commands
        .spawn(Grab::new(true))
        .insert(piece)
        .insert(MaterialMesh2dBundle {
            mesh: meshes
                .add(shape::Circle::new(size * 2.0 * UNIT_WIDTH).into())
                .into(),
            material: materials.add(ColorMaterial::from(color)),
            // transform: Transform::from_translation(Vec3::new(-150., 0., 0.)),
            ..default()
        })
        .insert(TransformBundle::from(Transform::from_xyz(
            0.0,
            BOX_SIZE_HEIHT * 2.0 / 3.0,
            0.0,
        )));
}

fn setup_physics(mut commands: Commands) {
    /* Create the ground. */

    commands.spawn(Collider::compound(vec![
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
    ]));
}

fn move_piece(
    keyboard_input: Res<Input<KeyCode>>,
    mut query: Query<&mut Transform, (With<AnimalPieceComponent>, With<Grab>)>,
    time: Res<Time>,
) {
    let mut transform = query.single_mut();
    let mut direction = 0.0;

    if keyboard_input.pressed(KeyCode::Left) {
        direction -= 1.0;
    }

    if keyboard_input.pressed(KeyCode::Right) {
        direction += 1.0;
    }

    let new_paddle_position =
        transform.translation.x + direction * PIECE_SPEED * time.delta_seconds();

    transform.translation.x = new_paddle_position;
}

fn collision_events(
    mut commands: Commands,
    mut collision_events: EventReader<CollisionEvent>,
    query: Query<(&AnimalPieceComponent, &Transform)>,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<ColorMaterial>>,
) {
    for collision_event in collision_events.read() {
        let entities = match collision_event {
            CollisionEvent::Started(entity1, entity2, _) => (entity1, entity2),
            CollisionEvent::Stopped(entity1, entity2, _) => (entity1, entity2),
        };

        let Ok((entity1, transform1)) = query.get(*entities.0) else {
            return;
        };

        let Ok((entity2, transform2)) = query.get(*entities.1) else {
            return;
        };

        if entity1.animal_piece.get_piece_type() != entity2.animal_piece.get_piece_type() {
            return;
        }

        commands.entity(*entities.0).despawn();
        commands.entity(*entities.1).despawn();

        let Some(evo_type) = entity1.animal_piece.get_piece_type().turn() else {
            return;
        };
        let piece = AnimalPieceComponent {
            animal_piece: PieceFactory::create_piece(&evo_type),
        };
        let size = piece.animal_piece.get_size().to_f32();

        let color: Color = match evo_type {
            PieceType::Cat => Color::YELLOW,
            PieceType::Dog => Color::RED,
            PieceType::Elephant => Color::GREEN,
            PieceType::Giraffe => Color::AQUAMARINE,
            PieceType::Horse => Color::BEIGE,
            PieceType::Panda => Color::BISQUE,
            PieceType::Penguin => Color::BLACK,
            PieceType::Rat => Color::BLUE,
        };

        let positionX = (transform1.translation.x + transform2.translation.x) / 2.0;
        let positionY = (transform1.translation.y + transform2.translation.y) / 2.0;

        commands
            .spawn(piece)
            .insert(MaterialMesh2dBundle {
                mesh: meshes
                    .add(shape::Circle::new(size * 2.0 * UNIT_WIDTH).into())
                    .into(),
                material: materials.add(ColorMaterial::from(color)),
                // transform: Transform::from_translation(Vec3::new(-150., 0., 0.)),
                ..default()
            })
            .insert(TransformBundle::from(Transform::from_xyz(
                positionX, positionY, 0.0,
            )))
            .insert(RigidBody::Dynamic)
            .insert(Collider::ball(size * 2.0 * UNIT_WIDTH))
            .insert(Restitution::coefficient(0.7))
            .insert(ActiveEvents::COLLISION_EVENTS);
    }
}

fn release_piece(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<ColorMaterial>>,
    keyboard_input: Res<Input<KeyCode>>,
    mut query: Query<(Entity, &AnimalPieceComponent), With<Grab>>,
) {
    let (entity, piece) = query.single_mut();
    if keyboard_input.just_released(KeyCode::Space) {
        commands.entity(entity).remove::<Grab>();
        commands
            .entity(entity)
            .insert(RigidBody::Dynamic)
            .insert(Collider::ball(
                piece.animal_piece.get_size().to_f32() * 2.0 * UNIT_WIDTH,
            ))
            .insert(Restitution::coefficient(0.7))
            .insert(ActiveEvents::COLLISION_EVENTS);

        spawn_piece(commands, meshes, materials);
    }
}
