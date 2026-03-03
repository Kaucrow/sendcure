pub struct HelpText {
    pub common: CommonHelpText,
    pub login: LoginHelpText,
    pub counter: CounterHelpText,
}

pub struct CommonHelpText {
    pub render_err: &'static str,
    pub no_server_response: &'static str,
}

pub struct LoginHelpText {
    pub main: &'static str,
    pub login_failed: &'static str,
    pub login_failed_lock: &'static str,
}

pub struct CounterHelpText {
    pub start: &'static str,
    pub select_client: &'static str,
    pub sidebar: &'static str,
    pub recv_pkg: &'static str,
    pub send_pkg: &'static str,
    pub send_pkg_send: &'static str,

    pub err_client_not_found: &'static str,
    pub err_get_recv_pkg: &'static str,
    pub err_pickup_pkg: &'static str,
    pub err_send_pkg: &'static str,
}

impl HelpText {
    pub const fn default() -> Self {
        Self {
            common: CommonHelpText {
                render_err: "The terminal is too smol :(",
                no_server_response: "The server isn't responding",
            },
            login: LoginHelpText {
                main: "(F2) switch input | (Esc) back",
                login_failed: "Login failed.",
                login_failed_lock: "Login failed. - Try again in: ",
            },
            counter: CounterHelpText {
                start: "(F2) navigation | (Enter) select client",
                select_client: "Select a client first",
                sidebar: "↑/↓ switch tab",
                recv_pkg: "↑/↓ select package | (Enter) mark as received",
                send_pkg: "↑/↓ select input",
                send_pkg_send: "↑/↓ select input | (Enter) send package",

                err_client_not_found: "Failed to find client",
                err_get_recv_pkg: "Failed to get received packages",
                err_pickup_pkg: "Failed to mark package as picked up",
                err_send_pkg: "Failed to send package",
            }
        }
    }
}