use bevy::{
    ecs::{
        change_detection::DetectChanges,
        component::Component,
        entity::Entity,
        query::With,
        system::{Query, Res, Resource},
    },
    text::Text,
};

pub fn update_text<TextMarker: Component, R: Resource + ToString>(
    mut text_query: Query<(&mut Text, With<TextMarker>)>,
    resource: Res<R>,
) {
    if !resource.is_changed() {
        return;
    }

    let current_text_value = resource.to_string();
    for mut text_iter in text_query.iter_mut() {
        for section in &mut text_iter.0.sections {
            section.value = current_text_value.clone();
        }
    }
}
