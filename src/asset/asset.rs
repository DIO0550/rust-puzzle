use bevy::{
    asset::{Asset, AssetServer, Handle},
    ecs::system::Res,
};

pub trait AssetNameTrait {
    fn asset_path(&self) -> String;
}

pub trait AssetTrait<T: AssetNameTrait> {
    fn asset<'a, A: Asset>(asset_server: &Res<AssetServer>, asset_name: &T) -> Handle<A> {
        asset_server.load(asset_name.asset_path())
    }
}
