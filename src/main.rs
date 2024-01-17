use energee::app::{App, AppResult, MeterPoint};
use energee::event::{Event, EventHandler};
use energee::handler::handle_key_events;
use energee::tui::Tui;
use std::{io, env};
use ratatui::backend::CrosstermBackend;
use ratatui::Terminal;
use clap::Parser;

#[derive(Parser, Debug)]
#[command(about = "Energee - Octopus Smart Meter TUI")]
struct Args {
  /// App tick rate
  #[arg(short='m', long, value_parser, num_args = 1.., value_delimiter = ' ', help="Meter mpan and serial number separated by :")]
  meters: Vec<String>,
}

#[tokio::main]
async fn main() -> AppResult<()> {
    let _api_key = env::var("API_KEY").expect("API_KEY");
    let args = Args::parse();
    let parsed_meters = args.meters.iter().map(|x| MeterPoint::parse(x.clone())).collect::<Vec<_>>();
    let ok_meters: Vec<_> = parsed_meters.into_iter().collect::<Result<_, _>>()?;

    if ok_meters.len() == 0 {
            return Err("Expected at least one meter point to be provided")?
    }
    // Create an application.
    let mut app = App::new(ok_meters);

    // Initialize the terminal user interface.
    let backend = CrosstermBackend::new(io::stderr());
    let terminal = Terminal::new(backend)?;
    let events = EventHandler::new(250);
    let mut tui = Tui::new(terminal, events);
    tui.init()?;

    // Start the main loop.
    while app.running {
        // Render the user interface.
        tui.draw(&mut app)?;
        // Handle events.
        match tui.events.next().await? {
            Event::Tick => app.tick(),
            Event::Key(key_event) => handle_key_events(key_event, &mut app)?,
            Event::Mouse(_) => {}
            Event::Resize(_, _) => {}
        }
    }

    // Exit the user interface.
    tui.exit()?;
    Ok(())
}
