pub mod component;
pub mod event;
pub mod resource;
pub mod system;

use bevy::prelude::*;

use self::system::{add_point_to_player, move_player, set_player_boundry, spawn_player};

pub struct PlayerPlugin;

impl Plugin for PlayerPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, spawn_player)
            /* Both are same */
            // .add_systems(Update, move_player.before(set_player_boundry))
            .add_systems(Update, (move_player, set_player_boundry).chain())
            .add_systems(Update, add_point_to_player);
    }
}
