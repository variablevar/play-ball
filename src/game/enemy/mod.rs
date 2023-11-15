pub mod component;
pub mod event;
pub mod resource;
pub mod system;

use bevy::prelude::*;

use self::system::{move_enemy, set_enemy_boundry, spawn_enemy};

pub struct EnemyPlugin;

impl Plugin for EnemyPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, spawn_enemy)
            .add_systems(Update, move_enemy.before(set_enemy_boundry))
            .add_systems(Update, set_enemy_boundry);
    }
}
