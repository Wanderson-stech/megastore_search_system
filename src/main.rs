mod preprocess;
mod index;
mod search;
mod cache;

use preprocess::preprocess_query;
use index::ProductIndex;
use search::search_products;
use cache::Cache;

fn main() {
    let products = vec![
        (1, "Smartphone Samsung Galaxy"),
        (2, "Notebook Dell Inspiron"),
        (3, "Fone de Ouvido Bluetooth"),
        (4, "Câmera Digital Canon"),
    ];

    let mut index = ProductIndex::new();
    for (id, name) in &products {
        index.insert(name.to_string(), *id);
    }

    let mut cache = Cache::new();
    let query = "fone bluetooth";
    let clean_query = preprocess_query(query);

    if let Some(results) = cache.get(&clean_query) {
        println!("(CACHE) Resultados: {:?}", results);
    } else {
        let results = search_products(&clean_query, &index);
        println!("Resultados: {:?}", results);
        cache.set(clean_query, results);
    }
}
