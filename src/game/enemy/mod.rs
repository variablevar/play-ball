pub mod component;
pub mod event;
pub mod resource;
pub mod system;

use bevy::prelude::*;

use crate::GameState;

use self::system::{despawn_enemy, move_enemy, set_enemy_boundry, spawn_enemy};

use super::SimulationState;

pub struct EnemyPlugin;

impl Plugin for EnemyPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(OnEnter(GameState::Game), spawn_enemy)
            .add_systems(OnExit(GameState::Game), despawn_enemy)
            .add_systems(
                Update,
                (set_enemy_boundry, move_enemy)
                    .chain()
                    .run_if(in_state(GameState::Game))
                    .run_if(in_state(SimulationState::Playing)),
            );
    }
}
