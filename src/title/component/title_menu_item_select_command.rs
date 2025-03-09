use bevy::ecs::{query::With, schedule::NextState, system::Commands, world::World};

use crate::{
    game::state::game_page_state::GamePageState,
    ui::menu::{menu_item_action::MenuItemSelectAction, menu_item_bundle::MenuItemSelected},
};

use super::title_menu_item::TitleMenuItem;

#[derive(Debug, Default)]
pub struct TitleMenuItemSelectCommand;

impl MenuItemSelectAction for TitleMenuItemSelectCommand {
    fn excecute(&self, world: &mut World) {
        let mut query_result = world.query_filtered::<&TitleMenuItem, With<MenuItemSelected>>();

        match query_result.get_single_mut(world) {
            Ok(TitleMenuItem::StartGame) => {
                world
                    .resource_mut::<NextState<GamePageState>>()
                    .set(GamePageState::Game);
            }
            Ok(TitleMenuItem::HighScore) => todo!(),
            Err(_) => todo!(),
        }
    }
}
