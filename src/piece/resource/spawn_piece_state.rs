use bevy::ecs::system::Resource;

#[derive(Resource, Debug, Copy, Clone, PartialEq)]
pub enum SpawnPieceState {
    Wait,
    ShouldSpawn,
}
