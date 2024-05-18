use super::{
    animal_piece::{AnimalPiece, Piece, PieceScore},
    piece_type::PieceType,
};
use bevy::prelude::*;

#[derive(Component)]
pub struct ElephantPiece {
    piece: Piece,
}

impl ElephantPiece {
    pub(crate) fn new() -> Self {
        Self {
            piece: Piece::new(16, PieceType::Elephant, 16),
        }
    }
}

impl AnimalPiece for ElephantPiece {
    fn can_evolve(&self) -> bool {
        return false;
    }

    fn get_size(&self) -> &super::animal_piece::PieceSize {
        return &self.piece.size;
    }

    fn get_piece_type(&self) -> &PieceType {
        return &self.piece.piece_type;
    }

    fn get_score(&self) -> &PieceScore {
        return &self.piece.score;
    }
}
