use crate::piece::{
    component::factory::piece_factory::{Factory, PieceFactory},
    next_piece::resource::next_piece::{NextPiece, NextPieceManager},
};

use super::{
    animal_piece::{self, AnimalPiece},
    piece_type::PieceType,
};
use bevy::ecs::{
    component::Component,
    system::{Res, ResMut, SystemParam},
};

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

impl AnimalPiece for AnimalPieceComponent {
    fn can_evolve(&self) -> bool {
        return self.animal_piece.can_evolve();
    }

    fn get_size(&self) -> &animal_piece::PieceSize {
        return self.animal_piece.get_size();
    }

    fn get_piece_type(&self) -> &PieceType {
        return self.animal_piece.get_piece_type();
    }

    fn get_score(&self) -> &animal_piece::PieceScore {
        return self.animal_piece.get_score();
    }
}

#[derive(SystemParam)]
pub(crate) struct AnimalPieceComponentGenerator<'w> {
    next_piece: NextPieceManager<'w>,
}

impl<'w> AnimalPieceComponentGenerator<'w> {
    pub fn generate(&mut self) -> AnimalPieceComponent {
        let animal_piece = AnimalPieceComponent::from(self.next_piece.get_next_piece().clone());
        self.next_piece.update_next_piece();
        return animal_piece;
    }
}
