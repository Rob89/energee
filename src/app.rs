use std::error;

use anyhow::Result;

pub type AppResult<T> = Result<T, Box<dyn error::Error>>;

/// Application.
#[derive(Debug)]
pub struct App {
    /// Is the application running?
    pub running: bool,
    /// counter
    pub selected_meter: usize,

    pub meters: Vec<MeterPoint>,
}

#[derive(Debug)]
pub struct MeterPoint {
    pub mpan: String,

    pub serial: String,
}

impl MeterPoint {
    pub fn parse(value: String) -> Result<MeterPoint, &'static str> {
        let parts: Vec<_> = value.split(':').collect();
        if parts.len() == 2 {
            return Ok(MeterPoint { mpan: String::from(parts[0]), serial: String::from(parts[1]) });
        }
        return Err("Failed to parse value as a meter point. Expected mpan and serial number separated by a colon. mpan:serial_number.")
    }
}

impl Default for App {
    fn default() -> Self {
        Self {
            running: true,
            selected_meter: 0,
            meters: Vec::new()
        }
    }
}

impl App {
    /// Constructs a new instance of [`App`].
    pub fn new(meters: Vec<MeterPoint>) -> Self {
        let mut res = Self::default();
        res.meters = meters;
        res
    }

    /// Handles the tick event of the terminal.
    pub fn tick(&self) {}

    /// Set running to false to quit the application.
    pub fn quit(&mut self) {
        self.running = false;
    }

    pub fn next_meter(&mut self) {
        if let Some(res) = self.selected_meter.checked_add(1) {
            if  res < self.meters.len() {
                self.selected_meter = res;
            }
        }
    }

    pub fn previous_meter(&mut self) {
        if let Some(res) = self.selected_meter.checked_sub(1) {
            self.selected_meter = res;
        }
    }
}
