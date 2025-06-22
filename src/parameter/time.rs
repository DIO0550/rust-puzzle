use bevy::{
    ecs::system::{Res, SystemParam},
    time::Time,
};

#[derive(SystemParam)]
pub struct TimeParams<'w> {
    time: Res<'w, Time>,
}

impl<'w> TimeParams<'w> {
    pub fn delta_seconds(&self) -> f32 {
        self.time.delta_seconds()
    }

    pub fn elapsed_seconds(&self) -> f32 {
        self.time.elapsed_seconds()
    }
}
