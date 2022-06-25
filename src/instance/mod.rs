pub mod solver;

use std::collections::HashMap;

use rand::seq::SliceRandom;
use serde::Deserialize;

#[derive(Debug, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct Instance {
    pub article_locations: Vec<ArticleLocation>,
    pub orders: Vec<Order>,
    pub articles: Vec<Article>,

    #[serde(skip)]
    pub order_id_article_ids_map: HashMap<usize, Vec<usize>>,
    #[serde(skip)]
    pub article_id_location_map: HashMap<usize, ArticleLocation>,
    #[serde(skip)]
    pub article_id_volume_map: HashMap<usize, usize>,
}

#[derive(Debug, Deserialize, Clone)]
#[serde(rename_all = "PascalCase")]
pub struct ArticleLocation {
    pub warehouse: usize,
    pub aisle: usize,
    // pub position: usize, // No need to parse the item position
    pub article_id: usize,
}

#[derive(Debug, Deserialize, Clone, Hash)]
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

impl Instance {
    fn generate_maps(&mut self) {
        self.order_id_article_ids_map = self
            .orders
            .iter()
            .map(|order| (order.order_id, order.article_ids.clone()))
            .collect();
        self.article_id_location_map = self
            .article_locations
            .iter()
            .map(|article_location| (article_location.article_id, article_location.clone()))
            .collect();
        self.article_id_volume_map = self
            .articles
            .iter()
            .map(|article| (article.article_id, article.volume))
            .collect();
    }

    pub fn new_from_file(path: &str) -> Result<Self, Box<dyn std::error::Error>> {
        let file = std::fs::File::open(path)?;

        let mut new_self: Self = serde_json::from_reader(file)?;

        new_self.generate_maps();

        Ok(new_self)
    }

    pub fn shuffle_orders(&mut self) {
        self.orders.shuffle(&mut rand::thread_rng());
    }
}
