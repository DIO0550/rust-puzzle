use bevy::ecs::system::{Commands, Res};

use crate::{
    asset::{
        font::font_assets::FontAssets,
        image::{game_image_assets::GameImageAssets, piece_image_assets::PieceImageAssets},
    },
    piece::next::{
        state::NextPiece,
        ui::{
            container::NextPieceContainer, image::NextPieceImage,
            title_text_container::NextPieceTitleTextContainer,
        },
    },
};

pub fn setup_next_piece(
    mut commands: Commands,
    game_image_assets: Res<GameImageAssets>,
    piece_image_assets: Res<PieceImageAssets>,
    next_piece_res: Res<NextPiece>,
    font_assets: Res<FontAssets>,
) {
    let next_piece_container = NextPieceContainer::spawn(&mut commands, &game_image_assets);
    NextPieceTitleTextContainer::spawn_as_child(&mut commands, next_piece_container, &font_assets);
    NextPieceImage::spawn_as_child(
        &mut commands,
        next_piece_container,
        &piece_image_assets,
        &next_piece_res,
    );
}
