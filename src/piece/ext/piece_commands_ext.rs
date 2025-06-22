use bevy::{ecs::entity::Entity, math::Vec2};
use bevy_rapier2d::prelude::{
    ActiveEvents, Collider, ColliderMassProperties, GravityScale, RigidBody, Sleeping, Velocity,
};

use crate::{
    consts::consts::UNIT_WIDTH,
    piece::component::animal_piece::animal_piece_component::AnimalPieceComponent,
};

pub trait PieceCommandsExt {
    fn convert_to_physical(&mut self, entity: Entity, piece: &AnimalPieceComponent);
}

impl PieceCommandsExt for bevy::ecs::system::Commands<'_, '_> {
    fn convert_to_physical(&mut self, entity: Entity, piece: &AnimalPieceComponent) {
        self.entity(entity)
            .insert(Velocity {
                linvel: Vec2::new(0.0, 0.0),
                angvel: 0.0,
            })
            .insert(GravityScale(10.0))
            .insert(RigidBody::Dynamic)
            .insert(Collider::ball(
                piece.animal_piece.get_size().to_f32() * 2.0 * UNIT_WIDTH,
            ))
            .insert(ActiveEvents::COLLISION_EVENTS)
            .insert(ColliderMassProperties::Mass(50.0))
            .insert(Sleeping::disabled());
    }
}
