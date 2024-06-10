use crate::{
    game::{
        component::on_display_over::OnDisplayGameOver,
        system::{despawn::despawn_component, game_state::GameState},
    },
    game_over::{
        resource::select_game_over_menu::SelectGameOverMenu,
        system::game_over_system::reset_select_menu, ui::game_over_ui::display_game_over,
    },
};
use bevy::{
    app::{App, Plugin},
    ecs::schedule::{IntoSystemConfigs, OnEnter, OnExit},
};

pub struct GameOverPlugin;
impl Plugin for GameOverPlugin {
    fn build(&self, app: &mut App) {
        app.insert_resource(SelectGameOverMenu::Restart)
            .add_systems(OnEnter(GameState::InGame), (display_game_over).chain())
            .add_systems(
                OnExit(GameState::GameOver),
                (despawn_component::<OnDisplayGameOver>, reset_select_menu),
            );
    }
}
