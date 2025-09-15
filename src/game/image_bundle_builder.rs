use bevy::{
    asset::{AssetServer, Handle},
    ecs::system::Res,
    prelude::Image,
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
        Self { height, width }
    }

    pub fn get_height(&self) -> &f32 {
        &self.height
    }

    pub fn get_width(&self) -> &f32 {
        &self.width
    }
}

pub trait IntoImageBundle<T> {
    fn into_image_bundle(
        image_name: ImageName,
        asset_server: &Res<AssetServer>,
        image_size: &T,
    ) -> Handle<Image>;
}

impl IntoImageBundle<f32> for ImageBundleBuilder {
    fn into_image_bundle(
        image_name: ImageName,
        asset_server: &Res<AssetServer>,
        _image_size: &f32,
    ) -> Handle<Image> {
        ImageAsset::asset(asset_server, &image_name)
    }
}

impl IntoImageBundle<ImageSize> for ImageBundleBuilder {
    fn into_image_bundle(
        image_name: ImageName,
        asset_server: &Res<AssetServer>,
        _image_size: &ImageSize,
    ) -> Handle<Image> {
        ImageAsset::asset(asset_server, &image_name)
    }
}
