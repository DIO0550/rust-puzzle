use bevy::{
    app::{App, Plugin, PreStartup, Update},
    ecs::schedule::{common_conditions::in_state, IntoSystemConfigs},
};

use crate::{
    asset::{
        image::image_assets::ImageAssets,
        state::asset_load_state::AssetLoadState,
        system::assets_sysmtem::{check_assets_ready, load_assets},
    },
    piece::component::animal_piece::piece_image::PieceImage,
};

pub struct AssetsPlugin;

impl Plugin for AssetsPlugin {
    fn build(&self, app: &mut App) {
        app.add_state::<AssetLoadState>()
            .add_systems(PreStartup, load_assets)
            .add_systems(
                Update,
                (check_assets_ready).run_if(in_state(AssetLoadState::Loading)),
            );
    }
}
