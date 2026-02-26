use crate::{
    prelude::*,
    model::screens,
};

pub fn update(state: &mut screens::counter::State, key_code: KeyCode) {
    match key_code {
        KeyCode::Char('j') | KeyCode::Down => {
            state.package_table_state.select(Some(0));
        }
        KeyCode::Char('k') | KeyCode::Up => {
            state.package_table_state.select(Some(0));
        }
        _ => {}
    }
}