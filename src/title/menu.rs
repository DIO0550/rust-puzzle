pub mod item;
pub mod item_select_action;

use bevy::ecs::component::Component;
use bevy::prelude::*;

use crate::title::menu::item::TitleMenuItem;
use crate::title::menu::item_select_action::TitleMenuItemSelectAction;
use crate::{
    asset::font::font_assets::FontAssets,
    ui::menu::{builder::MenuBuilder, item_builder::MenuItemBundleBuilder},
};

#[derive(Component)]
pub struct TitleMenu;

pub fn setup_title_menu(mut commands: Commands, font_assets: Res<FontAssets>) {
    let menu_item_height: f32 = 100.0;
    let menu_item_width: f32 = 400.0;

    let menu = MenuBuilder::new(TitleMenu).build(&mut commands);

    let start_game_menu_item = MenuItemBundleBuilder::new(
        &TitleMenuItem::StartGame.to_string(),
        TitleMenuItem::StartGame,
        TitleMenuItemSelectAction::StartGame,
    )
    .selected(true)
    .width(menu_item_width)
    .height(menu_item_height)
    .build(&mut commands, &font_assets);

    let high_score_menu_item = MenuItemBundleBuilder::new(
        &TitleMenuItem::HighScore.to_string(),
        TitleMenuItem::HighScore,
        TitleMenuItemSelectAction::HighScore,
    )
    .width(menu_item_width)
    .height(menu_item_height)
    .build(&mut commands, &font_assets);

    commands
        .entity(menu)
        .add_children(&[start_game_menu_item, high_score_menu_item]);
}
