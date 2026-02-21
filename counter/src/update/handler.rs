use crate::{
    prelude::*,
    update::{
        common,
        screens::*,
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

            match active_screen {
                Screen::Login(state) => login::update(state, data, key, tx).await,
            }
        }

        Event::Quit | Event::TimeoutTick(_) | Event::KeyInput(..) |
        Event::SwitchInput | Event::NextInput | Event::PrevInput | 
        Event::SwitchAction | Event::SelectAction | Event::EnterScreen(_) |
        Event::EnterPopup(_) | Event::SwitchDiv | Event::ToggleDisplayMsg |
        Event::UpdatePaymentInfo
        => common::update(app, event).await,

        Event::Resize
        => Ok(()),

        _ => panic!("received event {:?} without assigned update function", event)
    }
}