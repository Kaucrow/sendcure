from textual.app import App, ComposeResult
from textual.screen import Screen
from src.screens.login import LoginScreen
from src.screens.dashboard import DashboardScreen


class ShipmentApp(App):
    """
    shipment — Shipment subsystem
    Receives packages from the front desk and registers them for delivery.
    """

    CSS_PATH = "../styles/app.tcss"
    TITLE = "Sendcure · shipment"
    SCREENS = {
        "login": LoginScreen,
        "dashboard": DashboardScreen,
    }

    def on_mount(self) -> None:
        self.push_screen("login")
