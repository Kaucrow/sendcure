from __future__ import annotations
import httpx
import psycopg2
from psycopg2.extras import RealDictCursor
from src.config import endpoint, db_connection_params


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
            "POST",
            endpoint("login"),
            json={"ci": ci, "passwd": passwd, "role": "dispatch"},
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
        try:
            response = httpx.get(endpoint("shipments"), params=params)
            self._raise_for_status(response)
            return response.json()
        except (httpx.RequestError, ApiError) as err:
            if self._should_use_db_fallback(err):
                return self._db_get_shipments(status_id)
            raise

    def get_shipment(self, shipment_id: int) -> dict:
        """Get a single shipment by ID."""
        response = httpx.get(endpoint("shipment_by_id", id=shipment_id))
        self._raise_for_status(response)
        return response.json()

    def update_status(self, shipment_id: int, status_id: int) -> dict:
        """Update the status of a shipment."""
        try:
            response = httpx.patch(
                endpoint("update_status", id=shipment_id),
                json={"status_id": status_id},
            )
            self._raise_for_status(response)
            return response.json()
        except (httpx.RequestError, ApiError) as err:
            if self._should_use_db_fallback(err):
                return self._db_update_status(shipment_id, status_id)
            raise

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

    def _should_use_db_fallback(self, err: Exception) -> bool:
        if isinstance(err, httpx.RequestError):
            return True
        if isinstance(err, ApiError):
            return err.status_code in {404, 405, 501}
        return False

    def _db_get_shipments(self, status_id: int | None = None) -> list[dict]:
        query = """
            SELECT
                s.shipment_id AS "shipmentId",
                s.guide_num AS "guideNum",
                s.sender_cid AS "clientCid",
                s.receiver_cid AS "receiverCid",
                s.package_id AS "packageId",
                s.status_id AS "statusId",
                s.destination_address AS "destinationAddress",
                s.shipment_dt AS "shipmentDt",
                COALESCE(p.package_desc, '') AS "packageDesc"
            FROM shipment s
            LEFT JOIN package p ON p.package_id = s.package_id
            WHERE (%s IS NULL OR s.status_id = %s)
            ORDER BY s.shipment_dt DESC
        """

        with psycopg2.connect(**db_connection_params()) as conn:
            with conn.cursor(cursor_factory=RealDictCursor) as cursor:
                cursor.execute(query, (status_id, status_id))
                rows = [dict(row) for row in cursor.fetchall()]

        for row in rows:
            shipment_dt = row.get("shipmentDt")
            if shipment_dt is not None and hasattr(shipment_dt, "isoformat"):
                row["shipmentDt"] = shipment_dt.isoformat()
        return rows

    def _db_update_status(self, shipment_id: int, status_id: int) -> dict:
        query = """
            UPDATE shipment
            SET status_id = %s
            WHERE shipment_id = %s
            RETURNING
                shipment_id AS "shipmentId",
                status_id AS "statusId",
                guide_num AS "guideNum"
        """

        with psycopg2.connect(**db_connection_params()) as conn:
            with conn.cursor(cursor_factory=RealDictCursor) as cursor:
                cursor.execute(query, (status_id, shipment_id))
                row = cursor.fetchone()

        if not row:
            raise ApiError(404, f"Shipment #{shipment_id} not found.")
        return dict(row)


api = ApiClient()
