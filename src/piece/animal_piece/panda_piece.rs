use super::animal_piece::{AnimalPiece, Piece, PieceScore, PieceType};
use bevy::prelude::*;

#[derive(Component)]
pub struct PandaPiece {
    piece: Piece,
}
impl PandaPiece {
    pub(crate) fn new() -> Self {
        Self {
            piece: Piece::new(12, PieceType::Panda, 12),
        }
    }
}

impl AnimalPiece for PandaPiece {
    fn can_evolve(&self) -> bool {
        return true;
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

    fn get_score(&self) -> &PieceScore {
        return &self.piece.score;
    }
}
