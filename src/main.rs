use bevy::app::App;
use bevy::prelude::*;
use game::GamePlugin;
use main_menu::MainMenuPlugin;
use systems::toggle_game_mode;

pub mod game;
pub mod main_menu;
pub mod plugins;
pub mod systems;

pub mod constant;

fn main() {
    App::new()
        .add_state::<GameState>()
        .add_plugins(DefaultPlugins)
        .add_plugins(GamePlugin)
        .add_plugins(MainMenuPlugin)
        .add_systems(Update, toggle_game_mode)
        .run();
}

#[derive(States, Default, Debug, Hash, PartialEq, Eq, Clone)]
pub enum GameState {
    #[default]
    MainMenu,
    Game,
    GameOver,
}

#[cfg(test)]
mod tests {
    use rand::random;

    #[test]
    fn it_should_generate_random_number_u8() {
        for _ in 1..100 {
            let no: u8 = (random::<f32>() * 20.0) as u8;
            println!("{}", no);
            assert!(no < 20)
        }
    }
}
