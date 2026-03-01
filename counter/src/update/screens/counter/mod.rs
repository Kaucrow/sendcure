mod navigation;
mod recv_pkg;
mod send_pkg;

use crate::{
    prelude::*,
    settings::get_settings,
    HELP_TEXT,
    model::screens::{self, counter},
    update::common::{quit, input, navigate}
};

use navigation::switch_subscreen;

#[allow(unreachable_patterns)]
pub async fn update(
    app: &mut App,
    state: &mut screens::counter::State,
    key: KeyEvent,
    _tx: &Sender<Event>
) -> Result<()> {
    // Key inputs common to all tabs
    match key.code {
        KeyCode::Esc => quit(app)?,
        KeyCode::F(2) => {
            navigate(&mut state.action_sel, 2)?;

            // Update help text
            match state.action_sel {
                Some(0) => state.help_text = HELP_TEXT.counter.start.to_string(),
                Some(1) => state.help_text = HELP_TEXT.counter.sidebar.to_string(),
                Some(2) => {
                    if state.client.is_none() {
                        state.help_text = HELP_TEXT.counter.select_client.to_string();
                        return Ok(());
                    }

                    match state.sidebar_state.selected() {
                        Some(0) => state.help_text = HELP_TEXT.counter.recv_pkg.to_string(),
                        Some(1) => state.help_text = HELP_TEXT.counter.send_pkg.to_string(),
                        _ => {}
                    }
                }
                _ => {}
            }
        },
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
                            reqwest::get(&url).await
                        };

                        match result {
                            Ok(response) => {
                                // Got packages
                                if response.status().is_success() {
                                    let packages = response.json::<Vec<counter::recv_pkg::Package>>().await?;

                                    state.client = Some(ci);

                                    if let counter::Tab::Received(tab_state) = state.tabs.get_mut(0)? {
                                        tab_state.packages = packages;
                                    }

                                    if let counter::Tab::Send(tab_state) = state.tabs.get_mut(1)? {
                                        tab_state.inputs.clear();
                                        tab_state.inputs.deselect();
                                        tab_state.action_sel = Some(0);
                                    }
                                // Server responded with error
                                } else {
                                    state.temp_help_text.set(HELP_TEXT.counter.err_client_not_found, Duration::from_secs(3));
                                }
                            }
                            // Server didn't respond
                            Err(_) => {
                                state.temp_help_text.set(HELP_TEXT.common.no_server_response, Duration::from_secs(3));
                            }
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
            if let Some(client) = state.client {
                match state.tabs.get_mut(state.sidebar_state.selected().unwrap())? {
                    counter::Tab::Received(tab_state) => recv_pkg::update(tab_state, &mut state.temp_help_text, key.code).await?,
                    counter::Tab::Send(tab_state) => send_pkg::update(tab_state, client, &mut state.help_text, &mut state.temp_help_text, key).await?,
                    _ => unimplemented!()
                }
            }
        }
        _ => {}
    }

    Ok(())
}