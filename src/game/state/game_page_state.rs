use bevy::prelude::*;
#[derive(States, Debug, Clone, Copy, Eq, PartialEq, Hash, Default)]
pub enum GamePageState {
    #[default]
    Loading,
    Title,
    Game,
}
