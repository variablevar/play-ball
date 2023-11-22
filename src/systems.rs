use bevy::prelude::*;

use crate::GameState;
pub fn toggle_game_mode(
    keyboard_event: Res<Input<KeyCode>>,
    state: Res<State<GameState>>,
    mut next_state: ResMut<NextState<GameState>>,
) {
    if keyboard_event.just_pressed(KeyCode::Tab) {
        if *(state.get()) == GameState::MainMenu {
            next_state.set(GameState::Game);
        } else if *(state.get()) == GameState::GameOver {
            next_state.set(GameState::MainMenu);
        } else {
            next_state.set(GameState::MainMenu);
        }
        println!("Mode changed to : {:?}", state.get());
    }
}
