use crate::{
    prelude::*,
    model::screens::counter,
};

pub fn update(tab_state: &mut counter::recv_pkg::State, key_code: KeyCode) {
    match key_code {
        KeyCode::Char('j') | KeyCode::Down => {
            tab_state.package_table_state.select(Some(0));
        }
        KeyCode::Char('k') | KeyCode::Up => {
            tab_state.package_table_state.select(Some(0));
        }
        _ => {}
    }
}