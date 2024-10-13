use bevy::app::{App, Plugin, Startup};

use crate::asset::system::piece_sound_system::setup_piece_sound;

pub struct PieceSoundPlugin;
impl Plugin for PieceSoundPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, setup_piece_sound);
    }
}
