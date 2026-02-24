mod login;
mod enter_popup;

use crate::{
    prelude::*,
    model::{screens, Popup},
    update::{
        common::{quit, switch_input, input},
        popups,
    },
};
use login::try_login;

pub async fn update(
    app: &mut App,
    state: &mut screens::login::State,
    key: KeyEvent,
    tx: &Sender<Event>
) -> Result<()> {
    if let Some(mut popup) = state.active_popup.take() {
        match &mut popup {
            Popup::LoginSuccessful(pop_state) => {
                popups::login::successful::update(app, state, pop_state, key, tx).await?
            },
            _ => unimplemented!("Popup {:?} in login", popup)
        }

        // Return the active popup
        state.active_popup = Some(popup);

        return Ok(());
    }

    match key.code {
        KeyCode::Esc => quit(app)?,

        KeyCode::Tab => switch_input(&mut state.input_mode)?,

        KeyCode::Enter => try_login(state, tx).await?,

        _ => input(key, &state.input_mode, &mut state.inputs)?
    }

    Ok(())
}