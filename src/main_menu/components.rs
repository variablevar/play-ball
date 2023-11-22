use bevy::ecs::component::Component;

#[derive(Component)]
pub struct MainMenu;

#[derive(Component)]
pub enum ButtonComponent {
    PlayButton,
    QuitButton,
}
