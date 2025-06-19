use crate::piece::component::factory::piece_factory::{Factory, PieceFactory};

use super::piece_type::PieceType;

#[derive(Clone)]
pub struct PieceSize(u32);
impl PieceSize {
    pub fn to_f32(&self) -> f32 {
        self.0 as f32
    }
}

#[derive(Clone)]
pub struct PieceScore(u32);
impl PieceScore {
    pub fn to_u32(&self) -> u32 {
        self.0 as u32
    }
}

pub trait AnimalPiece: Send + Sync + 'static {
    fn can_evolve(&self) -> bool;
    fn evolve(&self) -> Option<Box<dyn AnimalPiece>> {
        let Some(piece_type) = self.get_piece_type().turn() else {
            return None;
        };

        return Some(PieceFactory::create_piece(&piece_type));
    }
    fn get_size(&self) -> &PieceSize;
    fn get_piece_type(&self) -> &PieceType;
    fn get_score(&self) -> &PieceScore;
    fn clone_box(&self) -> Box<dyn AnimalPiece>;
}

#[derive(Clone)]
pub struct Piece {
    pub(super) size: PieceSize,
    pub(super) piece_type: PieceType,
    pub(super) score: PieceScore,
}

impl Piece {
    pub fn new(size: u32, piece_type: PieceType, score: u32) -> Self {
        Piece {
            size: PieceSize(size),
            piece_type: piece_type,
            score: PieceScore(score),
        }
    }
}
