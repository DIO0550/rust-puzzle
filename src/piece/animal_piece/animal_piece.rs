use std::usize;

use bevy::ecs::component::Component;

pub struct PieceSize(u32);
impl PieceSize {
    pub fn to_f32(&self) -> f32 {
        self.0 as f32
    }
}

#[derive(Debug, Copy, Clone)]
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

const SPWANABLE_PIECES: &[&PieceType] = &[
    &PieceType::Rat,
    &PieceType::Cat,
    &PieceType::Dog,
    &PieceType::Penguin,
];

impl PieceType {
    pub fn new(rnd: &usize) -> Self {
        let piece_index = rnd % SPWANABLE_PIECES.len() + 1;
        match SPWANABLE_PIECES.get(piece_index) {
            None => PieceType::Rat,
            Some(v) => **v,
        }
    }

    fn turn(&self) -> Option<PieceType> {
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
    fn evolve(&self);
    fn get_size(&self) -> &PieceSize;
    fn get_piece_type(&self) -> &PieceType;
}

pub struct Piece {
    pub(super) size: PieceSize,
    pub(super) piece_type: PieceType,
}

impl Piece {
    pub fn new(size: u32, piece_type: PieceType) -> Self {
        Piece {
            size: PieceSize(size),
            piece_type: piece_type,
        }
    }
}

#[derive(Component)]
pub struct Grab(bool);
impl Grab {
    pub fn new(value: bool) -> Grab {
        Grab(value)
    }
}

#[derive(Component)]
pub struct AnimalPieceComponent {
    pub animal_piece: Box<dyn AnimalPiece>,
}
