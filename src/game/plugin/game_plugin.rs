use bevy::{
    app::{App, Plugin, Startup},
    prelude::OnEnter,
};

use crate::game::{
    system::{
        game_page_state::GamePageState,
        game_state::GameState,
        game_system::{restart, setup_cat_mug},
    },
    ui::{background_ui::desk_background, evolve_ui::evolve_describe},
};

pub struct GamePlugin;
impl Plugin for GamePlugin {
    fn build(&self, app: &mut App) {
        app.add_state::<GameState>()
            .add_state::<GamePageState>()
            .add_systems(OnEnter(GameState::InGame), restart)
            .add_systems(Startup, (evolve_describe, setup_cat_mug, desk_background));
    }
}
