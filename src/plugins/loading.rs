use bevy::prelude::*;
use bevy_asset_loader::prelude::*;

#[derive(AssetCollection, Resource)]
pub struct AppAssets {
    #[asset(path = "sprites/blue_body_circle.png")]
    pub image_player: Handle<Image>,
    #[asset(path = "sprites/red_body_circle.png")]
    pub image_enemy: Handle<Image>,
    #[asset(path = "audio/footstep_carpet_000.ogg")]
    pub music_hit_wall_one: Handle<AudioSource>,
    #[asset(path = "audio/footstep_carpet_001.ogg")]
    pub music_hit_wall_second: Handle<AudioSource>,
    #[asset(path = "audio/impactPlate_heavy_003.ogg")]
    pub music_game_over: Handle<AudioSource>,
}
