pub mod heuristic;

use std::{collections::HashMap, fmt, io};

use serde::Deserialize;

#[derive(Debug)]
pub struct Error {
    kind: String,
    message: String,

    #[allow(dead_code)]
    source: Option<Box<dyn std::error::Error>>,
}

impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "AoCError [{}]: {}", self.kind, self.message)
    }
}

impl From<io::Error> for Error {
    fn from(error: io::Error) -> Self {
        Error {
            kind: String::from("io"),
            message: error.to_string(),
            source: Some(Box::new(error)),
        }
    }
}

impl From<serde_json::Error> for Error {
    fn from(error: serde_json::Error) -> Self {
        Error {
            kind: String::from("json"),
            message: error.to_string(),
            source: Some(Box::new(error)),
        }
    }
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct Instance {
    pub article_locations: Vec<ArticleLocation>,
    pub orders: Vec<Order>,
    pub articles: Vec<Article>,

    #[serde(skip_deserializing)]
    pub order_id_article_ids_map: HashMap<usize, Vec<usize>>,
    #[serde(skip_deserializing)]
    pub article_id_location_map: HashMap<usize, ArticleLocation>,
    #[serde(skip_deserializing)]
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

#[derive(Debug, Deserialize, PartialEq, Eq, PartialOrd, Ord, Clone, Hash)]
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
    pub fn generate_maps(&mut self) {
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

    pub fn new_from_file(path: &str) -> Result<Instance, Error> {
        let instance_file = std::fs::File::open(path)?;

        let mut instance: Instance = serde_json::from_reader(instance_file)?;

        instance.generate_maps();

        Ok(instance)
    }
}
