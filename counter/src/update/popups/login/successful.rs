use crate::{
    prelude::*,
    model::{screens, popups, ScreenId},
};

pub async fn update(
    _app: &mut App,
    _state: &mut screens::login::State,
    _pop_state: &mut popups::login::successful::State,
    key: KeyEvent,
    tx: &Sender<Event>
) -> Result<()> {
    match key.code {
        _ => tx.send(Event::EnterScreen(ScreenId::Counter))?
    }

    Ok(())
}