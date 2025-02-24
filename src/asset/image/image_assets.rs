use bevy::{
    asset::{AssetServer, Handle},
    ecs::system::Res,
    render::texture::Image,
};

pub trait FromAssetServer {
    fn new(asset_server: &Res<AssetServer>) -> Self;
}
pub trait ImageAssets<T> {
    fn handle_image_from(&self, asset_name: &T) -> Handle<Image>;
}
