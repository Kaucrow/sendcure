from dataclasses import dataclass
from typing import Any

import requests


@dataclass(frozen=True)
class LoginResponse:
    ci: int
    email: str
    name: str
    phone_num: str | None
    role: str


class ApiClient:
    def __init__(self, base_url: str, timeout_seconds: int) -> None:
        self.base_url = base_url.rstrip("/")
        self.timeout_seconds = timeout_seconds

    def login_employee(self, ci: int, passwd: str) -> LoginResponse:
        url = f"{self.base_url}/employee/login"
        payload: dict[str, Any] = {
            "ci": ci,
            "passwd": passwd,
        }

        response = requests.get(url, json=payload, timeout=self.timeout_seconds)

        if response.status_code == 200:
            data = response.json()
            return LoginResponse(
                ci=data["ci"],
                email=data["email"],
                name=data["name"],
                phone_num=data.get("phone_num"),
                role=data.get("role", "counter"),
            )

        if response.status_code == 400:
            raise ValueError("CI y contraseña son obligatorios.")

        if response.status_code == 401:
            raise PermissionError("Credenciales inválidas.")

        try:
            message = response.json().get("message", "Error inesperado del servidor.")
        except ValueError:
            message = "Error inesperado del servidor."

        raise RuntimeError(f"Error {response.status_code}: {message}")

    def get_employee_questions(self) -> list[dict[str, Any]]:
        url = f"{self.base_url}/employee/questions"
        response = requests.get(url, timeout=self.timeout_seconds)

        if response.status_code == 200:
            data = response.json()

            if isinstance(data, dict) and "data" in data:
                payload = data["data"]
                return payload if isinstance(payload, list) else []

            return data if isinstance(data, list) else []

        try:
            message = response.json().get("message", "Error inesperado del servidor.")
        except ValueError:
            message = "Error inesperado del servidor."

        raise RuntimeError(f"Error {response.status_code}: {message}")
