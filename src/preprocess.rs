/// Pré-processa a query do usuário para busca.
/// Faz lowercase, remove caracteres indesejados, mantém espaços,
/// e ainda pode ser extendido pra stemming, stopwords, etc.
pub fn preprocess_query(query: &str) -> String {
    query
        .to_lowercase() // tudo minúsculo
        .chars()
        .filter(|c| c.is_alphanumeric() || c.is_whitespace()) // mantém só letras, números e espaços
        .collect()
}
