use bevy::{
    asset::AssetServer,
    prelude::{BuildChildren, ChildBuilder, Color, Commands, NodeBundle, Res},
    text::{TextAlignment, TextSection, TextStyle},
    ui::*,
    utils::default,
};
use prelude::TextBundle;

use crate::{
    asset::{
        asset::AssetTrait,
        font::{
            font::{FontAsset, FontName},
            font_assets::FontAssets,
        },
    },
    consts::color_theme::ColorTheme,
    title::component::{title_menu::TitleMenu, title_menu_item::TitleMenuItem},
    ui::menu::{menu_bundle::MenuEntityBuilder, menu_item_bundle::MenuItemEntityBuilder},
};

pub fn setup_title_menu(mut commands: Commands, font_assets: Res<FontAssets>) {
    let menu_item = MenuEntityBuilder::new("title_menu", TitleMenu)
        .size(400.0, 400.0)
        .build(&mut commands);

    MenuItemEntityBuilder::new(
        "start_game",
        &TitleMenuItem::StartGame.to_string(),
        TitleMenuItem::StartGame,
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
    )
    .style(Style {
        width: Val::Px(400.0),
        height: Val::Px(100.0),
        ..Default::default()
    })
    .build_as_child(&mut commands, menu_item, &font_assets);
}
