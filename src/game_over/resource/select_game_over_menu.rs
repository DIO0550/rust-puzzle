use bevy::ecs::system::Resource;

use crate::game_over::component::game_over_menu_item::GameOverMenu;

#[derive(Resource)]
pub struct SelectGameOverMenu(pub GameOverMenu);

impl SelectGameOverMenu {
    pub fn next(&self) -> SelectGameOverMenu {
        match self.0 {
            GameOverMenu::Restart => SelectGameOverMenu(GameOverMenu::GoTitle),
            GameOverMenu::GoTitle => SelectGameOverMenu(GameOverMenu::Restart),
        }
    }

    pub fn prev(&self) -> SelectGameOverMenu {
        match self.0 {
            GameOverMenu::Restart => SelectGameOverMenu(GameOverMenu::GoTitle),
            GameOverMenu::GoTitle => SelectGameOverMenu(GameOverMenu::Restart),
        }
    }
}
