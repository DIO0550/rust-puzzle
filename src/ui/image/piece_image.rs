use bevy::{
    ecs::system::Res,
    prelude::default,
    ui::{node_bundles::ImageBundle, Style, UiImage, Val},
};

use crate::{
    asset::image::{image_assets::ImageAssets, piece_image_assets::PieceImageAssets},
    piece::component::animal_piece::{animal_piece::Piece, piece_type::PieceType},
};

use super::game_image_bundle::{GameImageBundle, GameImageBundleWithStyle};

pub struct PieceImage {}

impl GameImageBundle<f32> for PieceImage {
    type ImageNameType = PieceType;
    type AssetType = PieceImageAssets;

    fn image_bundle(
        image_name: Self::ImageNameType,
        assets: &Res<Self::AssetType>,
        image_size: &f32,
    ) -> ImageBundle {
        let image = assets.handle_image_from(&image_name);
        let bundle = ImageBundle {
            style: Style {
                width: Val::Px(*image_size),
                height: Val::Px(*image_size),
                ..default()
            },
            image: UiImage::new(image),
            ..default()
        };

        return bundle;
    }
}

impl GameImageBundleWithStyle for PieceImage {
    type ImageNameType = PieceType;
    type AssetType = PieceImageAssets;

    fn image_bundle(
        image_name: Self::ImageNameType,
        assets: &Res<Self::AssetType>,
        style: Style,
    ) -> ImageBundle {
        let image = assets.handle_image_from(&image_name);
        let bundle = ImageBundle {
            style,
            image: UiImage::new(image),
            ..default()
        };

        return bundle;
    }
}
