use bevy::{
    app::{App, Plugin, Update},
    prelude::*,
};

use crate::{
    asset::image::piece_image_assets::PieceImageAssets,
    game::{screen_state::ScreenState, state::GameState},
    piece::next::{
        state::NextPiece,
        ui::{image::NextPieceImage, setup::setup_next_piece},
    },
    ui::image::update_image::update_image,
};

pub struct NextPiecePlugin;
impl Plugin for NextPiecePlugin {
    fn build(&self, app: &mut App) {
        app.insert_resource(NextPiece::new())
            .add_systems(OnEnter(ScreenState::Game), setup_next_piece)
            .add_systems(
                Update,
                update_image::<PieceImageAssets, NextPiece, NextPieceImage>
                    .run_if(in_state(GameState::InGame)),
            );
    }
}
