from textual.app import ComposeResult
from textual.screen import Screen
from textual.widgets import (
    Header, Footer, DataTable, Label, Button, LoadingIndicator
)
from textual.containers import Horizontal, Vertical, ScrollableContainer
from textual import on, work
from src.api_client import api, ApiError


STATUS_ON_COUNTER  = 1  
STATUS_IN_SHIPMENT   = 2   
STATUS_IN_TRANSIT   = 3   
STATUS_DELIVERED     = 4   


class DashboardScreen(Screen):
    """
    Main screen for the shipment subsystem.
    Displays two lists:
      - Pending to receive (front desk → shipment)
      - Pending to dispatch (shipment → transit)
    """

    BINDINGS = [
        ("r", "refresh", "Refresh"),
        ("q", "quit_app", "Quit"),
    ]

    DEFAULT_CSS = """
    DashboardScreen {
        layout: vertical;
    }
    #top-bar {
        height: 3;
        background: $primary;
        padding: 0 2;
        align: left middle;
    }
    #top-bar Label {
        color: $background;
        text-style: bold;
    }
    #panels {
        layout: horizontal;
        height: 1fr;
    }
    .panel {
        width: 1fr;
        border: round $primary;
        margin: 1;
        padding: 1;
        layout: vertical;
    }
    .panel-title {
        text-style: bold;
        color: $primary;
        margin-bottom: 1;
    }
    DataTable {
        height: 1fr;
    }
    .actions {
        height: 3;
        layout: horizontal;
        align: right middle;
        margin-top: 1;
    }
    Button {
        margin-left: 1;
    }
    #status-bar {
        height: 1;
        padding: 0 2;
        color: $text-muted;
    }
    """

    def compose(self) -> ComposeResult:
        employee = getattr(self.app, "_employee", {})
        name = employee.get("employeeName", employee.get("employee_name", "Employee"))

        yield Header(show_clock=True)
        with Horizontal(id="top-bar"):
            yield Label(f"shipment  ·  {name}")
        with Horizontal(id="panels"):
            with Vertical(classes="panel"):
                yield Label("Pending to receive (from front desk)", classes="panel-title")
                yield DataTable(id="tbl-pendientes", cursor_type="row")
                with Horizontal(classes="actions"):
                    yield Button("Mark as received", id="btn-recibir", variant="success")
            with Vertical(classes="panel"):
                yield Label("In shipment — ready to send", classes="panel-title")
                yield DataTable(id="tbl-shipment", cursor_type="row")
                with Horizontal(classes="actions"):
                    yield Button("Register dispatch", id="btn-shipment", variant="primary")
        yield Label("Press R to refresh  ·  Q to quit", id="status-bar")
        yield Footer()

    def on_mount(self) -> None:
        self._setup_tables()
        self.action_refresh()

    def _setup_tables(self) -> None:
        pendientes = self.query_one("#tbl-pendientes", DataTable)
        pendientes.add_columns("ID", "Guide", "Destination", "Client", "Package")

        shipment = self.query_one("#tbl-shipment", DataTable)
        shipment.add_columns("ID", "Guide", "Destination", "Client", "Package")

    @work(thread=True)
    def action_refresh(self) -> None:
        """Reload both tables from the API."""
        try:
            pendientes_data = api.get_shipments(status_id=STATUS_ON_COUNTER)
            despacho_data   = api.get_shipments(status_id=STATUS_IN_SHIPMENT)
            self.app.call_from_thread(self._populate_tables, pendientes_data, despacho_data)
        except ApiError as e:
            self.app.call_from_thread(
                self.notify, f"Error loading data: {e}", severity="error"
            )
        except Exception:
            self.app.call_from_thread(
                self.notify, "Could not connect to the server.", severity="error"
            )

    def _populate_tables(self, pendientes: list, shipment: list) -> None:
        tbl_p = self.query_one("#tbl-pendientes", DataTable)
        tbl_d = self.query_one("#tbl-shipment", DataTable)
        tbl_p.clear()
        tbl_d.clear()

        for s in pendientes:
            tbl_p.add_row(
                str(s.get("shipmentId", s.get("shipment_id", ""))),
                s.get("guideNum", s.get("guide_num", "")),
                s.get("destinationAddress", s.get("destination_address", ""))[:30],
                str(s.get("clientCid", s.get("client_cid", ""))),
                str(s.get("packageId", s.get("package_id", ""))),
                key=str(s.get("shipmentId", s.get("shipment_id", ""))),
            )

        for s in shipment:
            tbl_d.add_row(
                str(s.get("shipmentId", s.get("shipment_id", ""))),
                s.get("guideNum", s.get("guide_num", "")),
                s.get("destinationAddress", s.get("destination_address", ""))[:30],
                str(s.get("clientCid", s.get("client_cid", ""))),
                str(s.get("packageId", s.get("package_id", ""))),
                key=str(s.get("shipmentId", s.get("shipment_id", ""))),
            )

    # ── Actions ────────────────────────────────────────────────────────────

    @on(Button.Pressed, "#btn-recibir")
    @work(thread=True)
    def handle_recibir(self) -> None:
        """Mark selected shipment (from mostrador) as received in shipment."""
        tbl = self.query_one("#tbl-pendientes", DataTable)
        if tbl.row_count == 0:
            self.app.call_from_thread(self.notify, "No pending packages.", severity="warning")
            return
        row_key = tbl.cursor_row
        row = tbl.get_row_at(row_key)
        shipment_id = int(row[0])

        try:
            api.update_status(shipment_id, STATUS_IN_SHIPMENT)
            self.app.call_from_thread(
                self.notify, f"Package #{shipment_id} marked as Received in shipment."
            )
            self.app.call_from_thread(self.action_refresh)
        except ApiError as e:
            self.app.call_from_thread(self.notify, str(e), severity="error")

    @on(Button.Pressed, "#btn-shipment")
    def handle_shipment(self) -> None:
        """Open dispatch dialog for the selected shipment."""
        tbl = self.query_one("#tbl-shipment", DataTable)
        if tbl.row_count == 0:
            self.notify("No packages in shipment.", severity="warning")
            return
        row = tbl.get_row_at(tbl.cursor_row)
        shipment_id = int(row[0])
        self.app.push_screen(
            __import__("src.screens.shipment", fromlist=["ShipmentScreen"])
            .ShipmentScreen(shipment_id=shipment_id)
        )

    def action_quit_app(self) -> None:
        self.app.exit()
