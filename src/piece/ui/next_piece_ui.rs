use bevy::{
    asset::AssetServer,
    ecs::{component::Component, query::With, system::*},
    hierarchy::{BuildChildren, ChildBuilder},
    prelude::{default, Query},
    render::color::Color,
    text::*,
    ui::{node_bundles::*, *},
};

use crate::{
    asset::{
        asset::AssetTrait,
        font::font::{FontAsset, FontName},
        image::{
            image::ImageName, image_assets::ImageAssets, piece_image_assets::PieceImageAssets,
        },
    },
    game::ui::image_ui::{ImageUI, ImageUITrait},
    piece::resource::next_piece::NextPiece,
    ui::image::{game_image_bundle::GameImageBundle, piece_image::PieceImage},
};

const PIECE_IMAGE_SIZE: f32 = 100.0;

#[derive(Component)]
pub struct NextPieceImage;

fn next_piece_icon(
    child_builder: &mut ChildBuilder,
    piece_image_assets: &Res<PieceImageAssets>,
    next_piece_res: &Res<NextPiece>,
) {
    let piece_image_bundle =
        PieceImage::image_bundle(next_piece_res.0, piece_image_assets, &PIECE_IMAGE_SIZE);

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
            parent.spawn((piece_image_bundle, NextPieceImage));
        });
}

fn next_piece_title(child_builder: &mut ChildBuilder, asset_server: &Res<AssetServer>) {
    child_builder
        .spawn(NodeBundle {
            style: Style {
                flex_direction: FlexDirection::Row,
                margin: UiRect {
                    left: Val::Px(0.0),
                    right: Val::Px(0.0),
                    top: Val::Px(65.0),
                    bottom: Val::Px(0.0),
                },
                ..default()
            },
            ..default()
        })
        .with_children(|parent| {
            parent.spawn((TextBundle::from_sections([TextSection::new(
                "Next",
                TextStyle {
                    font: FontAsset::asset(&asset_server, &FontName::HachiMaruPopReg),
                    font_size: 50.,
                    color: Color::WHITE,
                    ..default()
                },
            )]),));
        });
}

pub fn setup_display_next_piece(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    piece_image_assets: Res<PieceImageAssets>,
    next_piece_res: Res<NextPiece>,
) {
    let image_size = 250.0;
    let mut piece_image_bundle =
        ImageUI::image_bundle(ImageName::CatSilhouette, &asset_server, &image_size);

    piece_image_bundle.style = Style {
        position_type: PositionType::Absolute,
        right: Val::Px(50.),
        top: Val::Px(65.),
        height: Val::Px(image_size),
        width: Val::Px(image_size),
        flex_direction: FlexDirection::Column,
        align_items: AlignItems::Center,
        ..piece_image_bundle.style
    };
    commands
        .spawn(piece_image_bundle)
        .with_children(|parent| {
            next_piece_title(parent, &asset_server);
        })
        .with_children(|parent| next_piece_icon(parent, &piece_image_assets, &next_piece_res));
}

pub fn update_display_next_piece(
    mut next_piece_image: Query<&mut UiImage, With<NextPieceImage>>,
    piece_image_assets: Res<PieceImageAssets>,
    next_piece_res: Res<NextPiece>,
) {
    let mut next_piece_image = next_piece_image.single_mut();

    let image = piece_image_assets.handle_image_from(&next_piece_res.0);
    next_piece_image.texture = image;
}
