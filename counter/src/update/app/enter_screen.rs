use crate::{
    prelude::*,
    HELP_TEXT,
    model::{
        Screen,
        ScreenId,
        screens::*,
        input::{InputFields, InputBlacklist},
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
                    inputs: InputFields::new(7).with_blacklist(vec![
                        (0, InputBlacklist::Numeric),
                        (3, InputBlacklist::Money),
                        (4, InputBlacklist::Numeric),
                        (5, InputBlacklist::Numeric),
                        (6, InputBlacklist::Numeric),
                    ]),
                    action_sel: Some(0),
                    ..Default::default()
                })
            ]);

            app.active_screen = Some(Screen::Counter(counter::State {
                action_sel: Some(0),
                tabs,
                sidebar_state: ListState::default().with_selected(Some(0)),
                inputs: InputFields::new(1)
                    .with_active(0)?
                    .with_blacklist(vec![(0, InputBlacklist::Numeric)]),
                help_text: HELP_TEXT.counter.start.to_string(),
                ..Default::default()
            }));
        }
    }

    Ok(())
}