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
            }));
        }
        ScreenId::Counter => {
            app.active_screen = Some(Screen::Counter(screens::counter::State {
                action_sel: Some(0),
                sidebar_state: ListState::default().with_selected(Some(0)),
                input_mode: InputMode::Editing(0),
                ..Default::default()
            }));
        }
    }

    Ok(())
}