use crate::{
    prelude::*,
    model::screens,
};

pub fn switch_subscreen(state: &mut screens::counter::State, key_code: KeyCode) {
    let item_count = 2; 

    match key_code {
        KeyCode::Char('j') | KeyCode::Down => {
            let i = match state.sidebar_state.selected() {
                // Wrap selection back to 0 when hitting the max
                Some(i) => (i + 1) % item_count,
                None => 0,
            };
            state.sidebar_state.select(Some(i));
        }
        KeyCode::Char('k') | KeyCode::Up => {
            let i = match state.sidebar_state.selected() {
                // If at the top, jump to the bottom. Otherwise, go up 1
                Some(i) => if i == 0 { item_count - 1 } else { i - 1 },
                None => 0,
            };
            state.sidebar_state.select(Some(i));
        }
        _ => {}
    }
}