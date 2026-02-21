pub struct HelpText {
    pub common: CommonHelpText,
    pub login: LoginHelpText,
}

pub struct CommonHelpText {
    pub render_err: &'static str,
}

pub struct LoginHelpText {
    pub main: &'static str,
    pub login_failed: &'static str,
    pub login_failed_lock: &'static str,
}

impl HelpText {
    pub const fn default() -> Self {
        Self {
            common: CommonHelpText {
                render_err: "The terminal is too smol :(",
            },
            login: LoginHelpText {
                main: "(Tab) switch input | (Esc) back",
                login_failed: "Login failed.",
                login_failed_lock: "Login failed. - Try again in: ",
            },
        }
    }
}