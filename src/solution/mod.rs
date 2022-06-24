pub mod cost;

use serde::{Deserialize, Serialize};

pub const BATCH_VOLUME_MAX: usize = 10000;
pub const WAVE_SIZE_MAX: usize = 250;

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct Solution {
    pub waves: Vec<Wave>,
    pub batches: Vec<Batch>,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct Wave {
    pub wave_id: usize,
    pub batch_ids: Vec<usize>,
    pub order_ids: Vec<usize>,
    pub wave_size: usize,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct Batch {
    pub batch_id: usize,
    pub items: Vec<Item>,
    pub batch_volume: usize,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct Item {
    pub order_id: usize,
    pub article_id: usize,
}
