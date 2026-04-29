use bevy::ecs::bundle::Bundle;
use bevy::ecs::component::Component;
use bevy::ui::{AlignItems, FlexDirection, JustifyContent, Node, UiRect, Val};

#[derive(Component)]
pub struct HighScoreContainer;

pub fn high_score_container() -> impl Bundle {
    (
        Node {
            padding: UiRect::all(Val::Px(10.0)),
            justify_content: JustifyContent::SpaceBetween,
            row_gap: Val::Px(10.0),
            flex_direction: FlexDirection::Column,
            align_items: AlignItems::Center,
            ..Default::default()
        },
        HighScoreContainer,
    )
}
