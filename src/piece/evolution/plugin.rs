use bevy::{
    app::{App, Plugin},
    ecs::schedule::{OnEnter, OnExit},
};

use crate::{
    game::{despawn::despawn_component, screen_state::ScreenState},
    piece::{
        evolution::ui::{container::PieceEvolveContainer, setup::setup_evolve_piece},
        next::state::NextPiece,
    },
};

pub struct PieceEvolvePlugin;
impl Plugin for PieceEvolvePlugin {
    fn build(&self, app: &mut App) {
        app.insert_resource(NextPiece::new())
            .add_systems(OnEnter(ScreenState::Game), setup_evolve_piece)
            .add_systems(
                OnExit(ScreenState::Game),
                despawn_component::<PieceEvolveContainer>,
            );
    }
}
