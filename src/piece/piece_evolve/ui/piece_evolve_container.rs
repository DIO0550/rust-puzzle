use bevy::{
    ecs::{component::Component, entity::Entity, system::Commands},
    ui::{node_bundles::NodeBundle, Display, FlexDirection, PositionType, Style, UiRect, Val},
    utils::default,
};

#[derive(Component)]
pub(crate) struct PieceEvolveContainer;
impl PieceEvolveContainer {
    fn style() -> Style {
        Style {
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
            .spawn((
                NodeBundle {
                    style: Self::style(),
                    ..default()
                },
                PieceEvolveContainer,
            ))
            .id();
    }
}
