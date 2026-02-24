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
            app.active_screen = Some(Screen::Login(screens::login::State {
                input_mode: InputMode::Editing(0),
                ..Default::default()
            }))
        }
        ScreenId::Counter => {
            app.active_screen = Some(Screen::Counter(screens::counter::State {
                ..Default::default()
            }))
        }
    }

    Ok(())
}