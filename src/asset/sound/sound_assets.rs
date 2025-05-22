use bevy::{
    asset::{AssetServer, Handle},
    audio::AudioSource,
    ecs::system::Res,
};

pub trait FromAssetServer {
    fn new(asset_server: &Res<AssetServer>) -> Self;
}
pub trait SoundAssets<T> {
    fn handle_sounds_from(&self, asset_name: &T) -> Handle<AudioSource>;
}
