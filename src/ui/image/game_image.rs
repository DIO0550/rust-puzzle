use bevy::{
    ecs::system::Res,
    ui::{node_bundles::ImageBundle, Style, UiImage, Val},
    utils::default,
};

use crate::asset::image::{
    game_image_assets::GameImageAssets, image::ImageName, image_assets::ImageAssets,
};

use super::game_image_bundle::GameImageBundle;

pub struct GameImage {}

impl GameImageBundle<f32> for GameImage {
    type ImageNameType = ImageName;
    type AssetType = GameImageAssets;

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
