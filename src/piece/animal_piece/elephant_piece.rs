use super::animal_piece::{AnimalPiece, Piece, PieceType};
use bevy::prelude::*;

#[derive(Component)]
pub struct ElephantPiece {
    piece: Piece,
}

impl ElephantPiece {
    pub(crate) fn new() -> Self {
        Self {
            piece: Piece::new(16, PieceType::Elephant),
        }
    }
}

impl AnimalPiece for ElephantPiece {
    fn can_evolve(&self) -> bool {
        return false;
    }

    fn evolve(&self) {
        todo!()
    }

    fn get_size(&self) -> &super::animal_piece::PieceSize {
        return &self.piece.size;
    }

    fn get_piece_type(&self) -> &PieceType {
        return &self.piece.piece_type;
    }
}
