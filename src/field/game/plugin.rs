use bevy::{
    app::{App, Plugin},
    prelude::*,
};

use crate::{
    field::game::cat_mug::{
        ear_sprite::CatMugEarSprite, handle_sprite::CatMugHandleSprite, setup_cat_mug,
        sprite::CatMugSprite,
    },
    game::{despawn::despawn_component, screen_state::ScreenState},
};

pub struct GameFieldPlugin;
impl Plugin for GameFieldPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(
            OnEnter(ScreenState::Game),
            (setup_cat_mug).run_if(in_state(ScreenState::Game)),
        )
        .add_systems(
            OnExit(ScreenState::Game),
            (
                despawn_component::<CatMugEarSprite>,
                despawn_component::<CatMugSprite>,
                despawn_component::<CatMugHandleSprite>,
            ),
        );
    }
}
