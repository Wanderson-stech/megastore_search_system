use std::collections::HashMap;

pub struct ProductIndex {
    index: HashMap<String, Vec<u32>>,
}

impl ProductIndex {
    pub fn new() -> Self {
        ProductIndex {
            index: HashMap::new(),
        }
    }

    // Insere o product_id para a chave exata
    pub fn insert(&mut self, key: String, product_id: u32) {
        self.index.entry(key).or_insert_with(Vec::new).push(product_id);
    }

    // Retorna o vetor de produtos pra chave, se existir
    pub fn get(&self, key: &str) -> Option<&Vec<u32>> {
        self.index.get(key)
    }
}

#[cfg(test)]
mod tests {
    use super::ProductIndex;

    #[test]
    fn test_insert_and_get() {
        let mut index = ProductIndex::new();
        index.insert("fone bluetooth".to_string(), 1);
        index.insert("fone bluetooth".to_string(), 2);

        let results = index.get("fone bluetooth");
        assert!(results.is_some());
        let products = results.unwrap();
        assert_eq!(products.len(), 2);
        assert_eq!(products, &vec![1, 2]);
    }
}
