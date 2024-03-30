use bevy::prelude::*;

struct ScorePlugin;
impl Plugin for ScorePlugin {
    fn build(&self, app: &mut App) {
        app.insert_resource(MyResource)
            .add_event::<MyEvent>()
            .add_startup_system(plugin_setup)
            .add_system(plugin_system);
    }
}
