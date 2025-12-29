use bevy::color::Color;
use bevy::ui::{BackgroundColor, BorderColor, BorderRadius, Node};

#[derive(Debug, Clone, Copy)]
pub struct Background {
    pub color: BackgroundColor,
}

#[derive(Debug, Clone, Copy)]
pub struct TextStyle {
    pub color: Color,
    // 必要なら font_size や font_weight などを将来的に追加
}
