use super::animal_piece::AnimalPiece;
use bevy::ecs::component::Component;

#[derive(Component)]
pub struct AnimalPieceComponent {
    pub animal_piece: Box<dyn AnimalPiece>,
}
