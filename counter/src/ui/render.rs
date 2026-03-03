use crate::{
    prelude::*,
    model::Screen,
    ui::{
        screens::*,
        utils::clear_chunks,
        err,
    }
};

pub fn render(app: &App, f: &mut Frame) {
    if app.should_clear_screen {
        clear_chunks(f, &Layout::default().split(f.area()));
    }

    match &app.active_screen {
        Some(Screen::Login(state)) => login::render(app, state, f),
        Some(Screen::Counter(state)) => counter::render(app, state, f),
        None => Ok(())
    }
    .unwrap_or_else(|_| { err::render(app, f) })
}