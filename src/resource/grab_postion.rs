use bevy::ecs::system::{ResMut, Resource, SystemParam};

use crate::{
    piece::component::animal_piece::animal_piece::AnimalPiece, BOX_SIZE_WIDTH, BOX_THICKNESS,
    UNIT_WIDTH,
};

#[derive(Resource)]
pub struct GrabPosition {
    pub x: f32,
}

impl GrabPosition {
    pub fn new(position: f32, animal_piece: &dyn AnimalPiece) -> Self {
        let piece_size = animal_piece.get_size().to_f32() * UNIT_WIDTH * 2.0;
        let range = BOX_SIZE_WIDTH / 2.0 - BOX_THICKNESS;
        let max = range - piece_size;
        let min = piece_size - range;

        if position < min {
            let new_position = min;
            return GrabPosition { x: new_position };
        }

        if max < position {
            let new_position = max;
            return GrabPosition { x: new_position };
        }

        return GrabPosition { x: position };
    }
}

#[derive(SystemParam)]
pub struct GrabPostionManager<'w> {
    pub grab_position: ResMut<'w, GrabPosition>,
}

impl<'w> GrabPostionManager<'w> {
    pub fn set_grab_position(&mut self, animal_piece: &dyn AnimalPiece) {
        self.grab_position.x = GrabPosition::new(self.grab_position.x, animal_piece).x;
    }
}
