use bevy::ecs::system::Resource;
use serde::{Deserialize, Serialize};

#[derive(Resource, Serialize, Deserialize)]
pub struct HighScore {
    pub date: String,
    pub score: u32,
}
