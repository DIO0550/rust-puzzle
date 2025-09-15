use bevy::prelude::Commands;

use crate::score::resource::Score;

pub fn restart(mut commands: Commands) {
    commands.insert_resource(Score(0));
}
