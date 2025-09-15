use bevy::prelude::*;

#[derive(Component)]
pub struct Menu {
    pub id: String,
}

pub struct MenuEntityBuilder<T: Component> {
    menu: Menu,
    marker: T,
    node: Node,
    background_color: Color,
}

impl<T: Component> MenuEntityBuilder<T> {
    pub fn new(id: &str, marker: T) -> Self {
        Self {
            menu: Menu { id: id.to_string() },
            marker: marker,
            node: Node {
                ..Default::default()
            },
            background_color: Color::NONE,
        }
    }

    pub fn style(mut self, node: Node) -> Self {
        self.node = node;
        self
    }

    pub fn background_color(mut self, color: Color) -> Self {
        self.background_color = color;
        self
    }

    pub fn build(self, commands: &mut Commands) -> Entity {
        let entity = commands
            .spawn((
                self.node,
                BackgroundColor(self.background_color),
                self.menu,
                self.marker,
            ))
            .id();
        entity
    }
}
