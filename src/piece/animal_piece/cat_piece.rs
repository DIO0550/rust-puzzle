use super::animal_piece::{AnimalPiece, Piece, PieceType};
use bevy::prelude::*;

#[derive(Component)]
pub struct CatPiece {
    piece: Piece,
}

impl CatPiece {
    pub(crate) fn new() -> Self {
        Self {
            piece: Piece::new(4, PieceType::Cat),
        }
    }
}

impl AnimalPiece for CatPiece {
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
