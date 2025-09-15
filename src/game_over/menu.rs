use crate::{
    asset::{
        asset::AssetTrait,
        font::font::{FontAsset, FontName},
    },
    consts::color_theme::ColorTheme,
    game_over::menu_selection::GameOverMenuSelection,
};
use bevy::prelude::*;

#[derive(Component)]
pub struct GameOverScreen;

#[derive(Component)]
pub struct GameOverMenuItem;

#[derive(Component, PartialEq, Eq)]
pub enum GameOverMenu {
    Restart,
    GoTitle,
}

pub fn display_game_over(mut commands: Commands, asset_server: Res<AssetServer>) {
    commands
        .spawn((
            Node {
                flex_direction: FlexDirection::Column,
                height: Val::Percent(100.),
                width: Val::Percent(100.),
                align_items: AlignItems::Center,
                justify_content: JustifyContent::Center,
                ..default()
            },
            GameOverScreen,
        ))
        .with_children(|parent| {
            parent
                .spawn((
                    Node {
                        flex_direction: FlexDirection::Column,
                        justify_content: JustifyContent::Center,
                        align_items: AlignItems::Center,
                        column_gap: Val::Px(16.0),
                        row_gap: Val::Px(16.0),
                        padding: UiRect::all(Val::Px(32.0)),
                        border: UiRect {
                            top: Val::Px(10.0),
                            left: Val::Px(10.0),
                            bottom: Val::Px(10.0),
                            right: Val::Px(10.0),
                        },
                        ..default()
                    },
                    BorderColor(ColorTheme::SPROUT),
                    BackgroundColor(ColorTheme::CHROME_WHITE),
                ))
                .with_children(|parent| {
                    // GAME OVER text
                    parent.spawn((
                        Text::new("GAME OVER"),
                        TextFont {
                            font: FontAsset::asset(&asset_server, &FontName::HachiMaruPopReg),
                            font_size: 100.,
                            ..default()
                        },
                        TextColor(Color::BLACK),
                        TextLayout::new_with_justify(JustifyText::Center),
                    ));

                    // Game Over Menu
                    parent
                        .spawn((
                            Node {
                                flex_direction: FlexDirection::Column,
                                align_items: AlignItems::Center,
                                column_gap: Val::Px(16.0),
                                row_gap: Val::Px(16.0),
                                padding: UiRect::all(Val::Px(32.0)),
                                border: UiRect {
                                    top: Val::Px(10.0),
                                    left: Val::Px(10.0),
                                    bottom: Val::Px(10.0),
                                    right: Val::Px(10.0),
                                },
                                ..default()
                            },
                            BorderColor(ColorTheme::SPROUT),
                            BackgroundColor(ColorTheme::CHROME_WHITE),
                        ))
                        .with_children(|parent| {
                            // Restart button
                            parent
                                .spawn((
                                    Node {
                                        flex_direction: FlexDirection::Column,
                                        justify_content: JustifyContent::Center,
                                        align_items: AlignItems::Center,
                                        width: Val::Px(300.0),
                                        height: Val::Px(100.0),
                                        ..default()
                                    },
                                    BackgroundColor(ColorTheme::SPROUT),
                                    GameOverMenu::Restart,
                                ))
                                .with_children(|parent| {
                                    parent.spawn((
                                        Text::new("リスタート"),
                                        TextFont {
                                            font: FontAsset::asset(
                                                &asset_server,
                                                &FontName::HachiMaruPopReg,
                                            ),
                                            font_size: 50.,
                                            ..default()
                                        },
                                        TextColor(Color::BLACK),
                                        GameOverMenu::Restart,
                                    ));
                                });

                            // Go to title button
                            parent
                                .spawn((
                                    Node {
                                        flex_direction: FlexDirection::Column,
                                        justify_content: JustifyContent::Center,
                                        align_items: AlignItems::Center,
                                        width: Val::Px(300.0),
                                        height: Val::Px(100.0),
                                        ..default()
                                    },
                                    BackgroundColor(ColorTheme::SPROUT),
                                    GameOverMenu::GoTitle,
                                ))
                                .with_children(|parent| {
                                    parent.spawn((
                                        Text::new("タイトルに戻る"),
                                        TextFont {
                                            font: FontAsset::asset(
                                                &asset_server,
                                                &FontName::HachiMaruPopReg,
                                            ),
                                            font_size: 50.,
                                            ..default()
                                        },
                                        TextColor(Color::BLACK),
                                        GameOverMenu::GoTitle,
                                    ));
                                });
                        });
                });
        });
}

pub fn update_menu(
    mut menu_query: Query<(&mut BackgroundColor, &GameOverMenu)>,
    mut menu_text_query: Query<(&mut TextColor, &GameOverMenu)>,
    select_menu_res: Res<GameOverMenuSelection>,
) {
    for (mut style, menu) in menu_query.iter_mut() {
        style.0 = match select_menu_res.0 == *menu {
            true => ColorTheme::NORWAY,
            false => ColorTheme::SPROUT,
        };
    }

    for (mut menu_text_color, menu) in menu_text_query.iter_mut() {
        menu_text_color.0 = match select_menu_res.0 == *menu {
            true => Color::WHITE,
            false => Color::BLACK,
        };
    }
}
