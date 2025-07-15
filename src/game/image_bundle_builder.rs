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

pub struct ImageBundleBuilder {}

pub struct ImageSize {
    height: f32,
    width: f32,
}

impl ImageSize {
    pub fn new(height: f32, width: f32) -> Self {
        Self {
            height: height,
            width: width,
        }
    }

    pub fn get_height(&self) -> &f32 {
        return &self.height;
    }

    pub fn get_width(&self) -> &f32 {
        return &self.width;
    }
}

pub trait IntoImageBundle<T> {
    fn into_image_bundle(
        image_name: ImageName,
        asset_server: &Res<AssetServer>,
        image_size: &T,
    ) -> ImageBundle;
}

impl IntoImageBundle<f32> for ImageBundleBuilder {
    fn into_image_bundle(
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

impl IntoImageBundle<ImageSize> for ImageBundleBuilder {
    fn into_image_bundle(
        image_name: ImageName,
        asset_server: &Res<AssetServer>,
        image_size: &ImageSize,
    ) -> ImageBundle {
        let image = ImageAsset::asset(asset_server, &image_name);
        let bundle = ImageBundle {
            style: Style {
                width: Val::Px((*image_size).width),
                height: Val::Px((*image_size).height),
                ..default()
            },
            image: UiImage::new(image),
            ..default()
        };

        return bundle;
    }
}
