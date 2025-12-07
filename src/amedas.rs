//! # Fetch AMeDAS Data
//! 
//! ## Example
//! ```rust
//! use jma::amedas::{station_information, Amedas, AmedasData};
//! 
//! #[tokio::main]
//! async fn main() {
//!     let amedas_station_sapporo = "14163";
//!     
//!     let information = station_information(amedas_station_sapporo).await.unwrap();
//!     println!("AMeDAS station: {}({})", information.kanji_name, information.english_name);
//!     println!("      Latitude: {}°{}′, Longitu: {}°{}′", information.lat.0, information.lat.1, information.lon.0, information.lon.1);
//!     
//!     let amedas = Amedas::new(amedas_station_sapporo).await.unwrap();
//!     println!("   Latest Time: {}", amedas.latest_time);
//!     let latest_raw = amedas.get_latest_data();
//!     let latest = match latest_raw {
//! 	Some(amedas_raw) => AmedasData::from(&amedas_raw),
//! 	None => {
//!             println!("None");
//!             return;
//!         },
//!     };
//!     println!("      Pressure: {} hPa", latest.pressure_hpa);
//!     println!("   Temperature: {} ℃", latest.temp_c);
//!     println!("      Humidity: {} %", latest.humidity_percent);
//!     println!("    Visibility: {} m", latest.visibility_m);
//!     println!("          Wind: {} {} m", latest.wind_direction_emoji, latest.wind_mps);
//!     println!("       Weather: {}", latest.weather_discord_emoji);
//! }
//! ```
//! 
//! Output:
//! ```console
//! AMeDAS station: 札幌(Sapporo)
//!       Latitude: 43°3.6′, Longitu: 141°19.7′
//! url: https://www.jma.go.jp/bosai/amedas/data/point/14163/20251118_09.json
//!    Latest Time: 2025-11-18T10:40:00+09:00
//!       Pressure: 1005.1 hPa
//!    Temperature: 0.4 ℃
//!       Humidity: 69 %
//!     Visibility: 20000 m
//!           Wind: ・ 0 m
//!        Weather: :sunny:
//! ```

use std::collections::HashMap;
use chrono::{Timelike, DateTime};
use serde::{Deserialize, Serialize};

#[derive(Debug)]
pub enum AmedasError {
    ChronoParseError(chrono::format::ParseError),
    ReqwestError(reqwest::Error),
    NoData(String),
}

impl std::fmt::Display for AmedasError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            AmedasError::ChronoParseError(e) => write!(f, "Chrone Parse error: {}", e),
            AmedasError::ReqwestError(e) => write!(f, "Reqwest error: {}", e),
            AmedasError::NoData(e) => write!(f, "No data: {}", e),
        }
    }
}

impl std::error::Error for AmedasError {}

impl From<chrono::format::ParseError> for AmedasError {
    fn from(err: chrono::format::ParseError) -> AmedasError {
        AmedasError::ChronoParseError(err)
    }
}

impl From<reqwest::Error> for AmedasError {
    fn from(err: reqwest::Error) -> AmedasError {
        AmedasError::ReqwestError(err)
    }
}


// e.g. https://www.jma.go.jp/bosai/amedas/data/point/14163/20251009_03.json
const AMEDAS_URL: &str = "https://www.jma.go.jp/bosai/amedas/data/point";
const AMEDAS_UPDATE: &str = "https://www.jma.go.jp/bosai/amedas/data/latest_time.txt";
const AMEDAS_SITES: &str = "https://www.jma.go.jp/bosai/amedas/const/amedastable.json";


pub async fn get_latest_time() -> Result<String, AmedasError> {
    let response = reqwest::get(AMEDAS_UPDATE).await?;
    let body = response.text().await?;
    let update = body
        .lines()
        .next()
        .map(|s| s.to_string())
        .unwrap_or_else(|| String::from(""));
    Ok(update)
}

pub fn create_amedas_url(amedas_code: &str, update_str: &str) -> Result<String, AmedasError>  {
    let datetime = DateTime::parse_from_rfc3339(&update_str)?;
    let date = datetime.format("%Y%m%d").to_string();
    let multiples_of_3_hour = datetime.hour() / 3 * 3;
    
    Ok(format!("{}/{}/{}_{:02}.json", AMEDAS_URL, amedas_code, date, multiples_of_3_hour))
}

