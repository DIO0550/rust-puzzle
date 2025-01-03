use bevy::{
    input::Input,
    prelude::{Color, KeyCode, NextState, Query, Res, ResMut, State},
    text::Text,
    ui::BackgroundColor,
};

use crate::{
    consts::color_theme::ColorTheme,
    game::state::game_page_state::GamePageState,
    title::{
        component::title_menu_item::TitleMenuItem, resource::select_title_menu::SelectTitleMenu,
    },
};

pub fn change_select_title_menu(
    mut select_title_menu: ResMut<SelectTitleMenu>,
    input: Res<Input<KeyCode>>,
) {
    match input {
        input if input.just_released(KeyCode::Up) => *select_title_menu = select_title_menu.prev(),
        input if input.just_released(KeyCode::Down) => {
            *select_title_menu = select_title_menu.next()
        }
        _ => (),
    }
}

pub fn select_title_menu(
    mut state: ResMut<NextState<GamePageState>>,
    select_menu_res: Res<SelectTitleMenu>,
    input: Res<Input<KeyCode>>,
) {
    if !input.just_released(KeyCode::Space) {
        return;
    }

    match select_menu_res.0 {
        TitleMenuItem::StartGame => {
            state.set(GamePageState::Game);
        }
        TitleMenuItem::HighScore => {}
        _ => (),
    }
}

pub fn update_title_menu(
    mut menu_query: Query<(&mut BackgroundColor, &TitleMenuItem)>,
    mut menu_text_query: Query<(&mut Text, &TitleMenuItem)>,
    select_menu_res: Res<SelectTitleMenu>,
) {
    for (mut style, menu) in menu_query.iter_mut() {
        style.0 = match select_menu_res.0 == *menu {
            true => ColorTheme::NORWAY,
            false => ColorTheme::SPROUT,
        };
    }

    for (mut menu_text, menu) in menu_text_query.iter_mut() {
        menu_text.sections[0].style.color = match select_menu_res.0 == *menu {
            true => Color::WHITE,
            false => Color::BLACK,
        };
    }
}
