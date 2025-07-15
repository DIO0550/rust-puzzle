use bevy::ecs::schedule::NextState;
use bevy::ecs::system::{Res, ResMut, Resource};
use bevy::input::keyboard::KeyCode;
use bevy::input::Input;

use crate::game::screen_state::ScreenState;
use crate::game::state::GameState;
use crate::game_over::menu::GameOverMenu;

#[derive(Resource)]
pub struct GameOverMenuSelection(pub GameOverMenu);

impl GameOverMenuSelection {
    pub fn next(&self) -> GameOverMenuSelection {
        match self.0 {
            GameOverMenu::Restart => GameOverMenuSelection(GameOverMenu::GoTitle),
            GameOverMenu::GoTitle => GameOverMenuSelection(GameOverMenu::Restart),
        }
    }

    pub fn prev(&self) -> GameOverMenuSelection {
        match self.0 {
            GameOverMenu::Restart => GameOverMenuSelection(GameOverMenu::GoTitle),
            GameOverMenu::GoTitle => GameOverMenuSelection(GameOverMenu::Restart),
        }
    }
}

pub fn reset_select_menu(mut game_state: ResMut<NextState<GameState>>) {
    game_state.set(GameState::InGame);
}

pub fn change_select_game_over_menu(
    keyboard_input: Res<Input<KeyCode>>,
    mut select_menu: ResMut<GameOverMenuSelection>,
) {
    if keyboard_input.pressed(KeyCode::Space) {
        return;
    }

    match keyboard_input {
        keyboard if keyboard.just_pressed(KeyCode::Up) => *select_menu = select_menu.prev(),
        keyboard if keyboard.just_pressed(KeyCode::Down) => *select_menu = select_menu.next(),
        _ => (),
    };
}

pub fn select_game_over_menu(
    keyboard_input: Res<Input<KeyCode>>,
    select_menu: Res<GameOverMenuSelection>,
    mut game_state: ResMut<NextState<GameState>>,
) {
    if !keyboard_input.just_released(KeyCode::Space) {
        return;
    }

    if select_menu.0 != GameOverMenu::Restart {
        return;
    }
    game_state.set(GameState::InGame);
}
