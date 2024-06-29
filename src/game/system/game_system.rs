use bevy::prelude::{Commands, ResMut};
use bevy_rapier2d::plugin::RapierConfiguration;

use crate::score::resource::score::Score;

pub fn restart(mut commands: Commands, mut config: ResMut<RapierConfiguration>) {
    commands.insert_resource(Score(0));
    config.physics_pipeline_active = true;
}
