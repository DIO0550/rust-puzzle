use bevy::{
    app::{App, Plugin},
    prelude::OnEnter,
};

use crate::game::system::{
    game_page_state::GamePageState, game_state::GameState, game_system::restart,
};

pub struct GamePlugin;
impl Plugin for GamePlugin {
    fn build(&self, app: &mut App) {
        app.add_state::<GameState>()
            .add_state::<GamePageState>()
            .add_systems(OnEnter(GameState::InGame), restart);
    }
}
