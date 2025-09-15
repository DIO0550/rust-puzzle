use bevy::prelude::*;

#[derive(Component)]
pub(crate) struct PieceEvolveContainer;
impl PieceEvolveContainer {
    fn style() -> Node {
        Node {
            position_type: PositionType::Absolute,
            right: Val::Px(15.),
            bottom: Val::Px(15.),
            flex_direction: FlexDirection::Column,
            display: Display::Flex,
            ..default()
        }
    }

    pub(crate) fn spawn(commands: &mut Commands) -> Entity {
        return commands
            .spawn((Node { ..Self::style() }, PieceEvolveContainer))
            .id();
    }
}
