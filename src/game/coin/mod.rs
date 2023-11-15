pub mod component;
pub mod event;
pub mod resource;
pub mod system;

use bevy::prelude::*;

use self::{
    resource::CoinResource,
    system::{spawn_coin, spawn_coin_over_time, spawn_coin_tick_time},
};

pub struct CoinPlugin;

impl Plugin for CoinPlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<CoinResource>()
            .add_systems(Startup, spawn_coin)
            .add_systems(Update, spawn_coin_over_time)
            .add_systems(Update, spawn_coin_tick_time);
    }
}
