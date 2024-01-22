use std::error;

use anyhow::Result;

use crate::api::{ConsumptionResponse, get_consumption_data};

pub type AppResult<T> = Result<T, Box<dyn error::Error>>;

/// Application.
#[derive(Debug)]
pub struct App {
    /// Is the application running?
    pub running: bool,
    /// counter
    pub selected_meter: usize,

    pub meters: Vec<MeterPoint>,

    pub api_key: String,
}

#[derive(Debug)]
pub enum MeterPoint {
    Gas(GasMeterPoint),
    Electric(ElectricMeterPoint),
}

#[derive(Debug)]
pub struct ElectricMeterPoint {
    pub mpan: String,
    pub serial: String,
    pub comsumption_data: Option<ConsumptionResponse>
}

#[derive(Debug)]
pub struct GasMeterPoint {
    pub mprn: String,
    pub serial: String,
    pub comsumption_data: Option<ConsumptionResponse>
}

impl MeterPoint {
    pub fn parse_electric(value: String) -> Result<MeterPoint, &'static str> {
        let parts: Vec<_> = value.split(':').collect();
        if parts.len() == 2 {
            return Ok(MeterPoint::Electric(ElectricMeterPoint { mpan: String::from(parts[0]), serial: String::from(parts[1]), comsumption_data: None }));
        }
        return Err("Failed to parse value as an electric meter point. Expected mpan and serial number separated by a colon. mpan:serial_number.")
    }
    pub fn parse_gas(value: String) -> Result<MeterPoint, &'static str> {
        let parts: Vec<_> = value.split(':').collect();
        if parts.len() == 2 {
            return Ok(MeterPoint::Gas(GasMeterPoint { mprn: String::from(parts[0]), serial: String::from(parts[1]), comsumption_data: None }));
        }
        return Err("Failed to parse value as a gas meter point. Expected mprn and serial number separated by a colon. mprn:serial_number.")
    }
}

impl Default for App {
    fn default() -> Self {
        Self {
            running: true,
            selected_meter: 0,
            api_key: "".into(),
            meters: Vec::new()
        }
    }
}

impl App {
    /// Constructs a new instance of [`App`].
    pub fn new(meters: Vec<MeterPoint>, api_key: String) -> Self {
        let mut res = Self::default();
        res.meters = meters;
        res.api_key = api_key;
        res
    }

    /// Handles the tick event of the terminal.
    pub fn tick(&self) {}

    /// Set running to false to quit the application.
    pub fn quit(&mut self) {
        self.running = false;
    }

    pub async fn next_meter(&mut self) -> AppResult<()> {
        if let Some(res) = self.selected_meter.checked_add(1) {
            if  res < self.meters.len() {
                self.selected_meter = res;
                self.load_data().await?;
            }
        }
        Ok(())
    }

    pub async fn previous_meter(&mut self) -> AppResult<()> {
        if let Some(res) = self.selected_meter.checked_sub(1) {
            self.selected_meter = res;
            self.load_data().await?;
        }
        Ok(())
    }

    pub async fn load_data(&mut self) -> AppResult<()> {
        let meter = &mut self.meters[self.selected_meter];
        let data = get_consumption_data(meter, &self.api_key.clone()).await?;
        match  meter {
            MeterPoint::Gas(ref mut g) => if g.comsumption_data.is_none() {
                g.comsumption_data = Some(data);
            },
            MeterPoint::Electric(ref mut e) => if e.comsumption_data.is_none() {
                e.comsumption_data = Some(data);
            },
        }
        Ok(())
    }
}
