use bevy::prelude::*;
use bevy::state::state::FreelyMutableState;

use super::menu_item_bundle::MenuItemSelected;

pub trait MenuItemSelectAction: Send + Sync {
    type TargetState: States + FreelyMutableState;
    fn excecute(&self, commands: &mut Commands, state: &mut ResMut<NextState<Self::TargetState>>);
}

pub fn select_menu_item_action<MenuAction>(
    mut command: Commands,
    keyboard_input: Res<ButtonInput<KeyCode>>,
    mut target_state: ResMut<NextState<MenuAction::TargetState>>,
    selected_item_action_query: Query<&MenuAction, With<MenuItemSelected>>,
) where
    MenuAction: MenuItemSelectAction + Component,
    MenuAction::TargetState: FreelyMutableState,
{
    if !keyboard_input.just_released(KeyCode::Space) {
        return;
    }

    let Ok(selected_item_action) = selected_item_action_query.single() else {
        return;
    };

    selected_item_action.excecute(&mut command, &mut target_state);
}
