mod navigation;
mod recv_pkg;

use crate::{
    prelude::*,
    model::{screens, screens::counter::Package},
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
        // Client C.I. input
        Some(0) => {
            match key.code {
                KeyCode::Enter => {
                    state.client = Some(state.inputs.get(0)?.input.value().parse::<u32>()?);
                    state.packages.push(Package {
                        package_id: 0,
                        package_desc: "Elatla".to_string(),
                        package_weight: 67.89,
                        package_height: 1,
                        package_length: 2,
                        package_width: 3,
                    });
                    state.packages.push(Package {
                        package_id: 1,
                        package_desc: "Elatlillo".to_string(),
                        package_weight: 6.7,
                        package_height: 3,
                        package_length: 2,
                        package_width: 1,
                    });
                },
                _ => input(key, &state.input_mode, &mut state.inputs)?
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
            match state.sidebar_state.selected() {
                Some(0) => recv_pkg::update(state, key.code),
                Some(1) => todo!("receive pkg update"),
                _ => unimplemented!()
            }
        }
        _ => {}
    }

    Ok(())
}