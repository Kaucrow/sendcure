use crate::{
    prelude::*,
    HELP_TEXT,
    model::screens::counter,
    settings::get_settings,
};

pub async fn update(
    tab_state: &mut counter::recv_pkg::State,
    temp_help_text: &mut counter::TimedHelpText,
    key_code: KeyCode)
-> Result<()> {
    match key_code {
        KeyCode::Char('j') | KeyCode::Down => {
            let item_count = tab_state.packages.len();

            if let Some(i) = tab_state.package_table_state.selected() {
                // Only increment if we aren't at the last index
                if i >= item_count.saturating_sub(1) {
                    return Ok(());
                }
            }

            tab_state.package_table_state.select_next();
        }

        KeyCode::Char('k') | KeyCode::Up => {
            tab_state.package_table_state.select_previous();
        }

        KeyCode::Enter => {
            if let Some(idx) = tab_state.package_table_state.selected() {
                // Get the guide_num from the package
                if let Some(package) = tab_state.packages.get(idx) {
                    let guide_num = package.guide_num.clone();

                    let settings = get_settings()?;

                    let url = format!(
                        "{}{}/{}", 
                        settings.server.url(), 
                        settings.server.endpoints.pickup_package,
                        guide_num
                    );

                    let client = reqwest::Client::new();
                    let result = client.put(&url).send().await;

                    match result {
                        Ok(response) => {
                            // Package picked up
                            if response.status().is_success() {
                                // Remove the package from the received packages list
                                tab_state.packages.remove(idx);

                                // Adjust the table selection so it doesn't crash 
                                // or point to a non-existent index
                                let len = tab_state.packages.len();
                                if len == 0 {
                                    tab_state.package_table_state.select(None);
                                } else if idx >= len {
                                    tab_state.package_table_state.select(Some(len - 1));
                                }
                            // Error picking up package
                            } else {
                                temp_help_text.set(HELP_TEXT.counter.err_pickup_pkg, Duration::from_secs(3));
                            }
                        }
                        Err(_) => {
                            temp_help_text.set(HELP_TEXT.common.no_server_response, Duration::from_secs(3));
                        }
                    }
                }
            }
        }
        _ => {}
    }

    Ok(())
}