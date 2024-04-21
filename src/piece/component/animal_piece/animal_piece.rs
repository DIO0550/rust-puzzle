use std::usize;

use crate::piece::component::factory::piece_factory::{Factory, PieceFactory};

pub struct PieceSize(u32);
impl PieceSize {
    pub fn to_f32(&self) -> f32 {
        self.0 as f32
    }
}

#[derive(Debug, Copy, Clone, PartialEq)]
pub enum PieceType {
    // ネズミ
    Rat = 0,
    // ネコ
    Cat,
    // 犬
    Dog,
    // ペンギン
    Penguin,
    // ウマ
    Horse,
    // パンダ
    Panda,
    // キリン
    Giraffe,
    // ゾウ
    Elephant,
}

pub struct PieceScore(u32);
impl PieceScore {
    pub fn to_u32(&self) -> u32 {
        self.0 as u32
    }
}

impl From<usize> for PieceType {
    fn from(value: usize) -> Self {
        match value {
            value if value == PieceType::Rat.to_usize() => PieceType::Rat,
            value if value == PieceType::Cat.to_usize() => PieceType::Cat,
            value if value == PieceType::Dog.to_usize() => PieceType::Dog,
            value if value == PieceType::Penguin.to_usize() => PieceType::Penguin,
            value if value == PieceType::Horse.to_usize() => PieceType::Horse,
            value if value == PieceType::Panda.to_usize() => PieceType::Panda,
            value if value == PieceType::Giraffe.to_usize() => PieceType::Giraffe,
            value if value == PieceType::Elephant.to_usize() => PieceType::Elephant,
            _ => PieceType::Rat,
        }
    }
}

impl PieceType {
    pub fn to_usize(&self) -> usize {
        *self as usize
    }

    pub fn turn(&self) -> Option<PieceType> {
        match self {
            PieceType::Rat => Some(PieceType::Cat),
            PieceType::Cat => Some(PieceType::Dog),
            PieceType::Dog => Some(PieceType::Penguin),
            PieceType::Penguin => Some(PieceType::Horse),
            PieceType::Horse => Some(PieceType::Panda),
            PieceType::Panda => Some(PieceType::Giraffe),
            PieceType::Giraffe => Some(PieceType::Elephant),
            PieceType::Elephant => None,
        }
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
}

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
