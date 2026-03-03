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
    // Reset app clear screen flag
    app.should_clear_screen = false;

    match event {
        // Terminal key inputs
        Event::CrosstermKey(key)
        => {
            // Key inputs common to all screens
            match key.code {
                KeyCode::Char('c') if key.modifiers == KeyModifiers::CONTROL => tx.send(Event::Quit)?,
                _ => {}
            }

            // Screen-specific key inputs
            if let Some(mut screen) = app.active_screen.take() {
                match &mut screen {
                    Screen::Login(state) => login::update(app, state, key, tx).await?,
                    Screen::Counter(state) => counter::update(app, state, key, tx).await?,
                }

                app.active_screen = Some(screen);
            }

            Ok(())
        }

        // Enter screen
        Event::EnterScreen(screen)
        => app::enter_screen(app, screen).await,

        Event::Quit
        => common::quit(app),

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