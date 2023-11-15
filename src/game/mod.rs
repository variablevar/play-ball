use bevy::prelude::*;

use self::event::GameOver;
use self::{coin::CoinPlugin, enemy::EnemyPlugin, player::PlayerPlugin, score::ScorePlugin};

pub mod coin;
pub mod enemy;
pub mod player;
pub mod score;

pub mod component;
pub mod event;
pub mod resource;
pub mod system;

use self::system::*;

pub struct GamePlugin;

impl Plugin for GamePlugin {
    fn build(&self, app: &mut App) {
        app.add_event::<GameOver>()
            .add_plugins(CoinPlugin)
            .add_plugins(PlayerPlugin)
            .add_plugins(EnemyPlugin)
            .add_plugins(ScorePlugin)
            .add_systems(Startup, spawn_camera)
            .add_systems(Update, on_game_over)
            .add_systems(Update, on_exit_game);
    }
}
