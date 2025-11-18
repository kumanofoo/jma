//! # Reference AMEDAS Observation Sites in Forecast Area (Office10)
//!
//! <https://www.jma.go.jp/bosai/forecast/const/forecast_area.json>
//!
//! forecast_area.json
//! ```json
//! {
//!   "200000": [
//!     {
//!       "class10": "200010",
//!       "amedas": ["48156"],
//!       "class20": "2020100"
//!     },
//!     {
//!       "class10": "200020",
//!       "amedas": ["48361", "48491", "48331"],
//!       "class20": "2020201"
//!     }
//!   ]
//! }
//! ```
//!
//! ## Example
//! ```rust
//! use jma::forecast_area::ForecastArea;
//!
//! #[tokio::main]
//! async fn main() {
//!   let forecast_area = ForecastArea::new().await.unwrap();
//!   assert_eq!(forecast_area.offices["200000"][0].class10, "200010");
//!   assert_eq!(forecast_area.offices["200000"][1].class10, "200020");
//!   assert_eq!(forecast_area.offices["200000"][1].amedas[0], "48361");
//!   assert_eq!(forecast_area.offices["200000"][1].amedas[1], "48491");
//!   assert_eq!(forecast_area.offices["200000"][1].amedas[2], "48331");
//!   assert_eq!(forecast_area.offices["200000"][1].class20, "2020201");
//! }
//! ```

use reqwest::Error;
use serde::Deserialize;
use serde_json::Value;
use std::collections::HashMap;

/// AMEDAS Observation Site.
#[derive(Deserialize, Debug)]
pub struct AmedasObservationSite {
    pub class10: String,
    pub amedas: Vec<String>,
    pub class20: String,
}

/// AMEDAS Observation Sites in Forecast Areas.
#[derive(Deserialize, Debug)]
pub struct ForecastArea {
    #[serde(flatten)]
    pub offices: HashMap<String, Vec<AmedasObservationSite>>,
}

impl ForecastArea {
    /// Fetch forecast_area.json.
    pub async fn new() -> Result<Self, Error> {
        let url = "https://www.jma.go.jp/bosai/forecast/const/forecast_area.json";
        let forecast_area_json = reqwest::get(url).await?.json::<Value>().await?;
        let area: ForecastArea = serde_json::from_value(forecast_area_json.clone()).unwrap();
        Ok(area)
    }

    /// Get AMeDAS observation site.
    pub fn get_amedas_by_class10(&self, code: &str) -> Option<&Vec<String>> {
        for (_, offices) in &self.offices {
            for office in offices {
                if office.class10 == code {
                    return Some(&office.amedas);
                }
            }
        }
        None
    }

    /// Get the office of the AMeDAS station.
    pub fn get_office_by_amedas(&self, amedas_code: &str) -> Option<String> {
        for (office_code, offices) in &self.offices {
            let site = offices
                .iter()
                .find(|o| {
                    o.amedas.iter().any(|c| c == amedas_code)
                });
            if site.is_some() {
                return Some(office_code.clone());
            }
        };
        None
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn get_forecast_area() {
        let forecast_area = ForecastArea::new().await.unwrap();
        assert_eq!(forecast_area.offices["200000"][0].class10, "200010");
        assert_eq!(forecast_area.offices["200000"][1].class10, "200020");
        assert_eq!(forecast_area.offices["200000"][1].amedas[0], "48361");
        assert_eq!(forecast_area.offices["200000"][1].amedas[1], "48491");
        assert_eq!(forecast_area.offices["200000"][1].amedas[2], "48331");
        assert_eq!(forecast_area.offices["200000"][1].class20, "2020201");
    }

    #[tokio::test]
    async fn get_amedas_by_class10() {
        let forecast_area = ForecastArea::new().await.unwrap();
        let amedas = forecast_area.get_amedas_by_class10("200020").unwrap();
        assert_eq!(amedas.len(), 3);
        assert_eq!(amedas[0], "48361");
        assert_eq!(amedas[1], "48491");
        assert_eq!(amedas[2], "48331");
    }
}
