use bevy::prelude::*;

use crate::consts::color_theme::ColorTheme;

macro_rules! node_setter {
    ($name:ident, $field:ident, $ty:ty) => {
        pub fn $name(mut self, value: $ty) -> Self {
            self.node.$field = value;
            self
        }
    };
    // Val::Px を自動でラップするバージョン
    ($name:ident, $field:ident) => {
        pub fn $name(mut self, value: f32) -> Self {
            self.node.$field = Val::Px(value);
            self
        }
    };
}
pub(crate) use node_setter;

pub struct MenuStyle {}
impl MenuStyle {
    pub const BG_COLOR: Color = ColorTheme::SUB_BACKGROUND;
    pub const BORDER_COLOR: Color = ColorTheme::BORDER_PRIMARY;
    pub const BORDER_RADIUS: f32 = 10.0;
    pub const BORDER_WIDTH: f32 = 3.0;
    pub const PADDING: UiRect = UiRect::all(Val::Px(15.0));
    pub const SPACING: Val = Val::Px(10.0);
    pub const ROW_GAP: f32 = 20.0;
}

pub struct MenuItemStyle {}

impl MenuItemStyle {
    pub const BG_NORMAL: Color = ColorTheme::SUB_BACKGROUND;
    pub const BG_HOVERED: Color = Color::srgb(0.3, 0.3, 0.3);
    pub const BG_SELECTED: Color = Color::srgb(0.2, 0.5, 0.8);
    pub const BG_SELECTED_HOVERED: Color = Color::srgb(0.3, 0.6, 0.9);

    pub const PADDING: UiRect = UiRect::all(Val::Px(15.0));

    pub const SPACING: Val = Val::Px(10.0);

    /// メニューアイテムのテキスト色
    pub const TEXT_NORMAL: Color = ColorTheme::TEXT;
    pub const TEXT_SELECTED: Color = ColorTheme::SELECT_TEXT;

    /// メニューアイテムのボーダー色
    pub const BORDER_NORMAL: Color = ColorTheme::BORDER_PRIMARY;
    pub const BORDER_SELECTED: Color = ColorTheme::BORDER_PRIMARY;

    pub const BORDER_WIDTH: f32 = 2.0;
    pub const BORDER_RADIUS: f32 = 10.0;
}

pub(crate) struct MenuItemColors {
    pub(crate) background: Color,
    pub(crate) text: Color,
    pub(crate) border: Color,
}

impl MenuItemColors {
    pub(crate) fn new(is_selected: bool) -> Self {
        if is_selected {
            Self {
                background: MenuItemStyle::BG_SELECTED,
                text: MenuItemStyle::TEXT_SELECTED,
                border: MenuItemStyle::BORDER_SELECTED,
            }
        } else {
            Self {
                background: MenuItemStyle::BG_NORMAL,
                text: MenuItemStyle::TEXT_NORMAL,
                border: MenuItemStyle::BORDER_NORMAL,
            }
        }
    }
}
