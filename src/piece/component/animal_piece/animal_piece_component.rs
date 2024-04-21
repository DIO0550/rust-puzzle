use crate::piece::component::factory::piece_factory::{Factory, PieceFactory};

use super::animal_piece::{AnimalPiece, PieceType};
use bevy::ecs::component::Component;
use rand::distributions::{Distribution, Uniform};

#[derive(Component)]
pub struct AnimalPieceComponent {
    pub animal_piece: Box<dyn AnimalPiece>,
}

const SPWANABLE_PIECES: &[&PieceType] = &[
    &PieceType::Rat,
    &PieceType::Cat,
    &PieceType::Dog,
    &PieceType::Penguin,
];

impl AnimalPieceComponent {
    pub fn spawn() -> Self {
        let mut rng = rand::thread_rng();
        let side = Uniform::from(0..(SPWANABLE_PIECES.len() - 1));
        let piece_type = PieceType::from(side.sample(&mut rng));

        let piece = AnimalPieceComponent {
            animal_piece: PieceFactory::create_piece(&piece_type),
        };

        return piece;
    }

    pub fn evolve(&self) -> Option<Self> {
        let Some(piece) = self.animal_piece.evolve() else {
            return None;
        };

        return Some(AnimalPieceComponent {
            animal_piece: piece,
        });
    }
}
