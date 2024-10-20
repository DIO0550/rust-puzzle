use bevy::{asset::Handle, audio::AudioSource, prelude::Resource};

#[derive(Resource)]
pub struct PieceFallSound(pub Handle<AudioSource>);

#[derive(Resource)]
pub struct PieceUnionSound(pub Handle<AudioSource>);
