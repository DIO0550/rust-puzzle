use bevy::{
    ecs::{
        component::Component,
        entity::Entity,
        query::{Added, With},
        removal_detection::RemovedComponents,
        system::{Commands, Query, Res},
    },
    hierarchy::Children,
    input::{keyboard::KeyCode, Input},
    ui::BackgroundColor,
};

use super::{
    menu_bundle::Menu,
    menu_item_bundle::{MenuItemColor, MenuItemSelected},
};

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

struct MenuController {}

impl<MenuItemMarker: Component> MenuControll<MenuItemMarker> for MenuController {
    fn select_next(
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
        for (index, &child) in children.iter().enumerate() {
            if select_item_query.contains(child) {
                selected_index = Some(index);
                break;
            }
        }

        // 次のインデックスを計算
        let next_index = match selected_index {
            Some(current) => Some((current + 1) % children.len()),
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

    fn select_previous(
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
        for (index, &child) in children.iter().enumerate() {
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
}

pub fn menu_navigation<MenuMaker: Component, MenuItemMarker: Component>(
    mut commands: Commands,
    children_query: Query<&Children>,
    menu_query: Query<Entity, (With<Menu>, With<MenuMaker>)>,
    select_item_query: Query<Entity, (With<MenuItemSelected>, With<MenuItemMarker>)>,
    keyboard_input: Res<Input<KeyCode>>,
) {
    let Ok(menu_entity) = menu_query.get_single() else {
        return;
    };

    match keyboard_input {
        keyboard if keyboard.just_pressed(KeyCode::Up) => MenuController::select_previous(
            menu_entity,
            &mut commands,
            &children_query,
            &select_item_query,
        ),
        keyboard if keyboard.just_pressed(KeyCode::Down) => MenuController::select_next(
            menu_entity,
            &mut commands,
            &children_query,
            &select_item_query,
        ),
        _ => (),
    };
}

pub fn update_menu_item_colors<MenuMaker: Component, MenuItemMarker: Component>(
    changed_select_item_query: Query<Entity, Added<MenuItemSelected>>,
    mut removed: RemovedComponents<MenuItemSelected>,
    mut color_query: Query<(&mut BackgroundColor, &MenuItemColor)>,
) {
    for changed_entity in changed_select_item_query.iter() {
        let Ok((mut bg_color, color)) = color_query.get_mut(changed_entity) else {
            continue;
        };
        bg_color.0 = color.selected;
    }

    for entity in removed.read() {
        let Ok((mut bg_color, color)) = color_query.get_mut(entity) else {
            continue;
        };
        bg_color.0 = color.normal;
    }
}
