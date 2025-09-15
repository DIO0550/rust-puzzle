use bevy::{
    ecs::system::Res,
    prelude::*,
};

pub trait GameImageBundle<T> {
    type ImageNameType;
    type AssetType: Resource;

    fn image_bundle(
        image_name: Self::ImageNameType,
        assets: &Res<Self::AssetType>,
        image_size: &T,
    ) -> impl Bundle;
}

pub trait GameImageBundleWithStyle {
    type ImageNameType;
    type AssetType: Resource;

    fn image_bundle(
        image_name: Self::ImageNameType,
        assets: &Res<Self::AssetType>,
        style: Node,
    ) -> impl Bundle;
}
