# 🛍️ MegaStore Search System

Sistema de busca e recomendação otimizada para um catálogo de produtos, desenvolvido como estudo de caso acadêmico com foco em desempenho, escalabilidade e boas práticas em Rust.

---

## 📚 Descrição

Este projeto simula a implementação de um sistema de busca textual com cache inteligente, utilizando um índice invertido simples e técnicas de pré-processamento de texto. A proposta busca explorar conceitos fundamentais de algoritmos de busca, estruturas de dados, modularização e testes automatizados.

---

## 🚀 Funcionalidades

- 🔍 Indexação de nomes de produtos
- 🧠 Pré-processamento de consultas (limpeza e normalização)
- ⚡ Sistema de cache para acelerar buscas repetidas
- ✅ Testes unitários e de integração

---

## 🧪 Tecnologias Utilizadas

- 🦀 [Rust](https://www.rust-lang.org/)
- 📦 Cargo (gerenciador de pacotes e build system)
- 🧪 `#[cfg(test)]` para testes unitários
- 💾 Estruturas de dados eficientes (HashMap, etc)

---

## 📁 Estrutura do Projeto

```bash
megastore_search_system/
├── src/
│   ├── main.rs            # Função principal e exemplo de uso
│   ├── lib.rs             # Biblioteca central (ponto de entrada dos testes)
│   ├── preprocess.rs      # Módulo de pré-processamento de texto
│   ├── index.rs           # Módulo de indexação dos produtos
│   ├── search.rs          # Módulo de busca no índice
│   └── cache.rs           # Módulo de cache em memória
├── tests/
│   └── integration_tests.rs # Testes de integração
├── Cargo.toml             # Manifesto do projeto Rust
├── relatorio.docx         # Relatório acadêmico (anexado conforme exigência)
└── README.md              # Este arquivo
