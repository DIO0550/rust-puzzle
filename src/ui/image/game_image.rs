use bevy::prelude::*;

use crate::asset::image::{
    game_image_assets::GameImageAssets, image::ImageName, image_assets::ImageAssets,
};

use super::game_image_bundle::{GameImageBundle, GameImageBundleWithStyle};

pub struct GameImage {}

impl GameImageBundle<f32> for GameImage {
    type ImageNameType = ImageName;
    type AssetType = GameImageAssets;

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

impl GameImageBundleWithStyle for GameImage {
    type ImageNameType = ImageName;
    type AssetType = GameImageAssets;

    fn image_bundle(
        image_name: Self::ImageNameType,
        assets: &Res<Self::AssetType>,
        style: Node,
    ) -> impl Bundle {
        let image = assets.handle_image_from(&image_name);
        (style, ImageNode::new(image))
    }
}