pub async fn amedas_data(url: &str) -> Result<HashMap<String, AmedasRawData>, AmedasError> {
    let response = reqwest::get(url).await?;
    let data = response.json::<HashMap<String, AmedasRawData>>().await?;
    Ok(data)
}

pub async fn station_information(amedas_id: &str) -> Result<AmedasStation, AmedasError> {
    let response = reqwest::get(AMEDAS_SITES).await?;
    let stations = response.json::<HashMap<String, AmedasStation>>().await?;
    match stations.get(amedas_id) {
        Some(station) => return Ok(station.clone()),
        None => Err(AmedasError::NoData(format!("{} not found", amedas_id))),
    }
}

pub const AMEDAS_WIND_DIRECTION_STR: [&str; 17] = [
    "--",  // 0
    "NNE", // 1
    "NE",  // 2
    "ENE", // 3
    "E",   // 4
    "ESE", // 5
    "SE",  // 6
    "SSE", // 7
    "S",   // 8
    "SSW", // 9
    "SW",  // 10
    "WSW", // 11
    "W",   // 12
    "WNW", // 13
    "NW",  // 14
    "NNW", // 15
    "N",   // 16
];

pub const AMEDAS_WIND_DIRECTION_ARROW: [&str; 17] = [
    "・", // 0
    "⇙",  // 1
    "⇙",  // 2
    "⇐",  // 3
    "⇐",  // 4
    "⇖",  // 5
    "⇖",  // 6
    "⇑",  // 7
    "⇑",  // 8
    "⇗",  // 9
    "⇗",  // 10
    "⇒",  // 11
    "⇒",  // 12
    "⇘",  // 13
    "⇘",  // 14
    "⇓",  // 15
    "⇓",  // 16
];

pub const AMEDAS_WEATHER_EMOJI_SLACK: [(u32, &str); 19]  = [
    (0, ":sunny:"),
    (1, ":cloud:"),
    (2, ":fog:"),
    (3, ":fogggy:"),
    (4, ":umbrella_with_rain_drops:"),
    (5, ":umbrella_with_rain_drops:"),
    (6, ":fog:"),
    (7, ":umbrella_with_rain_drops:"),
    (8, ":umbrella_with_rain_drops:"),
    (9, ":snowflake:"),
    (10, ":snowman:"),
    (11, ":snow_cloud:"),
    (12, ":snow_cloud:"),
    (13, ":partly_sunny_rain:"),
    (14, ":partly_sunny_rain:"),
    (15, ":snowflake:"),
    (16, ":lightning:"),
    (100, ":night_with_stars:"),
    (999, ":construction:"),
];

pub const AMEDAS_WEATHER_EMOJI_DISCORD: [(u32, &str); 19]  = [
    (0, ":sunny:"),
    (1, ":cloud:"),
    (2, ":fog:"),
    (3, ":fogggy:"),
    (4, ":umbrella:"),
    (5, ":umbrella:"),
    (6, ":fog:"),
    (7, ":umbrella:"),
    (8, ":umbrella:"),
    (9, ":snowflake:"),
    (10, ":snowman2:"),
    (11, ":cloud_snow:"),
    (12, ":cloud_snow:"),
    (13, ":white_sun_rain_cloud:"),
    (14, ":white_sun_rain_cloud:"),
    (15, ":snowflake:"),
    (16, ":thunder_cloud_rain:"),
    (100, ":night_with_stars:"),
    (999, ":construction:"),
];

