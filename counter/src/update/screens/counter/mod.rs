mod navigation;
mod recv_pkg;
mod send_pkg;

use crate::{
    prelude::*,
    settings::get_settings,
    model::{input::*, screens::{self, counter}},
    update::common::{quit, input, navigate}
};

use navigation::switch_subscreen;

#[allow(unreachable_patterns)]
pub async fn update(
    app: &mut App,
    state: &mut screens::counter::State,
    key: KeyEvent,
    tx: &Sender<Event>
) -> Result<()> {
    // Key inputs common to all tabs
    match key.code {
        KeyCode::Esc => quit(app)?,
        KeyCode::Tab => navigate(&mut state.action_sel, 2)?,
        _ => {}
    }

    // Tab-specific key inputs
    match state.action_sel {
        // Client C.I. input
        Some(0) => {
            match key.code {
                KeyCode::Enter => {
                    let ci_input = state.inputs.get(0)?.input.value().parse::<u32>().ok();

                    if let Some(ci) = ci_input {
                        let settings = get_settings()?;

                        // Fetch received packages
                        let result = {
                            let url = format!("{}{}/{}", settings.server.url(), settings.server.endpoints.received_packages, ci);
                            let response = reqwest::get(&url).await?;

                            response.json::<Vec<counter::recv_pkg::Package>>().await
                        };

                        match result {
                            // Got packages
                            Ok(packages) if !packages.is_empty() => {
                                state.client = Some(ci);

                                let tabs = counter::Tabs::new(vec![
                                    counter::Tab::Received(counter::recv_pkg::State {
                                        packages,
                                        ..Default::default()
                                    }),
                                    counter::Tab::Send(counter::send_pkg::State {
                                        inputs: InputFields::new(7),
                                        action_sel: Some(0),
                                    })
                                ]);

                                state.tabs = tabs;
                            },
                            // Server responded with an error
                            _ => {
                                //state. = Some("Client not found or no packages received".to_string());
                                return Ok(()); 
                            },
                        }
                    }
                }
                _ => input(key, &mut state.inputs)?
            }
        }
        // Sidebar
        Some(1) => {
            match key.code {
                KeyCode::Char('j') | KeyCode::Char('k') | KeyCode::Up | KeyCode::Down
                    => switch_subscreen(state, key.code),
                _ => {}
            }
        }
        // Main tab
        Some(2) => {
            match state.tabs.get_mut(state.sidebar_state.selected().unwrap())? {
                counter::Tab::Received(tab_state) => recv_pkg::update(tab_state, key.code),
                counter::Tab::Send(tab_state) => send_pkg::update(tab_state, key)?,
                _ => unimplemented!()
            }
        }
        _ => {}
    }

    Ok(())
}