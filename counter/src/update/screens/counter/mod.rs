mod navigation;
mod recv_pkg;
mod send_pkg;

use crate::{
    prelude::*,
    model::screens::{self, counter},
    update::common::{quit, input, navigate}
};

use navigation::switch_subscreen;

#[allow(unreachable_patterns)]
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
        // Client C.I. input
        Some(0) => {
            match key.code {
                KeyCode::Enter => {
                    state.client = Some(state.inputs.get(0)?.input.value().parse::<u32>()?);
                },
                _ => input(key, &mut state.inputs)?
            };
        }
        // Sidebar
        Some(1) => {
            match key.code {
                KeyCode::Char('j') | KeyCode::Char('k') | KeyCode::Up | KeyCode::Down
                    => switch_subscreen(state, key.code),
                _ => {}
            }
        }
        // Main tab
        Some(2) => {
            match state.tabs.get_mut(state.sidebar_state.selected().unwrap())? {
                counter::Tab::Received(tab_state) => recv_pkg::update(tab_state, key.code),
                counter::Tab::Send(tab_state) => send_pkg::update(tab_state, key)?,
                _ => unimplemented!()
            }
        }
        _ => {}
    }

    Ok(())
}