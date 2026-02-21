use counter::{
    prelude::*,
    event::EventHandler,
    model::help_text::HelpText,
    tui::Tui,
    update::update,
};

const HELP_TEXT: HelpText = HelpText::default();

/*
lazy_static! {
    static ref BIN_PATH: Mutex<String> = Mutex::new({
            let mut path = String::from(std::env::current_exe().unwrap().to_string_lossy());
            path = path[..=path.find("rushcargo-app").expect("could not find the `rushcargo-app` folder") + ("rushcargo-app").len()].to_string();
            if cfg!(windows){
                path.push_str("bin\\");
            } else {
                path.push_str("bin/");
            }
            path
        });
    static ref GRAPH_URL: Mutex<String> = Mutex::new({
        let args = AppArgs::parse();
        args.graphserver
    });
}
*/

#[tokio::main]
async fn main() -> Result<()> {
    /*x{let mut title_file = std::fs::File::create(BIN_PATH.lock().unwrap().clone() + "title.bin")?;
    title_file.write_all(&bincode::serialize("    ____             __    ______                     
   / __ \\__  _______/ /_  / ____/___ __________ _____ 
  / /_/ / / / / ___/ __ \\/ /   / __ `/ ___/ __ `/ __ \\
 / _, _/ /_/ (__  ) / / / /___/ /_/ / /  / /_/ / /_/ /
/_/ |_|\\__,_/____/_/ /_/\\____/\\__,_/_/   \\__, /\\____/ 
                                        /____/        ").unwrap())?;
    }*/
    //crate::check_files::check_files();

    /*let pool = {
        let args = AppArgs::parse();
        sqlx::postgres::PgPool::connect(&args.db).await?
    };*/

    let mut app = App::default();
    //let mut app_arc = Arc::new(Mutex::new(app));

    let backend = CrosstermBackend::new(std::io::stderr());
    let terminal = Terminal::new(backend)?;
    let event_handler = EventHandler::new(100);
    let mut tui = Tui::new(terminal, event_handler);
    tui.enter()?;

    //app_arc.lock().unwrap().enter_screen(Screen::Login, &pool).await;

    tui.draw(&mut app)?;

    while !app.data.should_quit {
        if let Ok(event) = tui.event_handler.next() {
            update(&mut app, event, &tui.event_handler.tx).await.unwrap_or_else(|error| panic!("{}", error));
            tui.draw(&mut app)?;
        }
    }

    tui.exit()?;

    Ok(())
}