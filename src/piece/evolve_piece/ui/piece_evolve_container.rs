use bevy::{
    ecs::{component::Component, entity::Entity, system::Commands},
    ui::{node_bundles::NodeBundle, FlexDirection, Style, UiRect, Val},
    utils::default,
};

#[derive(Component)]
pub(crate) struct PieceEvolveContainer;
impl PieceEvolveContainer {
    fn style() -> Style {
        Style {
            margin: UiRect {
                left: (Val::Px(50.0)),
                right: (Val::Px(42.0)),
                top: (Val::Px(40.0)),
                bottom: (Val::Px(0.0)),
            },

            row_gap: Val::Px(60.0),
            flex_direction: FlexDirection::Column,
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
