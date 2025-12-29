use crate::asset::font::font_assets::FontAssets;
use crate::ui::menu::style::{node_setter, MenuItemColors, MenuItemStyle};
use bevy::prelude::*;

use super::item_action::MenuItemSelectAction;

#[derive(Debug, Component)]
pub struct MenuItem {
    pub id: String,
    pub label: String,
}

#[derive(Component)]
pub struct MenuItemSelected;

pub struct MenuItemBundleBuilder<T: Component, A: MenuItemSelectAction + Component> {
    is_selected: bool,
    text: String,
    node: Node,
    marker: T,
    action: A,
}

impl<T: Component, A: MenuItemSelectAction + Component> MenuItemBundleBuilder<T, A> {
    pub fn new(text: &str, marker: T, action: A) -> Self {
        Self {
            text: text.to_string(),
            is_selected: false,
            marker,
            action,
            node: Node {
                padding: MenuItemStyle::PADDING,
                flex_direction: FlexDirection::Column,
                align_items: AlignItems::Center,
                justify_content: JustifyContent::Center,
                border: UiRect::all(Val::Px(MenuItemStyle::BORDER_WIDTH)),
                ..Default::default()
            },
        }
    }

    // 選択状態を設定
    pub fn selected(mut self, selected: bool) -> Self {
        self.is_selected = selected;
        self
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

    // エンティティを構築
    pub fn build(self, commands: &mut Commands, font_assets: &Res<FontAssets>) -> Entity {
        let style_colors = MenuItemColors::new(self.is_selected);

        let text = Text::new(self.text.clone());
        let text_font = TextFont {
            font: font_assets.hachi_maru_pop_regular.clone(),
            font_size: 55.,
            ..default()
        };

        // 基本的なエンティティを作成
        let mut entity_commands = commands.spawn((
            self.node,
            self.marker,
            self.action,
            BorderColor(style_colors.border.clone()),
            BackgroundColor(style_colors.background.clone()),
            BorderRadius::all(Val::Px(MenuItemStyle::BORDER_RADIUS)),
        ));
        if self.is_selected {
            entity_commands.insert(MenuItemSelected);
        }

        let entity = entity_commands
            .with_children(|parent| {
                parent.spawn((text, text_font, TextColor(style_colors.text.clone())));
            })
            .id();

        entity
    }
}
