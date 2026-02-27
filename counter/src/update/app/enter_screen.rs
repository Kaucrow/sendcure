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
                counter::Tab::Received(counter::recv_pkg::State {
                    packages: vec![
                        counter::recv_pkg::Package {
                            package_id: 0,
                            package_desc: "Elatla".to_string(),
                            package_weight: 67.89,
                            package_height: 1,
                            package_length: 2,
                            package_width: 3,
                        },
                        counter::recv_pkg::Package {
                            package_id: 1,
                            package_desc: "Elatlillo".to_string(),
                            package_weight: 6.7,
                            package_height: 3,
                            package_length: 2,
                            package_width: 1,
                        }
                    ],
                    ..Default::default()
                }),
                counter::Tab::Send(counter::send_pkg::State {
                    inputs: InputFields::new(6),
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