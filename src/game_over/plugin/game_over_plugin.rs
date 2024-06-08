use crate::{
    game::{
        component::on_display_over::OnDisplayGameOver,
        system::{despawn::despawn_component, game_state::GameState},
    },
    game_over::ui::game_over_ui::{display_game_over, display_game_over_menu},
};
use bevy::{
    app::{App, Plugin},
    ecs::schedule::{IntoSystemConfigs, OnEnter, OnExit},
};

pub struct GameOverPlugin;
impl Plugin for GameOverPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(OnEnter(GameState::InGame), (display_game_over_menu).chain())
            .add_systems(
                OnExit(GameState::GameOver),
                despawn_component::<OnDisplayGameOver>,
            );
    }
}
