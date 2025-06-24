use bevy::{
    app::{App, Plugin, Update},
    ecs::schedule::{common_conditions::in_state, IntoSystemConfigs, OnEnter},
};

use crate::{
    asset::image::piece_image_assets::PieceImageAssets,
    game::state::{game_page_state::GamePageState, game_state::GameState},
    piece::next::state::NextPiece,
    piece::next::ui::image::NextPieceImage,
    piece::next::ui::setup::setup_next_piece,
    ui::image::update_image::update_image,
};

pub struct NextPiecePlugin;
impl Plugin for NextPiecePlugin {
    fn build(&self, app: &mut App) {
        app.insert_resource(NextPiece::new())
            .add_systems(OnEnter(GamePageState::Game), setup_next_piece)
            .add_systems(
                Update,
                (update_image::<PieceImageAssets, NextPiece, NextPieceImage>)
                    .run_if(in_state(GameState::InGame)),
            );
    }
}
