use crate::{
    game::{
        component::on_display_over::OnDisplayGameOver, state::game_state::GameState,
        system::despawn::despawn_component,
    },
    game_over::{
        component::game_over_menu_item::GameOverMenu,
        resource::select_game_over_menu::SelectGameOverMenu,
        system::game_over_system::{
            change_select_game_over_menu, reset_select_menu, select_game_over_menu,
        },
        ui::game_over_ui::{display_game_over, update_menu},
    },
    piece::component::animal_piece::animal_piece_component::AnimalPieceComponent,
};
use bevy::{
    app::{App, Plugin, Update},
    ecs::schedule::{IntoSystemConfigs, OnEnter, OnExit},
    prelude::in_state,
};

pub struct GameOverPlugin;
impl Plugin for GameOverPlugin {
    fn build(&self, app: &mut App) {
        app.insert_resource(SelectGameOverMenu(GameOverMenu::Restart))
            .add_systems(OnEnter(GameState::GameOver), (display_game_over).chain())
            .add_systems(
                Update,
                (
                    update_menu,
                    change_select_game_over_menu,
                    select_game_over_menu,
                )
                    .run_if(in_state(GameState::GameOver)),
            )
            .add_systems(
                OnExit(GameState::GameOver),
                (
                    despawn_component::<OnDisplayGameOver>,
                    despawn_component::<AnimalPieceComponent>,
                    reset_select_menu,
                ),
            );
    }
}
