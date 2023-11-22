use bevy::prelude::*;

use super::{component::ScoreComponent, resource::GameScore};

pub fn on_update_score(
    player_score: Res<GameScore>,
    mut score_query: Query<&mut Text, With<ScoreComponent>>,
) {
    if player_score.is_changed() {
        match score_query.get_single_mut() {
            Ok(mut score_text) => {
                let text_section = score_text.sections.get_mut(0).unwrap();
                text_section.value = format!("Score {}", player_score.score());
            }
            Err(_) => {}
        }
    }
}

pub fn spawn_score(
    mut commands: Commands,
    game_score: Res<GameScore>,
    asset_server: Res<AssetServer>,
) {
    commands
        .spawn(NodeBundle {
            style: Style {
                display: Display::Flex,
                flex_direction: FlexDirection::Row,
                justify_content: JustifyContent::SpaceAround,
                align_content: AlignContent::Center,
                align_items: AlignItems::Center,
                height: Val::Percent(10.0),
                width: Val::Percent(10.0),
                ..Default::default()
            },
            background_color: Color::rgba_u8(0, 0, 0, 64).into(),
            ..Default::default()
        })
        .with_children(|parent| {
            // COIN IMAGE
            // SCORE
            parent.spawn((
                TextBundle {
                    text: Text {
                        sections: vec![TextSection {
                            value: format!("SCORE {}", game_score.score()),
                            style: TextStyle {
                                font: asset_server.load("fonts/Ghosts.ttf"),
                                font_size: 24.0,
                                color: Color::WHITE,
                            },
                        }],
                        ..Default::default()
                    },
                    ..Default::default()
                },
                ScoreComponent,
            ));
        });
}

pub fn despawn_score(mut commands: Commands, mut score_query: Query<Entity, With<ScoreComponent>>) {
    for socre in score_query.iter_mut() {
        commands.entity(socre).despawn_recursive();
    }
}

pub fn destory_score(mut game_score: ResMut<GameScore>) {
    game_score.reset();
}

pub fn init_score(mut commands: Commands) {
    commands.init_resource::<GameScore>();
}
