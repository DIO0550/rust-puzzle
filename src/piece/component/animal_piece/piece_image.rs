// use bevy::{asset::Handle, ecs::system::Res, render::texture::Image};

// use crate::asset::image::piece_image_assets::PieceImageAssets;

// use super::piece_type::PieceType;

// pub struct PieceImage {}

// impl PieceImage {
//     pub fn from_piece_type(
//         piece_image_assets: &Res<PieceImageAssets>,
//         piece_type: &PieceType,
//     ) -> Handle<Image> {
//         let image = match piece_type {
//             PieceType::Cat => piece_image_assets.cat.clone(),
//             PieceType::Dog => piece_image_assets.dog.clone(),
//             PieceType::Elephant => piece_image_assets.elephant.clone(),
//             PieceType::Giraffe => piece_image_assets.giraffe.clone(),
//             PieceType::Horse => piece_image_assets.horse.clone(),
//             PieceType::Panda => piece_image_assets.panda.clone(),
//             PieceType::Penguin => piece_image_assets.penguin.clone(),
//             PieceType::Rat => piece_image_assets.rat.clone(),
//         };

//         return image;
//     }
// }
