use bevy::{app::AppExit, prelude::*, window::PrimaryWindow};

use crate::constant::BOX_SIZE;

use super::{
    enemy::component::Enemy, event::GameOver, player::component::Player, score::resource::GameScore,
};

pub fn spawn_camera(mut commands: Commands, window_query: Query<&Window, With<PrimaryWindow>>) {
    let window = window_query.get_single().unwrap();
    commands.spawn(Camera2dBundle {
        transform: Transform::from_xyz(window.width() / 2.0, window.height() / 2.0, 0.0),
        ..Default::default()
    });
}

pub fn on_game_over(
    mut commands: Commands,
    mut player_query: Query<(Entity, &Transform), With<Player>>,
    asset_server: Res<AssetServer>,
    enemy_query: Query<&Transform, With<Enemy>>,
    mut game_over_event_writer: EventWriter<GameOver>,
    game_score: Res<GameScore>,
) {
    for enemy_transform in enemy_query.iter() {
        match player_query.get_single_mut() {
            Ok((player_entity, player_transform)) => {
                let distance = enemy_transform
                    .translation
                    .distance(player_transform.translation);

                if distance < BOX_SIZE {
                    // Add the AudioSource to the world with the Handle
                    commands.spawn(AudioBundle {
                        source: asset_server.load("audio/impactBell_heavy_001.ogg"),
                        ..Default::default()
                    });
                    commands.entity(player_entity).despawn();
                    game_over_event_writer.send(GameOver {
                        score: game_score.score(),
                    });
                }
            }
            Err(_) => println!("Game Over"),
        }
    }
}

pub fn handle_game_over(mut game_over_event_reader: EventReader<GameOver>) {
    for event in game_over_event_reader.read() {
        println!("Your final score is: {}", event.score.to_string());
    }
}

pub fn on_exit_game(mut event_writer: EventWriter<AppExit>, keyboard_event: Res<Input<KeyCode>>) {
    if keyboard_event.just_released(KeyCode::Escape) {
        event_writer.send(AppExit);
    }
}
