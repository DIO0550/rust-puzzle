use bevy::{
    asset::Handle,
    ecs::system::{Res, Resource},
    render::texture::Image,
};

pub trait ImageHandleResource<A: Resource> {
    fn image_handle_resource(&self, assets: &Res<A>) -> Handle<Image>;
}
