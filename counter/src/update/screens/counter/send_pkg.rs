use crate::{
    prelude::*,
    model::screens::counter,
    update::common::input::input,
};

pub fn update(tab_state: &mut counter::send_pkg::State, key_event: KeyEvent) -> Result<()> {
    match key_event.code {
        KeyCode::Down => {
            match tab_state.action_sel {
                // Inputs
                Some(0) => {
                    match tab_state.inputs.selected_idx() {
                        None | Some(0) | Some(1) | Some(2) | Some(3) | Some(4) | Some(5) => tab_state.inputs.next(),
                        Some(6) => {
                            tab_state.action_sel = Some(1);
                            tab_state.inputs.deselect();
                        },
                        _ => {}
                    }
                }
                // Send package button
                Some(1) => {}
                _ => {}
            }
        }
        KeyCode::Up => {
            match tab_state.action_sel {
                // Inputs
                Some(0) => {
                    match tab_state.inputs.selected_idx() {
                        None | Some(1) | Some(2) | Some(3) | Some(4) | Some(5) | Some(6) => tab_state.inputs.prev(),
                        _ => {}
                    }
                }
                // Send package button
                Some(1) => {
                    tab_state.action_sel = Some(0);
                    tab_state.inputs.prev();
                }
                _ => {}
            }

        }
        _ => {
            match tab_state.action_sel {
                // Inputs
                Some(0) => {
                    if tab_state.inputs.selected().is_some() {
                        input(key_event, &mut tab_state.inputs)?;
                    }
                }
                _ => {}
            }
        }
    }

    Ok(())
}