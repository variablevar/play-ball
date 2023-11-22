use bevy::{
    app::AppExit,
    ecs::{
        event::EventWriter,
        query::{Changed, With},
        schedule::NextState,
        system::{Query, ResMut},
    },
    ui::{BackgroundColor, Interaction},
};

use crate::{
    main_menu::{
        components::ButtonComponent,
        styles::{BUTTON_HOVER_COLOR, BUTTON_NONE_COLOR, BUTTON_PRESSED_COLOR},
    },
    GameState,
};

pub fn intract_with_buttons(
    mut button_query: Query<
        (&Interaction, &ButtonComponent, &mut BackgroundColor),
        (Changed<Interaction>, With<ButtonComponent>),
    >,
    mut event_writer: EventWriter<AppExit>,
    mut app_state: ResMut<NextState<GameState>>,
) {
    let Ok((interaction, button_component, mut background_color)) = button_query.get_single_mut()
    else {
        return;
    };
    match *interaction {
        Interaction::Pressed => {
            *background_color = BUTTON_PRESSED_COLOR.into();
            match button_component {
                ButtonComponent::PlayButton => {
                    app_state.set(GameState::Game);
                }
                ButtonComponent::QuitButton => {
                    event_writer.send(AppExit);
                }
            }
        }
        Interaction::Hovered => {
            *background_color = BUTTON_HOVER_COLOR.into();
        }
        Interaction::None => {
            *background_color = BUTTON_NONE_COLOR.into();
        }
    }
}
