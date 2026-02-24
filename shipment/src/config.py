import sys
import tomllib
from pathlib import Path


def load_settings() -> dict:
    """Loads settings.toml from the project root."""
    settings_path = Path(__file__).parent.parent / "settings.toml"
    if not settings_path.exists():
        print(
            "[ERROR] settings.toml not found.\n"
            "Copy settings.example.toml to settings.toml and fill in your values."
        )
        sys.exit(1)
    with open(settings_path, "rb") as f:
        return tomllib.load(f)


settings = load_settings()
SERVER = settings["server"]


def server_url() -> str:
    return f"{SERVER['protocol']}://{SERVER['host']}:{SERVER['port']}"


def endpoint(key: str, **kwargs) -> str:
    """Returns a full URL for a given endpoint key, with optional path params."""
    path: str = SERVER["endpoints"][key]
    return server_url() + path.format(**kwargs)
