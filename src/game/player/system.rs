use bevy::{prelude::*, window::PrimaryWindow};

use crate::{
    constant::{BOX_SIZE, COIN_SIZE, PLAYER_SPEED},
    game::{coin::component::Coin, score::resource::GameScore},
};

use super::component::Player;

pub fn spawn_player(
    mut commands: Commands,
    window_query: Query<&Window, With<PrimaryWindow>>,
    asset_server: Res<AssetServer>,
) {
    let window = window_query.get_single().unwrap();
    commands.spawn((
        SpriteBundle {
            texture: asset_server.load("sprites/blue_body_circle.png"),
            transform: Transform::from_xyz(window.width() / 2.0, window.height() / 2.0, 0.0),
            ..default()
        },
        Player::default(),
    ));
}

pub fn move_player(
    keyboard_event: Res<Input<KeyCode>>,
    time: Res<Time>,
    mut query: Query<&mut Transform, With<Player>>,
) {
    if let Ok(mut transform) = query.get_single_mut() {
        let mut direction = Vec3::ZERO;
        if keyboard_event.pressed(KeyCode::Down) {
            direction += Vec3::new(0.0, -1.0, 0.0)
        }
        if keyboard_event.pressed(KeyCode::Up) {
            direction += Vec3::new(0.0, 1.0, 0.0)
        }
        if keyboard_event.pressed(KeyCode::Left) {
            direction += Vec3::new(-1.0, 0.0, 0.0)
        }
        if keyboard_event.pressed(KeyCode::Right) {
            direction += Vec3::new(1.0, 0.0, 0.0)
        }

        transform.translation += direction * PLAYER_SPEED * time.delta_seconds();
    }
}

pub fn set_player_boundry(
    mut movement_query: Query<&mut Transform, With<Player>>,
    window_query: Query<&Window, With<PrimaryWindow>>,
) {
    let window = window_query.get_single().unwrap();
    if let Ok(mut transform) = movement_query.get_single_mut() {
        let half_size = BOX_SIZE / 2.0;
        if transform.translation.x < half_size {
            transform.translation.x = half_size;
        }
        if transform.translation.x > window.width() - half_size {
            transform.translation.x = window.width() - half_size;
        }
        if transform.translation.y < half_size {
            transform.translation.y = half_size;
        }
        if transform.translation.y > window.height() - half_size {
            transform.translation.y = window.height() - half_size;
        }
    }
}

pub fn add_point_to_player(
    mut commands: Commands,
    mut player_query: Query<&Transform, With<Player>>,
    coin_query: Query<(Entity, &Transform), With<Coin>>,
    mut player: ResMut<GameScore>,
    asset_server: Res<AssetServer>,
) {
    for (coin_entity, coin_transform) in coin_query.iter() {
        if let Ok(player_transform) = player_query.get_single_mut() {
            let distance = player_transform
                .translation
                .distance(coin_transform.translation);
            if distance < ((BOX_SIZE + COIN_SIZE) / 2.0) {
                commands.entity(coin_entity).despawn();
                player.add_point();
                // Add the AudioSource to the world with the Handle
                commands.spawn(AudioBundle {
                    source: asset_server.load("audio/impactTin_medium_000.ogg"),
                    ..Default::default()
                });
                println!("The player points {}", player.score());
            }
        }
    }
}
