use bevy::ecs::component::Component;

#[derive(Debug, Component)]
pub struct UpdateableText {
    pub id: String,
}
