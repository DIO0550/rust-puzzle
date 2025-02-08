use bevy::{
    asset::{Asset, AssetId, AssetServer, Handle},
    ecs::system::Res,
};

pub trait AssetNameTrait {
    fn asset_path(&self) -> String;
}

pub trait AssetIdCollection<A: Asset> {
    fn assets_ids(&self) -> Vec<AssetId<A>>;
}

pub trait AssetLoadTrait<A: Asset>
where
    Self: AssetIdCollection<A>,
{
    fn is_loaded(&self, asset_server: &Res<AssetServer>) -> bool {
        self.assets_ids()
            .iter()
            .all(|id| asset_server.is_loaded_with_dependencies(*id))
    }
}

pub trait AssetTrait<T: AssetNameTrait, A: Asset> {
    fn asset(asset_server: &Res<AssetServer>, asset_name: &T) -> Handle<A> {
        asset_server.load(asset_name.asset_path())
    }
}
