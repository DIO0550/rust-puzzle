use bevy::{
    ecs::system::{Commands, Res},
    input::{
        keyboard::{self, KeyCode},
        Input,
    },
    prelude::{NextState, ResMut},
};

use crate::{
    game::state::game_state::GameState,
    game_over::{
        component::game_over_menu_item::GameOverMenu,
        resource::select_game_over_menu::SelectGameOverMenu,
    },
};

pub fn reset_select_menu(mut commands: Commands) {
    commands.insert_resource(SelectGameOverMenu(GameOverMenu::Restart));
}

pub fn change_select_menu(
    mut commands: Commands,
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

    // commands.insert_resource(new_select_menu);
}

pub fn select_menu(
    mut _commands: Commands,
    keyboard_input: Res<Input<KeyCode>>,
    select_menu: Res<SelectGameOverMenu>,
    mut app_state: ResMut<NextState<GameState>>,
) {
    if !keyboard_input.just_released(KeyCode::Space) {
        return;
    }

    // if *select_menu == GameOverMenu::Restart {
    //     app_state.set(GameState::InGame);
    // }
}
