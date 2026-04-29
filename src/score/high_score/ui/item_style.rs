use bevy::ui::{UiRect, Val};
use bevy::prelude::Color;
use crate::consts::color_theme::ColorTheme;

pub struct ItemStyle{}
impl ItemStyle {
    pub const BG_COLOR: Color = ColorTheme::SUB_BACKGROUND;
    pub const BORDER_COLOR: Color = ColorTheme::BORDER_PRIMARY;
    pub const BORDER_RADIUS: f32 = 10.0;
    pub const BORDER_WIDTH: f32 = 3.0;
    pub const PADDING: UiRect = UiRect::all(Val::Px(15.0));
    pub const SPACING: Val = Val::Px(10.0);
    pub const ROW_GAP: f32 = 20.0;
}

