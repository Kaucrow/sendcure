use crate::prelude::*;

pub fn quit(app: &mut App) -> Result<()> {
    app.should_quit = true;
    Ok(())
}