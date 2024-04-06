use std::collections::HashMap;
use std::sync::atomic::{AtomicUsize, Ordering};
use rand::Rng;
use serde::{Deserialize, Serialize};

use tokio::fs;
use crate::movie_enum::str_to_movie;

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct QuoteItem {
    pub quote: String,
    pub author: String,
    pub name: String,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct IdentifiableQuoteItem {
    pub id: usize,

    #[serde(flatten)]
    pub item: QuoteItem,
}

impl IdentifiableQuoteItem {
    pub fn new(id: usize, item: QuoteItem) -> IdentifiableQuoteItem {
        IdentifiableQuoteItem { id, item }
    }
}

#[derive(Serialize, Deserialize, Debug, Clone, Default)]
pub struct Pagination {
    pub offset: Option<usize>,
    pub limit: Option<usize>,
}

#[derive(thiserror::Error, Debug)]
pub enum QuoteStoreError {
    #[error("serialization error")]
    SerializationError(#[from] serde_json::error::Error),
}

#[derive(Default)]
pub struct QuoteStore {
    store: HashMap<usize, IdentifiableQuoteItem>,
    id_generator: AtomicUsize,
}

impl QuoteStore {

    pub fn get_quotes(&self, pagination: Pagination) -> Vec<IdentifiableQuoteItem> {
        self.store
            .values()
            .skip(pagination.offset.unwrap_or(0))
            .take(pagination.limit.unwrap_or(usize::MAX))
            .cloned()
            .collect::<Vec<_>>()
    }

    pub fn get_random_quote(&self, pagination: Pagination) -> Option<&IdentifiableQuoteItem> {
        let min_offset: usize = 1;
        let max_offset: usize = self.get_quotes(pagination).len() - 1;

        let random_offset: usize = rand::thread_rng()
            .gen_range(min_offset..=max_offset);
        self.store.get(&random_offset)
    }

    pub fn get_name_random_quote(&self, name: String)
        -> Option<&IdentifiableQuoteItem> {
        match str_to_movie(name) {
            Ok(movie_name) => {
                let filtered_quotes: Vec<&IdentifiableQuoteItem> = self.store.values()
                    .filter(|quote| quote.item.name == movie_name)
                    .collect();
                let random_offset = rand::thread_rng().gen_range(0..filtered_quotes.len());
                Some(filtered_quotes[random_offset])
            }
            Err(_) => None
        }

    }

    pub async fn add_quotes(&mut self) {
        let quotes_dir = "./src/quotes";
        let mut dir = fs::read_dir(quotes_dir).await.unwrap();

        while let Some(entry) = dir.next_entry().await.unwrap() {
            let file_path = entry.path();

            if file_path.is_file() {
                let quotes_file = fs::read_to_string(file_path).await.unwrap();
                let quotes: Vec<QuoteItem> = serde_json::from_str(&quotes_file).unwrap();

                for quote in quotes {
                    let id = self.id_generator.fetch_add(1, Ordering::Relaxed);
                    let new_item = IdentifiableQuoteItem::new(id, quote);
                    self.store.insert(id, new_item.clone());
                }
            }
        }
    }
}
