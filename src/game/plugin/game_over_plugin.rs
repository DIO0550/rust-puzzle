use crate::game::system::despawn::despawn_component;
use bevy::{
    app::{App, Plugin},
    ecs::schedule::{OnEnter, OnExit},
};

use crate::game::{
    component::on_display_over::OnDisplayGameOver, system::game_state::GameState,
    ui::game_over_ui::display_game_over,
};

pub struct GameOverPlugin;
impl Plugin for GameOverPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(OnEnter(GameState::GameOver), display_game_over)
            .add_systems(
                OnExit(GameState::GameOver),
                despawn_component::<OnDisplayGameOver>,
            );
    }
}
