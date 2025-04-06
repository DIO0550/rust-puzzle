use bevy::{
    ecs::system::Res,
    prelude::Resource,
    ui::{node_bundles::ImageBundle, Style},
};

pub trait GameImageBundle<T> {
    type ImageNameType;
    type AssetType: Resource;

    fn image_bundle(
        image_name: Self::ImageNameType,
        assets: &Res<Self::AssetType>,
        image_size: &T,
    ) -> ImageBundle;
}

pub trait GameImageBundleWithStyle {
    type ImageNameType;
    type AssetType: Resource;

    fn image_bundle(
        image_name: Self::ImageNameType,
        assets: &Res<Self::AssetType>,
        style: Style,
    ) -> ImageBundle;
}
