use crate::{
    game::{despawn::despawn_component, screen_state::ScreenState, state::GameState},
    game_over::{
        menu::{display_game_over, update_menu, GameOverMenu, GameOverScreen},
        menu_selection::{
            change_select_game_over_menu, reset_select_menu, select_game_over_menu,
            GameOverMenuSelection,
        },
        sensor::{setup_gameover_sensor, GameOverSensor},
    },
    piece::component::animal_piece::animal_piece_component::AnimalPieceComponent,
};
use bevy::{
    app::{App, Plugin, Startup, Update},
    ecs::schedule::{IntoSystemConfigs, OnEnter, OnExit},
    prelude::in_state,
};

pub struct GameOverPlugin;
impl Plugin for GameOverPlugin {
    fn build(&self, app: &mut App) {
        app.insert_resource(GameOverMenuSelection(GameOverMenu::Restart))
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
            .add_systems(OnEnter(ScreenState::Game), setup_gameover_sensor)
            .add_systems(
                OnExit(ScreenState::Game),
                despawn_component::<GameOverSensor>,
            )
            .add_systems(
                OnExit(GameState::GameOver),
                (
                    despawn_component::<GameOverScreen>,
                    despawn_component::<AnimalPieceComponent>,
                    reset_select_menu,
                ),
            );
    }
}
