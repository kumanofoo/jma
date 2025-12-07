use jma::amedas::{
    station_information, Amedas, AmedasData,
};

#[tokio::main]
async fn main() {
    let amedas_station_sapporo = "14163";
    
    let information = station_information(amedas_station_sapporo).await.unwrap();
    println!("AMeDAS station: {}({})", information.kanji_name, information.english_name);
    println!("      Latitude: {}°{}′, Longitu: {}°{}′", information.lat.0, information.lat.1, information.lon.0, information.lon.1);
    
    let amedas = Amedas::new(amedas_station_sapporo).await.unwrap();
    println!("          Latest Time: {}", amedas.latest_time);
    let latest_raw = amedas.get_latest_data();
    let latest = match latest_raw {
	Some(amedas_raw) => AmedasData::from(&amedas_raw),
	None => {
            println!("None");
            return;
        },
    };
    let snow1h = match latest.snow1h {
        Some(s) => s.to_string(),
        None => "N/A".to_string(),
    };
    println!("             Pressure: {} hPa", latest.pressure_hpa);
    println!("          Temperature: {} ℃", latest.temp_c);
    println!("             Humidity: {} %", latest.humidity_percent);
    println!("           Visibility: {} m", latest.visibility_m);
    println!("                 Wind: {} {} m", latest.wind_direction_emoji, latest.wind_mps);
    println!("              Weather: {}", latest.weather_discord_emoji);
    println!("          show 1 hour: {} cm", snow1h);
    println!("participitatio 10 min: {} mm", latest.precipitation10m);
}
