use bevy::{ecs::system::Resource, sprite::ColorMaterial};

#[derive(Resource)]
pub struct Materials {
    pub colors: Vec<ColorMaterial>,
}
