pub mod login;

#[derive(Debug)]
pub enum Popup {
    // Login
    LoginSuccessful(login::successful::State),
    ServerUnavailable(login::server_unavail::State),
}

#[derive(Debug)]
pub enum PopupId {
    LoginSuccessful,
    ServerUnavailable,
}