use bevy::ecs::entity::Entity;

use crate::piece::component::animal_piece::animal_piece_component::AnimalPieceComponent;

pub trait PieceCommandsExt {
    fn convert_to_physical(&mut self, entity: Entity, piece: &AnimalPieceComponent);
}

impl PieceCommandsExt for bevy::ecs::system::Commands<'_, '_> {
    fn convert_to_physical(&mut self, entity: Entity, piece: &AnimalPieceComponent) {
        self.entity(entity)
            .insert(bevy_rapier2d::prelude::RigidBody::Dynamic)
            .insert(bevy_rapier2d::prelude::Collider::ball(
                piece.animal_piece.get_size().to_f32() * 2.0,
            ))
            .insert(bevy_rapier2d::prelude::ActiveEvents::COLLISION_EVENTS)
            .insert(bevy_rapier2d::prelude::ColliderMassProperties::Mass(50.0))
            .insert(bevy_rapier2d::prelude::Sleeping::disabled());
    }
}
