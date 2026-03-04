from dataclasses import dataclass
import os

from dotenv import load_dotenv


load_dotenv()


@dataclass(frozen=True)
class AppConfig:
    api_base_url: str
    api_timeout_seconds: int


def get_config() -> AppConfig:
    base_url = os.getenv("API_BASE_URL", "http://localhost:8000").rstrip("/")
    timeout_seconds = int(os.getenv("API_TIMEOUT_SECONDS", "10"))

    return AppConfig(
        api_base_url=base_url,
        api_timeout_seconds=timeout_seconds,
    )
