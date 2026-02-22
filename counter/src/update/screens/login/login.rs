use crate::{
    prelude::*,
    settings::get_settings,
    model::{PopupId, screens},
};
use super::enter_popup::enter_popup;

#[derive(Serialize)]
struct LoginRequest {
    ci: String,
    passwd: String,
}

#[derive(Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Employee {
    pub ci: i64,
    pub name: String,
    pub email: String,
    pub phone_num: String,
    pub role: String,
}

pub async fn try_login(state: &mut screens::login::State, tx: &Sender<Event>) -> Result<()> {
    let settings = get_settings()?;
    let url = format!("{}{}", settings.server.url(), settings.server.endpoints.login);
    let client = reqwest::Client::new();

    let user_credentials = LoginRequest {
        ci: state.inputs.0.input.value().to_string(),
        passwd: state.inputs.1.input.value().to_string(),
    };

    let response = client
        .get(url)
        .json(&user_credentials)
        .send()
        .await?;

    match response.status() {
        // Login successful
        StatusCode::OK => {
            let user_data = response.json::<Employee>().await?;

            enter_popup(state, Some(PopupId::LoginSuccessful)).await?;
        }
        // Login failed
        StatusCode::UNAUTHORIZED => {
            state.failed_logins += 1;
            //bail!("Login failed");
        }
        // Unexpected response
        _ => {
            enter_popup(state, Some(PopupId::ServerUnavailable)).await?;
        }
    }

    Ok(())
}