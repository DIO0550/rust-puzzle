use bevy::ecs::{
    component::Component,
    schedule::NextState,
    system::{Commands, ResMut},
};

use crate::{game::screen_state::ScreenState, ui::menu::menu_item_action::MenuItemSelectAction};

#[derive(Debug, Component)]
pub enum TitleMenuItemSelectAction {
    StartGame,
    HighScore,
}

impl MenuItemSelectAction for TitleMenuItemSelectAction {
    type TargetState = ScreenState;

    fn excecute(&self, _: &mut Commands, state: &mut ResMut<NextState<Self::TargetState>>) {
        match self {
            TitleMenuItemSelectAction::StartGame => {
                state.set(ScreenState::Game);
            }
            TitleMenuItemSelectAction::HighScore => {
                // ハイスコア画面への遷移処理
                todo!("Implement high score menu item select action");
            }
        }
    }
}
