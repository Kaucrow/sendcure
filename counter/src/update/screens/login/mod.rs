mod login;

use crate::{
    prelude::*,
    model::{ScreenId, screens},
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

        KeyCode::Enter => {
            if try_login().is_ok() {
                tx.send(Event::EnterScreen(ScreenId::Login))?;
            }
        }

        _ => input(key, &state.input_mode, &mut state.inputs)?
    }

    Ok(())
}