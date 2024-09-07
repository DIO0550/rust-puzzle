use bevy::{
    ecs::system::{Commands, Res},
    input::{keyboard::KeyCode, Input},
    prelude::{NextState, ResMut},
};

use crate::{
    game::system::game_state::GameState,
    game_over::resource::select_game_over_menu::SelectGameOverMenu,
};

pub fn reset_select_menu(mut commands: Commands) {
    commands.insert_resource(SelectGameOverMenu::Restart);
}

pub fn change_select_menu(
    mut commands: Commands,
    keyboard_input: Res<Input<KeyCode>>,
    select_menu: Res<SelectGameOverMenu>,
) {
    if keyboard_input.pressed(KeyCode::Space) {
        return;
    }

    let mut new_select_menu: SelectGameOverMenu = *select_menu;

    if keyboard_input.just_pressed(KeyCode::Up) {
        new_select_menu = select_menu.prev_menu();
    }

    if keyboard_input.just_pressed(KeyCode::Down) {
        new_select_menu = select_menu.next_menu();
    }

    commands.insert_resource(new_select_menu);
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

    if *select_menu == SelectGameOverMenu::Restart {
        println!("select_menu");
        app_state.set(GameState::InGame);
    }
}
