//! # Forecast
//!
//! <https://www.jma.go.jp/bosai/forecast/data/forecast/>{office_code}.json
//!
//! 020000.json
//! ```json
//! [
//!   { // [0]
//!     "publishingOffice":  "青森地方気象台",
//!     "reportDatetime":    "2025-03-28T17:00:00+09:00",
//!     "timeSeries": [
//!       { // [0].timeSeries[0]
//!         "timeDafines": [
//!           "2025-03-28T17:00:00+09:00",
//!           "2025-03-29T00:00:00+09:00",
//!           "2025-03-30T00:00:00+09:00"
//!         ],
//!         "areas": [
//!           {
//!             "area": { "name": "津軽", "code": "020010" },
//!             "weatherCodes": ["200", "200", "270"],
//!             "weathers": [
//!               "くもり　所により　雨",
//!               "くもり　所により　昼過ぎ　から　よるのはじめ頃　雪",
//!               "くもり　時々　雪か雨"
//!             ],
//!             "winds": [
//!               "西の風　後　やや強く　海上　では　後　西の風　強く",
//!               "西の風　はじめ　やや強く　海上　では　西の風　強く",
//!               "西の風　やや強く"
//!             ],
//!             "waves": [
//!               "１．５メートル　後　２メートル",
//!               "２．５メートル　後　１．５メートル",
//!               "１．５メートル　後　３メートル"
//!             ]
//!           }
//!         ]
//!       },
//!       { // [0].timeSeries[1]
//!         "timeDefines": [
//!           "2025-03-28T18:00:00+09:00",
//!           "2025-03-29T00:00:00+09:00",
//!           "2025-03-29T06:00:00+09:00",
//!           "2025-03-29T12:00:00+09:00",
//!           "2025-03-29T18:00:00+09:00"
//!         ],
//!         "areas": [
//!           {
//!             "area": { "name": "津軽", "code": "020010" },
//!             "pops": ["30", "10", "10", "30", "20"]
//!           }
//!         ]
//!       },
//!       { // [0].timeSeries[2]
//!         "timeDefines": [
//!           "2025-03-29T00:00:00+09:00",
//!           "2025-03-29T09:00:00+09:00"
//!         ],
//!         "areas": [ // [0].timeSeries[2].areas
//!           {
//!             "area": { "name": "青森", "code": "31312" },
//!             "temps": ["3", "9"]
//!           }
//!         ]
//!       }
//!     ]
//!   },    
//!   {
//!     "publishingOffice": "青森地方気象台",
//!     "reportDatetime": "2025-03-28T17:00:00+09:00",
//!     "timeSeries": [
//!       {
//!         "timeDefines": [
//!           "2025-03-29T00:00:00+09:00",
//!           "2025-03-30T00:00:00+09:00",
//!           "2025-03-31T00:00:00+09:00",
//!           "2025-04-01T00:00:00+09:00",
//!           "2025-04-02T00:00:00+09:00",
//!           "2025-04-03T00:00:00+09:00",
//!           "2025-04-04T00:00:00+09:00"
//!         ],
//!         "areas": [
//!           {
//!             "area": { "name": "津軽・下北", "code": "020100" },
//!             "weatherCodes": [
//!               "200", "270", "201", "201", "202", "202", "201"
//!             ],
//!             "pops": ["", "70", "30", "20", "50", "50", "30"],
//!             "reliabilities": ["", "", "B", "A", "C", "C", "B"]
//!           }
//!         ]
//!       },
//!       {
//!         "timeDefines": [
//!           "2025-03-29T00:00:00+09:00",
//!           "2025-03-30T00:00:00+09:00",
//!           "2025-03-31T00:00:00+09:00",
//!           "2025-04-01T00:00:00+09:00",
//!           "2025-04-02T00:00:00+09:00",
//!           "2025-04-03T00:00:00+09:00",
//!           "2025-04-04T00:00:00+09:00"
//!         ],
//!         "areas": [
//!           {
//!             "area": { "name": "青森", "code": "31312" },
//!             "tempsMin": ["", "-1", "0", "-1", "2", "3", "3"],
//!             "tempsMinUpper": ["", "1", "1", "1", "4", "5", "5"],
//!             "tempsMinLower": ["", "-3", "-2", "-4", "-2", "1", "0"],
//!             "tempsMax": ["", "5", "6", "10", "11", "9", "11"],
//!             "tempsMaxUpper": ["", "9", "8", "13", "14", "13", "14"],
//!             "tempsMaxLower": ["", "4", "4", "8", "8", "6", "8"]
//!           },
//!         ]
//!       }
//!     ],
//!     "tempAverage": {
//!       "areas": [
//!         {
//!           "area": { "name": "青森", "code": "31312" },
//!           "min": "1.5",
//!           "max": "10.3"
//!         },
//!       ]
//!     },
//!     "precipAverage": {
//!       "areas": [
//!         {
//!           "area": { "name": "青森", "code": "31312" },
//!           "min": "7.6",
//!           "max": "19.5"
//!         },
//!       ]
//!     }
//!   }
//! ]
//! ```

