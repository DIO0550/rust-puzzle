#[derive(SystemParam)]
pub struct SoundPlayer<'w, 's> {
    commands: Commands<'w, 's>,
}

impl<'w, 's> SoundPlayer<'w, 's> {
    pub fn play_sound(&mut self, sound: Handle<AudioSource>) {
        self.commands.spawn(AudioBundle {
            source: sound,
            settings: PlaybackSettings::DESPAWN,
        });
    }
}
