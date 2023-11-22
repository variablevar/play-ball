use bevy::asset::AssetServer;
use bevy::prelude::*;
pub fn spawn_main_menu(mut commands: Commands, asset_server: Res<AssetServer>) {
    let entity = build_main_menu(&mut commands, asset_server);
}

pub fn despawn_main_menu() {}

pub fn build_main_menu(commands: &mut Commands, asset_server: Res<AssetServer>) -> Entity {
    commands
        .spawn(NodeBundle {
            style: Style {
                height: Val::Percent(100.0),
                width: Val::Percent(100.0),
                ..Default::default()
            },
            background_color: BackgroundColor(Color::CRIMSON),
            ..Default::default()
        })
        .id()
}
