use bevy::prelude::{Commands, ResMut, Transform, TransformBundle};
use bevy_rapier2d::{
    plugin::RapierConfiguration,
    prelude::{Collider, Sensor},
};

use crate::{score::resource::Score, BOX_SIZE_HEIHT, BOX_SIZE_WIDTH};

pub fn restart(mut commands: Commands, mut config: ResMut<RapierConfiguration>) {
    commands.insert_resource(Score(0));
    config.physics_pipeline_active = true;
}
