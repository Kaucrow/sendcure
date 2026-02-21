use crate::{
    prelude::*,
    event::Event,
};

pub async fn update(app: &mut App, event: Event) -> Result<()> {
    match event {
        Event::Quit => {
            app.data.should_quit = true;
            Ok(())
        }
        _ => panic!("An event of type {:?} was passed to the common update function", event)
    }
}