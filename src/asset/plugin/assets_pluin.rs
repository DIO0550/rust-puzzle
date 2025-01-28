use bevy::app::{App, Plugin, PreStartup};

use crate::asset::system::assets_sysmtem::load_assets;

pub struct AssetsPlugin;

impl Plugin for AssetsPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(PreStartup, load_assets);
    }
}
