mod navigation;

use crate::{
    prelude::*,
    model::screens,
    update::common::{quit, input, navigate}
};

use navigation::switch_subscreen;

pub async fn update(
    app: &mut App,
    state: &mut screens::counter::State,
    key: KeyEvent,
    tx: &Sender<Event>
) -> Result<()> {
    // Key inputs common to all tabs
    match key.code {
        KeyCode::Esc => quit(app)?,
        KeyCode::Tab => navigate(&mut state.action_sel, 2)?,
        _ => {}
    }

    // Tab-specific key inputs
    match state.action_sel {
        Some(0) => {
            match key.code {
                _ => input(key, &state.input_mode, &mut state.inputs)?
            };
        }
        Some(1) => {
            match key.code {
                KeyCode::Char('j') | KeyCode::Char('k') | KeyCode::Up | KeyCode::Down
                    => switch_subscreen(state, key.code),
                _ => {}
            }
        }
        _ => {}
    }

    Ok(())
}