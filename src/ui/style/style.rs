use bevy::color::Color;
use bevy::ui::{BackgroundColor, BorderColor, BorderRadius, Node};

#[derive(Debug, Clone, Copy)]
pub struct Background {
    pub color: BackgroundColor,
}

#[derive(Debug, Clone, Copy)]
pub struct TextStyle {
    pub color: Color,
    pub font_size: f32,
}
