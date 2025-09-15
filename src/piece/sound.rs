use bevy::{audio::PlaybackSettings, ecs::system::SystemParam, prelude::*};

use crate::asset::sound::piece_sound_assets::PieceSoundAssets;

#[derive(SystemParam)]
pub struct PieceSoundPlayer<'w, 's> {
    commands: Commands<'w, 's>,
    piece_sound_assets: Res<'w, PieceSoundAssets>,
}

impl<'w, 's> PieceSoundPlayer<'w, 's> {
    pub fn play_union_sound(&mut self) {
        self.commands.spawn((
            AudioPlayer::new(self.piece_sound_assets.piece_union.clone()),
            PlaybackSettings::DESPAWN,
        ));
    }

    pub fn play_fall_sound(&mut self) {
        self.commands.spawn((
            AudioPlayer::new(self.piece_sound_assets.piece_fall.clone()),
            PlaybackSettings::DESPAWN,
        ));
    }
}
