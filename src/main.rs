use bevy::{prelude::*, sprite::MaterialMesh2dBundle};
use bevy_rapier2d::{
    dynamics::{GravityScale, RigidBody, Sleeping, Velocity},
    geometry::{ActiveCollisionTypes, ActiveEvents, Collider, ColliderMassProperties, Sensor},
    pipeline::{CollisionEvent, ContactForceEvent},
    plugin::{NoUserData, RapierPhysicsPlugin},
    render::RapierDebugRenderPlugin,
};

use consts::consts::*;
// use game::component::game_over_sensor::GameOverSeonsor;
// use game::{
//     component::game_over_sensor::GameOverSeonsor,
//     system::game_over_system::*,
// };
use piece::{
    component::{
        animal_piece::{animal_piece_component::AnimalPieceComponent, piece_image::PieceImage},
        falling::Falling,
        grab::Grab,
    },
    system::piece_system::move_piece,
};
use resource::{grab_postion::GrabPostion, puzzle_score::PuzzleScore};
use score::{plugin::score_plugin::ScorePlugin, resource::score::Score};

mod asset;
mod consts;
// mod game;
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
        .insert_resource(ClearColor(BACKGROUND_COLOR))
        .insert_resource(GrabPostion { x: 0.0 })
        .insert_resource(PuzzleScore(0))
        .add_systems(Startup, setup)
        .add_systems(Startup, spawn_piece_system)
        .add_systems(Startup, setup_physics)
        // .add_systems(FixedUpdate, game_over_sensor_intersection_events)
        .add_systems(FixedUpdate, (move_piece).chain())
        .add_systems(Update, (release_piece, collision_events))
        .run();
}

fn setup(mut commands: Commands) {
    commands.spawn(Camera2dBundle::default());
}

/**
 * ピース生成
 */
fn spawn_piece(
    commands: &mut Commands,
    resource: &mut Res<GrabPostion>,
    meshes: &mut ResMut<Assets<Mesh>>,
    materials: &mut ResMut<Assets<ColorMaterial>>,
    asset_server: &Res<AssetServer>,
) {
    let piece = AnimalPieceComponent::spawn();
    let size = piece.animal_piece.get_size().to_f32();
    let image = PieceImage::from_piece_type(asset_server, &piece.animal_piece.get_piece_type());

    commands
        .spawn(Grab)
        .insert(piece)
        .insert(MaterialMesh2dBundle {
            mesh: meshes.add(Circle::new(size * 2.0 * UNIT_WIDTH)).into(),
            material: materials.add(image),
            ..default()
        })
        .insert(ActiveEvents::COLLISION_EVENTS)
        .insert(ActiveCollisionTypes::all())
        .insert(TransformBundle::from(Transform::from_xyz(
            resource.x,
            BOX_SIZE_HEIHT * 2.0 / 3.0,
            0.0,
        )));
}

fn spawn_piece_system(
    mut commands: Commands,
    mut resource: Res<GrabPostion>,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<ColorMaterial>>,
    asset_server: Res<AssetServer>,
) {
    spawn_piece(
        &mut commands,
        &mut resource,
        &mut meshes,
        &mut materials,
        &asset_server,
    )
}

pub fn release_piece(
    mut commands: Commands,
    keyboard_input: Res<ButtonInput<KeyCode>>,
    mut query: Query<(Entity, &AnimalPieceComponent), With<Grab>>,
) {
    let Ok((entity, piece)) = query.get_single_mut() else {
        return;
    };

    if keyboard_input.just_released(KeyCode::Space) {
        commands.entity(entity).remove::<Grab>();
        commands
            .entity(entity)
            .insert(RigidBody::Dynamic)
            .insert(Collider::ball(
                piece.animal_piece.get_size().to_f32() * 2.0 * UNIT_WIDTH,
            ))
            .insert(ActiveEvents::COLLISION_EVENTS)
            .insert(ColliderMassProperties::Mass(50.0))
            .insert(GravityScale(10.0))
            .insert(Sleeping::disabled())
            .insert(Falling);
    }
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
        // .insert(GameOverSeonsor)
        .insert(Sensor);

    let piece = AnimalPieceComponent::spawn();
    commands
        .spawn(piece)
        .insert(TransformBundle::from(Transform::from_xyz(0.0, 0.0, 0.0)))
        .insert(RigidBody::Dynamic)
        .insert(Collider::ball(4.0 * 2.0 * UNIT_WIDTH))
        .insert(ColliderMassProperties::Mass(50.0))
        .insert(GravityScale(10.0))
        .insert(ActiveEvents::COLLISION_EVENTS)
        .insert(Velocity {
            linvel: Vec2::new(0.0, 0.0),
            angvel: 0.0,
        });
}

