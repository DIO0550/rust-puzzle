use bevy::ecs::entity::Entity;
use bevy::ecs::system::Commands;
use bevy::render::color::Color;
use bevy::ui::node_bundles::NodeBundle;
use bevy::ui::{BackgroundColor, Style, Val};

use bevy::ecs::component::Component;

#[derive(Component)]
pub struct Menu {
    pub id: String,
}

pub struct MenuEntityBuilder<T: Component> {
    menu: Menu,
    marker: T,
    style: Style,
    background_color: Color,
}

impl<T: Component> MenuEntityBuilder<T> {
    pub fn new(id: &str, marker: T) -> Self {
        Self {
            menu: Menu { id: id.to_string() },
            marker: marker,
            style: Style {
                ..Default::default()
            },
            background_color: Color::NONE,
        }
    }

    pub fn style(mut self, style: Style) -> Self {
        self.style = style;
        self
    }

    pub fn background_color(mut self, color: Color) -> Self {
        self.background_color = color;
        self
    }

    pub fn build(self, commands: &mut Commands) -> Entity {
        let entity = commands
            .spawn((
                NodeBundle {
                    style: { self.style },
                    background_color: BackgroundColor(self.background_color),
                    ..Default::default()
                },
                self.menu,
                self.marker,
            ))
            .id();
        entity
    }
}
