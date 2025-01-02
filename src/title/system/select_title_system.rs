pub fn change_select_title_menu(
    mut select_title_menu: ResMut<SelectTitleMenu>,
    input: Res<Input<KeyCode>>,
) {
    match input {
        input if input.just_released(KeyCode::Up) => {
            select_title_menu.0 = select_title_menu.0.prev()
        }
        input if input.just_released(KeyCode::Down) => {
            select_title_menu.0 = select_title_menu.0.next()
        }
        _ => (),
    }
}

pub fn update_select_menu(
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
        style.0 = match select_menu_res.0 == *menu {
            true => Color::WHITE,
            false => Color::BLACK,
        };
    }
}
