from rich.console import Console
from rich.panel import Panel
from rich.prompt import IntPrompt, Prompt
from rich.table import Table
from rich import box
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

    table = Table(
        title="Preguntas",
        box=box.SIMPLE_HEAVY,
        show_lines=False,
        expand=False,
        header_style="bold cyan",
    )

    table.add_column("ID", justify="right", no_wrap=True, width=4)
    table.add_column("Cliente", justify="right", no_wrap=True, width=8)
    table.add_column("Pregunta", style="white", no_wrap=True, overflow="ellipsis", min_width=45, max_width=70)
    table.add_column("Respuesta", style="white", no_wrap=True, overflow="ellipsis", min_width=30, max_width=55)

    for row in questions:
        question_id = row.get("questionId", "-")
        client_cid = row.get("clientCid", "-")

        question_text = row.get("questionText")
        response_text = row.get("response")

        question_value = "-" if question_text is None else str(question_text)
        response_value = "Sin respuesta" if response_text is None else str(response_text)

        question_value = question_value.replace("\n", " ").strip()
        response_value = response_value.replace("\n", " ").strip()

        if "Ã" in question_value or "Â" in question_value:
            try:
                question_value = question_value.encode("latin1").decode("utf-8")
            except (UnicodeEncodeError, UnicodeDecodeError):
                pass

        if "Ã" in response_value or "Â" in response_value:
            try:
                response_value = response_value.encode("latin1").decode("utf-8")
            except (UnicodeEncodeError, UnicodeDecodeError):
                pass

        table.add_row(str(question_id), str(client_cid), question_value, response_value)

    console.print(table)


def _show_questions(client: ApiClient) -> None:
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


def _answer_question(client: ApiClient) -> None:
    console.print("\n[bold]Ingresa el ID de la pregunta a responder[/bold]")
    question_id = IntPrompt.ask("→")

    console.print("\n[bold]Escribe la respuesta[/bold]")
    response_text = Prompt.ask("→").strip()

    try:
        message = client.answer_question(question_id=question_id, response_text=response_text)
    except ValueError as err:
        console.print(f"[yellow]{err}[/yellow]")
        return
    except LookupError as err:
        console.print(f"[yellow]{err}[/yellow]")
        return
    except RequestException as err:
        console.print(f"[red]No se pudo consultar el endpoint de respuesta:[/red] {err}")
        return
    except Exception as err:  # noqa: BLE001
        console.print(f"[red]Error respondiendo pregunta:[/red] {err}")
        return

    console.print(f"[green]{message}[/green]")


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

    console.print("[bold green]Sesión iniciada.[/bold green]")

    while True:
        console.print("\n[bold]Menú[/bold]")
        console.print("1. Mostrar preguntas")
        console.print("2. Responder pregunta")
        console.print("0. Salir")

        option = Prompt.ask("Selecciona una opción", choices=["1", "2", "0"], default="1")

        if option == "1":
            _show_questions(client)
            continue

        if option == "2":
            _answer_question(client)
            continue

        console.print("\n[cyan]Sesión finalizada.[/cyan]")
        break
