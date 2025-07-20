use bevy::{
    ecs::{
        component::Component,
        entity::Entity,
        system::{Commands, Res},
    },
    hierarchy::BuildChildren,
    render::color::Color,
    text::{TextAlignment, TextSection, TextStyle},
    ui::{
        node_bundles::{NodeBundle, TextBundle},
        BackgroundColor, Style, Val,
    },
    utils::default,
};

use crate::{asset::font::font_assets::FontAssets, consts::color_theme::ColorTheme};

use super::menu_item_action::MenuItemSelectAction;

#[derive(Debug, Component)]
pub struct MenuItem {
    pub id: String,
    pub label: String,
}

#[derive(Debug, Component)]
pub struct MenuItemSelected;

#[derive(Debug, Component)]
pub struct MenuItemColor {
    pub normal: Color,
    pub selected: Color,
}

pub struct MenuItemEntityBuilder<T: Component, A: MenuItemSelectAction + Component> {
    item: MenuItem,
    is_selected: bool,
    color: MenuItemColor,
    marker: T,
    action: A,
    style: Style,
}

impl<T: Component, A: MenuItemSelectAction + Component> MenuItemEntityBuilder<T, A> {
    pub fn new(id: &str, text: &str, marker: T, action: A) -> Self {
        Self {
            item: MenuItem {
                id: id.to_string(),
                label: text.to_string(),
            },
            is_selected: false,
            color: MenuItemColor {
                normal: ColorTheme::CHROME_WHITE,
                selected: ColorTheme::SPROUT,
            },
            marker,
            style: Style {
                width: Val::Percent(100.0),
                height: Val::Percent(100.0),
                ..Default::default()
            },
            action,
        }
    }

    // メニューアイテムを変更するメソッド
    pub fn menu_item(mut self, menu_item: MenuItem) -> Self {
        self.item = menu_item;
        self
    }

    // 選択状態を設定
    pub fn selected(mut self, is_selected: bool) -> Self {
        self.is_selected = is_selected;

        self
    }

    // 色を設定
    pub fn color(mut self, normal: Color, selected: Color) -> Self {
        self.color = MenuItemColor { normal, selected };
        self
    }

    // スタイルを設定
    pub fn style(mut self, style: Style) -> Self {
        self.style = style;
        self
    }

    // エンティティを構築
    pub fn build(self, commands: &mut Commands, font_assets: &Res<FontAssets>) -> Entity {
        let background_color: BackgroundColor = if self.is_selected {
            BackgroundColor(self.color.selected)
        } else {
            BackgroundColor(self.color.normal)
        };

        let label = self.item.label.clone();

        let button_bundle = NodeBundle {
            style: self.style,
            background_color: background_color,
            ..Default::default()
        };

        // 基本的なエンティティを作成
        let mut entity_commands = commands.spawn((
            self.item,
            self.color,
            self.marker,
            button_bundle,
            self.action,
        ));

        // 選択状態に応じて条件付きでマーカーコンポーネントを追加
        if self.is_selected {
            entity_commands.insert(MenuItemSelected);
        }

        let entity = entity_commands
            .with_children(|parent| {
                parent
                    .spawn(NodeBundle {
                        style: Style { ..default() },
                        ..default()
                    })
                    .with_children(|parent| {
                        parent.spawn((TextBundle::from_sections([TextSection::new(
                            label,
                            TextStyle {
                                font: font_assets.hachi_maru_pop_regular.clone(),
                                font_size: 100.,
                                color: Color::BLACK,
                                ..default()
                            },
                        )])
                        .with_text_alignment(TextAlignment::Center),));
                    });
            })
            .id();

        entity
    }

    pub fn build_as_child(
        self,
        commands: &mut Commands,
        parent_entity: Entity,
        font_assets: &Res<FontAssets>,
    ) -> Entity {
        let entity = self.build(commands, font_assets);
        commands.entity(parent_entity).add_child(entity);
        entity
    }
}
