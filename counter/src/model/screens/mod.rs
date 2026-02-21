pub mod login;

#[derive(Debug)]
pub enum Screen {
    Login(login::State),
}

impl Default for Screen {
    fn default() -> Self {
        Self::Login(login::State::default())
    }
}