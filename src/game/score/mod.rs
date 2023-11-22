pub mod component;
pub mod event;
pub mod resource;
pub mod system;

use bevy::prelude::*;

use crate::GameState;

use self::{
    resource::{GameScore, HighScore},
    system::{despawn_score, on_update_score, spawn_score},
};

use super::SimulationState;

pub struct ScorePlugin;

impl Plugin for ScorePlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<GameScore>()
            .init_resource::<HighScore>()
            .add_systems(OnEnter(GameState::Game), spawn_score)
            .add_systems(OnExit(GameState::Game), despawn_score)
            .add_systems(
                Update,
                on_update_score
                    .run_if(in_state(GameState::Game))
                    .run_if(in_state(SimulationState::Playing)),
            );
    }
}
