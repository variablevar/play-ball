pub mod systems;

pub mod components;
pub mod styles;

use bevy::prelude::*;

use crate::GameState;

use self::systems::layouts::spawn_main_menu;

pub struct MainMenuPlugin;

impl Plugin for MainMenuPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(OnEnter(GameState::MainMenu), spawn_main_menu);
    }
}
