use bevy::{
    asset::AssetServer,
    ecs::{component::Component, query::With, system::*},
    hierarchy::BuildChildren,
    prelude::{default, Query},
    render::color::Color,
    text::*,
    ui::{node_bundles::*, *},
};

use crate::{
    asset::{
        asset::AssetTrait,
        font::font::{FontAsset, FontName},
    },
    game::ui::piece_ui::{PieceImageUITrait, PieceUI},
    piece::{component::animal_piece::piece_image::PieceImage, resource::next_piece::NextPiece},
};

const PIECE_IMAGE_SIZE: f32 = 50.0;

#[derive(Component)]
pub struct NextPieceImage;

pub fn setup_display_next_piece(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    next_piece_res: Res<NextPiece>,
) {
    let piece_image_bundle =
        PieceUI::new(next_piece_res.0).image_bundle(&asset_server, &PIECE_IMAGE_SIZE);

    commands
        .spawn((NodeBundle {
            style: Style {
                position_type: PositionType::Absolute,
                right: Val::Px(50.),
                top: Val::Px(50.),
                flex_direction: FlexDirection::Column,
                ..default()
            },
            ..default()
        },))
        .with_children(|parent| {
            parent.spawn((TextBundle::from_sections([TextSection::new(
                "Next",
                TextStyle {
                    font: FontAsset::asset(&asset_server, &FontName::HachiMaruPopReg),
                    font_size: 50.,
                    color: Color::BLACK,
                    ..default()
                },
            )]),));
        })
        .with_children(|parent| {
            parent
                .spawn(NodeBundle {
                    style: Style {
                        flex_direction: FlexDirection::Row,
                        ..default()
                    },
                    ..default()
                })
                .with_children(|parent| {
                    parent.spawn((piece_image_bundle, NextPieceImage));
                });
        });
}

pub fn update_display_next_piece(
    mut next_piece_image: Query<&mut UiImage, With<NextPieceImage>>,
    asset_server: Res<AssetServer>,
    next_piece_res: Res<NextPiece>,
) {
    let mut next_piece_image = next_piece_image.single_mut();

    let image = PieceImage::from_piece_type(&asset_server, &next_piece_res.0);
    next_piece_image.texture = image;
}
