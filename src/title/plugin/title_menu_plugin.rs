use bevy::prelude::*;

use crate::{
    game::{state::game_page_state::GamePageState, system::despawn::despawn_component},
    title::{
        component::{title_menu::TitleMenu, title_menu_item::TitleMenuItem},
        resource::select_title_menu::SelectTitleMenu,
        system::select_title_system::*,
        ui::menu_ui::*,
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
                    change_select_title_menu,
                    select_title_menu,
                    update_title_menu,
                )
                    .run_if(in_state(GamePageState::Title)),
            );
    }
}
