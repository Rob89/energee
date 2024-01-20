use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use base64::prelude::*;

use crate::app::{App, AppResult};

// {"consumption":0.0,"interval_start":"2024-01-16T23:00:00Z","interval_end":"2024-01-16T23:30:00Z"}
#[derive(Debug, Deserialize, Serialize)]
pub struct ConsumptionDatum {
    pub consumption: f64,
    pub interval_start: DateTime<Utc>,
    pub interval_end: DateTime<Utc>,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct ConsumptionResponse {
    pub results: Vec<ConsumptionDatum>,
    pub count: i64,
}

pub async fn get_consumption_data(app: &App, api_key: &str) -> AppResult<ConsumptionResponse> {
    let uri = format!("https://api.octopus.energy/v1/electricity-meter-points/{}/meters/{}/consumption/", app.meters[app.selected_meter].mpan, app.meters[app.selected_meter].serial);

    let b64 = BASE64_STANDARD.encode(api_key.as_bytes());
    let client = reqwest::Client::new();
    let body = client.get(&uri)
        .header("Authorization", "Basic ".to_owned() + &b64 + ":")
        .send()
        .await?;

    if body.status().as_u16() != 200 {
        println!("{}", body.text().await?);
        Err("Response failed")?
    }
    else {
        Ok(body.json::<ConsumptionResponse>().await?)
    }

}
