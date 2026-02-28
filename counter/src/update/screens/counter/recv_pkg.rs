use crate::{
    prelude::*,
    model::screens::counter,
};

pub fn update(tab_state: &mut counter::recv_pkg::State, key_code: KeyCode) {
    match key_code {
        KeyCode::Char('j') | KeyCode::Down => {
            let item_count = 2 as usize;

            if let Some(i) = tab_state.package_table_state.selected() {
                // Only increment if we aren't at the last index
                if i >= item_count.saturating_sub(1) {
                    return;
                }
            }

            tab_state.package_table_state.select_next();
        }
        KeyCode::Char('k') | KeyCode::Up => {
            tab_state.package_table_state.select_previous();
        }
        _ => {}
    }
}