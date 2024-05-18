use super::{
    animal_piece::{AnimalPiece, Piece, PieceScore},
    piece_type::PieceType,
};
use bevy::prelude::*;

#[derive(Component)]
pub struct PenguinPiece {
    pub(super) piece: Piece,
}

impl PenguinPiece {
    pub(crate) fn new() -> Self {
        Self {
            piece: Piece::new(8, PieceType::Penguin, 8),
        }
    }
}

impl AnimalPiece for PenguinPiece {
    fn can_evolve(&self) -> bool {
        return true;
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