pub const JMA_WEATHER_SVG_URL: &str = "https://www.jma.go.jp/bosai/forecast/img";
pub const AMEDAS_WEATHER_JMA_WEATHER_CODES: [(u32, u32, u32); 19]  = [
    (0, 100, 500),
    (1, 200, 200),
    (2, 200, 200),
    (3, 300, 300),
    (4, 300, 300),
    (5, 300, 300),
    (6, 400, 400),
    (7, 300, 300),
    (8, 400, 400),
    (9, 403, 403),
    (10, 400, 400),
    (11, 403, 403),
    (12, 400, 400),
    (13, 302, 302),
    (14, 402, 402),
    (15, 400, 400),
    (16, 300, 300),
    (100, 500, 500),
    (999, 308, 308), // Weather code Not found
];
pub fn svg_url(code: u32, night: bool) -> String {
    let mut code_999 = None;
    for (c, day_svg, night_svg) in AMEDAS_WEATHER_JMA_WEATHER_CODES {
        if code == c {
            let svg = match night {
                true => night_svg,
                false => day_svg,
            };
            return format!("{}/{}.svg", JMA_WEATHER_SVG_URL, svg);
        }
        if c == 999 {
            code_999 = Some(match night { true => night_svg, false => day_svg });
        }
    }

    // The code is not found in AMEDAS_WEATHER_JMA_WEATHER_CODES.
    return format!("{}/{}.svg", JMA_WEATHER_SVG_URL, code_999.unwrap());
}

pub fn weather_emoji(code: u32, emoji: [(u32, &str); 19]) -> String {
    let mut code_999 = None;
    for (c, e) in emoji {
        if code == c {
            return e.to_string();
        }
        if c == 999 {
            code_999 = Some(e.to_string());
        }
    }

    return code_999.unwrap();
}

#[derive(Debug, Deserialize, Clone)]
pub struct AmedasRawData {
    pub pressure: (f32, u32),
    pub temp: (f32, u32),
    pub humidity: (f32, u32),
    pub visibility: (f32, u32),
    pub weather: Option<(u32, u32)>,
    pub snow1h: Option<(f32, u32)>,
    pub precipitation10m: (f32, u32),
    #[serde(rename = "windDirection")]    
    pub wind_direction: (Option<u32>, u32),
    pub wind: (Option<f32>, u32),
}

#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct AmedasData {
    pub pressure_hpa: f32,
    pub temp_c: f32,
    pub humidity_percent: f32,
    pub visibility_m: f32,
    pub weather: Option<u32>,
    pub snow1h: Option<f32>,
    pub precipitation10m: f32,
    #[serde(rename = "windDirection")]    
    pub wind_direction: u32,
    pub wind_mps: f32,
    
    pub weather_slack_emoji: String,
    pub weather_discord_emoji: String,
    pub wind_direction_str: String,
    pub wind_direction_emoji: String,
}

impl From<&AmedasRawData> for AmedasData {
    fn from(amedas: &AmedasRawData) -> Self {
        let (weather, slack, discord) = match amedas.weather {
            Some(w) => {
                let slack = weather_emoji(w.0, AMEDAS_WEATHER_EMOJI_SLACK);
                let discord = weather_emoji(w.0, AMEDAS_WEATHER_EMOJI_DISCORD);
                (Some(w.0), slack, discord)
            },
            None => {
                let slack = weather_emoji(999, AMEDAS_WEATHER_EMOJI_SLACK);
                let discord = weather_emoji(999, AMEDAS_WEATHER_EMOJI_DISCORD);
                (None, slack, discord)
            }
        };
        let wind_direction = amedas.wind_direction.0.unwrap_or(0);
        let wind = amedas.wind.0.unwrap_or(0.0);
        let wind_direction_str = AMEDAS_WIND_DIRECTION_STR[wind_direction as usize].to_string();
        let wind_direction_emoji = AMEDAS_WIND_DIRECTION_ARROW[wind_direction as usize].to_string();
        let snow1h = match amedas.snow1h {
            Some(s) => Some(s.0),
            None => None,
        };
        AmedasData {
            pressure_hpa: amedas.pressure.0,
            temp_c: amedas.temp.0,
            humidity_percent: amedas.humidity.0,
            visibility_m: amedas.visibility.0,
            weather,
            snow1h,
            precipitation10m: amedas.precipitation10m.0,
            wind_direction: wind_direction,
            wind_mps: wind,
            weather_slack_emoji: slack,
            weather_discord_emoji: discord,
            wind_direction_str,
            wind_direction_emoji,
        }
    }
}

#[derive(Debug, Deserialize, Clone)]
pub struct Amedas {
    pub amedas_code: String,
    pub data: HashMap<String, AmedasRawData>,
    pub latest_time: String,
}

