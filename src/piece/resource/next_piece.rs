use bevy::ecs::system::Resource;

use crate::piece::component::animal_piece::piece_type::PieceType;

#[derive(Resource)]
pub struct NextPiece(pub PieceType);

impl NextPiece {
    pub fn new() -> NextPiece {
        let piece_type = PieceType::new_random();
        let next_piece = NextPiece(piece_type);

        return next_piece;
    }
}
