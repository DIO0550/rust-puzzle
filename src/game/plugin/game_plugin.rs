use bevy::{
    app::{App, Plugin, Startup},
    prelude::{in_state, IntoSystemConfigs, OnEnter},
};

use crate::game::{
    state::{game_page_state::GamePageState, game_state::GameState},
    system::game_system::{restart, setup_cat_mug, setup_gameover_sensor},
    ui::{
        background_ui::{desk_background, desk_book_background},
        evolve_ui::evolve_describe,
    },
};

pub struct GamePlugin;
impl Plugin for GamePlugin {
    fn build(&self, app: &mut App) {
        let game_systems = (
            evolve_describe,
            setup_cat_mug,
            setup_gameover_sensor,
            desk_background,
            desk_book_background,
        );

        app.add_state::<GameState>()
            .add_state::<GamePageState>()
            .add_systems(
                OnEnter(GameState::InGame),
                (restart).run_if(in_state(GamePageState::Game)),
            )
            .add_systems(
                OnEnter(GamePageState::Game),
                game_systems.run_if(in_state(GamePageState::Game)),
            )
            .add_systems(Startup, game_systems.run_if(in_state(GamePageState::Game)));
    }
}
