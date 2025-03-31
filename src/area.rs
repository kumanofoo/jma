//! # Fetch and Search Area Code
//!
//! <https://www.jma.go.jp/bosai/common/const/area.json>
//!
//! centers: Local forecast areas.
//!
//! offices: Prefecture forecast areas.
//!
//! class10s: Primary Subdivision areas.
//!
//! class15s: Regions that have a collection of municipalities.
//!
//! class20s: Secondary Subdivision areas (Cities).
//!
//! area.json
//! ```json
//! {
//!   "centers": {
//!     "010100": {
//!       name:       "北海道地方",
//!       enName:     "Hokkaido",
//!       officeName: "札幌管区気象台",
//!       children: [
//!         "011000", "012000"
//!       ]
//!     }
//!   },
//!   "offices": {
//!     "100000": {
//!       name:       "群馬県",
//!       enName:     "Gunma",
//!       officeName: "前橋地方気象台",
//!       parent:     "010300",
//!       children: [
//!         "100010", "100020"
//!       ]
//!     }
//!   },
//!   "class10s": {
//!     "011000": {
//!       "name": "宗谷地方",
//!       "enName": "Soya Region",
//!       "parent": "011000",
//!       "children": [
//!         "011011",
//!         "011012",
//!         "011013"
//!       ]
//!     }
//!   },
//!   "class15s": {
//!     "011011": {
//!       "name": "宗谷北部",
//!       "enName": "Northern Soya",
//!       "parent": "011000",
//!       "children": [
//!         "0121400",
//!         "0151100",
//!         "0151600",
//!         "0152000"
//!       ]
//!     }
//!   },
//!   "class20s": {
//!     "0110000": {
//!       "name": "札幌市",
//!       "enName": "Sapporo City",
//!       "kana": "さっぽろし",
//!       "parent": "016012"
//!     }
//!   }
//! }
//! ```
//!
//! ## Example
//! ```rust
//! use jma::area::{Areas, JmaAreaClass};
//!
//! #[tokio::main]
//! async fn main() {
//!     // Fetch area.json
//!     let areas = Areas::new().await.unwrap();
//!
//!     // Get values of 1020100 in Class20
//!     let v = areas.values(&JmaAreaClass::Class20, "1020100").unwrap();
//!     assert_eq!(v.area.name, "前橋市");
//!     assert_eq!(v.area.en_name, "Maebashi City");
//!     assert_eq!(v.area.kana, Some("まえばしし".to_string()));
//!     assert_eq!(v.area.parent, Some("100011".to_string()));
//!     assert_eq!(v.area.office_name, None);
//!     assert_eq!(v.area.children, None);
//!     assert_eq!(v.class, JmaAreaClass::Class20);
//!     assert_eq!(v.code, "1020100");
//!
//!     // Get ancestor in Office
//!     let a = areas.ancestor(&v, &JmaAreaClass::Office).unwrap();
//!     assert_eq!(a.area.name, "群馬県");
//!     assert_eq!(a.area.en_name, "Gunma");
//!     assert_eq!(a.area.kana, None);
//!     assert_eq!(a.area.parent, Some("010300".to_string()));
//!     assert_eq!(a.area.office_name, Some("前橋地方気象台".to_string()));
//!     assert_eq!(
//!         a.area.children,
//!         Some(vec!["100010".to_string(), "100020".to_string(),])
//!     );
//!     assert_eq!(a.class, JmaAreaClass::Office);
//!     assert_eq!(a.code, "100000");
//!
//!     // Search keyword '100011'.
//!     let k = areas.search("100011");
//!     assert_eq!(k.len(), 1);
//!     assert_eq!(k[0].area.name, "前橋・桐生地域");
//!     assert_eq!(k[0].area.en_name, "Maebashi Kiryu Area");
//!     assert_eq!(k[0].area.kana, None);
//!     assert_eq!(k[0].area.parent, Some("100010".to_string()));
//!     assert_eq!(k[0].area.office_name, None);
//!     assert_eq!(
//!         k[0].area.children,
//!         Some(vec![
//!             "1020100".to_string(),
//!             "1020300".to_string(),
//!             "1020800".to_string(),
//!             "1021200".to_string(),
//!             "1034400".to_string(),
//!             "1034500".to_string(),
//!         ])
//!     );
//! }
//! ```

use reqwest::Error;
use serde::Deserialize;
use serde_json::Value;
use std::collections::HashMap;
use std::fmt;

#[derive(Deserialize, Debug, PartialEq, Clone)]
pub enum JmaAreaClass {
    Center,
    Office,
    Class10,
    Class15,
    Class20,
}

