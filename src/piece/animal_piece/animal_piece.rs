pub struct PieceSize(u32);
pub enum PieceType {
    // ネズミ
    Rat,
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

impl PieceType {
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

pub trait AnimalPiece {
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
