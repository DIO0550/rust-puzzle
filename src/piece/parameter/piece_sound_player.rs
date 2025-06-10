use bevy::{
    audio::{AudioBundle, PlaybackSettings},
    ecs::system::{Commands, Res, SystemParam},
};

use crate::asset::sound::piece_sound_assets::PieceSoundAssets;

#[derive(SystemParam)]
pub struct PieceSoundPlayer<'w, 's> {
    commands: Commands<'w, 's>,
    piece_sound_assets: Res<'w, PieceSoundAssets>,
}

impl<'w, 's> PieceSoundPlayer<'w, 's> {
    pub fn play_union_sound(&mut self) {
        self.commands.spawn(AudioBundle {
            source: self.piece_sound_assets.piece_union.clone(),
            settings: PlaybackSettings::DESPAWN,
        });
    }

    pub fn play_fall_sound(&mut self) {
        self.commands.spawn(AudioBundle {
            source: self.piece_sound_assets.piece_fall.clone(),
            settings: PlaybackSettings::DESPAWN,
        });
    }
}
