use bevy::{
    app::{App, Plugin},
    ecs::schedule::OnExit,
    prelude::{in_state, IntoSystemConfigs, OnEnter},
};

use crate::{
    field::game::{
        material::{
            cat_mug_ear_sprite::CatMugEarSprite, cat_mug_handle_sprite::CatMugHandleSprite,
            cat_mug_sprite::CatMugSprite,
        },
        system::cat_mug_system::setup_cat_mug,
    },
    game::{state::game_page_state::GamePageState, system::despawn::despawn_component},
};

pub struct GameFieldPlugin;
impl Plugin for GameFieldPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(
            OnEnter(GamePageState::Game),
            (setup_cat_mug).run_if(in_state(GamePageState::Game)),
        )
        .add_systems(
            OnExit(GamePageState::Game),
            (
                despawn_component::<CatMugEarSprite>,
                despawn_component::<CatMugSprite>,
                despawn_component::<CatMugHandleSprite>,
            ),
        );
    }
}
