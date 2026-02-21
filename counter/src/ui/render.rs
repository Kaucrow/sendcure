use crate::{
    prelude::*,
    model::Screen,
    ui::{
        utils::clear_chunks,
        screens::login,
        err,
    }
};

pub fn render(app: &mut App, f: &mut Frame) {
    {
        if app.data.should_clear_screen {
            clear_chunks(f, &Layout::default().split(f.area()));
            app.data.should_clear_screen = false;
        }
    }

    let App { active_screen, data, .. } = app;

    match active_screen {
        Screen::Login(state) => login::render(state, data, f),
    }
    .unwrap_or_else(|_| { err::render(app, f) })
}