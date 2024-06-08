use crate::{
    asset::{
        asset::AssetTrait,
        font::font::{FontAsset, FontName},
    },
    game::component::on_display_over::OnDisplayGameOver,
};
use bevy::{
    asset::AssetServer,
    ecs::system::{Commands, Res},
    hierarchy::BuildChildren,
    prelude::default,
    render::color::Color,
    text::{TextAlignment, TextSection, TextStyle},
    ui::{
        node_bundles::{NodeBundle, TextBundle},
        AlignItems, BackgroundColor, FlexDirection, JustifyContent, PositionType, Style, Val,
    },
};

pub fn display_game_over(mut commands: Commands, asset_server: Res<AssetServer>) {
    commands
        .spawn((
            NodeBundle {
                style: Style {
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
                    style: Style { ..default() },
                    ..default()
                })
                .with_children(|parent| {
                    parent.spawn((TextBundle::from_sections([TextSection::new(
                        "GAME OVER",
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
