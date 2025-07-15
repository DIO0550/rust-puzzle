use bevy::ecs::system::Resource;

#[derive(Resource)]
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
