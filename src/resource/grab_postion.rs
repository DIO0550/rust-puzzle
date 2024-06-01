use bevy::ecs::system::Resource;

use crate::{
    piece::component::animal_piece::animal_piece::AnimalPiece, BOX_SIZE_WIDTH, BOX_THICKNESS,
    UNIT_WIDTH,
};

#[derive(Resource)]
pub struct GrabPostion {
    pub x: f32,
}

impl GrabPostion {
    pub fn new(position: f32, animal_piece: &dyn AnimalPiece) -> Self {
        let piece_size = animal_piece.get_size().to_f32() * UNIT_WIDTH * 2.0;
        let range = BOX_SIZE_WIDTH - BOX_THICKNESS;
        let max = range - piece_size;
        let min = piece_size - range;

        if position < min {
            let new_position = min;
            return GrabPostion { x: new_position };
        }

        if max < position {
            let new_position = max;
            return GrabPostion { x: new_position };
        }

        return GrabPostion { x: position };
    }
}