impl JmaAreaClass {
    /// convert str to JmaAraClass.
    pub fn to_jma_area_class(class: &str) -> Option<JmaAreaClass> {
        let cls = class.to_lowercase();
        match cls.as_str() {
            "center" | "centers" => Some(JmaAreaClass::Center),
            "office" | "offices" => Some(JmaAreaClass::Office),
            "class10" | "class10s" => Some(JmaAreaClass::Class10),
            "class15" | "class15s" => Some(JmaAreaClass::Class15),
            "class20" | "class20s" => Some(JmaAreaClass::Class20),
            _ => None,
        }
    }

    /// return parent JmaAreaClass.
    pub fn parent(&self) -> Option<JmaAreaClass> {
        match self {
            JmaAreaClass::Center => None,
            JmaAreaClass::Office => Some(JmaAreaClass::Center),
            JmaAreaClass::Class10 => Some(JmaAreaClass::Office),
            JmaAreaClass::Class15 => Some(JmaAreaClass::Class10),
            JmaAreaClass::Class20 => Some(JmaAreaClass::Class15),
        }
    }

    /// return child JmaAreaClass.
    pub fn child(&self) -> Option<JmaAreaClass> {
        match self {
            JmaAreaClass::Center => Some(JmaAreaClass::Office),
            JmaAreaClass::Office => Some(JmaAreaClass::Class10),
            JmaAreaClass::Class10 => Some(JmaAreaClass::Class15),
            JmaAreaClass::Class15 => Some(JmaAreaClass::Class20),
            JmaAreaClass::Class20 => None,
        }
    }
}

impl fmt::Display for JmaAreaClass {
    /// convert JmaAreaClass to str.
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let class = match self {
            JmaAreaClass::Center => "Center",
            JmaAreaClass::Office => "Office",
            JmaAreaClass::Class10 => "Class10",
            JmaAreaClass::Class15 => "Class15",
            JmaAreaClass::Class20 => "Class20",
        };
        write!(f, "{}", class)
    }
}

#[derive(Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct RawArea {
    pub name: String,
    pub en_name: String,
    pub kana: Option<String>,
    pub parent: Option<String>,
    pub office_name: Option<String>,
    pub children: Option<Vec<String>>,
}

#[derive(Deserialize, Debug, Clone)]
pub struct Area {
    pub area: RawArea,
    pub class: JmaAreaClass,
    pub code: String,
}

impl Area {
    pub fn new(class: &JmaAreaClass, code: &str, raw: &RawArea) -> Self {
        Area {
            area: raw.clone(),
            class: class.clone(),
            code: code.to_string(),
        }
    }
}

#[derive(Deserialize, Debug)]
pub struct Areas {
    centers: HashMap<String, RawArea>,
    offices: HashMap<String, RawArea>,
    class10s: HashMap<String, RawArea>,
    class15s: HashMap<String, RawArea>,
    class20s: HashMap<String, RawArea>,
}

impl Areas {
    /// Fetch area.json from JMA and parse it.
    pub async fn new() -> Result<Areas, Error> {
        let url = "https://www.jma.go.jp/bosai/common/const/area.json";
        let area_json = reqwest::get(url).await?.json::<Value>().await?;
        let areas: Areas = serde_json::from_value(area_json.clone()).unwrap();
        Ok(areas)
    }

    /// Get area information.
    pub fn values(&self, class: &JmaAreaClass, code: &str) -> Option<Area> {
        let cls = match class {
            JmaAreaClass::Center => &self.centers,
            JmaAreaClass::Office => &self.offices,
            JmaAreaClass::Class10 => &self.class10s,
            JmaAreaClass::Class15 => &self.class15s,
            JmaAreaClass::Class20 => &self.class20s,
        };

        match cls.get(code) {
            Some(v) => Some(Area::new(class, code, &v)),
            None => None,
        }
    }

    pub fn areas(&self, class: &JmaAreaClass) -> &HashMap<String, RawArea> {
        match class {
            JmaAreaClass::Center => &self.centers,
            JmaAreaClass::Office => &self.offices,
            JmaAreaClass::Class10 => &self.class10s,
            JmaAreaClass::Class15 => &self.class15s,
            JmaAreaClass::Class20 => &self.class20s,
        }
    }

