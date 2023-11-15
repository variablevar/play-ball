use bevy::prelude::*;
#[derive(Event, Debug, Clone, Default)]
pub struct GameOver {
    pub score: u32,
}
