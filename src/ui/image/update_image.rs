use bevy::{
    ecs::{
        change_detection::DetectChanges,
        component::Component,
        query::With,
        system::{Query, Res, Resource},
    },
    ui::UiImage,
};

use super::image_handle_resource::ImageHandleResource;

pub fn update_image<
    Assets: Resource,
    TargetResource: Resource + ImageHandleResource<Assets>,
    MarkerComponent: Component,
>(
    mut query: Query<&mut UiImage, With<MarkerComponent>>,
    resource: Res<TargetResource>,
    assets: Res<Assets>,
) {
    if !resource.is_changed() {
        return;
    }

    for mut image in query.iter_mut() {
        let image_handle = resource.image_handle_resource(&assets);
        image.texture = image_handle;
    }
}
