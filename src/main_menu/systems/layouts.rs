use bevy::asset::AssetServer;
use bevy::prelude::*;

use crate::main_menu::{
    components::{ButtonComponent, MainMenu},
    styles::universal_style,
};
pub fn spawn_main_menu(mut commands: Commands, asset_server: Res<AssetServer>) {
    let _ = build_main_menu(&mut commands, asset_server);
}

pub fn despawn_main_menu(mut commands: Commands, main_menu_query: Query<Entity, With<MainMenu>>) {
    let main_menu_entity = main_menu_query.get_single().unwrap();
    commands.entity(main_menu_entity).despawn_recursive();
}

pub fn build_main_menu(commands: &mut Commands, asset_server: Res<AssetServer>) -> Entity {
    commands
        .spawn((
            NodeBundle {
                style: Style {
                    flex_direction: FlexDirection::Column,
                    justify_content: JustifyContent::SpaceAround,
                    align_content: AlignContent::Center,
                    align_items: AlignItems::Center,
                    height: Val::Percent(100.0),
                    width: Val::Percent(100.0),
                    ..Default::default()
                },
                background_color: BackgroundColor(Color::DARK_GRAY),
                ..Default::default()
            },
            MainMenu,
        ))
        .with_children(|parent| {
            // TITLE
            parent
                .spawn(NodeBundle {
                    style: universal_style(),
                    ..Default::default()
                })
                .with_children(|parent| {
                    parent.spawn(TextBundle {
                        text: Text {
                            sections: vec![TextSection {
                                value: "PLAY BALL GAME".to_string(),
                                style: TextStyle {
                                    font: asset_server.load("fonts/Ghosts.ttf"),
                                    font_size: 72.0,
                                    color: Color::WHITE,
                                },
                            }],
                            ..Default::default()
                        },
                        ..Default::default()
                    });
                });
            // BUTTONS
            parent
                .spawn(NodeBundle {
                    style: Style {
                        flex_direction: FlexDirection::Column,
                        justify_content: JustifyContent::SpaceAround,
                        align_content: AlignContent::Center,
                        align_items: AlignItems::Center,
                        height: Val::Percent(50.0),
                        width: Val::Percent(50.0),
                        ..Default::default()
                    },
                    ..Default::default()
                })
                .with_children(|parent| {
                    // PLAY BUTTON
                    parent
                        .spawn((
                            ButtonBundle {
                                border_color: BorderColor(Color::BLACK),
                                style: universal_style(),
                                background_color: Color::SEA_GREEN.into(),
                                ..Default::default()
                            },
                            ButtonComponent::PlayButton,
                        ))
                        .with_children(|parent| {
                            parent.spawn(TextBundle {
                                text: Text {
                                    sections: vec![TextSection {
                                        value: "Play".to_string(),
                                        style: TextStyle {
                                            font: asset_server.load("fonts/Anywhere.ttf"),
                                            font_size: 54.0,
                                            color: Color::BLACK,
                                        },
                                    }],
                                    ..Default::default()
                                },
                                ..Default::default()
                            });
                        });

                    // QUIT BUTTON
                    parent
                        .spawn((
                            ButtonBundle {
                                border_color: BorderColor(Color::BLACK),
                                style: universal_style(),
                                background_color: Color::SEA_GREEN.into(),
                                ..Default::default()
                            },
                            ButtonComponent::QuitButton,
                        ))
                        .with_children(|parent| {
                            parent.spawn(TextBundle {
                                text: Text {
                                    sections: vec![TextSection {
                                        value: "Quit".to_string(),
                                        style: TextStyle {
                                            font: asset_server.load("fonts/Anywhere.ttf"),
                                            font_size: 54.0,
                                            color: Color::BLACK,
                                        },
                                    }],
                                    ..Default::default()
                                },
                                ..Default::default()
                            });
                        });
                });
        })
        .id()
}
