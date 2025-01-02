use bevy::ecs::system::Resource;

use crate::title::component::title_menu_item::TitleMenuItem;

#[derive(Resource)]
pub struct SelectTitleMenu(pub TitleMenuItem);

impl SelectTitleMenu {
    pub fn next(&self) -> SelectTitleMenu {
        match self.0 {
            TitleMenuItem::StartGame => SelectTitleMenu(TitleMenuItem::HighScore),
            TitleMenuItem::HighScore => SelectTitleMenu(TitleMenuItem::StartGame),
        }
    }

    pub fn prev(&self) -> SelectTitleMenu {
        match self.0 {
            TitleMenuItem::StartGame => SelectTitleMenu(TitleMenuItem::HighScore),
            TitleMenuItem::HighScore => SelectTitleMenu(TitleMenuItem::StartGame),
        }
    }
}
