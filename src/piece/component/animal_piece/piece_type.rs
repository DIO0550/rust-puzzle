use rand::distributions::{Distribution, Uniform};

const SPWANABLE_PIECE_TYPES: &[&PieceType] = &[
    &PieceType::Rat,
    &PieceType::Cat,
    &PieceType::Dog,
    &PieceType::Penguin,
];

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

    pub fn new_random() -> PieceType {
        let mut rng = rand::thread_rng();
        let side = Uniform::from(0..(SPWANABLE_PIECE_TYPES.len() - 1));
        let piece_type = PieceType::from(side.sample(&mut rng));

        return piece_type;
    }
}
