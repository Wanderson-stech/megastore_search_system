use megastore_search_system::{preprocess, index, search, cache};

#[test]
fn test_preprocess_query() {
    let query = "Fone Bluetooth!!!";
    let processed = preprocess::preprocess_query(query);
    assert_eq!(processed, "fone bluetooth");
}

#[test]
fn test_index_and_search() {
    let products = vec![
        (1, "fone bluetooth"),
        (2, "notebook dell inspiron"),
        (3, "fone bluetooth"),
    ];

    let mut index = index::ProductIndex::new();
    for (id, name) in &products {
        index.insert(name.to_string(), *id);
    }

    let results = search::search_products("fone bluetooth", &index);
    assert_eq!(results, vec![1, 3]);
}

#[test]
fn test_cache() {
    let mut cache = cache::Cache::new();
    cache.set("test".to_string(), vec![1, 2, 3]);
    let cached = cache.get("test");
    assert!(cached.is_some());
    assert_eq!(cached.unwrap(), vec![1, 2, 3]);
}
