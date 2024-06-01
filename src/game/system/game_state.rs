use bevy::prelude::*;
use std::fmt;
#[derive(States, Debug, Clone, Copy, Eq, PartialEq, Hash, Default)]
pub enum GameState {
    MainMenu,
    #[default]
    InGame,
    GameOver,
}
