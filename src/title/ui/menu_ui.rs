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

pub fn menu_item(
    parent: &mut ChildBuilder,
    title_menu_item: TitleMenuItem,
    asset_server: &Res<AssetServer>,
) {
    let title = title_menu_item.to_string();
    parent
        .spawn((
            NodeBundle {
                style: Style {
                    flex_direction: FlexDirection::Column,
                    justify_content: JustifyContent::Center,
                    align_items: AlignItems::Center,
                    column_gap: Val::Px(16.0),
                    row_gap: Val::Px(16.0),
                    padding: UiRect::all(Val::Px(32.0)),
                    border: UiRect::all(Val::Px(10.0)),
                    ..default()
                },

                border_color: BorderColor(ColorTheme::SPROUT),
                background_color: BackgroundColor(ColorTheme::CHROME_WHITE),
                ..default()
            },
            title_menu_item,
        ))
        .with_children(|parent| {
            parent
                .spawn(NodeBundle {
                    style: Style { ..default() },
                    ..default()
                })
                .with_children(|parent| {
                    parent.spawn((TextBundle::from_sections([TextSection::new(
                        title,
                        TextStyle {
                            font: FontAsset::asset(&asset_server, &FontName::HachiMaruPopReg),
                            font_size: 100.,
                            color: Color::BLACK,
                            ..default()
                        },
                    )])
                    .with_text_alignment(TextAlignment::Center),));
                });
        });
}

pub fn setup_title_menu(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    font_assets: Res<FontAssets>,
) {
    // commands
    //     .spawn((
    //         NodeBundle {
    //             style: Style {
    //                 flex_direction: FlexDirection::Column,
    //                 height: Val::Percent(100.),
    //                 width: Val::Percent(100.),
    //                 align_items: AlignItems::Center,
    //                 justify_content: JustifyContent::Center,
    //                 ..default()
    //             },

    //             ..default()
    //         },
    //         TitleMenu,
    //     ))
    //     .with_children(|parent| {
    //         menu_item(parent, TitleMenuItem::StartGame, &asset_server);
    //         menu_item(parent, TitleMenuItem::HighScore, &asset_server);
    //     });

    let menu_item = MenuEntityBuilder::new("title_menu", TitleMenu)
        .size(400.0, 400.0)
        .build(&mut commands);

    let start_game = MenuItemEntityBuilder::new(
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

    let high_score = MenuItemEntityBuilder::new(
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
