use bevy::ecs::system::Resource;

#[derive(Resource)]
pub struct GameScore {
    score: u32,
}
impl Default for GameScore {
    fn default() -> Self {
        Self { score: 0 }
    }
}
impl GameScore {
    pub fn add_point(&mut self) {
        self.score += 1;
    }

    pub fn score(&self) -> u32 {
        self.score
    }
}
