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
    pub _ci: i64,
    pub _name: String,
    pub _email: String,
    pub _phone_num: String,
    pub _role: String,
}

pub async fn try_login(state: &mut screens::login::State, _tx: &Sender<Event>) -> Result<()> {
    let settings = get_settings()?;
    let url = format!("{}{}", settings.server.url(), settings.server.endpoints.login);
    let client = reqwest::Client::new();

    let user_credentials = LoginRequest {
        ci: state.inputs.get(0)?.input.value().to_string(),
        passwd: state.inputs.get(1)?.input.value().to_string(),
    };

    let response = client
        .post(url)
        .json(&user_credentials)
        .send()
        .await?;

    match response.status() {
        // Login successful
        StatusCode::OK => {
            let _user_data = response.json::<Employee>().await?;

            enter_popup(state, Some(PopupId::LoginSuccessful)).await?;
        }
        // Login failed
        StatusCode::UNAUTHORIZED => {
            state.failed_logins += 1;
        }
        // Unexpected response
        _ => {
            bail!("Unexpected response from server: {:?}", response);
        }
    }

    Ok(())
}