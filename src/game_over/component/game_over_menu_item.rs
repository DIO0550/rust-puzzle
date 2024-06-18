use bevy::ecs::component::Component;

#[derive(Component)]
pub struct GameOverMenuItem;

#[derive(Component, PartialEq, Eq)]
pub enum GameOverMenu {
    Restart,
    GoTitle,
}