impl Amedas {
    pub async fn new(amedas_code: &str) -> Result<Amedas, AmedasError> {
        let latest_time = get_latest_time().await?;
        let url = create_amedas_url(amedas_code, &latest_time)?;
        println!("url: {}", url);
	    let data = amedas_data(&url).await?;
	    Ok(Amedas { amedas_code: amedas_code.to_string(), data, latest_time })
    }

    pub async fn update(&mut self) -> Result<bool, AmedasError> {
        let latest_time= get_latest_time().await?;
        if latest_time == self.latest_time {
            return Ok(false);
        }
        
        let url = create_amedas_url(&self.amedas_code, &latest_time)?;
        let data = amedas_data(&url).await?;

        self.data = data;
        self.latest_time = latest_time.clone();
        Ok(true)
    }
    
    pub fn get_latest_data(&self) -> Option<AmedasRawData> {
	let datetime: Vec<u64> = self.data.keys().filter_map(|k| k.parse::<u64>().ok()).collect();
	let latest = match datetime.iter().max() {
	    Some(latest) => latest.to_string(),
	    None => return None,
	};

	let latest_weather_datetime = self.data.iter().filter_map(|(k, d)| {
	    if let Some(_w) = d.weather {Some(k)} else {None}
	}).collect::<Vec<&String>>().into_iter().min();

        let mut latest_data = match self.data.get(&latest) {
            Some(latest) => latest.clone(),
            None => return None,
        };
        
	if let Some(dt) = latest_weather_datetime {
            latest_data.weather = self.data[dt].weather;
            latest_data.snow1h = self.data[dt].snow1h;
        }
	
	Some(latest_data)
    }
    
    pub fn print(&self) {
	println!("amedas_code: {}", self.amedas_code);
	println!("latest_time: {}", self.latest_time);
        for (datetime, amedas) in &self.data {
	    println!("datetime: {:?}", datetime);
	    println!("  {:?}", amedas);
        }
    }
}

#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct AmedasStation {
    #[serde(rename = "type")]
    pub station_type: String,
    pub elems: String,
    pub lat: (f32, f32),
    pub lon: (f32, f32),
    pub alt: i32,
    #[serde(rename = "kjName")]
    pub kanji_name: String,
    #[serde(rename = "knName")]
    pub kana_name: String,
    #[serde(rename = "enName")]
    pub english_name: String,
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_create_amedas_url() {
        let amedas_code = "12345";

        let latest_time_str = "2025-10-10T02:15:35+09:00";
        let normal0 = create_amedas_url(amedas_code, latest_time_str).unwrap();
        assert_eq!(normal0, "https://www.jma.go.jp/bosai/amedas/data/point/12345/20251010_00.json");
        
        let latest_time_str = "2025-10-10T23:15:35+09:00";
        let normal21 = create_amedas_url(amedas_code, latest_time_str).unwrap();
        assert_eq!(normal21, "https://www.jma.go.jp/bosai/amedas/data/point/12345/20251010_21.json");
 
        let latest_time_parse_error_str = "2025-10-10";
        assert!(create_amedas_url(amedas_code, latest_time_parse_error_str).is_err());
    }
    
    #[tokio::test]
    async fn test_latest() {
        let amedas = Amedas::new("14163").await.unwrap();
	let data = amedas.get_latest_data();
        println!("amedas: {:?}", data);
    }

    #[tokio::test]
    async fn test_svg_url() {
        // Day
        for (code, _day_svg, _night_svg) in AMEDAS_WEATHER_JMA_WEATHER_CODES {
            let url = svg_url(code, false);
            let response = reqwest::get(url).await.unwrap();
            assert_eq!(response.status(), reqwest::StatusCode::OK);
        }
        // Night
        for (code, _day_svg, _night_svg) in AMEDAS_WEATHER_JMA_WEATHER_CODES {
            let url = svg_url(code, true);
            let response = reqwest::get(url).await.unwrap();
            assert_eq!(response.status(), reqwest::StatusCode::OK);
        }
        // Undefined code
        for code in [17, 200] {
            let url = svg_url(code, true);
            let response = reqwest::get(url).await.unwrap();
            assert_eq!(response.status(), reqwest::StatusCode::OK);
        }
    }
}
