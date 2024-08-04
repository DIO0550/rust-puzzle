use bevy::{
    asset::AssetServer,
    ecs::system::Res,
    prelude::default,
    ui::{node_bundles::ImageBundle, Style, UiImage, Val},
};

use crate::asset::{
    asset::AssetTrait,
    image::image::{ImageAsset, ImageName},
};

pub struct ImageUI {}

pub trait ImageUITrait {
    fn image_bundle(
        image_name: ImageName,
        asset_server: &Res<AssetServer>,
        image_size: &f32,
    ) -> ImageBundle;
}

impl ImageUITrait for ImageUI {
    fn image_bundle(
        image_name: ImageName,
        asset_server: &Res<AssetServer>,
        image_size: &f32,
    ) -> ImageBundle {
        let image = ImageAsset::asset(asset_server, &image_name);
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
