use crate::{
    prelude::*,
    model::{
        Popup,
        PopupId,
        popups,
        screens,
    },
};

pub async fn enter_popup(state: &mut screens::login::State, popup: Option<PopupId>) -> Result<()> {
    match popup {
        Some(PopupId::LoginSuccessful) => {
            state.active_popup = Some(Popup::LoginSuccessful(popups::login::successful::State {}))
        },
        None => {
            state.active_popup = None;
        }
        _ => bail!("Unsupported popup for login screen: {:?}", popup)
    }

    Ok(())
}