/**
 * 衝突イベント
 */
fn collision_events(
    mut commands: Commands,
    mut collision_events: EventReader<CollisionEvent>,
    piece_query: Query<(&AnimalPieceComponent, &Transform)>,
    falling_query: Query<&Falling>,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<ColorMaterial>>,
    mut grab_postion: Res<GrabPostion>,
    score_res: Res<Score>,
    asset_server: Res<AssetServer>,
) {
    for collision_event in collision_events.read() {
        println!("collision_event");
        let entities = match collision_event {
            CollisionEvent::Started(entity1, entity2, _) => (entity1, entity2),
            CollisionEvent::Stopped(entity1, entity2, _) => (entity1, entity2),
        };

        if falling_query.get(*entities.0).is_ok() {
            println!("remove falling");
            commands.entity(*entities.0).remove::<Falling>();
            spawn_piece(
                &mut commands,
                &mut grab_postion,
                &mut meshes,
                &mut materials,
                &asset_server,
            );
        };

        if falling_query.get(*entities.1).is_ok() {
            println!("remove falling");
            commands.entity(*entities.1).remove::<Falling>();
            spawn_piece(
                &mut commands,
                &mut grab_postion,
                &mut meshes,
                &mut materials,
                &asset_server,
            );
        };

        let Ok((entity1, transform1)) = piece_query.get(*entities.0) else {
            // println!("not animal piece entity 0");

            return;
        };

        let Ok((entity2, transform2)) = piece_query.get(*entities.1) else {
            // println!("not animal piece entity 1");
            return;
        };

        if entity1.animal_piece.get_piece_type() != entity2.animal_piece.get_piece_type() {
            println!("not same type!");
            println!("entity1 : {:?}", entity1.animal_piece.get_piece_type());
            println!("entity2 : {:?}", entity2.animal_piece.get_piece_type());

            return;
        }

        commands.entity(*entities.0).despawn();
        commands.entity(*entities.1).despawn();

        let Some(piece) = entity1.evolve() else {
            return;
        };
        let new_score = score_res.0 + entity1.animal_piece.get_score().to_u32();
        commands.insert_resource(Score(new_score));

        let size = piece.animal_piece.get_size().to_f32();
        let image = PieceImage::from_piece_type(&asset_server, piece.animal_piece.get_piece_type());
        let position_x = (transform1.translation.x + transform2.translation.x) / 2.0;
        let position_y = (transform1.translation.y + transform2.translation.y) / 2.0;

        commands
            .spawn(piece)
            .insert(MaterialMesh2dBundle {
                mesh: meshes.add(Circle::new(size * 2.0 * UNIT_WIDTH)).into(),
                material: materials.add(image),
                ..default()
            })
            .insert(TransformBundle::from(Transform::from_xyz(
                position_x, position_y, 0.0,
            )))
            .insert(RigidBody::Dynamic)
            .insert(Collider::ball(size * 2.0 * UNIT_WIDTH))
            .insert(ColliderMassProperties::Mass(50.0))
            .insert(GravityScale(10.0))
            .insert(ActiveEvents::COLLISION_EVENTS)
            .insert(Velocity {
                linvel: Vec2::new(0.0, 0.0),
                angvel: 0.0,
            })
            .insert(Sleeping::disabled());
    }
}
