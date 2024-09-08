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

pub trait ImageUITrait<T> {
    fn image_bundle(
        image_name: ImageName,
        asset_server: &Res<AssetServer>,
        image_size: &T,
    ) -> ImageBundle;
}

impl ImageUITrait<f32> for ImageUI {
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

impl ImageUITrait<ImageSize> for ImageUI {
    fn image_bundle(
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
