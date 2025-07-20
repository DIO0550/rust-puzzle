pub mod item;
pub mod item_select_action;

use bevy::a11y::accesskit::Size;
use bevy::ecs::component::Component;
use bevy::{
    prelude::{Commands, Res},
    ui::*,
};

use crate::consts::color_theme::ColorTheme;
use crate::game_over::menu;
use crate::title::menu::item::TitleMenuItem;
use crate::title::menu::item_select_action::TitleMenuItemSelectAction;
use crate::{
    asset::font::font_assets::FontAssets,
    ui::menu::{menu_bundle::MenuEntityBuilder, menu_item_bundle::MenuItemEntityBuilder},
};

#[derive(Component)]
pub struct TitleMenu;

pub fn setup_title_menu(mut commands: Commands, font_assets: Res<FontAssets>) {
    let menu_item_height: f32 = 100.0;
    let menu_item_width: f32 = 400.0;
    let row_gap: f32 = 50.0;

    let menu_height: f32 = (menu_item_height * 2.0) + row_gap + 200.0; // 2つのメニューアイテムと間隔分
    let menu_width: f32 = menu_item_width + 100.0; // メニューアイテムの幅を基準にする

    let style = Style {
        top: Val::Percent(50.0),
        left: Val::Percent(50.0),
        margin: UiRect {
            top: Val::Px(-menu_height / 2.0), // 高さの半分 (400/2)
            left: Val::Px(-menu_width / 2.0), // 幅の半分 (400/2)
            ..Default::default()
        },
        width: Val::Px(menu_width),
        height: Val::Px(menu_height),
        row_gap: Val::Px(row_gap),
        flex_direction: FlexDirection::Column,
        align_items: AlignItems::Center,
        justify_content: JustifyContent::Center,
        ..Default::default()
    };

    let menu = MenuEntityBuilder::new("title_menu", TitleMenu)
        .style(style)
        .build(&mut commands);

    MenuItemEntityBuilder::new(
        "start_game",
        &TitleMenuItem::StartGame.to_string(),
        TitleMenuItem::StartGame,
        TitleMenuItemSelectAction::StartGame,
    )
    .style(Style {
        width: Val::Px(menu_item_width),
        height: Val::Px(menu_item_height),
        position_type: PositionType::Relative,
        ..Default::default()
    })
    .selected(true)
    .build_as_child(&mut commands, menu, &font_assets);

    MenuItemEntityBuilder::new(
        "high_score",
        &TitleMenuItem::HighScore.to_string(),
        TitleMenuItem::HighScore,
        TitleMenuItemSelectAction::HighScore,
    )
    .style(Style {
        width: Val::Px(menu_item_width),
        height: Val::Px(menu_item_height),
        position_type: PositionType::Relative,
        ..Default::default()
    })
    .build_as_child(&mut commands, menu, &font_assets);
}
