use jma::forecast::JmaForecast;

#[tokio::main]
async fn main() {
    let sapporo = ("016000", "14163");
    let (office, area) = sapporo;

    let forecast = JmaForecast::new(office).await.unwrap();
    let peak = forecast.temperature_forecast(area).unwrap();
    println!("report_datetime: {}", peak.report_datetime);
    println!("      area_name: {}", peak.area_name);
    println!("      area_code: {}", peak.area_code);
    println!("         lowest: {}", peak.lowest);
    println!("lowest_datetime: {}", peak.lowest_datetime);
    println!("        highest: {}", peak.highest);
    println!("highest_datetime: {}", peak.highest_datetime);
}
