use bevy::prelude::*;
#[derive(States, Debug, Clone, Copy, Eq, PartialEq, Hash, Default)]
pub enum ScreenState {
    #[default]
    Loading,
    Title,
    Game,
}
