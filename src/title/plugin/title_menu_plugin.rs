use bevy::prelude::*;

use crate::{
    game::system::game_page_state::GamePageState,
    title::{
        component::title_menu_item::TitleMenuItem, resource::select_title_menu::SelectTitleMenu,
        ui::menu_ui::setup_title_menu,
    },
};

pub struct ScorePlugin;
impl Plugin for ScorePlugin {
    fn build(&self, app: &mut App) {
        app.insert_resource(SelectTitleMenu(TitleMenuItem::StartGame))
            .add_systems(
                Startup,
                setup_title_menu.run_if(in_state(GamePageState::StartPage)),
            );
        // .add_systems(
        //     Update,
        //     update_current_score_text.run_if(in_state(GameState::InGame)),
        // );
    }
}
