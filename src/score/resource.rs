use bevy::prelude::Resource;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Clone, Copy, Debug, Resource, PartialEq, Eq, PartialOrd, Ord)]
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
