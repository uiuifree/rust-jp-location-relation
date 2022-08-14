# 隣接エリアの取得
-[ ] 隣接都道府県の取得
-[x] 隣接市区町村の取得
-[ ] 隣接街名の取得


## データソース
https://www.stat.go.jp/data/mesh/m_itiran.html

## 実行例

```
let city = City::find_by_id(13101).unwrap();

println!("{:?}",city.relation_city_id);
// [13103, 13104, 13105, 13102, 13106]
println!("{:?}",city.relation_city_name
// ["港区", "新宿区", "文京区", "中央区", "台東区"]
```