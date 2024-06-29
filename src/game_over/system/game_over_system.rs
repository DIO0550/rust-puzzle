use bevy::{
    ecs::system::{Commands, Res},
    input::{keyboard::KeyCode, Input},
    prelude::{NextState, ResMut},
};

use crate::{
    game::system::{despawn::despawn_component, game_state::GameState},
    game_over::resource::select_game_over_menu::SelectGameOverMenu,
    score::resource::score::Score,
};

pub fn reset_select_menu(mut commands: Commands) {
    commands.insert_resource(SelectGameOverMenu::Restart);
}

pub fn change_select_menu(
    mut commands: Commands,
    keyboard_input: Res<Input<KeyCode>>,
    select_menu: Res<SelectGameOverMenu>,
) {
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
    if !keyboard_input.just_pressed(KeyCode::Space) {
        return;
    }

    if *select_menu == SelectGameOverMenu::Restart {
        app_state.set(GameState::InGame);
    }
}

pub fn restart(mut commands: Commands) {
    commands.insert_resource(Score(0));
}
