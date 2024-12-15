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
        font::font::{FontAsset, FontName},
    },
    consts::color_theme::ColorTheme,
    title::component::{title_menu::TitleMenu, title_menu_item::TitleMenuItem},
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

pub fn setup_title_menu(mut commands: Commands, asset_server: Res<AssetServer>) {
    commands
        .spawn((
            NodeBundle {
                style: Style {
                    flex_direction: FlexDirection::Column,
                    height: Val::Percent(100.),
                    width: Val::Percent(100.),
                    align_items: AlignItems::Center,
                    justify_content: JustifyContent::Center,
                    ..default()
                },

                ..default()
            },
            TitleMenu,
        ))
        .with_children(|parent| {
            menu_item(parent, TitleMenuItem::StartGame, &asset_server);
            menu_item(parent, TitleMenuItem::HighScore, &asset_server);
        });
}
