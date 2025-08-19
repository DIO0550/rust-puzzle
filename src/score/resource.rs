use bevy::ecs::system::Resource;
use serde::{Deserialize, Serialize};

#[derive(Resource, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Serialize, Deserialize)]
pub struct Score(pub u32);

impl ToString for Score {
    fn to_string(&self) -> String {
        self.0.to_string()
    }
}

impl Default for Score {
    fn default() -> Self {
        Self(0)
    }
}
