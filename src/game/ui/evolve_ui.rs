use bevy::{
    asset::AssetServer,
    ecs::system::{Commands, Res},
    hierarchy::BuildChildren,
    prelude::default,
    render::color::Color,
    text::{TextSection, TextStyle},
    ui::{
        node_bundles::{ImageBundle, NodeBundle, TextBundle},
        FlexDirection, PositionType, Style, UiImage, UiRect, Val,
    },
};

use crate::{
    asset::{
        asset::AssetTrait,
        font::font::{FontAsset, FontName},
        image::image::{PieceImageAsset, PieceImageName},
    },
    piece::component::animal_piece::{piece_image::PieceImage, piece_type::PieceType},
};

use super::piece_ui::{PieceImageUITrait, PieceUI};

fn piece_image_bundle(
    asset_server: &Res<AssetServer>,
    piece_image_name: &PieceImageName,
) -> ImageBundle {
    let piece_image_size = 50.0;

    let bundle = ImageBundle {
        style: Style {
            width: Val::Px(piece_image_size),
            height: Val::Px(piece_image_size),
            ..default()
        },
        image: UiImage::new(PieceImageAsset::asset(&asset_server, &piece_image_name)),
        ..default()
    };

    return bundle;
}

pub fn evolve_describe(mut commands: Commands, asset_server: Res<AssetServer>) {
    let image_size = 50.0;
    commands
        .spawn((NodeBundle {
            style: Style {
                position_type: PositionType::Absolute,
                right: Val::Px(50.),
                bottom: Val::Px(50.),
                flex_direction: FlexDirection::Column,
                ..default()
            },
            ..default()
        },))
        .with_children(|parent| {
            parent.spawn((TextBundle::from_sections([TextSection::new(
                "進化の順番",
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
                    parent.spawn(
                        PieceUI::new(PieceType::Cat).image_bundle(&asset_server, &image_size),
                    );
                    parent.spawn(
                        PieceUI::new(PieceType::Dog).image_bundle(&asset_server, &image_size),
                    );
                    parent.spawn(
                        PieceUI::new(PieceType::Penguin).image_bundle(&asset_server, &image_size),
                    );
                });
        });
}
