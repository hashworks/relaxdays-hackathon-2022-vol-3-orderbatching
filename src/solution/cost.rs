use crate::instance;
use std::collections::{HashMap, HashSet};

use super::Batch;
use super::Solution;

impl Batch {
    fn cost(&self, instance: &instance::Instance) -> usize {
        let mut warehouses: HashMap<usize, HashSet<usize>> = HashMap::new();

        for item in &self.items {
            let location = &instance.article_id_location_map[&item.article_id];

            warehouses
                .entry(location.warehouse)
                .or_insert(HashSet::new())
                .insert(location.aisle);
        }

        warehouses.len() * 10
            + warehouses
                .iter()
                .map(|(_, aisles)| aisles.len())
                .sum::<usize>()
                * 5
    }
}

impl Solution {
    pub fn cost(&self, instance: &instance::Instance) -> usize {
        let tour_cost = self
            .batches
            .iter()
            .map(|batch| batch.cost(instance))
            .sum::<usize>();

        let rest_cost = self.waves.len() * 10 + self.batches.len() * 5;

        tour_cost + rest_cost
    }
}

#[test]
fn calculation() {
    let instance = instance::Instance::new_from_file("instances/instance0.json")
        .expect("Failed to load instances/instance0.json");

    let solution_file =
        std::fs::File::open("solutions/solution0.json").expect("Failed to open solution JSON file");

    let solution: Solution =
        serde_json::from_reader(solution_file).expect("Failed to parse solution JSON");

    assert_eq!(solution.cost(&instance), 665);
}
