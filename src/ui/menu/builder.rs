use bevy::prelude::*;

use crate::ui::menu::style::{node_setter, MenuStyle};

#[derive(Component)]
pub struct Menu;

pub struct MenuBuilder<T: Component> {
    menu: Menu,
    marker: T,
    node: Node,
}

impl<T: Component> MenuBuilder<T> {
    pub fn new(marker: T) -> Self {
        Self {
            menu: Menu,
            marker: marker,
            node: Node {
                padding: MenuStyle::PADDING,
                flex_direction: FlexDirection::Column,
                align_items: AlignItems::Center,
                justify_content: JustifyContent::Center,
                row_gap: Val::Px(MenuStyle::ROW_GAP),
                border: UiRect::all(Val::Px(MenuStyle::BORDER_WIDTH)),
                ..Default::default()
            },
        }
    }

    // マクロで一括定義
    node_setter!(width, width);
    node_setter!(height, height);
    node_setter!(top, top);
    node_setter!(left, left);
    node_setter!(right, right);
    node_setter!(bottom, bottom);
    node_setter!(row_gap, row_gap);
    node_setter!(column_gap, column_gap);
    node_setter!(padding, padding, UiRect);
    node_setter!(margin, margin, UiRect);
    node_setter!(flex_direction, flex_direction, FlexDirection);

    // 複合メソッド（よく使う組み合わせ）
    pub fn size(mut self, width: f32, height: f32) -> Self {
        self.node.width = Val::Px(width);
        self.node.height = Val::Px(height);
        self
    }

    pub fn center(mut self) -> Self {
        self.node.top = Val::Percent(50.0);
        self.node.left = Val::Percent(50.0);
        self.node.margin = UiRect {
            top: Val::Px(-self.node.height.resolve(1.0, Vec2::ZERO).unwrap_or(0.0) / 2.0),
            left: Val::Px(-self.node.width.resolve(1.0, Vec2::ZERO).unwrap_or(0.0) / 2.0),
            ..default()
        };
        self
    }

    pub fn build(self, commands: &mut Commands) -> Entity {
        let wrapper = Node {
            width: Val::Percent(100.0),
            height: Val::Percent(100.0),
            display: Display::Flex,
            justify_content: JustifyContent::Center,
            align_items: AlignItems::Center,
            column_gap: MenuStyle::SPACING,
            ..default()
        };

        let bg_color = BackgroundColor(MenuStyle::BG_COLOR);
        let border_color = BorderColor(MenuStyle::BORDER_COLOR);

        let mut menu_entity = Entity::PLACEHOLDER;

        commands.spawn(wrapper).with_children(|parent| {
            menu_entity = parent
                .spawn((
                    self.node,
                    border_color,
                    bg_color,
                    BorderRadius {
                        top_left: Val::Px(MenuStyle::BORDER_RADIUS),
                        top_right: Val::Px(MenuStyle::BORDER_RADIUS),
                        bottom_left: Val::Px(MenuStyle::BORDER_RADIUS),
                        bottom_right: Val::Px(MenuStyle::BORDER_RADIUS),
                    },
                    self.menu,
                    self.marker,
                ))
                .id();
        });

        menu_entity
    }
}
