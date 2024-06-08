use crate::{
    asset::{
        asset::AssetTrait,
        font::font::{FontAsset, FontName},
    },
    consts::color_theme::ColorTheme,
    game::component::on_display_over::OnDisplayGameOver,
};
use bevy::{
    asset::AssetServer,
    ecs::system::{Commands, Res},
    hierarchy::{BuildChildren, ChildBuilder},
    prelude::default,
    render::color::Color,
    text::{TextAlignment, TextSection, TextStyle},
    ui::{
        node_bundles::{NodeBundle, TextBundle},
        AlignItems, BackgroundColor, BorderColor, FlexDirection, JustifyContent, PositionType,
        Style, UiRect, Val,
    },
};

pub fn display_game_over(mut commands: Commands, asset_server: Res<AssetServer>) {}

fn game_over_menu(parent: &mut ChildBuilder, asset_server: &Res<AssetServer>) {
    parent
        .spawn(NodeBundle {
            style: Style {
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
            // border_color: BorderColor(ColorTheme::SPROUT),
            // background_color: BackgroundColor(ColorTheme::CHROME_WHITE),
            ..default()
        })
        .with_children(|parent| {
            parent
                .spawn(NodeBundle {
                    style: Style {
                        flex_direction: FlexDirection::Column,
                        justify_content: JustifyContent::Center,
                        align_items: AlignItems::Center,
                        width: Val::Px(300.0),
                        height: Val::Px(100.0),
                        ..default()
                    },
                    background_color: BackgroundColor(ColorTheme::SPROUT),
                    ..default()
                })
                .with_children(|parent| {
                    parent.spawn(TextBundle::from_sections([TextSection::new(
                        "リスタート",
                        TextStyle {
                            font: FontAsset::asset(&asset_server, &FontName::HachiMaruPopReg),
                            font_size: 50.,
                            color: Color::BLACK,
                            ..default()
                        },
                    )]));
                });

            parent
                .spawn(NodeBundle {
                    style: Style {
                        flex_direction: FlexDirection::Column,
                        justify_content: JustifyContent::Center,
                        align_items: AlignItems::Center,
                        width: Val::Px(300.0),
                        height: Val::Px(100.0),
                        ..default()
                    },
                    background_color: BackgroundColor(ColorTheme::SPROUT),
                    ..default()
                })
                .with_children(|parent| {
                    parent.spawn(TextBundle::from_sections([TextSection::new(
                        "タイトルに戻る",
                        TextStyle {
                            font: FontAsset::asset(&asset_server, &FontName::HachiMaruPopReg),
                            font_size: 50.,
                            color: Color::BLACK,
                            ..default()
                        },
                    )]));
                });
        });
}

pub fn display_game_over_menu(mut commands: Commands, asset_server: Res<AssetServer>) {
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
            OnDisplayGameOver,
        ))
        .with_children(|parent| {
            parent
                .spawn(NodeBundle {
                    style: Style {
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

                    border_color: BorderColor(ColorTheme::SPROUT),
                    background_color: BackgroundColor(ColorTheme::CHROME_WHITE),
                    ..default()
                })
                .with_children(|parent| {
                    parent
                        .spawn(NodeBundle {
                            style: Style { ..default() },
                            ..default()
                        })
                        .with_children(|parent| {
                            parent.spawn((TextBundle::from_sections([TextSection::new(
                                "GAME OVER",
                                TextStyle {
                                    font: FontAsset::asset(
                                        &asset_server,
                                        &FontName::HachiMaruPopReg,
                                    ),
                                    font_size: 100.,
                                    color: Color::BLACK,
                                    ..default()
                                },
                            )])
                            .with_text_alignment(TextAlignment::Center),));
                        });

                    game_over_menu(parent, &asset_server);
                });
        });
}
