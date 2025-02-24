use bevy::{
    asset::AssetServer,
    ecs::system::{Commands, Res},
    hierarchy::{BuildChildren, ChildBuilder},
    prelude::default,
    render::color::Color,
    text::{TextSection, TextStyle},
    ui::{
        node_bundles::{NodeBundle, TextBundle},
        AlignItems, FlexDirection, JustifyContent, PositionType, Style, UiRect, Val,
    },
};

use crate::{
    asset::{
        asset::AssetTrait,
        font::font::{FontAsset, FontName},
        image::{image::ImageName, piece_image_assets::PieceImageAssets},
    },
    piece::component::animal_piece::piece_type::PieceType,
    ui::image::{game_image_bundle::GameImageBundle, piece_image::PieceImage},
};

use super::image_ui::{ImageUI, ImageUITrait};

fn evolve_title_text(child_builder: &mut ChildBuilder, asset_server: &Res<AssetServer>) {
    child_builder
        .spawn(NodeBundle {
            style: Style {
                flex_direction: FlexDirection::Column,
                justify_content: JustifyContent::Center,
                align_items: AlignItems::Center,
                ..default()
            },
            ..default()
        })
        .with_children(|parent| {
            parent.spawn((TextBundle::from_sections([TextSection::new(
                "進化順",
                TextStyle {
                    font: FontAsset::asset(&asset_server, &FontName::HachiMaruPopReg),
                    font_size: 50.,
                    color: Color::BLACK,
                    ..default()
                },
            )]),));
        });
}

pub fn evolve_describe(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    piece_image_assets: Res<PieceImageAssets>,
) {
    let image_size = 50.0;
    let mut piece_evolve_image_bundle =
        ImageUI::image_bundle(ImageName::PieceEvolve, &asset_server, &(image_size * 7.0));

    piece_evolve_image_bundle.style = Style {
        flex_direction: FlexDirection::Column,
        ..piece_evolve_image_bundle.style
    };
    commands
        .spawn((NodeBundle {
            style: Style {
                position_type: PositionType::Absolute,
                right: Val::Px(15.),
                bottom: Val::Px(15.),
                flex_direction: FlexDirection::Column,
                ..default()
            },
            ..default()
        },))
        .with_children(|parent| {
            evolve_title_text(parent, &asset_server);
        })
        .with_children(|parent| {
            parent
                .spawn(piece_evolve_image_bundle)
                .with_children(|parent| {
                    parent
                        .spawn(NodeBundle {
                            style: Style {
                                margin: UiRect {
                                    left: (Val::Px(50.0)),
                                    right: (Val::Px(42.0)),
                                    top: (Val::Px(40.0)),
                                    bottom: (Val::Px(0.0)),
                                },

                                row_gap: Val::Px(60.0),
                                flex_direction: FlexDirection::Column,
                                ..default()
                            },
                            ..default()
                        })
                        .with_children(|parent| {
                            parent
                                .spawn(NodeBundle {
                                    style: Style {
                                        width: Val::Percent(100.),
                                        flex_direction: FlexDirection::Row,
                                        justify_content: JustifyContent::SpaceBetween,
                                        ..default()
                                    },
                                    ..default()
                                })
                                .with_children(|parent| {
                                    parent.spawn(PieceImage::image_bundle(
                                        PieceType::Rat,
                                        &piece_image_assets,
                                        &image_size,
                                    ));
                                    parent.spawn(PieceImage::image_bundle(
                                        PieceType::Cat,
                                        &piece_image_assets,
                                        &image_size,
                                    ));
                                    parent.spawn(PieceImage::image_bundle(
                                        PieceType::Dog,
                                        &piece_image_assets,
                                        &image_size,
                                    ));
                                });
                            parent
                                .spawn(NodeBundle {
                                    style: Style {
                                        width: Val::Percent(100.),
                                        flex_direction: FlexDirection::Row,
                                        justify_content: JustifyContent::SpaceBetween,
                                        ..default()
                                    },
                                    ..default()
                                })
                                .with_children(|parent| {
                                    parent.spawn(PieceImage::image_bundle(
                                        PieceType::Elephant,
                                        &piece_image_assets,
                                        &image_size,
                                    ));
                                    parent.spawn(PieceImage::image_bundle(
                                        PieceType::Penguin,
                                        &piece_image_assets,
                                        &image_size,
                                    ));
                                });
                            parent
                                .spawn(NodeBundle {
                                    style: Style {
                                        width: Val::Percent(100.),
                                        flex_direction: FlexDirection::Row,
                                        justify_content: JustifyContent::SpaceBetween,
                                        ..default()
                                    },
                                    ..default()
                                })
                                .with_children(|parent| {
                                    parent.spawn(PieceImage::image_bundle(
                                        PieceType::Horse,
                                        &piece_image_assets,
                                        &image_size,
                                    ));
                                    parent.spawn(PieceImage::image_bundle(
                                        PieceType::Panda,
                                        &piece_image_assets,
                                        &image_size,
                                    ));
                                    parent.spawn(PieceImage::image_bundle(
                                        PieceType::Giraffe,
                                        &piece_image_assets,
                                        &image_size,
                                    ));
                                });
                        });
                });
        });
}
