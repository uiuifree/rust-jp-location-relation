use csv::{ WriterBuilder};
use std::collections::HashMap;
use std::io::Write;


fn file_name(prefecture_id: u8) -> String {
    format!("./storage/{:>02}", prefecture_id)
}


// https://www.stat.go.jp/data/mesh/m_itiran.html
fn main() {
    convert_utf8();
    let mut mesh_map = HashMap::new();
    let mut city_map = HashMap::new();
    let mut relation_city_map = HashMap::new();


    let mut records = vec![];

    for i in 1..47 {
        let name = file_name(i) + "_utf8.csv";
        let mut reader = csv::Reader::from_path(name).unwrap();
        for record in reader.records() {
            let record = record.unwrap();
            let id:i32 =record[0].to_string().parse().unwrap();
            records.push(vec![
                id.to_string(),
                record[1].to_string(),
                record[2].to_string(),
            ])
        }
    }

    for record in records.clone() {
        let city_id = record[0].to_string();
        let city_name = record[1].to_string();
        let mesh = record[2].to_string();
        let city_mesh = mesh_map.entry(mesh).or_insert(vec![]);
        if !city_mesh.contains(&city_id) {
            city_mesh.push(city_id.to_string());
        }
        city_map.entry(city_id).or_insert(city_name);
    }
    for record in records {
        let city_id = record[0].to_string();
        let mesh = record[2].to_string();
        let relation_cities = relation_city_map
            .entry(city_id.to_string())
            .or_insert(vec![]);
        let city_ids = mesh_map.get(&mesh);
        if city_ids.is_none() {
            continue;
        }
        for id in city_ids.unwrap() {
            if city_id.as_str() == id.as_str() {
                continue;
            }
            if relation_cities.contains(id) {
                continue;
            }
            relation_cities.push(id.to_string())
        }
    }

    let mut writer = WriterBuilder::new().delimiter(b'\t').from_path("./src/data/city_relation.tsv").unwrap();

    for (city_id, relation_city_ids) in relation_city_map {
        let base_city = city_map.get(&city_id).unwrap();
        println!("---{}", base_city);
        let mut list = vec![];
        for relation_city_id in relation_city_ids.clone() {
            list.push(city_map.get(&relation_city_id).unwrap().to_string());
        }
        println!("{:?}", list);
        writer
            .write_record(vec![
                city_id.to_string(),
                base_city.to_string(),
                list.join(",").to_string(),
                relation_city_ids.join(",").to_string(),
            ])
            .unwrap()
    }
}

use encoding_rs;
use std::fs;
use std::path::Path;

pub fn rewrite_file_sjis_to_utf8(path: &str, out1: &str) {
    if Path::new(path).exists() {
        return;
    }
    // 出力先ファイルパスの設定
    let mut file = fs::File::create(out1).unwrap();
    // 読み込み
    let s = fs::read(path).unwrap();
    // Shift_JISのバイト列(Vec<u8>) を UTF-8の文字列(String) に変換
    let (res, _, _) = encoding_rs::SHIFT_JIS.decode(&s);
    let text = res.into_owned();
    file.write(text.as_bytes()).unwrap();
}

fn convert_utf8() -> Vec<String> {
    let mut names = vec![];
    for i in 1..47 {
        let input = file_name(i) + ".csv";
        let output = file_name(i) + "_utf8.csv";
        rewrite_file_sjis_to_utf8(input.as_str(), output.as_str());
        names.push(output);
    }
    names
}
