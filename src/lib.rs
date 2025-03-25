use reqwest::Error;
use serde::Deserialize;
use serde_json::Value;
use std::collections::HashMap;
use std::fmt;

#[derive(Deserialize, Debug, Clone, PartialEq)]
pub enum JmaAreaClass {
    Offices(Option<String>),
    Class10s(Option<String>),
    Class15s(Option<String>),
    Class20s(Option<String>),
}

impl JmaAreaClass {
    pub fn new(class: &str) -> Result<JmaAreaClass, String> {
        match class {
            "offices" => Ok(JmaAreaClass::Offices(None)),
            "class10s" => Ok(JmaAreaClass::Class10s(None)),
            "class15s" => Ok(JmaAreaClass::Class15s(None)),
            "class20s" => Ok(JmaAreaClass::Class20s(None)),
            _ => Err(format!("class '{}' not found", class)),
        }
    }

    pub fn new_with_code(class: &str, code: &str) -> Result<JmaAreaClass, String> {
        match class {
            "offices" => Ok(JmaAreaClass::Offices(Some(code.to_string()))),
            "class10s" => Ok(JmaAreaClass::Class10s(Some(code.to_string()))),
            "class15s" => Ok(JmaAreaClass::Class15s(Some(code.to_string()))),
            "class20s" => Ok(JmaAreaClass::Class20s(Some(code.to_string()))),
            _ => Err(format!("class '{}' not found", class)),
        }
    }

    pub fn set(&self, code: &str) -> JmaAreaClass {
        match self {
            JmaAreaClass::Offices(_) => JmaAreaClass::Offices(Some(code.to_string())),
            JmaAreaClass::Class10s(_) => JmaAreaClass::Class10s(Some(code.to_string())),
            JmaAreaClass::Class15s(_) => JmaAreaClass::Class15s(Some(code.to_string())),
            JmaAreaClass::Class20s(_) => JmaAreaClass::Class20s(Some(code.to_string())),
        }
    }

    pub fn parent(&self) -> Option<JmaAreaClass> {
        match self {
            JmaAreaClass::Offices(_) => None,
            JmaAreaClass::Class10s(_) => Some(JmaAreaClass::Offices(None)),
            JmaAreaClass::Class15s(_) => Some(JmaAreaClass::Class10s(None)),
            JmaAreaClass::Class20s(_) => Some(JmaAreaClass::Class15s(None)),
        }
    }

    pub fn child(&self) -> Option<JmaAreaClass> {
        match self {
            JmaAreaClass::Offices(_) => Some(JmaAreaClass::Class10s(None)),
            JmaAreaClass::Class10s(_) => Some(JmaAreaClass::Class15s(None)),
            JmaAreaClass::Class15s(_) => Some(JmaAreaClass::Class20s(None)),
            JmaAreaClass::Class20s(_) => None,
        }
    }

    pub fn equal(class1: &JmaAreaClass, class2: &JmaAreaClass) -> bool {
        match (class1, class2) {
            (JmaAreaClass::Offices(_), JmaAreaClass::Offices(_)) => true,
            (JmaAreaClass::Class10s(_), JmaAreaClass::Class10s(_)) => true,
            (JmaAreaClass::Class15s(_), JmaAreaClass::Class15s(_)) => true,
            (JmaAreaClass::Class20s(_), JmaAreaClass::Class20s(_)) => true,
            _ => false,
        }
    }

    pub fn eq(&self, class: &JmaAreaClass) -> bool {
        JmaAreaClass::equal(self, class)
    }
}

impl fmt::Display for JmaAreaClass {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let (class, code) = match self {
            JmaAreaClass::Offices(code) => ("offices", code),
            JmaAreaClass::Class10s(code) => ("class10s", code),
            JmaAreaClass::Class15s(code) => ("class15s", code),
            JmaAreaClass::Class20s(code) => ("class20s", code),
        };
        if let Some(c) = code {
            write!(f, "{}({})", class, c)
        } else {
            write!(f, "{}(None)", class)
        }
    }
}

fn offices_for_url(offices: &str) -> &str {
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

pub async fn get_weather_forecast(offices: &str) -> Result<Value, Error> {
    let url_offices = offices_for_url(offices);
    let url = format!(
        "https://www.jma.go.jp/bosai/forecast/data/forecast/{}.json",
        url_offices
    );
    let response = reqwest::get(&url).await?.json::<Value>().await?;
    Ok(response)
}

#[derive(Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct Area {
    pub name: String,
    pub en_name: String,
    pub parent: String,
    pub office_name: Option<String>,
    pub children: Option<Vec<String>>,
    pub class: Option<JmaAreaClass>,
}

impl fmt::Display for Area {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let _ = writeln!(f, "name: {}", self.name);
        let _ = writeln!(f, "en_name: {}", self.en_name);
        let _ = writeln!(f, "parent: {}", self.parent);
        let _ = writeln!(
            f,
            "office_name: {}",
            self.office_name.clone().unwrap_or("None".to_string())
        );
        if let Some(children) = &self.children {
            let _ = write!(f, "children: [{}]", children.join(", "));
        }
        write!(f, "")
    }
}

impl Area {
    pub fn add_class(area: &Area, class: JmaAreaClass) -> Area {
        Area {
            name: area.name.clone(),
            en_name: area.en_name.clone(),
            parent: area.parent.clone(),
            office_name: area.office_name.clone(),
            children: area.children.clone(),
            class: Some(class),
        }
    }

