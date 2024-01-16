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
  #[arg(short='e', long, help="Electricity meter mpan and serial number separated by :")]
  electricity: String,

  #[arg(short, long, help="Gas meter mpan and serial number separated by :")]
  gas: String,
}

#[tokio::main]
async fn main() -> AppResult<()> {

    let _api_key = env::var("API_KEY").expect("API_KEY");

    let args = Args::parse();

    println!("Electricity {}, Gas {}", args.electricity, args.gas);

    let electricity = MeterPoint::parse(args.electricity).expect("e argument should be populated with mpan:serial_number");
    let gas = MeterPoint::parse(args.gas).expect("g argument should be populated with mpan:serial_number");

    // Create an application.
    let mut app = App::new(electricity, gas);

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
