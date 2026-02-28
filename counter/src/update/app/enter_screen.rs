use crate::{
    prelude::*,
    model::{
        Screen,
        ScreenId,
        screens::*,
        input::InputFields,
    },
};

pub async fn enter_screen(app: &mut App, screen: ScreenId) -> Result<()> {
    match screen {
        ScreenId::Login => {
            app.active_screen = Some(Screen::Login(login::State {
                inputs: InputFields::new(2).with_active(0)?,
                ..Default::default()
            }));
        }
        ScreenId::Counter => {
            let tabs = counter::Tabs::new(vec![
                counter::Tab::Received(counter::recv_pkg::State::default()),
                counter::Tab::Send(counter::send_pkg::State {
                    inputs: InputFields::new(7),
                    action_sel: Some(0),
                })
            ]);

            app.active_screen = Some(Screen::Counter(counter::State {
                action_sel: Some(0),
                tabs,
                sidebar_state: ListState::default().with_selected(Some(1)),
                inputs: InputFields::new(1).with_active(0)?,
                ..Default::default()
            }));
        }
    }

    Ok(())
}