    /// Returns the area if the beginning of the name, en_name, or code string contains the key.
    pub fn search(&self, keyword: &str) -> Vec<Area> {
        let mut result = Vec::new();
        for class in [
            JmaAreaClass::Center,
            JmaAreaClass::Office,
            JmaAreaClass::Class10,
            JmaAreaClass::Class15,
            JmaAreaClass::Class20,
        ] {
            let cls = self.areas(&class);
            for (key, value) in cls {
                if key == keyword {
                    result.push(self.values(&class, &key).unwrap());
                }
                for k in ["name", "en_name"] {
                    let v = match k {
                        "name" => &value.name,
                        "en_name" => &value.en_name.to_lowercase(),
                        _ => panic!("key '{}' not found", k),
                    };
                    if v.starts_with(&keyword.to_lowercase()) {
                        result.push(self.values(&class, &key).unwrap());
                    }
                }
            }
        }
        result
    }

    /// Returns the area if the beginning of the name, en_name, or code string in class20 contains the key.
    pub fn search_class20s(&self, keyword: &str) -> Vec<Area> {
        let mut result = Vec::new();

        for (key, value) in &self.class20s {
            if key == keyword {
                result.push(self.values(&JmaAreaClass::Class20, &key).unwrap());
            }
            for k in ["name", "en_name"] {
                let v = match k {
                    "name" => &value.name,
                    "en_name" => &value.en_name.to_lowercase(),
                    _ => panic!("key '{}' not found", k),
                };
                if v.starts_with(&keyword.to_lowercase()) {
                    result.push(self.values(&JmaAreaClass::Class20, &key).unwrap());
                }
            }
        }
        result
    }

    /// Returns the area's parent.
    pub fn parent(&self, area: &Area) -> Option<Area> {
        let parent_code = match &area.area.parent {
            Some(code) => code,
            None => return None,
        };
        match area.class.parent() {
            Some(class) => self.values(&class, &parent_code),
            None => None,
        }
    }

    /// Returns the area's ancestor in the JmaAreaClass.
    pub fn ancestor(&self, area: &Area, class: &JmaAreaClass) -> Option<Area> {
        let mut p = area.clone();
        loop {
            if p.class == *class {
                return Some(p);
            }
            p = match self.parent(&p) {
                Some(parent) => parent,
                None => break,
            };
        }
        None
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn values() {
        let areas = Areas::new().await.unwrap();
        let v = areas.values(&JmaAreaClass::Class20, "1020100").unwrap();
        assert_eq!(v.area.name, "前橋市");
        assert_eq!(v.area.en_name, "Maebashi City");
        assert_eq!(v.area.kana, Some("まえばしし".to_string()));
        assert_eq!(v.area.parent, Some("100011".to_string()));
        assert_eq!(v.area.office_name, None);
        assert_eq!(v.area.children, None);
        assert_eq!(v.class, JmaAreaClass::Class20);
        assert_eq!(v.code, "1020100");
    }

    #[tokio::test]
    async fn ancestor() {
        let areas = Areas::new().await.unwrap();
        let v = areas.values(&JmaAreaClass::Class20, "4062500").unwrap();
        let a = areas.ancestor(&v, &JmaAreaClass::Office).unwrap();
        assert_eq!(a.area.name, "福岡県");
        assert_eq!(a.area.en_name, "Fukuoka");
        assert_eq!(a.area.kana, None);
        assert_eq!(a.area.parent, Some("010900".to_string()));
        assert_eq!(a.area.office_name, Some("福岡管区気象台".to_string()));
        assert_eq!(
            a.area.children,
            Some(vec![
                "400010".to_string(),
                "400020".to_string(),
                "400030".to_string(),
                "400040".to_string()
            ])
        );
        assert_eq!(a.class, JmaAreaClass::Office);
        assert_eq!(a.code, "400000");
    }

    #[tokio::test]
    async fn not_ancestor() {
        let areas = Areas::new().await.unwrap();
        let v = areas.values(&JmaAreaClass::Office, "400000").unwrap();
        let a = areas.ancestor(&v, &JmaAreaClass::Office).unwrap();
        assert_eq!(a.area.name, "福岡県");
        assert_eq!(a.area.en_name, "Fukuoka");
        assert_eq!(a.area.kana, None);
        assert_eq!(a.area.parent, Some("010900".to_string()));
        assert_eq!(a.area.office_name, Some("福岡管区気象台".to_string()));
        assert_eq!(
            a.area.children,
            Some(vec![
                "400010".to_string(),
                "400020".to_string(),
                "400030".to_string(),
                "400040".to_string()
            ])
        );
        assert_eq!(a.class, JmaAreaClass::Office);
        assert_eq!(a.code, "400000");
    }
}