use reqwest::Error;
use serde::Deserialize;
use serde_json::Value;

///
/// When accessing Office code 140030 or 460040, 404 Not Found is returned.
/// On the JMA website, 140100 or 460100 is used.
///
fn office_for_url(offices: &str) -> &str {
    let data = [
        ("014030", "014100"), // 北海道地方 / 十勝地方
        ("460040", "460100"), // 九州南部・奄美地方 / 奄美地方
    ];

    let mut result = offices;
    for (key, value) in &data {
        if offices.to_string() == key.to_string() {
            result = value;
            break;
        }
    }

    result
}

/// Store fetched a forecast from JMA site.
pub struct JmaForecast {
    json: Value,
}

impl JmaForecast {
    /// Fetch a forecast JSON in a Office region and store.
    pub async fn new(office: &str) -> Result<JmaForecast, Error> {
        let url_office = office_for_url(office);
        let url = format!(
            "https://www.jma.go.jp/bosai/forecast/data/forecast/{}.json",
            url_office
        );
        let json = reqwest::get(&url).await?.json::<Value>().await?;
        Ok(JmaForecast { json })
    }

    /// Convert the old name used for a weather forecast region to the current city name.
    pub fn find_newcity(oldcity: &str) -> Option<String> {
        let data = [
            ("古川", "大崎市"),
            ("鷹巣", "北秋田市"),
            ("小名浜", "いわき市"),
            ("若松", "会津若松市"),
            ("田島", "南会津町"),
            ("八丈島", "八丈町"),
            ("父島", "小笠原村"),
            ("高田", "上越市"),
            ("相川", "佐渡市"),
            ("津川", "阿賀町"),
            ("伏木", "高岡市"),
            ("河口湖", "富士河口湖町"),
            ("網代", "熱海市"),
            ("石廊崎", "南伊豆町"),
            ("風屋", "十津川村"),
            ("潮岬", "串本町"),
            ("日和佐", "美波町"),
            ("室戸岬", "室戸市"),
            ("厳原", "対馬市"),
            ("福江", "五島市"),
            ("阿蘇乙姫", "阿蘇市"),
            ("牛深", "天草市"),
            ("油津", "日南市"),
            ("種子島", "西之表市"),
            ("沖永良部", "和泊町"),
            ("石垣島", "石垣市"),
            ("与那国島", "与那国町"),
            ("東京", "千代田区"),
            ("名瀬", "奄美市"),
            ("八幡", "北九州市"),
            ("34216", "大崎市"),
            ("32126", "北秋田市"),
            ("36846", "いわき市"),
            ("36361", "会津若松市"),
            ("36641", "南会津町"),
            ("44263", "八丈町"),
            ("44301", "小笠原村"),
            ("54651", "上越市"),
            ("54157", "佐渡市"),
            ("54421", "阿賀町"),
            ("55091", "高岡市"),
            ("49251", "富士河口湖町"),
            ("50281", "熱海市"),
            ("50561", "南伊豆町"),
            ("64227", "十津川村"),
            ("65356", "串本町"),
            ("71266", "美波町"),
            ("74372", "室戸市"),
            ("84072", "対馬市"),
            ("84536", "五島市"),
            ("86111", "阿蘇市"),
            ("86491", "天草市"),
            ("87492", "日南市"),
            ("88612", "西之表市"),
            ("88971", "和泊町"),
            ("94081", "石垣市"),
            ("94017", "与那国町"),
            ("44132", "千代田区"),
            ("88837", "奄美市"),
            ("82056", "北九州市"),
        ];

        let mut result: Option<String> = None;
        for (key, newcity) in data {
            if oldcity.to_string() == key.to_string() {
                result = Some(newcity.to_string());
                break;
            }
        }

        result
    }

    /// Get temperature points of the class10 regions.
    pub fn get_temperature_points(&self) -> Vec<Temps> {
        serde_json::from_value(self.json[0]["timeSeries"][2]["areas"].clone()).unwrap()
    }
}

/// The area name and code of an temperature points.
///
/// - List of AMeDAS observation sites: <https://www.jma.go.jp/bosai/amedas/const/amedastable.json>
#[derive(Deserialize, Debug, Clone)]
pub struct TempsArea {
    /// AMeDAS observation site ID.
    pub code: String,
    /// AMeDAS observation site name.
    pub name: String,
}

/// The temperature points of the class10 regions.
///
/// `{office}.json: [0].timeSeries[2].areas[.]`
#[derive(Deserialize, Debug)]
pub struct Temps {
    pub area: TempsArea,
    //temps: Vec<String>,
}
