use bevy::ecs::{
    component::Component,
    entity::Entity,
    system::{Commands, SystemParam},
};
use bevy_rapier2d::prelude::GravityScale;

use crate::piece::{component::active_piece::ActivePiece, sound::PieceSoundPlayer};

#[derive(Component)]
pub struct Falling;

#[derive(SystemParam)]
pub struct PieceFaller<'w, 's> {
    commands: Commands<'w, 's>,
    piece_sound_player: PieceSoundPlayer<'w, 's>,
}

impl<'w, 's> PieceFaller<'w, 's> {
    pub fn make_falling(&mut self, entity: Entity) {
        self.commands
            .entity(entity)
            .remove::<ActivePiece>()
            .insert(GravityScale(10.0))
            .insert(Falling);

        self.piece_sound_player.play_fall_sound();
    }
}
