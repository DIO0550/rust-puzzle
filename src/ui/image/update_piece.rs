use bevy::{
    ecs::{
        component::Component,
        query::With,
        system::{Query, Res, Resource},
    },
    ui::UiImage,
};

use crate::asset::image::image_assets::ImageAssets;

use super::image_handle_resource::ImageHandleResource;

pub fn update_image<
    R: Resource,
    Assets: Resource + ImageAssets<R>,
    TargetResource: Resource + ImageHandleResource<Assets>,
    MarkerComponent: Component,
>(
    mut query: Query<&mut UiImage, With<MarkerComponent>>,
    resource: Res<TargetResource>,
    assets: Res<Assets>,
) {
    for mut image in query.iter_mut() {
        let image_handle = resource.image_handle_resource(&assets);
        image.texture = image_handle;
    }
}
