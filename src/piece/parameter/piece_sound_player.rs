use bevy::{
    audio::{AudioBundle, PlaybackSettings},
    ecs::{
        entity::Entity,
        system::{Commands, Res, SystemParam},
    },
};
use bevy_rapier2d::prelude::{ActiveEvents, Collider, ColliderMassProperties, RigidBody, Sleeping};

use crate::{
    asset::sound::piece_sound_assets::PieceSoundAssets, consts::consts::UNIT_WIDTH,
    piece::component::animal_piece::animal_piece_component::AnimalPieceComponent,
};

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
