use bevy::prelude::{Commands, ResMut, Transform, TransformBundle};
use bevy_rapier2d::{
    plugin::RapierConfiguration,
    prelude::{Collider, Sensor},
};

use crate::{
    game::component::game_over_sensor::GameOverSensor, score::resource::score::Score,
    BOX_SIZE_HEIHT, BOX_SIZE_WIDTH, BOX_THICKNESS,
};

pub fn restart(mut commands: Commands, mut config: ResMut<RapierConfiguration>) {
    commands.insert_resource(Score(0));
    config.physics_pipeline_active = true;
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
