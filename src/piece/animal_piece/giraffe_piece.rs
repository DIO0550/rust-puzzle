use super::animal_piece::{AnimalPiece, Piece, PieceType};
use bevy::prelude::*;

#[derive(Component)]
pub struct GiraffePiece {
    piece: Piece,
}

impl GiraffePiece {
    pub(crate) fn new() -> Self {
        Self {
            piece: Piece::new(14, PieceType::Giraffe),
        }
    }
}

impl AnimalPiece for GiraffePiece {
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
}
