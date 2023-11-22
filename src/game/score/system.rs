use bevy::{prelude::*, transform::commands, window::PrimaryWindow};

use super::{component::ScoreComponent, resource::GameScore};

pub fn on_update_score(
    player_score: Res<GameScore>,
    mut score_query: Query<&mut Text, With<ScoreComponent>>,
) {
    if player_score.is_changed() {
        let mut score_text = score_query.get_single_mut().unwrap();
        let text_section = score_text.sections.get_mut(0).unwrap();
        text_section.value = format!("Score {}", player_score.score());
    }
}

pub fn spawn_score(
    mut commands: Commands,
    game_score: Res<GameScore>,
    asset_server: Res<AssetServer>,
    window_query: Query<&Window, With<PrimaryWindow>>,
) {
    let window = window_query.get_single().unwrap();
    commands.spawn((
        Text2dBundle {
            text: Text {
                sections: vec![TextSection {
                    value: format!("Score {}", game_score.score()),
                    style: TextStyle {
                        font: asset_server.load("fonts/Anywhere.ttf"),
                        font_size: 100.0,
                        color: Color::WHITE,
                    },
                }],
                ..Default::default()
            },
            transform: Transform::from_xyz(window.width() - 100.0, window.height() - 25.0, 0.0),
            ..Default::default()
        },
        ScoreComponent,
    ));
}

pub fn despawn_score(mut commands: Commands, mut score_query: Query<Entity, With<ScoreComponent>>) {
    for socre in score_query.iter_mut() {
        commands.entity(socre).despawn();
    }
}

pub fn destory_score(mut game_score: ResMut<GameScore>) {
    game_score.reset();
}

pub fn init_score(mut commands: Commands) {
    commands.init_resource::<GameScore>();
}
