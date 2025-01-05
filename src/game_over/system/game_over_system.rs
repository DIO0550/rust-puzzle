use bevy::{
    ecs::system::Res,
    input::{keyboard::KeyCode, Input},
    prelude::{NextState, ResMut},
};

use crate::{
    game::state::game_state::GameState,
    game_over::{
        component::game_over_menu_item::GameOverMenu,
        resource::select_game_over_menu::SelectGameOverMenu,
    },
};

pub fn reset_select_menu(mut game_state: ResMut<NextState<GameState>>) {
    game_state.set(GameState::InGame);
}

pub fn change_select_game_over_menu(
    keyboard_input: Res<Input<KeyCode>>,
    mut select_menu: ResMut<SelectGameOverMenu>,
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
    select_menu: Res<SelectGameOverMenu>,
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
