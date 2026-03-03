pub mod login;
pub mod counter;

#[derive(Debug)]
pub enum Popup {
    // Login
    LoginSuccessful(login::successful::State),
    // Counter
    PackageSent(counter::send_pkg::State),
}

#[derive(Debug)]
pub enum PopupId {
    LoginSuccessful,
    PackageSent,
}