pub mod login;

#[derive(Debug)]
pub enum Popup {
    // Login
    LoginSuccessful(login::successful::State),
}

#[derive(Debug)]
pub enum PopupId {
    LoginSuccessful,
}