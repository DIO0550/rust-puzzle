use bevy::color::Color;

// pub enum ColorTheme {}

// impl ColorTheme {
//     pub const PANACHE: Color = Color::srgb(244.0 / 255.0, 250.0 / 255.0, 247.0 / 255.0);
//     pub const CHROME_WHITE: Color = Color::srgb(225.0 / 255.0, 236.0 / 255.0, 200.0 / 255.0);
//     pub const SPROUT: Color = Color::srgb(196.0 / 255.0, 215.0 / 255.0, 178.0 / 255.0);
//     pub const NORWAY: Color = Color::srgb(160.0 / 255.0, 196.0 / 255.0, 157.0 / 255.0);
// }

// ===== オーシャン&アクアティック（海の動物）=====
pub enum ColorTheme {}

impl ColorTheme {
    // 基本カラー
    pub const BACKGROUND: Color = Color::srgb(240.0 / 255.0, 248.0 / 255.0, 255.0 / 255.0);
    pub const SUB_BACKGROUND: Color = Color::srgb(232.0 / 255.0, 244.0 / 255.0, 248.0 / 255.0);
    pub const TEXT: Color = Color::srgb(74.0 / 255.0, 144.0 / 255.0, 226.0 / 255.0);
    pub const SELECT_TEXT: Color = Color::WHITE;

    // プライマリカラー
    pub const PRIMARY: Color = Color::srgb(74.0 / 255.0, 144.0 / 255.0, 226.0 / 255.0);
    pub const PRIMARY_HOVER: Color = Color::srgb(58.0 / 255.0, 123.0 / 255.0, 200.0 / 255.0);
    pub const BORDER_PRIMARY: Color = Color::srgb(74.0 / 255.0, 144.0 / 255.0, 226.0 / 255.0);

    // 警告・強調カラー
    pub const WARNING: Color = Color::srgb(255.0 / 255.0, 140.0 / 255.0, 66.0 / 255.0);
    pub const WARNING_HOVER: Color = Color::srgb(230.0 / 255.0, 122.0 / 255.0, 50.0 / 255.0);
    pub const BORDER_WARNING: Color = Color::srgb(230.0 / 255.0, 122.0 / 255.0, 50.0 / 255.0);

    // ランキングカラー
    pub const RANK_GOLD_START: Color = Color::srgb(255.0 / 255.0, 215.0 / 255.0, 0.0 / 255.0);
    pub const RANK_GOLD_END: Color = Color::srgb(255.0 / 255.0, 165.0 / 255.0, 0.0 / 255.0);
    pub const RANK_SILVER_START: Color = Color::srgb(192.0 / 255.0, 192.0 / 255.0, 192.0 / 255.0);
    pub const RANK_SILVER_END: Color = Color::srgb(168.0 / 255.0, 168.0 / 255.0, 168.0 / 255.0);
    pub const RANK_BRONZE_START: Color = Color::srgb(205.0 / 255.0, 127.0 / 255.0, 50.0 / 255.0);
    pub const RANK_BRONZE_END: Color = Color::srgb(184.0 / 255.0, 115.0 / 255.0, 51.0 / 255.0);

    // トグルスイッチ
    pub const TOGGLE_OFF: Color = Color::srgb(204.0 / 255.0, 204.0 / 255.0, 204.0 / 255.0);
    pub const TOGGLE_ON: Color = Color::srgb(74.0 / 255.0, 144.0 / 255.0, 226.0 / 255.0);

    pub const GOLD: Color = Color::srgb(1.0, 0.843, 0.0);
    pub const SILVER: Color = Color::srgb(0.75, 0.75, 0.75);
    pub const BRONZE: Color = Color::srgb(0.804, 0.498, 0.196);
    pub const GRAY: Color = Color::srgb(0.5, 0.5, 0.5);
    

    pub const ORANGE: Color = Color::srgb(1.0, 0.647, 0.0);
    pub const DARK_GRAY: Color = Color::srgb(0.4, 0.4, 0.4);
    pub const BROWN: Color = Color::srgb(0.545, 0.271, 0.075);
}

