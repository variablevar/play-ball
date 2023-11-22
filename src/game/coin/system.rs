use bevy::{prelude::*, window::PrimaryWindow};
use rand::random;

use crate::constant::COIN_SIZE;

use super::{component::Coin, resource::CoinResource};

pub fn spawn_coin(
    window_query: Query<&Window, With<PrimaryWindow>>,
    asset_server: Res<AssetServer>,
    mut commands: Commands,
) {
    let window = window_query.get_single().unwrap();
    let half_coin_size = COIN_SIZE / 2.0;
    let no_of_coin_to_spawn: u8 = (random::<f32>() * 20.0) as u8;

    for id in 1..no_of_coin_to_spawn + 5 {
        commands.spawn((
            SpriteBundle {
                texture: asset_server.load("sprites/tile_coin.png"),
                transform: Transform::from_xyz(
                    (random::<f32>() * window.width()) - half_coin_size,
                    (random::<f32>() * window.height()) - half_coin_size,
                    0.0,
                ),
                ..default()
            },
            Coin { id },
        ));
    }
}

pub fn despawn_coin(mut coin_query: Query<Entity, With<Coin>>, mut commands: Commands) {
    for coin_entity in coin_query.iter_mut() {
        commands.entity(coin_entity).despawn();
    }
}
pub fn spawn_coin_over_time(
    window_query: Query<&Window, With<PrimaryWindow>>,
    coin_resource: Res<CoinResource>,
    asset_server: Res<AssetServer>,
    mut commands: Commands,
) {
    if coin_resource.timer.finished() {
        let window = window_query.get_single().unwrap();
        let half_coin_size = COIN_SIZE / 2.0;
        commands.spawn((
            SpriteBundle {
                texture: asset_server.load("sprites/tile_coin.png"),
                transform: Transform::from_xyz(
                    (random::<f32>() * window.width()) - half_coin_size,
                    (random::<f32>() * window.height()) - half_coin_size,
                    0.0,
                ),
                ..default()
            },
            Coin { id: 0 },
        ));
    }
}

pub fn spawn_coin_tick_time(mut coin_resource: ResMut<CoinResource>, time: Res<Time>) {
    coin_resource.timer.tick(time.delta());
}
