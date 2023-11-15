use bevy::{
    prelude::Resource,
    time::{Timer, TimerMode},
};

use crate::constant::DURATION_SPAWN_COIN;

#[derive(Resource)]
pub struct CoinResource {
    pub timer: Timer,
}

impl Default for CoinResource {
    fn default() -> Self {
        Self {
            timer: Timer::from_seconds(DURATION_SPAWN_COIN, TimerMode::Repeating),
        }
    }
}
