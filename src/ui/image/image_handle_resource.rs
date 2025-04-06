use bevy::{
    asset::Handle,
    ecs::system::{Res, Resource},
    render::texture::Image,
};

use crate::asset::image::image_assets::ImageAssets;

pub trait ImageHandleResource<A: Resource> {
    fn image_handle_resource(&self, assets: &Res<A>) -> Handle<Image>;
}
