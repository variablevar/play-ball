pub mod component;
pub mod event;
pub mod resource;
pub mod system;

use bevy::prelude::*;

use crate::GameState;

use self::system::{
    add_point_to_player, despawn_player, move_player, set_player_boundry, spawn_player,
};

use super::SimulationState;

pub struct PlayerPlugin;

impl Plugin for PlayerPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(OnEnter(GameState::Game), spawn_player)
            .add_systems(OnExit(GameState::Game), despawn_player)
            /* Both are same */
            .add_systems(
                Update,
                (move_player, set_player_boundry)
                    .chain()
                    .run_if(in_state(GameState::Game))
                    .run_if(in_state(SimulationState::Playing)),
            )
            .add_systems(
                Update,
                add_point_to_player
                    .run_if(in_state(GameState::Game))
                    .run_if(in_state(SimulationState::Playing)),
            );
    }
}
