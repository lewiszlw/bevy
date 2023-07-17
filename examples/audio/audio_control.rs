//! This example illustrates how to load and play an audio file, and control how it's played.

use bevy::prelude::*;
use bevy_internal::audio::RequestAudioPlayback;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_systems(Startup, setup)
        .add_systems(Update, (update_speed, pause, volume, playback))
        .run();
}

fn setup(mut commands: Commands, asset_server: Res<AssetServer>) {
    commands.spawn((
        AudioBundle {
            source: asset_server.load("sounds/Windless Slopes.ogg"),
            settings: PlaybackSettings::MANUALLY,
            ..default()
        },
        MyMusic,
    ));
}

#[derive(Component)]
struct MyMusic;

fn playback(
    keyboard_input: Res<Input<KeyCode>>,
    music_controller: Query<Entity, With<MyMusic>>,
    mut request_audio_playback_ew: EventWriter<RequestAudioPlayback>,
) {
    if keyboard_input.just_pressed(KeyCode::F1) {
        if let Ok(entity) = music_controller.get_single() {
            request_audio_playback_ew.send(RequestAudioPlayback { audio: entity });
        }
    }
}

fn update_speed(music_controller: Query<&AudioSink, With<MyMusic>>, time: Res<Time>) {
    if let Ok(sink) = music_controller.get_single() {
        sink.set_speed(((time.elapsed_seconds() / 5.0).sin() + 1.0).max(0.1));
    }
}

fn pause(keyboard_input: Res<Input<KeyCode>>, music_controller: Query<&AudioSink, With<MyMusic>>) {
    if keyboard_input.just_pressed(KeyCode::Space) {
        if let Ok(sink) = music_controller.get_single() {
            sink.toggle();
        }
    }
}

fn volume(keyboard_input: Res<Input<KeyCode>>, music_controller: Query<&AudioSink, With<MyMusic>>) {
    if let Ok(sink) = music_controller.get_single() {
        if keyboard_input.just_pressed(KeyCode::Plus) {
            sink.set_volume(sink.volume() + 0.1);
        } else if keyboard_input.just_pressed(KeyCode::Minus) {
            sink.set_volume(sink.volume() - 0.1);
        }
    }
}
