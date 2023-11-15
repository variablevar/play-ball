pub mod component;
pub mod event;
pub mod resource;
pub mod system;

use bevy::prelude::*;

use self::{
    resource::GameScore,
    system::{on_update_score, spawn_score},
};

pub struct ScorePlugin;

impl Plugin for ScorePlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<GameScore>()
            .add_systems(Startup, spawn_score)
            .add_systems(Update, on_update_score);
    }
}
