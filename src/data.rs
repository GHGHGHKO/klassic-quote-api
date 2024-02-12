use std::collections::HashMap;
use std::sync::atomic::{AtomicUsize, Ordering};
use rand::Rng;
use serde::{Deserialize, Serialize};

use tokio::fs;

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct QuoteItem {
    pub quote: String,
    pub author: String,
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

    pub async fn add_quotes(&mut self) {
        let quotes_file = fs::read_to_string("./src/quotes/the-war-of-flower.json")
            .await
            .unwrap();
        let quotes: Vec<QuoteItem> = serde_json::from_str(&quotes_file).unwrap();
        for quote in quotes {
            let id = self.id_generator.fetch_add(1, Ordering::Relaxed);
            let new_item = IdentifiableQuoteItem::new(id, quote);
            self.store.insert(id, new_item.clone());
        }
    }
}
