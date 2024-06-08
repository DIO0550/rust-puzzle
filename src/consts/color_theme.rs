use bevy::render::color::Color;

pub enum ColorTheme {}

impl ColorTheme {
    pub const PANACHE: Color = Color::rgb(244.0 / 255.0, 250.0 / 255.0, 247.0 / 255.0);
    pub const CHROME_WHITE: Color = Color::rgb(225.0 / 255.0, 236.0 / 255.0, 200.0 / 255.0);
    pub const SPROUT: Color = Color::rgb(196.0 / 255.0, 215.0 / 255.0, 178.0 / 255.0);
    pub const NORWAY: Color = Color::rgb(160.0 / 255.0, 196.0 / 255.0, 157.0 / 255.0);
}
