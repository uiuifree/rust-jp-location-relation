use lazy_static::lazy_static;
use std::collections::HashMap;

#[derive(Default, Debug, Clone)]
pub struct City {
    pub id: i32,
    pub prefecture_id: i32,
    pub name: String,
    pub relation_city_id: Vec<i32>,
    pub relation_city_name: Vec<String>,
}

impl City {
    pub fn get_cities() -> HashMap<i32, City> {
        return GET_CITIES.clone();
    }
    pub fn find_by_id(id: i32) -> Option<City> {
        return GET_CITIES.get(&id).cloned();
    }
    pub fn find_by_name(name: &str) -> Option<City> {
        let cities = &GET_CITIES;
        for (_, value) in cities.iter() {
            if value.name == name {
                return Some(value.clone());
            }
        }
        return None;
    }
}

lazy_static! {
    static ref GET_CITIES: HashMap<i32, City> = load_cities();
}

fn load_cities() -> HashMap<i32, City> {
    return include_str!("data/city_relation.tsv")
        .lines()
        // .skip(1)
        .map(|line| {
            let mut cols = line.split("\t");
            // let prefecture_id = cols.next().unwrap().parse::<i32>().unwrap();
            let city_id = cols.next().unwrap().parse::<i32>().unwrap();
            let city_name = cols.next().unwrap().parse::<String>().unwrap();
            let relation_names = cols.next().unwrap().parse::<String>().unwrap();
            let relation_ids = cols.next().unwrap().parse::<String>().unwrap();
            let r_names = relation_names.split(",").map(|m| m.to_string()).collect();
            let mut r_ids = vec![];

            for id in relation_ids.split(",") {
                if !id.is_empty() {
                    let id = id.to_string().parse().unwrap();
                    r_ids.push(id);
                }
            }
            let prefecture_id = format!("{:>05}", city_id);
            let prefecture_id = &prefecture_id[0..2];
            let prefecture_id = prefecture_id.parse().unwrap();

            City {
                id: city_id,
                prefecture_id,
                name: city_name,
                relation_city_name: r_names,
                relation_city_id: r_ids,
            }
        })
        .map(|city| {
            return (city.id, city);
        })
        .collect::<HashMap<i32, City>>();
}

#[cfg(test)]
mod tests {
    use crate::City;
    use crate::load_cities;

    #[test]
    fn test_load_cities() {
        load_cities();
        let cities = City::get_cities();
        assert_eq!(cities.len(), 1868);
        let city = City::find_by_id(1231).unwrap();
        assert_eq!(city.prefecture_id, 1);
        assert_eq!(city.id, 1231);
        let city = City::find_by_id(13101).unwrap();
        println!("{:?}",city.relation_city_id);
        println!("{:?}",city.relation_city_name);
        assert_eq!(city.prefecture_id, 13);
        assert_eq!(city.id, 13101);
        assert_eq!(city.relation_city_name, vec!["港区".to_string(), "新宿区".to_string(), "文京区".to_string(), "中央区".to_string(), "台東区".to_string()]);


        let prefecture = City::find_by_id(0).unwrap_or(City::default());
        assert_eq!(prefecture.id, 0);
        let city = City::find_by_name("堺市堺区").unwrap_or(City::default());
        assert_eq!(city.id, 27141);
        assert_eq!(city.name, "堺市堺区");
        assert_eq!(city.relation_city_name, vec!["堺市西区".to_string(), "堺市北区".to_string(), "大阪市住吉区".to_string(), "大阪市住之江区".to_string()]);
    }
}