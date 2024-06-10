use bevy::ecs::system::Resource;

#[derive(Resource, Debug, Copy, Clone, PartialEq)]
pub enum SelectGameOverMenu {
    Restart,
    BackTitle,
}

impl SelectGameOverMenu {
    pub fn next_menu(&self) -> SelectGameOverMenu {
        match self {
            Self::Restart => Self::BackTitle,
            Self::BackTitle => Self::Restart,
        }
    }

    pub fn prev_menu(&self) -> SelectGameOverMenu {
        match self {
            Self::Restart => Self::BackTitle,
            Self::BackTitle => Self::Restart,
        }
    }
}
