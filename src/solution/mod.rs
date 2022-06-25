pub mod cost;

use serde::{Deserialize, Serialize};

use crate::instance::{Instance, Order};

pub const BATCH_VOLUME_MAX: usize = 10000;
pub const WAVE_SIZE_MAX: usize = 250;

#[derive(Debug, Serialize, Deserialize, Clone)]
#[serde(rename_all = "PascalCase")]
pub struct Solution {
    pub waves: Vec<Wave>,
    pub batches: Vec<Batch>,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
#[serde(rename_all = "PascalCase")]
pub struct Wave {
    pub wave_id: usize,
    pub batch_ids: Vec<usize>,
    pub order_ids: Vec<usize>,
    pub wave_size: usize,

    #[serde(skip)]
    pub batches: Vec<Batch>,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
#[serde(rename_all = "PascalCase")]
pub struct Batch {
    pub batch_id: usize,
    pub items: Vec<Item>,
    pub batch_volume: usize,

    #[serde(skip)]
    pub initial_warehouse: usize,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
#[serde(rename_all = "PascalCase")]
pub struct Item {
    pub order_id: usize,
    pub article_id: usize,
}

impl Solution {
    pub fn new() -> Self {
        Solution {
            waves: Vec::new(),
            batches: Vec::new(),
        }
    }

    pub fn _new_from_file(path: &str) -> Result<Self, Box<dyn std::error::Error>> {
        let file = std::fs::File::open(path)?;

        let new_self: Self = serde_json::from_reader(file)?;

        Ok(new_self)
    }

    pub fn save_to_file(&mut self, path: &str) -> Result<(), Box<dyn std::error::Error>> {
        let file = std::fs::File::create(path)?;
        serde_json::to_writer(file, self)?;
        Ok(())
    }

    pub fn finalize(&mut self) {
        self.optimize_wave_sizes();

        self.batches = Vec::new();
        let mut batch_id = 0;

        for (wave_index, wave) in self.waves.iter_mut().enumerate() {
            wave.optimize_batch_volumes();

            wave.wave_id = wave_index;

            wave.batch_ids = Vec::new();
            wave.order_ids = Vec::new();

            for batch in wave.batches.iter_mut() {
                batch.batch_id = batch_id;
                wave.batch_ids.push(batch_id);
                batch_id += 1;

                self.batches.push(batch.clone());

                for order_id in batch.items.iter().map(|i| i.order_id) {
                    if !wave.order_ids.contains(&order_id) {
                        wave.order_ids.push(order_id);
                    }
                }

                batch
                    .items
                    .sort_unstable_by(|a, b| a.order_id.cmp(&b.order_id));
            }

            // just for looks
            wave.order_ids.sort_unstable();
            wave.batches
                .sort_unstable_by(|a, b| a.batch_id.cmp(&b.batch_id));
        }
    }

    fn optimize_wave_sizes(&mut self) {
        // Sort it so the largest waves are first
        let mut sorted_waves = self.waves.clone();
        sorted_waves.sort_unstable_by(|a, b| b.wave_size.cmp(&a.wave_size));

        self.waves = Vec::new();

        'outer: for wave in sorted_waves {
            // Try to fit it in an existing branch
            for existing_wave in self.waves.iter_mut() {
                if existing_wave.wave_size + wave.wave_size <= WAVE_SIZE_MAX {
                    existing_wave.batches.extend(wave.batches.clone());
                    existing_wave.wave_size += wave.wave_size;

                    continue 'outer;
                }
            }
            // Couldn't fit it anywhere else, keep it
            self.waves.push(wave);
        }
    }
}

impl Wave {
    pub fn new() -> Self {
        Wave {
            wave_id: 0,            // filled later in finalize()
            batch_ids: Vec::new(), // filled later in finalize()
            order_ids: Vec::new(), // filled later in finalize()
            wave_size: 0,
            batches: Vec::new(), // internal only
        }
    }

    // Batches need to be merged / optimized by volume later on
    pub fn add_order(&mut self, instance: &Instance, order: Order) -> bool {
        if self.wave_size + order.article_ids.len() > WAVE_SIZE_MAX {
            return false;
        }

        self.wave_size += order.article_ids.len();

        'outer: for article_id in &order.article_ids {
            let article_warehouse = instance.article_id_location_map[article_id].warehouse;
            let article_volume = instance.article_id_volume_map[article_id];
            let item = Item {
                order_id: order.order_id,
                article_id: *article_id,
            };

            // Try to fit it by warehouse
            for batch in self.batches.iter_mut() {
                if batch.batch_volume + article_volume <= BATCH_VOLUME_MAX
                    && batch.initial_warehouse == article_warehouse
                {
                    batch.items.push(item.clone());
                    batch.batch_volume += article_volume;
                    continue 'outer;
                }
            }

            // If it can't fit, create a new batch
            self.batches.push(Batch {
                batch_id: 0, // filled later in finalize()
                items: vec![item],
                batch_volume: article_volume,
                initial_warehouse: article_warehouse,
            });
        }

        true
    }

    // optimizes rest_cost
    fn optimize_batch_volumes(&mut self) {
        // Sort it so the largest batches are first
        let mut sorted_batches = self.batches.clone();
        sorted_batches.sort_unstable_by(|a, b| b.batch_volume.cmp(&a.batch_volume));

        self.batches = Vec::new();

        'outer: for batch in sorted_batches {
            // Try to fit it in an existing branch
            for existing_batch in self.batches.iter_mut() {
                if existing_batch.batch_volume + batch.batch_volume <= BATCH_VOLUME_MAX {
                    existing_batch.items.extend(batch.items.clone());
                    existing_batch.batch_volume += batch.batch_volume;

                    continue 'outer;
                }
            }
            // Couldn't fit it anywhere else, keep it
            self.batches.push(batch);
        }
    }
}
