from __future__ import annotations
import httpx
from src.config import endpoint


class ApiError(Exception):
    def __init__(self, status_code: int, message: str):
        super().__init__(message)
        self.status_code = status_code


class ApiClient:
    """
    HTTP client for communicating with the main-server (sendcure).
    All shipment operations go through here.
    """

    def __init__(self):
        self._token: str | None = None

    # ── Auth ──────────────────────────────────────────────────────────────

    def login(self, ci: int, passwd: str) -> dict:
        """Login as an employee. Stores session data on success."""
        response = httpx.request(
            "GET",
            endpoint("login"),
            json={"ci": ci, "passwd": passwd},
        )
        if response.status_code == 401:
            raise ApiError(401, "ID or password incorrect.")
        if response.status_code != 200:
            raise ApiError(response.status_code, "Error logging in.")
        data = response.json()
        return data

    # ── Shipments ──────────────────────────────────────────────────────────

    def get_shipments(self, status_id: int | None = None) -> list[dict]:
        """Get all shipments, optionally filtered by status."""
        params = {}
        if status_id is not None:
            params["status_id"] = status_id
        response = httpx.get(endpoint("shipments"), params=params)
        self._raise_for_status(response)
        return response.json()

    def get_shipment(self, shipment_id: int) -> dict:
        """Get a single shipment by ID."""
        response = httpx.get(endpoint("shipment_by_id", id=shipment_id))
        self._raise_for_status(response)
        return response.json()

    def update_status(self, shipment_id: int, status_id: int) -> dict:
        """Update the status of a shipment."""
        response = httpx.patch(
            endpoint("update_status", id=shipment_id),
            json={"status_id": status_id},
        )
        self._raise_for_status(response)
        return response.json()

    def assign_delivery(self, shipment_id: int, delivery_id: int) -> dict:
        """Assign a delivery route to a shipment."""
        response = httpx.patch(
            endpoint("assign_delivery", id=shipment_id),
            json={"delivery_id": delivery_id},
        )
        self._raise_for_status(response)
        return response.json()

    # ── Deliveries ─────────────────────────────────────────────────────────

    def get_deliveries(self) -> list[dict]:
        """Get all available delivery routes."""
        response = httpx.get(endpoint("deliveries"))
        self._raise_for_status(response)
        return response.json()

    # ── Helpers ────────────────────────────────────────────────────────────

    def _raise_for_status(self, response: httpx.Response):
        if response.status_code >= 400:
            try:
                msg = response.json().get("message", "Unknown error.")
            except Exception:
                msg = response.text
            raise ApiError(response.status_code, msg)


api = ApiClient()
