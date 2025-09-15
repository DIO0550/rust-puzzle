use bevy::prelude::*;

pub trait ImageHandleResource<A: Resource> {
    fn image_handle_resource(&self, assets: &Res<A>) -> Handle<Image>;
}
