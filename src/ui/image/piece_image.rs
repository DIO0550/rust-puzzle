use bevy::prelude::*;

use crate::{
    asset::image::{image_assets::ImageAssets, piece_image_assets::PieceImageAssets},
    piece::component::animal_piece::piece_type::PieceType,
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
    ) -> impl Bundle {
        let image = assets.handle_image_from(&image_name);
        (
            Node {
                width: Val::Px(*image_size),
                height: Val::Px(*image_size),
                ..default()
            },
            ImageNode::new(image),
        )
    }
}

impl GameImageBundleWithStyle for PieceImage {
    type ImageNameType = PieceType;
    type AssetType = PieceImageAssets;

    fn image_bundle(
        image_name: Self::ImageNameType,
        assets: &Res<Self::AssetType>,
        style: Node,
    ) -> impl Bundle {
        let image = assets.handle_image_from(&image_name);
        (style, ImageNode::new(image))
    }
}
