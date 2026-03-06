from textual.app import ComposeResult
from textual.screen import Screen
from textual.widgets import Label, Button
from textual.containers import Vertical, Horizontal
from textual import on, work
from src.api_client import api, ApiError

STATUS_DELIVERED = 2


class ShipmentScreen(Screen):
    """
    Modal to confirm package arrival at destination.
    """

    DEFAULT_CSS = """
    ShipmentScreen {
        align: center middle;
    }
    #dialog {
        width: 60;
        height: auto;
        border: round $primary;
        padding: 2 4;
        background: $surface;
    }
    #dialog-title {
        text-style: bold;
        color: $primary;
        margin-bottom: 1;
    }
    #shipment-info {
        color: $text-muted;
        margin-bottom: 2;
    }
    Select {
        margin-bottom: 1;
    }
    #error-msg {
        color: $error;
        height: 1;
    }
    .actions {
        layout: horizontal;
        height: 3;
        align: right middle;
        margin-top: 1;
    }
    Button {
        margin-left: 1;
    }
    """

    def __init__(self, shipment_id: int, **kwargs):
        super().__init__(**kwargs)
        self.shipment_id = shipment_id

    def compose(self) -> ComposeResult:
        with Vertical(id="dialog"):
            yield Label("Mark Package as Delivered", id="dialog-title")
            yield Label(f"Package / Guide ID: {self.shipment_id}", id="shipment-info")
            yield Label("", id="error-msg")
            with Horizontal(classes="actions"):
                yield Button("Cancel", id="btn-cancel", variant="default")
                yield Button("Confirm delivered", id="btn-confirm", variant="primary")

    @on(Button.Pressed, "#btn-cancel")
    def handle_cancel(self) -> None:
        self.app.pop_screen()

    @on(Button.Pressed, "#btn-confirm")
    @work(thread=True)
    def handle_confirm(self) -> None:
        error_label = self.query_one("#error-msg", Label)

        try:
            api.update_status(self.shipment_id, STATUS_DELIVERED)
            self.app.call_from_thread(
                self.app.notify,
                f"Package #{self.shipment_id} marked as delivered.",
            )
            self.app.call_from_thread(self.app.pop_screen)
            from src.screens.dashboard import DashboardScreen
            for screen in self.app.screen_stack:
                if isinstance(screen, DashboardScreen):
                    self.app.call_from_thread(screen.action_refresh)
        except ApiError as e:
            self.app.call_from_thread(error_label.update, f"{e}")
