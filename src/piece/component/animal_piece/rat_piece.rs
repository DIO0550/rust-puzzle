use super::{
    animal_piece::{AnimalPiece, Piece, PieceScore},
    piece_type::PieceType,
};
use bevy::prelude::*;

#[derive(Component)]
pub struct RatPiece {
    piece: Piece,
}

impl RatPiece {
    pub(crate) fn new() -> Self {
        Self {
            piece: Piece::new(2, PieceType::Rat, 2),
        }
    }
}

impl AnimalPiece for RatPiece {
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

    fn clone_box(&self) -> Box<dyn AnimalPiece> {
        return Box::new(RatPiece {
            piece: self.piece.clone(),
        });
    }
}
