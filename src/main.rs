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
  #[arg(short='e', long, value_parser, num_args = 1.., value_delimiter = ' ', help="Electric meter mpan and serial number separated by :")]
  electric_meters: Vec<String>,

  #[arg(short='g', long, value_parser, num_args = 1.., value_delimiter = ' ', help="Gas meter mprn and serial number separated by :")]
  gas_meters: Vec<String>,
}

#[tokio::main]
async fn main() -> AppResult<()> {
    let api_key = env::var("API_KEY").expect("API_KEY");
    let args = Args::parse();
    let parsed_electric_meters = args.electric_meters.iter().map(|x| MeterPoint::parse_electric(x.clone())).collect::<Vec<_>>();
    let parsed_gas_meters = args.gas_meters.iter().map(|x| MeterPoint::parse_gas(x.clone())).collect::<Vec<_>>();
    let mut meters: Vec<_> = parsed_electric_meters.into_iter().collect::<Result<_, _>>()?;
    let mut ok_gas: Vec<_> = parsed_gas_meters.into_iter().collect::<Result<_, _>>()?;
    meters.append(&mut ok_gas);


    if meters.len() == 0 {
            return Err("Expected at least one meter point to be provided")?
    }
    // Create an application.
    let mut app = App::new(meters, api_key.clone());


    // Initialize the terminal user interface.
    let backend = CrosstermBackend::new(io::stderr());
    let terminal = Terminal::new(backend)?;
    let events = EventHandler::new(250);
    let mut tui = Tui::new(terminal, events);
    tui.init()?;
    tui.draw(&mut app)?;

    app.load_data().await?;

    // Start the main loop.
    while app.running {
        // Render the user interface.
        tui.draw(&mut app)?;
        // Handle events.
        match tui.events.next().await? {
            Event::Tick => app.tick(),
            Event::Key(key_event) => handle_key_events(key_event, &mut app).await?,
            Event::Mouse(_) => {}
            Event::Resize(_, _) => {}
        }
    }

    // Exit the user interface.
    tui.exit()?;
    Ok(())
}
