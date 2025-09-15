use bevy::prelude::*;

pub fn update_text<TextMarker: Component, R: Resource + ToString>(
    mut text_query: Query<&mut Text, With<TextMarker>>,
    resource: Res<R>,
) {
    if !resource.is_changed() {
        return;
    }

    let current_text_value = resource.to_string();
    for mut text in text_query.iter_mut() {
        **text = current_text_value.clone();
    }
}
