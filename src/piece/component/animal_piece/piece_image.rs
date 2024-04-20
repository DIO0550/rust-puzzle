use bevy::render::color::Color;

use super::animal_piece::PieceType;

pub struct PieceImage {}

impl PieceImage {
    pub fn from_piece_type(piece_type: &PieceType) -> Color {
        let color = match piece_type {
            PieceType::Cat => Color::YELLOW,
            PieceType::Dog => Color::RED,
            PieceType::Elephant => Color::GREEN,
            PieceType::Giraffe => Color::AQUAMARINE,
            PieceType::Horse => Color::BEIGE,
            PieceType::Panda => Color::BISQUE,
            PieceType::Penguin => Color::BLACK,
            PieceType::Rat => Color::BLUE,
        };

        return color;
    }
}
