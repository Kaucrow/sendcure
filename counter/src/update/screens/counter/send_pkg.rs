use crate::{
    prelude::*,
    settings::get_settings,
    HELP_TEXT,
    model::{popups, Popup, screens::counter},
    update::common::input::input,
};

pub async fn update(
    tab_state: &mut counter::send_pkg::State,
    client: u32,
    help_text: &mut String,
    temp_help_text: &mut counter::TimedHelpText,
    key_event: KeyEvent
) -> Result<()> {
    match tab_state.active_popup {
        Some(Popup::PackageSent(_)) => {
            match key_event.code {
                KeyCode::Enter => {
                    tab_state.active_popup = None;
                    tab_state.inputs.clear();
                    tab_state.inputs.deselect();
                    tab_state.action_sel = Some(0);
                },
                _ => {}
            }

            return Ok(());
        }
        _ => {}
    }

    match key_event.code {
        KeyCode::Down => {
            match tab_state.action_sel {
                // Inputs
                Some(0) => {
                    match tab_state.inputs.selected_idx() {
                        None | Some(0) | Some(1) | Some(2) | Some(3) | Some(4) | Some(5) => {
                            tab_state.inputs.next();
                        }
                        Some(6) => {
                            tab_state.action_sel = Some(1);
                            tab_state.inputs.deselect();
                            *help_text = HELP_TEXT.counter.send_pkg.to_string();
                        },
                        _ => {}
                    }
                }
                // Send package button
                Some(1) => {}
                _ => {}
            }
        }
        KeyCode::Up => {
            match tab_state.action_sel {
                // Inputs
                Some(0) => {
                    match tab_state.inputs.selected_idx() {
                        None | Some(1) | Some(2) | Some(3) | Some(4) | Some(5) | Some(6) => tab_state.inputs.prev(),
                        _ => {}
                    }
                }
                // Send package button
                Some(1) => {
                    tab_state.action_sel = Some(0);
                    tab_state.inputs.prev();
                    *help_text = HELP_TEXT.counter.send_pkg.to_string();
                }
                _ => {}
            }
        }
        KeyCode::Enter => {
            match tab_state.action_sel {
                // Send package button
                Some(1) => {
                    let inputs = &tab_state.inputs;

                    let receiver_ci = inputs.get(0)?.input.value();
                    let address = inputs.get(1)?.input.value();
                    let desc = inputs.get(2)?.input.value();
                    let weight = inputs.get(3)?.input.value();
                    let length = inputs.get(4)?.input.value();
                    let width = inputs.get(5)?.input.value();
                    let height = inputs.get(6)?.input.value();

                    let required_inputs = [
                        (receiver_ci, "Receiver C.I."),
                        (address, "Address"),
                        (weight, "Weight"),
                        (length, "Length"),
                        (width, "Width"),
                        (height, "Height"),
                    ];

                    for (value, input_name) in required_inputs {
                        if value.is_empty() {
                            temp_help_text.set(format!("{} is required", input_name), Duration::from_secs(3));
                            return Ok(());
                        }
                    }

                    if address.len() < 5 {
                        temp_help_text.set("Address must be 5 characters or longer", Duration::from_secs(3));
                        return Ok(());
                    }

                    let payload = counter::send_pkg::SendPackage { 
                        sender_cid: client,
                        receiver_cid: receiver_ci.parse::<u32>()?,
                        desc: if desc.is_empty() { None } else { Some(desc.to_string()) },
                        destination_address: address.to_string(),
                        weight: weight.parse::<f64>()?,
                        length: length.parse::<u32>()?,
                        width: width.parse::<u32>()?,
                        height: height.parse::<u32>()?,
                    };

                    let settings = get_settings()?;
                    let url = format!("{}{}", settings.server.url(), settings.server.endpoints.send_package);

                    let req_client = reqwest::Client::new(); 

                    let result = req_client.post(&url)
                        .json(&payload)
                        .send()
                        .await;

                    match result {
                        Ok(response) => {
                            // Package sent
                            if response.status().is_success() {
                                let popup_state = response.json::<popups::counter::send_pkg::State>().await?;
                                tab_state.active_popup = Some(Popup::PackageSent(popup_state));
                            // Server responded with error
                            } else {
                                temp_help_text.set(HELP_TEXT.counter.err_send_pkg, Duration::from_secs(3));
                            }
                        }
                        // Server not responding
                        Err(_) => {
                            temp_help_text.set(HELP_TEXT.common.no_server_response, Duration::from_secs(3));
                        }
                    }
                }
                _ => {}
            }
        }
        _ => {
            match tab_state.action_sel {
                // Inputs
                Some(0) => {
                    if tab_state.inputs.selected().is_some() {
                        input(key_event, &mut tab_state.inputs)?;
                    }
                }
                _ => {}
            }
        }
    }

    Ok(())
}