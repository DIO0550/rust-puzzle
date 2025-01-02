use bevy::prelude::*;

use crate::{
    game::{state::game_page_state::GamePageState, system::despawn::despawn_component},
    title::{
        component::{title_menu::TitleMenu, title_menu_item::TitleMenuItem},
        resource::select_title_menu::SelectTitleMenu,
        ui::menu_ui::setup_title_menu,
    },
};

pub struct ScorePlugin;
impl Plugin for ScorePlugin {
    fn build(&self, app: &mut App) {
        app.insert_resource(SelectTitleMenu(TitleMenuItem::StartGame))
            .add_systems(
                Startup,
                setup_title_menu.run_if(in_state(GamePageState::Title)),
            )
            .add_systems(OnExit(GamePageState::Title), despawn_component::<TitleMenu>);
    }
}
