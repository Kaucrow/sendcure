use crate::prelude::*;

pub fn quit(data: &mut AppData) -> Result<()> {
    data.should_quit = true;
    Ok(())
}