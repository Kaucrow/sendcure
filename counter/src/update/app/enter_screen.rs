use crate::{
    prelude::*,
    model::{
        Screen,
        ScreenId,
        screens,
        input::InputMode,
    },
};

pub async fn enter_screen(app: &mut App, screen: ScreenId) -> Result<()> {
    match screen {
        ScreenId::Login => {
            app.active_screen = Screen::Login(screens::login::State {
                input_mode: InputMode::Editing(0),
                ..Default::default()
            })
        }
    }

    Ok(())
}