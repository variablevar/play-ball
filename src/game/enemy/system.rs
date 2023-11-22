use bevy::{prelude::*, window::PrimaryWindow};
use rand::random;

use crate::constant::{BOX_SIZE, ENEMY_COUNT, ENEMY_SPEED};

use super::component::Enemy;

pub fn spawn_enemy(
    mut commands: Commands,
    window_query: Query<&Window, With<PrimaryWindow>>,
    asset_server: Res<AssetServer>,
) {
    let window = window_query.get_single().unwrap();

    for id in 0..ENEMY_COUNT {
        let x = random::<f32>() * window.width();
        let y = random::<f32>() * window.height();
        commands.spawn((
            SpriteBundle {
                texture: asset_server.load("sprites/red_body_circle.png"),
                transform: Transform::from_xyz(x, y, 0.0),
                ..default()
            },
            Enemy {
                direction: Vec2::new(random::<f32>(), random::<f32>()).normalize(),
                id,
            },
        ));
    }
}

pub fn despawn_enemy(mut commands: Commands, mut enemy_query: Query<Entity, With<Enemy>>) {
    for entity in enemy_query.iter_mut() {
        commands.entity(entity).despawn();
    }
}

pub fn move_enemy(time: Res<Time>, mut query: Query<(&mut Transform, &Enemy)>) {
    for (mut transfrom, enemy) in query.iter_mut() {
        transfrom.translation += Vec3::new(enemy.direction.x, enemy.direction.y, 0.0)
            * ENEMY_SPEED
            * time.delta_seconds();
    }
}

pub fn set_enemy_boundry(
    mut movement_query: Query<(&Transform, &mut Enemy)>,
    window_query: Query<&Window, With<PrimaryWindow>>,
) {
    let window = window_query.get_single().unwrap();
    for (transform, mut enemy) in movement_query.iter_mut() {
        let mut is_hitting_wall = false;
        let half_size = BOX_SIZE / 2.0;
        if transform.translation.x < half_size {
            enemy.direction.x *= -1.0;
            is_hitting_wall = true;
        }
        if transform.translation.x > window.width() - half_size {
            enemy.direction.x *= -1.0;
            is_hitting_wall = true;
        }
        if transform.translation.y < half_size {
            enemy.direction.y *= -1.0;
            is_hitting_wall = true;
        }
        if transform.translation.y > window.height() - half_size {
            enemy.direction.y *= -1.0;
            is_hitting_wall = true;
        }

        let _ = enemy.direction.normalize();

        if is_hitting_wall {}
    }
}
