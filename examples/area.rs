use jma::area::Areas;

///
/// Search for a city name in area.json.
/// 
/// Below is an example of searching for 'akit':
/// ```console
/// Search for 'akita'
/// --
/// name: 秋田県
/// en_name: Akita
/// parent: 010200
/// office_name: 秋田地方気象台
/// child: 050010
/// child: 050020
/// class: Office
/// code: 050000
/// --
/// name: 秋田中央地域
/// en_name: Akita Central Area
/// parent: 050010
/// office_name: None
/// child: 0520100
/// child: 0520600
/// child: 0521100
/// child: 0536100
/// child: 0536300
/// child: 0536600
/// child: 0536800
/// class: Class15
/// code: 050011
/// --
/// name: 秋田市
/// en_name: Akita City
/// parent: 050011
/// office_name: None
/// class: Class20
/// code: 0520100
/// --
/// name: 安芸高田市
/// en_name: Akitakata City
/// parent: 340022
/// office_name: None
/// class: Class20
/// code: 3421400
/// ```
#[tokio::main]
async fn main() {
    let city_name = "akita";

    // Fetch area.json from JMA.
    let areas = Areas::new().await.unwrap();

    // Returns the area if the beginning of the name, en_name, or code string contains the key.
    let area_list = areas.search(city_name);
    
    if area_list.is_empty() {
        println!("{} not fornd in the area codes", city_name);
        return;
    }
    println!("Search for '{}'", city_name);
    for area in area_list {
        println!("--");
        println!("name: {}", area.area.name);
        println!("en_name: {}", area.area.en_name);
        println!("parent: {}", area.area.parent.unwrap_or("None".to_string()));
        println!("office_name: {}", area.area.office_name.unwrap_or("None".to_string()));
        if let Some(children) = area.area.children {
            for child in children {
                println!("child: {}", child);
            }
        }
        println!("class: {}", area.class);
        println!("code: {}", area.code);
    }
}
