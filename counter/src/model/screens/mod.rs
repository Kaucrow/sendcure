pub mod login;
pub mod counter;

#[derive(Debug)]
pub enum ScreenId {
    Login,
    Counter,
}

#[derive(Debug)]
pub enum Screen {
    Login(login::State),
    Counter(counter::State),
}

impl Default for Screen {
    fn default() -> Self {
        Self::Login(login::State::default())
    }
}