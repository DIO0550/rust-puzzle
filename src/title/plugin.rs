use bevy::prelude::*;

use crate::{
    game::{despawn::despawn_component, screen_state::ScreenState},
    title::menu::{
        item::TitleMenuItem, item_select_action::TitleMenuItemSelectAction, setup_title_menu,
        TitleMenu,
    },
    ui::menu::{
        item_action::select_menu_item_action,
        system::{menu_navigation, update_menu_item_colors},
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
                    select_menu_item_action::<TitleMenuItemSelectAction>,
                    menu_navigation::<TitleMenu, TitleMenuItem>,
                    update_menu_item_colors::<TitleMenu, TitleMenuItem>,
                )
                    .run_if(in_state(ScreenState::Title)),
            );
    }
}
