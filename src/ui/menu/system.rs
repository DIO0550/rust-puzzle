use bevy::prelude::*;

use crate::ui::menu::builder::Menu;
use crate::ui::menu::item_builder::MenuItemSelected;
use crate::ui::menu::style::MenuItemColors;

pub trait MenuControll<MenuItemMarker: Component> {
    fn select_next(
        menu_entity: Entity,
        commands: &mut Commands,
        children_query: &Query<&Children>,
        select_item_query: &Query<Entity, (With<MenuItemSelected>, With<MenuItemMarker>)>,
    );

    fn select_previous(
        menu_entity: Entity,
        commands: &mut Commands,
        children_query: &Query<&Children>,
        select_item_query: &Query<Entity, (With<MenuItemSelected>, With<MenuItemMarker>)>,
    );
}

fn select_next<MenuItemMarker: Component>(
    menu_entity: Entity,
    commands: &mut Commands,
    children_query: &Query<&Children>,
    select_item_query: &Query<Entity, (With<MenuItemSelected>, With<MenuItemMarker>)>,
) {
    // select_nextと同様の実装（逆方向）
    let Ok(children) = children_query.get(menu_entity) else {
        return;
    };

    // 現在選択されているアイテムのインデックスを探す
    let mut selected_index = None;
    for (index, child) in children.iter().enumerate() {
        if select_item_query.contains(child) {
            selected_index = Some(index);
            break;
        }
    }

    // 次のインデックスを計算
    let next_index = match selected_index {
        Some(current) if current == 0 => Some(children.len() - 1),
        Some(current) => Some((current - 1) % children.len()),
        None if !children.is_empty() => Some(0),
        None => None,
    };

    // 選択状態を更新
    if let Some(selected_index) = selected_index {
        let selected_child = children[selected_index];
        commands.entity(selected_child).remove::<MenuItemSelected>();
    }

    if let Some(next_index) = next_index {
        let next_child = children[next_index];
        commands.entity(next_child).insert(MenuItemSelected);
    }
}

fn select_previous<MenuItemMarker: Component>(
    menu_entity: Entity,
    commands: &mut Commands,
    children_query: &Query<&Children>,
    select_item_query: &Query<Entity, (With<MenuItemSelected>, With<MenuItemMarker>)>,
) {
    let Ok(children) = children_query.get(menu_entity) else {
        return;
    };
    // 現在選択されているアイテムのインデックスを探す
    let mut selected_index = None;
    for (index, child) in children.iter().enumerate() {
        if select_item_query.contains(child) {
            selected_index = Some(index);
            break;
        }
    }
    // 次のインデックスを計算
    let next_index = match selected_index {
        Some(current) if current + 1 >= children.len() => Some(0),
        Some(current) => Some((current + 1) % children.len()),
        None if !children.is_empty() => Some(0),
        None => None,
    };

    println!(
        "selected_index: {:?}, next_index: {:?}",
        selected_index, next_index
    );

    // 選択状態を更新
    if let Some(selected_index) = selected_index {
        let selected_child = children[selected_index];
        commands.entity(selected_child).remove::<MenuItemSelected>();
    }
    if let Some(next_index) = next_index {
        let next_child = children[next_index];
        commands.entity(next_child).insert(MenuItemSelected);
    }
}

pub fn menu_navigation<MenuMaker: Component, MenuItemMarker: Component>(
    mut commands: Commands,
    children_query: Query<&Children>,
    menu_query: Query<Entity, (With<Menu>, With<MenuMaker>)>,
    select_item_query: Query<Entity, (With<MenuItemSelected>, With<MenuItemMarker>)>,
    keyboard_input: Res<ButtonInput<KeyCode>>,
) {
    let Ok(menu_entity) = menu_query.get_single() else {
        return;
    };

    match keyboard_input {
        keyboard if keyboard.just_pressed(KeyCode::ArrowUp) => select_previous(
            menu_entity,
            &mut commands,
            &children_query,
            &select_item_query,
        ),
        keyboard if keyboard.just_pressed(KeyCode::ArrowDown) => select_next(
            menu_entity,
            &mut commands,
            &children_query,
            &select_item_query,
        ),
        _ => (),
    };
}

pub fn update_menu_item_colors<MenuMaker: Component, MenuItemMarker: Component>(
    changed_selection_state_query: Query<Entity, Changed<MenuItemSelected>>,
    mut colors_query: Query<
        (&mut BackgroundColor, &mut BorderColor, Option<&Children>),
        With<MenuItemMarker>,
    >,
    mut text_query: Query<&mut TextColor>,
    mut removed: RemovedComponents<MenuItemSelected>,
) {
    for changed_entity in changed_selection_state_query.iter() {
        let Ok((mut bg_color, mut border_color, children)) = colors_query.get_mut(changed_entity)
        else {
            continue;
        };

        println!("changed_entity: {:?}", changed_entity);

        let colors = MenuItemColors::new(true);

        bg_color.0 = colors.background;
        border_color.0 = colors.border;

        let Some(children) = children else {
            continue;
        };

        for child in children.iter() {
            if let Ok(mut text_color) = text_query.get_mut(child) {
                text_color.0 = colors.text;
            }
        }
    }

    for entity in removed.read() {
        let Ok((mut bg_color, mut border_color, option_children)) = colors_query.get_mut(entity)
        else {
            continue;
        };

        println!("removed entity: {:?}", entity);

        let colors = MenuItemColors::new(false);

        bg_color.0 = colors.background;
        border_color.0 = colors.border;

        if let Some(children) = option_children {
            for child in children.iter() {
                if let Ok(mut text_color) = text_query.get_mut(child) {
                    text_color.0 = colors.text;
                }
            }
        }
    }
}
