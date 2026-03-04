from rich.console import Console
from rich.panel import Panel
from rich.prompt import IntPrompt, Prompt
from rich.table import Table
from requests import RequestException

from client_help.api import ApiClient
from client_help.config import get_config


console = Console()


def _render_employee_table(ci: int, email: str, name: str, phone_num: str | None, role: str) -> None:
    table = Table(title="Sesión iniciada", show_lines=True)
    table.add_column("Campo", style="cyan", no_wrap=True)
    table.add_column("Valor", style="white")

    table.add_row("CI", str(ci))
    table.add_row("Nombre", name)
    table.add_row("Correo", email)
    table.add_row("Teléfono", phone_num or "-")
    table.add_row("Rol", role)

    console.print(table)


def _render_questions_table(questions: list[dict]) -> None:
    if not questions:
        console.print("\n[yellow]No hay preguntas para mostrar.[/yellow]")
        return

    ordered_columns = ["id", "name", "description"]
    dynamic_columns = [column for column in questions[0].keys() if column not in ordered_columns]
    columns = [column for column in ordered_columns if column in questions[0]] + dynamic_columns

    table = Table(title="Tabla question", show_lines=True)
    for column in columns:
        table.add_column(column, style="white")

    for row in questions:
        table.add_row(*[str(row.get(column, "-")) for column in columns])

    console.print(table)


def run() -> None:
    config = get_config()
    client = ApiClient(config.api_base_url, config.api_timeout_seconds)

    console.print(
        Panel.fit(
            "[bold green]SendCure Client Help[/bold green]\n"
            "Inicio de sesión de empleado (Counter)",
            title="Terminal App",
            border_style="green",
        )
    )

    console.print("\n[bold]Ingresa tu CI[/bold]")
    ci = IntPrompt.ask("→")

    console.print("\n[bold]Ingresa tu contraseña[/bold]")
    passwd = Prompt.ask("→", password=True)

    console.print("\n[cyan]Validando credenciales...[/cyan]")

    try:
        employee = client.login_employee(ci=ci, passwd=passwd)
    except ValueError as err:
        console.print(f"[yellow]{err}[/yellow]")
        return
    except PermissionError as err:
        console.print(f"[red]{err}[/red]")
        return
    except RequestException as err:
        console.print(f"[red]No se pudo conectar al main-server:[/red] {err}")
        return
    except Exception as err:  # noqa: BLE001
        console.print(f"[red]Error:[/red] {err}")
        return

    console.print("[bold green]Login exitoso.[/bold green]\n")
    _render_employee_table(
        ci=employee.ci,
        email=employee.email,
        name=employee.name,
        phone_num=employee.phone_num,
        role=employee.role,
    )

    console.print("\n[cyan]Cargando preguntas...[/cyan]")

    try:
        questions = client.get_employee_questions()
    except RequestException as err:
        console.print(f"[red]No se pudo consultar /employee/questions:[/red] {err}")
        return
    except Exception as err:  # noqa: BLE001
        console.print(f"[red]Error consultando preguntas:[/red] {err}")
        return

    _render_questions_table(questions)
