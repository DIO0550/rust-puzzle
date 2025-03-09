use bevy::prelude::*;

use crate::{
    game::{state::game_page_state::GamePageState, system::despawn::despawn_component},
    title::{
        component::{
            title_menu::TitleMenu, title_menu_item::TitleMenuItem,
            title_menu_item_select_command::TitleMenuItemSelectCommand,
        },
        resource::select_title_menu::SelectTitleMenu,
        system::select_title_system::*,
        ui::menu_ui::*,
    },
    ui::menu::{
        menu_controller::{menu_navigation, update_menu_item_colors},
        menu_item_action::select_menu_item_action,
    },
};

pub struct TitlePlugin;
impl Plugin for TitlePlugin {
    fn build(&self, app: &mut App) {
        app.insert_resource(SelectTitleMenu(TitleMenuItem::StartGame))
            .add_systems(OnEnter(GamePageState::Title), setup_title_menu)
            .add_systems(OnExit(GamePageState::Title), despawn_component::<TitleMenu>)
            .add_systems(
                Update,
                (
                    menu_navigation::<TitleMenu, TitleMenuItem>,
                    update_menu_item_colors::<TitleMenu, TitleMenuItem>,
                    select_menu_item_action::<TitleMenuItemSelectCommand>,
                )
                    .run_if(in_state(GamePageState::Title)),
            );
    }
}
