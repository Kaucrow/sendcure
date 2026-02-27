use counter::{
    prelude::*,
    event::EventHandler,
    model::ScreenId,
    tui::Tui,
    update::update,
};

#[tokio::main]
async fn main() -> Result<()> {
    /*let pool = {
        let args = AppArgs::parse();
        sqlx::postgres::PgPool::connect(&args.db).await?
    };*/

    let mut app = App::default();

    let backend = CrosstermBackend::new(std::io::stderr());
    let terminal = Terminal::new(backend)?;
    let event_handler = EventHandler::new(100);
    let mut tui = Tui::new(terminal, event_handler);
    tui.enter()?;

    tui.event_handler.tx.send(Event::EnterScreen(ScreenId::Counter))?;

    while !app.should_quit {
        if let Ok(event) = tui.event_handler.next() {
            update(&mut app, event, &tui.event_handler.tx).await.unwrap_or_else(|error| panic!("{}", error));
            tui.draw(&mut app)?;
        }
    }

    tui.exit()?;

    Ok(())
}