    pub fn class_name(&self) -> Option<String> {
        match &self.class {
            Some(class) => match class {
                JmaAreaClass::Offices(_) => Some("offices".to_string()),
                JmaAreaClass::Class10s(_) => Some("class10s".to_string()),
                JmaAreaClass::Class15s(_) => Some("class15s".to_string()),
                JmaAreaClass::Class20s(_) => Some("class20s".to_string()),
            },
            None => None,
        }
    }

    pub fn class_code(&self) -> Option<String> {
        let code = match &self.class {
            Some(class) => match class {
                JmaAreaClass::Offices(c) => c,
                JmaAreaClass::Class10s(c) => c,
                JmaAreaClass::Class15s(c) => c,
                JmaAreaClass::Class20s(c) => c,
            },
            None => return None,
        };
        match code {
            Some(c) => Some(c.to_string()),
            None => None,
        }
    }
}

#[derive(Deserialize, Debug)]
pub struct Areas {
    offices: HashMap<String, Area>,
    class10s: HashMap<String, Area>,
    class15s: HashMap<String, Area>,
    class20s: HashMap<String, Area>,
}

impl Areas {
    pub async fn new() -> Result<Areas, Error> {
        let url = "https://www.jma.go.jp/bosai/common/const/area.json";
        let area_json = reqwest::get(url).await?.json::<Value>().await?;
        let areas: Areas = serde_json::from_value(area_json.clone()).unwrap();
        Ok(areas)
    }

    pub fn search(&self, keyword: &str) -> Vec<(JmaAreaClass, String)> {
        let mut result = Vec::new();
        for class in [
            JmaAreaClass::Offices(None),
            JmaAreaClass::Class10s(None),
            JmaAreaClass::Class15s(None),
            JmaAreaClass::Class20s(None),
        ] {
            let cls = match class {
                JmaAreaClass::Offices(_) => &self.offices,
                JmaAreaClass::Class10s(_) => &self.class10s,
                JmaAreaClass::Class15s(_) => &self.class15s,
                JmaAreaClass::Class20s(_) => &self.class20s,
            };
            for (key, value) in cls {
                if key == keyword {
                    result.push((class.set(key), key.to_string()));
                }
                for k in ["name", "en_name"] {
                    let v = match k {
                        "name" => &value.name,
                        "en_name" => &value.en_name.to_lowercase(),
                        _ => panic!("key '{}' not found", k),
                    };
                    if v.starts_with(&keyword.to_lowercase()) {
                        result.push((class.set(key), key.to_string()));
                    }
                }
            }
        }
        result
    }

    pub fn search_class20s(&self, keyword: &str) -> Vec<Area> {
        let mut result = Vec::new();

        for (key, value) in &self.class20s {
            if key == keyword {
                let mut area = value.clone();
                area.class = Some(JmaAreaClass::Class20s(Some(key.to_string())));
                result.push(area);
            }
            for k in ["name", "en_name"] {
                let v = match k {
                    "name" => &value.name,
                    "en_name" => &value.en_name.to_lowercase(),
                    _ => panic!("key '{}' not found", k),
                };
                if v.starts_with(&keyword.to_lowercase()) {
                    let mut area = value.clone();
                    area.class = Some(JmaAreaClass::Class20s(Some(key.to_string())));
                    result.push(area);
                }
            }
        }
        result
    }

    pub fn values(&self, class: &JmaAreaClass) -> Option<Area> {
        let (cls, code) = match class {
            JmaAreaClass::Offices(code) => (&self.offices, code),
            JmaAreaClass::Class10s(code) => (&self.class10s, code),
            JmaAreaClass::Class15s(code) => (&self.class15s, code),
            JmaAreaClass::Class20s(code) => (&self.class20s, code),
        };

        let c = match code {
            Some(c) => c,
            None => return None,
        };

        match cls.get(c) {
            Some(v) => {
                let mut with_class = v.clone();
                with_class.class = Some(class.set(c));
                Some(with_class)
            }
            None => None,
        }
    }

    pub fn parent(&self, area: &Area) -> Option<Area> {
        let parent_class = match &area.class {
            Some(class) => class.parent(),
            None => return None,
        };
        match parent_class {
            Some(class) => self.values(&class.set(&area.parent)),
            None => None,
        }
    }

    pub fn ancestor(&self, area: &Area, class: &JmaAreaClass) -> Option<Area> {
        let mut parent_area = area.clone();
        let mut parent_class;

        loop {
            match &parent_area.class {
                Some(c) => {
                    if class.eq(&c) {
                        return Some(parent_area);
                    }
                }
                None => (),
            };
            parent_class = match &parent_area.class {
                Some(area_class) => match area_class.parent() {
                    Some(c) => c,
                    None => return None,
                },
                None => return None,
            };
            parent_area = match self.values(&parent_class.set(&parent_area.parent)) {
                Some(a) => a,
                None => return None,
            };
        }
    }
}

pub struct JmaForecast {
    json: Value,
}

impl JmaForecast {
    pub async fn new(offices: &str) -> Result<JmaForecast, Error> {
        let json = get_weather_forecast(offices).await?;
        Ok(JmaForecast { json })
    }

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

    pub fn get_temperature_points(&self) -> Vec<Temps> {
        serde_json::from_value(self.json[0]["timeSeries"][2]["areas"].clone()).unwrap()
    }
}

#[derive(Deserialize, Debug, Clone)]
pub struct TempsArea {
    pub code: String,
    pub name: String,
}

#[derive(Deserialize, Debug)]
pub struct Temps {
    pub area: TempsArea,
    //temps: Vec<String>,
}
