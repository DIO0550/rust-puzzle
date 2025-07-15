use bevy::prelude::*;

use crate::{
    game::{despawn::despawn_component, screen_state::ScreenState},
    title::menu::{
        item::TitleMenuItem, item_select_action::TitleMenuItemSelectAction, setup_title_menu,
        TitleMenu,
    },
    ui::menu::{
        menu_controller::{menu_navigation, update_menu_item_colors},
        menu_item_action::select_menu_item_action,
    },
};

pub struct TitlePlugin;
impl Plugin for TitlePlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(OnEnter(ScreenState::Title), setup_title_menu)
            .add_systems(OnExit(ScreenState::Title), despawn_component::<TitleMenu>)
            .add_systems(
                Update,
                (
                    menu_navigation::<TitleMenu, TitleMenuItem>,
                    update_menu_item_colors::<TitleMenu, TitleMenuItem>,
                    select_menu_item_action::<TitleMenuItemSelectAction>,
                )
                    .run_if(in_state(ScreenState::Title)),
            );
    }
}
