use bevy::ecs::system::Resource;

use crate::title::component::title_menu_item::TitleMenuItem;

#[derive(Resource)]
pub struct SelectTitleMenu(pub TitleMenuItem);
