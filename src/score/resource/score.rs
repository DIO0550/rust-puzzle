use bevy::ecs::system::Resource;

#[derive(Resource)]
pub struct Score(pub u32);
