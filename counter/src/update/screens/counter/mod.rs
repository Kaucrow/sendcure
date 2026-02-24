use crate::{
    prelude::*,
    model::screens,
};

pub async fn update(
    app: &mut App,
    state: &mut screens::counter::State,
    key: KeyEvent,
    tx: &Sender<Event>
) -> Result<()> {
    Ok(())
}