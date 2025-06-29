use bevy::ecs::system::{Commands, Res};

use crate::{
    asset::{
        font::font_assets::FontAssets,
        image::{game_image_assets::GameImageAssets, piece_image_assets::PieceImageAssets},
    },
    piece::evolution::ui::{
        background_image_container::PieceEvolveBackgroundImageContainer,
        container::PieceEvolveContainer, describe_container::PieceEvolveDescribeContainer,
        title_text_container::PieceEvolveTitleTextContainer,
    },
};

pub(crate) fn setup_evolve_piece(
    mut commands: Commands,
    game_image_assets: Res<GameImageAssets>,
    font_assets: Res<FontAssets>,
    piece_image_assets: Res<PieceImageAssets>,
) {
    let evolve_piece_container = PieceEvolveContainer::spawn(&mut commands);
    PieceEvolveTitleTextContainer::spawn_as_child(
        &mut commands,
        evolve_piece_container,
        &font_assets,
    );

    let piece_evolve_bg_image = PieceEvolveBackgroundImageContainer::spawn_as_child(
        &mut commands,
        evolve_piece_container,
        &game_image_assets,
        350.0,
    );

    PieceEvolveDescribeContainer::spawn_as_child(
        &mut commands,
        piece_evolve_bg_image,
        &piece_image_assets,
        50.0,
    );
}
