use bevy::prelude::*;
#[derive(States, Debug, Clone, Copy, Eq, PartialEq, Hash, Default)]
pub enum AssetLoadState {
    #[default]
    Loading,
    Loaded,
    Failed,
}
