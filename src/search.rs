use crate::index::ProductIndex;

pub fn search_products(query: &str, index: &ProductIndex) -> Vec<u32> {
    let mut results: Vec<u32> = Vec::new();

    // Aqui simplifico a busca: uso a query inteira como chave
    if let Some(product_ids) = index.get(query) {
        results.extend(product_ids.iter());
    }

    results
}
