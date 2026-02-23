from textual.app import ComposeResult
from textual.screen import Screen
from textual.widgets import Label, Input, Button, Header, Footer
from textual.containers import Center, Middle, Vertical
from textual import on
from src.api_client import api, ApiError


class LoginScreen(Screen):
    """Login screen for shipment employees."""

    DEFAULT_CSS = """
    LoginScreen {
        align: center middle;
    }
    #login-box {
        width: 50;
        height: auto;
        border: round $primary;
        padding: 2 4;
        background: $surface;
    }
    #title {
        text-align: center;
        text-style: bold;
        margin-bottom: 1;
        color: $primary;
    }
    #subtitle {
        text-align: center;
        color: $text-muted;
        margin-bottom: 2;
    }
    Input {
        margin-bottom: 1;
    }
    #error-msg {
        color: $error;
        text-align: center;
        height: 1;
    }
    Button {
        width: 100%;
        margin-top: 1;
    }
    """

    def compose(self) -> ComposeResult:
        with Vertical(id="login-box"):
            yield Label("SENDCURE", id="title")
            yield Label("Shipment Subsystem", id="subtitle")
            yield Input(placeholder="ID Card", id="input-ci", type="integer")
            yield Input(placeholder="Password", id="input-passwd", password=True)
            yield Label("", id="error-msg")
            yield Button("Login", id="btn-login", variant="primary")

    @on(Button.Pressed, "#btn-login")
    def handle_login(self) -> None:
        self._do_login()

    @on(Input.Submitted)
    def handle_enter(self) -> None:
        self._do_login()

    def _do_login(self) -> None:
        error_label = self.query_one("#error-msg", Label)
        ci_input = self.query_one("#input-ci", Input)
        passwd_input = self.query_one("#input-passwd", Input)

        ci_raw = ci_input.value.strip()
        passwd = passwd_input.value.strip()

        if not ci_raw or not passwd:
            error_label.update("Please enter ID and password.")
            return

        try:
            ci = int(ci_raw)
        except ValueError:
            error_label.update("ID must be a number.")
            return

        try:
            employee = api.login(ci, passwd)
            # Save session globally in the app
            self.app._employee = employee
            self.app.push_screen("dashboard")
        except ApiError as e:
            passwd_input.clear()
            error_label.update(f"{e}")
        except Exception as e:
            error_label.update("Could not connect to the server.")
