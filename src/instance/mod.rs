// TODO: Remove
#![allow(dead_code)]

pub mod greedy;
pub mod heuristic;

use serde::Deserialize;

#[derive(Debug, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct Instance {
    pub article_locations: Vec<ArticleLocation>,
    pub orders: Vec<Order>,
    pub articles: Vec<Article>,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct ArticleLocation {
    pub warehouse: usize,
    pub aisle: usize,
    // pub position: usize, // No need to parse the item position
    pub article_id: usize,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct Order {
    pub order_id: usize,
    pub article_ids: Vec<usize>,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct Article {
    pub article_id: usize,
    pub volume: usize,
}
