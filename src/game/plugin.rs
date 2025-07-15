use bevy::{
    app::{App, Plugin, Startup},
    prelude::{in_state, IntoSystemConfigs, OnEnter},
};

use crate::game::{
    background::desk::{desk_background, desk_book_background},
    screen_state::ScreenState,
    state::GameState,
    system::restart,
};

pub struct GamePlugin;
impl Plugin for GamePlugin {
    fn build(&self, app: &mut App) {
        let game_systems = (desk_background, desk_book_background);

        app.add_state::<GameState>()
            .add_state::<ScreenState>()
            .add_systems(
                OnEnter(GameState::InGame),
                (restart).run_if(in_state(ScreenState::Game)),
            )
            .add_systems(
                OnEnter(ScreenState::Game),
                game_systems.run_if(in_state(ScreenState::Game)),
            )
            .add_systems(Startup, game_systems.run_if(in_state(ScreenState::Game)));
    }
}
