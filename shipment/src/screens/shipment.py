from textual.app import ComposeResult
from textual.screen import Screen
from textual.widgets import Label, Button, Select, Header, LoadingIndicator
from textual.containers import Vertical, Horizontal
from textual import on, work
from src.api_client import api, ApiError

STATUS_IN_TRANSIT = 3


class ShipmentScreen(Screen):
    """
    Modal for registering a package's dispatch to delivery.
    Allows selecting a route (Delivery) and confirming the shipment.
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
        self._deliveries: list[dict] = []

    def compose(self) -> ComposeResult:
        with Vertical(id="dialog"):
            yield Label("Register Dispatch to Delivery", id="dialog-title")
            yield Label(f"Package / Guide ID: {self.shipment_id}", id="shipment-info")
            yield LoadingIndicator(id="loading")
            yield Select([], id="select-delivery", prompt="Select a delivery route...")
            yield Label("", id="error-msg")
            with Horizontal(classes="actions"):
                yield Button("Cancel", id="btn-cancel", variant="default")
                yield Button("Confirm shipment", id="btn-confirm", variant="primary")

    def on_mount(self) -> None:
        self._load_deliveries()

    @work(thread=True)
    def _load_deliveries(self) -> None:
        try:
            deliveries = api.get_deliveries()
            self.app.call_from_thread(self._populate_select, deliveries)
        except ApiError as e:
            self.app.call_from_thread(
                self.query_one("#error-msg", Label).update,
                f"Error loading routes: {e}",
            )
        finally:
            self.app.call_from_thread(self.query_one("#loading", LoadingIndicator).remove)

    def _populate_select(self, deliveries: list[dict]) -> None:
        self._deliveries = deliveries
        options = [
            (
                f"{d.get('deliveryName', d.get('delivery_name', ''))} "
                f"(ID: {d.get('deliveryId', d.get('delivery_id', ''))})",
                str(d.get("deliveryId", d.get("delivery_id", "")))
            )
            for d in deliveries
        ]
        select = self.query_one("#select-delivery", Select)
        select.set_options(options)

    @on(Button.Pressed, "#btn-cancel")
    def handle_cancel(self) -> None:
        self.app.pop_screen()

    @on(Button.Pressed, "#btn-confirm")
    @work(thread=True)
    def handle_confirm(self) -> None:
        select = self.query_one("#select-delivery", Select)
        error_label = self.query_one("#error-msg", Label)

        if select.value is Select.BLANK:
            self.app.call_from_thread(error_label.update, "Select a delivery route.")
            return

        delivery_id = int(select.value)

        try:
            api.assign_delivery(self.shipment_id, delivery_id)
            api.update_status(self.shipment_id, STATUS_IN_TRANSIT)
            self.app.call_from_thread(
                self.app.notify,
                f"Package #{self.shipment_id} successfully dispatched.",
            )
            self.app.call_from_thread(self.app.pop_screen)
            from src.screens.dashboard import DashboardScreen
            for screen in self.app.screen_stack:
                if isinstance(screen, DashboardScreen):
                    self.app.call_from_thread(screen.action_refresh)
        except ApiError as e:
            self.app.call_from_thread(error_label.update, f"{e}")
