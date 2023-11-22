pub mod component;
pub mod event;
pub mod resource;
pub mod system;

use bevy::prelude::*;

use crate::GameState;

use self::{
    resource::CoinResource,
    system::{despawn_coin, spawn_coin, spawn_coin_over_time, spawn_coin_tick_time},
};

use super::SimulationState;

pub struct CoinPlugin;

impl Plugin for CoinPlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<CoinResource>()
            .add_systems(OnEnter(GameState::Game), spawn_coin)
            .add_systems(OnExit(GameState::Game), despawn_coin)
            .add_systems(
                Update,
                (spawn_coin_tick_time, spawn_coin_over_time)
                    .chain()
                    .run_if(in_state(GameState::Game))
                    .run_if(in_state(SimulationState::Playing)),
            );
    }
}
