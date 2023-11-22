use bevy::{app::AppExit, prelude::*, window::PrimaryWindow};

use crate::{constant::BOX_SIZE, GameState};

use super::{
    enemy::component::Enemy,
    event::GameOver,
    player::component::Player,
    score::resource::{GameScore, HighScore},
    SimulationState,
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
    mut player_query: Query<&Transform, With<Player>>,
    asset_server: Res<AssetServer>,
    enemy_query: Query<&Transform, With<Enemy>>,
    mut game_over_event_writer: EventWriter<GameOver>,
    mut game_score: ResMut<GameScore>,
    mut high_score: ResMut<HighScore>,
) {
    for enemy_transform in enemy_query.iter() {
        match player_query.get_single_mut() {
            Ok(player_transform) => {
                let distance = enemy_transform
                    .translation
                    .distance(player_transform.translation);

                if distance < BOX_SIZE {
                    // Add the AudioSource to the world with the Handle
                    commands.spawn(AudioBundle {
                        source: asset_server.load("audio/impactBell_heavy_001.ogg"),
                        ..Default::default()
                    });
                    game_over_event_writer.send(GameOver {
                        score: game_score.score(),
                    });
                    high_score.add_score(game_score.to_owned());
                    game_score.reset();
                }
            }
            Err(_) => {}
        }
    }
}

pub fn handle_game_over(
    mut commands: Commands,
    mut game_over_event_reader: EventReader<GameOver>,
    high_score: Res<HighScore>,
) {
    for event in game_over_event_reader.read() {
        println!("The previous score are : {:?}", high_score.get_high_score());
        println!("Your final score is: {}", event.score.to_string());
        commands.insert_resource(NextState(Some(SimulationState::Paused)));
        commands.insert_resource(NextState(Some(GameState::GameOver)));
    }
}

pub fn on_exit_game(mut event_writer: EventWriter<AppExit>, keyboard_event: Res<Input<KeyCode>>) {
    if keyboard_event.just_released(KeyCode::Escape) {
        event_writer.send(AppExit);
    }
}

pub fn toggle_simulation(
    mut commands: Commands,
    keyboard_event: Res<Input<KeyCode>>,
    simulation_state: Res<State<SimulationState>>,
) {
    if keyboard_event.just_pressed(KeyCode::Space) {
        let state = simulation_state.get();
        if *state == SimulationState::Paused {
            commands.insert_resource(NextState(Some(SimulationState::Playing)));
        } else {
            commands.insert_resource(NextState(Some(SimulationState::Paused)));
        }
    }
}
