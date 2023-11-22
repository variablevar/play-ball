use bevy::ecs::system::Resource;

#[derive(Resource, Clone)]
pub struct HighScore {
    scores: Vec<GameScore>,
}

impl HighScore {
    pub fn add_score(&mut self, score: GameScore) {
        self.scores.push(score);
        if self.scores.len() > 10 {
            self.scores.remove(0);
        }
    }

    pub fn get_high_score(&self) -> &Vec<GameScore> {
        &self.scores
    }
}
impl Default for HighScore {
    fn default() -> Self {
        Self { scores: vec![] }
    }
}

#[derive(Resource, Clone, Debug)]
pub struct GameScore {
    player: String,
    score: u32,
}
impl Default for GameScore {
    fn default() -> Self {
        Self {
            player: String::from("var"),
            score: 0,
        }
    }
}
impl GameScore {
    pub fn add_point(&mut self) {
        self.score += 1;
    }

    pub fn score(&self) -> u32 {
        self.score
    }

    pub fn player(&self) -> &str {
        self.player.as_ref()
    }

    pub fn set_player(&mut self, player: String) {
        self.player = player;
    }

    pub fn reset(&mut self) {
        self.score = 0;
    }
}
