use bevy::ecs::component::Component;

#[derive(Component, PartialEq, Eq)]
pub enum TitleMenuItem {
    StartGame,
    HighScore,
}

impl TitleMenuItem {
    pub fn next(&self) -> Self {
        match self {
            Self::StartGame => Self::HighScore,
            Self::HighScore => Self::StartGame,
        }
    }

    pub fn prev(&self) -> Self {
        match self {
            Self::StartGame => Self::HighScore,
            Self::HighScore => Self::StartGame,
        }
    }

    pub fn to_string(&self) -> String {
        match self {
            Self::StartGame => "スタート".to_string(),
            Self::HighScore => "ハイスコア".to_string(),
        }
    }
}
