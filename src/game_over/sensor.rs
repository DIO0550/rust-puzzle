use bevy::ecs::component::Component;
use bevy::ecs::system::Commands;
use bevy::transform::components::Transform;
use bevy::transform::TransformBundle;
use bevy_rapier2d::prelude::{Collider, Sensor};

use crate::consts::consts::{BOX_SIZE_HEIHT, BOX_SIZE_WIDTH, BOX_THICKNESS};

#[derive(Component)]
pub struct GameOverSensor;

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
