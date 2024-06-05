use crate::piece::component::factory::piece_factory::{Factory, PieceFactory};

use super::{animal_piece::AnimalPiece, piece_type::PieceType};
use bevy::ecs::component::Component;

#[derive(Component)]
pub struct AnimalPieceComponent {
    pub animal_piece: Box<dyn AnimalPiece>,
}

impl From<PieceType> for AnimalPieceComponent {
    fn from(value: PieceType) -> Self {
        let piece = AnimalPieceComponent {
            animal_piece: PieceFactory::create_piece(&value),
        };

        return piece;
    }
}

impl AnimalPieceComponent {
    pub fn evolve(&self) -> Option<Self> {
        let Some(piece) = self.animal_piece.evolve() else {
            return None;
        };

        return Some(AnimalPieceComponent {
            animal_piece: piece,
        });
    }
}
