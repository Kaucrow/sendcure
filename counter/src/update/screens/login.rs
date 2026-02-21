use crate::{
    prelude::*,
    model::screens,
};

pub async fn update(
    state: &mut screens::login::State,
    data: &mut AppData,
    key: KeyEvent,
    tx: &Sender<Event>
) -> Result<()> {
    match key.code {
        KeyCode::Tab => {
            tx.send(Event::Quit)?;
        }

        _ => {}
    }

    Ok(())
}