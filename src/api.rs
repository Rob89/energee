use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use base64::prelude::*;

use crate::app::{AppResult, MeterPoint, GroupBy};

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

pub async fn get_consumption_data(mp: &MeterPoint, api_key: &str, group_by: &GroupBy) -> AppResult<ConsumptionResponse> {
    let group_by = match group_by {
        GroupBy::HalfHour => "",
        GroupBy::Hour => "?group_by=hour",
        GroupBy::Day => "?group_by=day",
        GroupBy::Week => "?group_by=week",
    };
    let uri = match mp {
        MeterPoint::Gas(g) => format!("https://api.octopus.energy/v1/gas-meter-points/{}/meters/{}/consumption{}", &g.mprn, &g.serial, group_by),
        MeterPoint::Electric(e) => format!("https://api.octopus.energy/v1/electricity-meter-points/{}/meters/{}/consumption{}", &e.mpan, &e.serial, group_by),
    };

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
