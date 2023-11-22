use bevy::prelude::Style;
use bevy::prelude::*;

pub fn universal_style() -> Style {
    Style {
        justify_content: JustifyContent::Center,
        align_content: AlignContent::Center,
        align_items: AlignItems::Center,
        border: UiRect {
            top: Val::Px(5.0),
            left: Val::Px(5.0),
            bottom: Val::Px(5.0),
            right: Val::Px(5.0),
        },
        padding: UiRect {
            top: Val::Percent(1.0),
            left: Val::Percent(1.0),
            bottom: Val::Percent(1.0),
            right: Val::Percent(1.0),
        },
        ..Style::DEFAULT
    }
}

pub const BUTTON_PRESSED_COLOR: Color = Color::TEAL;
pub const BUTTON_HOVER_COLOR: Color = Color::GREEN;
pub const BUTTON_NONE_COLOR: Color = Color::SEA_GREEN;
