use crate::{
    prelude::*,
    update::{
        common,
        screens::*,
        app
    },
    model::Screen,
};

pub async fn update(app: &mut App, event: Event, tx: &Sender<Event>) -> Result<()> {
    match event {
        // Terminal key inputs
        Event::CrosstermKey(key)
        => {
            let App { active_screen, data, .. } = app;

            // Key inputs common to all screens
            match key.code {
                KeyCode::Char('c') if key.modifiers == KeyModifiers::CONTROL => tx.send(Event::Quit)?,
                _ => {}
            }

            // Screen-specific key inputs
            match active_screen {
                Screen::Login(state) => login::update(state, data, key, tx).await,
            }
        }

        // Enter screen
        Event::EnterScreen(screen)
        => app::enter_screen(app, screen).await,

        Event::Quit
        => common::quit(&mut app.data),

        /*Event::Quit | Event::TimeoutTick(_) | Event::KeyInput(..) |
        Event::SwitchInput | Event::NextInput | Event::PrevInput |
        Event::SwitchAction | Event::SelectAction |
        Event::EnterPopup(_) | Event::SwitchDiv | Event::ToggleDisplayMsg |
        Event::UpdatePaymentInfo
        => common::update(app, event).await,
        */

        Event::Resize
        => Ok(()),

        _ => panic!("Received event {:?} without assigned update function", event)
    }
}