use super::animal_piece::{
    animal_piece::{AnimalPiece, PieceType},
    cat_piece::CatPiece,
    dog_piece::DogPiece,
    elephant_piece::ElephantPiece,
    giraffe_piece::GiraffePiece,
    horse_piece::HorsePiece,
    panda_piece::PandaPiece,
    penguin_piece::PenguinPiece,
    rat_piece::RatPiece,
};

pub trait Factory {
    fn create_piece(piece_type: &PieceType) -> Box<dyn AnimalPiece>;
}

pub struct PieceFactory {}
impl Factory for PieceFactory {
    fn create_piece(piece_type: &PieceType) -> Box<dyn AnimalPiece> {
        match piece_type {
            PieceType::Rat => Box::new(RatPiece::new()),
            PieceType::Cat => Box::new(CatPiece::new()),
            PieceType::Dog => Box::new(DogPiece::new()),
            PieceType::Penguin => Box::new(PenguinPiece::new()),
            PieceType::Horse => Box::new(HorsePiece::new()),
            PieceType::Panda => Box::new(PandaPiece::new()),
            PieceType::Giraffe => Box::new(GiraffePiece::new()),
            PieceType::Elephant => Box::new(ElephantPiece::new()),
        }
    }
}
