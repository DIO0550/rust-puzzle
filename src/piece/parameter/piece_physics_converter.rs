use bevy::ecs::{
    entity::Entity,
    system::{Commands, SystemParam},
};
use bevy_rapier2d::prelude::{ActiveEvents, Collider, ColliderMassProperties, RigidBody, Sleeping};

use crate::{
    consts::consts::UNIT_WIDTH,
    piece::component::animal_piece::animal_piece_component::AnimalPieceComponent,
};

#[derive(SystemParam)]
pub struct PiecePhysicsConverter<'w, 's> {
    commands: Commands<'w, 's>,
}

impl<'w, 's> PiecePhysicsConverter<'w, 's> {
    pub fn convert_to_physical(&mut self, entity: Entity, piece: &AnimalPieceComponent) {
        self.commands
            .entity(entity)
            .insert(RigidBody::Dynamic)
            .insert(Collider::ball(
                piece.animal_piece.get_size().to_f32() * 2.0 * UNIT_WIDTH,
            ))
            .insert(ActiveEvents::COLLISION_EVENTS)
            .insert(ColliderMassProperties::Mass(50.0))
            .insert(Sleeping::disabled());
    }
}
