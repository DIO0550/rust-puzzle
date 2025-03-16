use bevy::ecs::entity::Entity;
use bevy::ecs::system::Commands;
use bevy::ui::node_bundles::NodeBundle;
use bevy::ui::{Style, Val};

use bevy::ecs::component::Component;

#[derive(Component)]
pub struct Menu {
    pub id: String,
}

pub struct MenuEntityBuilder<T: Component> {
    menu: Menu,
    marker: T,
    style: Style,
}

impl<T: Component> MenuEntityBuilder<T> {
    pub fn new(id: &str, marker: T) -> Self {
        Self {
            menu: Menu { id: id.to_string() },
            marker: marker,
            style: Style {
                ..Default::default()
            },
        }
    }

    pub fn style(mut self, style: Style) -> Self {
        self.style = style;
        self
    }

    pub fn size(mut self, width: f32, height: f32) -> Self {
        self.style.width = Val::Px(width);
        self.style.height = Val::Px(height);
        self
    }

    pub fn build(self, commands: &mut Commands) -> Entity {
        let entity = commands
            .spawn((
                NodeBundle {
                    style: Default::default(),
                    ..Default::default()
                },
                self.menu,
                self.marker,
            ))
            .id();
        entity
    }
}
