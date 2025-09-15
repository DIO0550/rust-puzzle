use bevy::prelude::{ButtonInput, KeyCode, NextState, Res, ResMut, Resource};

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

pub fn change_select_game_over_menu(
    keyboard_input: Res<ButtonInput<KeyCode>>,
    mut select_menu: ResMut<GameOverMenuSelection>,
) {
    if keyboard_input.pressed(KeyCode::Space) {
        return;
    }

    match keyboard_input {
        keyboard if keyboard.just_pressed(KeyCode::ArrowUp) => select_menu.0 = select_menu.next().0,
        keyboard if keyboard.just_pressed(KeyCode::ArrowDown) => {
            select_menu.0 = select_menu.prev().0
        }
        _ => (),
    };
}

pub fn select_game_over_menu(
    keyboard_input: Res<ButtonInput<KeyCode>>,
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

pub fn reset_select_menu(mut select_menu: ResMut<GameOverMenuSelection>) {
    select_menu.0 = GameOverMenu::Restart;
}
