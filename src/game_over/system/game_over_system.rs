use bevy::{
    ecs::system::{Commands, Res},
    input::{keyboard::KeyCode, Input},
};

use crate::game_over::resource::select_game_over_menu::SelectGameOverMenu;

pub fn reset_select_menu(mut commands: Commands) {
    commands.insert_resource(SelectGameOverMenu::Restart);
}

pub fn change_select_menu(
    mut commands: Commands,
    keyboard_input: Res<Input<KeyCode>>,
    select_menu: Res<SelectGameOverMenu>,
) {
    let mut new_select_menu: SelectGameOverMenu = *select_menu;
    if keyboard_input.pressed(KeyCode::Up) {
        new_select_menu = select_menu.prev_menu();
    }

    if keyboard_input.pressed(KeyCode::Down) {
        new_select_menu = select_menu.next_menu();
    }

    // println!("{:?}", new_select_menu);

    commands.insert_resource(new_select_menu);
}
