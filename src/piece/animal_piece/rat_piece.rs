use super::animal_piece::{AnimalPiece, Piece, PieceType};
use bevy::prelude::*;

#[derive(Component)]
pub struct RatPiece {
    piece: Piece,
}

impl RatPiece {
    pub(crate) fn new() -> Self {
        Self {
            piece: Piece::new(2, PieceType::Rat),
        }
    }
}

impl AnimalPiece for RatPiece {
    fn can_evolve(&self) -> bool {
        return true;
    }

    fn evolve(&self) {
        // todo
    }

    fn get_size(&self) -> &super::animal_piece::PieceSize {
        return &self.piece.size;
    }

    fn get_piece_type(&self) -> &PieceType {
        return &self.piece.piece_type;
    }
}
