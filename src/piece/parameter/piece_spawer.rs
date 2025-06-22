use bevy::{
    ecs::{
        entity::Entity,
        system::{Commands, SystemParam},
    },
    transform::{components::Transform, TransformBundle},
};
use bevy_rapier2d::prelude::ActiveCollisionTypes;

use crate::{
    consts::consts::{BOX_SIZE_HEIHT, PIECE_POSITION_Y_MARGIN},
    piece::{
        component::{
            active_piece::ActivePiece,
            animal_piece::animal_piece_component::{
                AnimalPieceComponent, AnimalPieceComponentGenerator,
            },
        },
        system::piece_renderer::PieceRenderer,
    },
    resource::drop_postion::DropPositionController,
};

#[derive(SystemParam)]
pub struct PieceSpawner<'w, 's> {
    commands: Commands<'w, 's>,
    piece_renderer: PieceRenderer<'w>,
    animal_piece_generator: AnimalPieceComponentGenerator<'w>,
    grab_position_manager: DropPositionController<'w>,
}

impl<'w, 's> PieceSpawner<'w, 's> {
    pub fn spawn(&mut self) -> Entity {
        let animal_piece_component = self.animal_piece_generator.generate();
        self.grab_position_manager
            .set_grab_position(animal_piece_component.animal_piece.as_ref());
        let new_grab_postion = self.grab_position_manager.grab_position.x;
        let material_mesh = self
            .piece_renderer
            .create_material_mesh(&animal_piece_component);

        return self
            .commands
            .spawn(ActivePiece)
            .insert(animal_piece_component)
            .insert(material_mesh)
            .insert(ActiveCollisionTypes::all())
            .insert(TransformBundle::from(Transform::from_xyz(
                new_grab_postion,
                BOX_SIZE_HEIHT / 2.0 + PIECE_POSITION_Y_MARGIN,
                0.0,
            )))
            .id();
    }

    pub fn spawn_with_position(&mut self, position_x: f32, position_y: f32) -> Entity {
        let animal_piece_component = self.animal_piece_generator.generate();
        self.grab_position_manager
            .set_grab_position(animal_piece_component.animal_piece.as_ref());
        let material_mesh = self
            .piece_renderer
            .create_material_mesh(&animal_piece_component);

        return self
            .commands
            .spawn(ActivePiece)
            .insert(animal_piece_component)
            .insert(material_mesh)
            .insert(ActiveCollisionTypes::all())
            .insert(TransformBundle::from(Transform::from_xyz(
                position_x, position_y, 0.0,
            )))
            .id();
    }

    pub fn spawn_inactive_piece_with_position(
        &mut self,
        position_x: f32,
        position_y: f32,
        animal_piece_component: AnimalPieceComponent,
    ) -> Entity {
        self.grab_position_manager
            .set_grab_position(animal_piece_component.animal_piece.as_ref());
        let material_mesh = self
            .piece_renderer
            .create_material_mesh(&animal_piece_component);

        return self
            .commands
            .spawn(animal_piece_component)
            .insert(material_mesh)
            .insert(TransformBundle::from(Transform::from_xyz(
                position_x, position_y, 0.0,
            )))
            .id();
    }
}
