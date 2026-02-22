mod login;
mod enter_popup;

use crate::{
    prelude::*,
    model::screens,
    update::{
        common::{quit, switch_input, input},
    },
};
use login::try_login;

pub async fn update(
    state: &mut screens::login::State,
    data: &mut AppData,
    key: KeyEvent,
    tx: &Sender<Event>
) -> Result<()> {
    match key.code {
        KeyCode::Esc => quit(data)?,

        KeyCode::Tab => switch_input(&mut state.input_mode)?,

        KeyCode::Enter => try_login(state, tx).await?,

        _ => input(key, &state.input_mode, &mut state.inputs)?
    }

    Ok(())
}