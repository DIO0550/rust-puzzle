pub mod item;
pub mod item_select_action;

use bevy::ecs::component::Component;
use bevy::{
    prelude::{Commands, Res},
    ui::*,
};

use crate::title::menu::item::TitleMenuItem;
use crate::title::menu::item_select_action::TitleMenuItemSelectAction;
use crate::{
    asset::font::font_assets::FontAssets,
    ui::menu::{menu_bundle::MenuEntityBuilder, menu_item_bundle::MenuItemEntityBuilder},
};

#[derive(Component)]
pub struct TitleMenu;

pub fn setup_title_menu(mut commands: Commands, font_assets: Res<FontAssets>) {
    let menu_item = MenuEntityBuilder::new("title_menu", TitleMenu)
        .size(400.0, 400.0)
        .build(&mut commands);

    MenuItemEntityBuilder::new(
        "start_game",
        &TitleMenuItem::StartGame.to_string(),
        TitleMenuItem::StartGame,
        TitleMenuItemSelectAction::StartGame,
    )
    .style(Style {
        width: Val::Px(400.0),
        height: Val::Px(100.0),
        ..Default::default()
    })
    .build_as_child(&mut commands, menu_item, &font_assets);

    MenuItemEntityBuilder::new(
        "high_score",
        &TitleMenuItem::HighScore.to_string(),
        TitleMenuItem::HighScore,
        TitleMenuItemSelectAction::HighScore,
    )
    .style(Style {
        width: Val::Px(400.0),
        height: Val::Px(100.0),
        ..Default::default()
    })
    .build_as_child(&mut commands, menu_item, &font_assets);
}
