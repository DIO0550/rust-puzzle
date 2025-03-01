use bevy::a11y::accesskit::Size;
use bevy::ecs::entity::Entity;
use bevy::ecs::event::{Event, EventReader};
use bevy::ecs::system::{Commands, Query};
use bevy::ui::node_bundles::NodeBundle;
use bevy::ui::{Style, Val};

use bevy::ecs::component::Component;

#[derive(Component)]
pub struct Menu {
    pub id: String,
}

pub struct MenuEntityBuilder {
    menu: Menu,
    style: Style,
}

impl MenuEntityBuilder {
    pub fn new(id: &str) -> Self {
        Self {
            menu: Menu { id: id.to_string() },
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
            ))
            .id();
        entity
    }
}

#[derive(Event)]
pub struct MenuSelectEvent {
    pub menu_id: String,
    pub item_id: String,
}
