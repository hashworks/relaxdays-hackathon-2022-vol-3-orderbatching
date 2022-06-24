// TODO: Remove
#![allow(dead_code)]

use serde::{Deserialize, Serialize};
use std::collections::{HashMap, HashSet};
use std::env;
use std::process::exit;

const BATCH_VOLUME_MAX: usize = 10000;
const WAVE_SIZE_MAX: usize = 250;

#[derive(Debug, Deserialize)]
#[serde(rename_all = "PascalCase")]
struct Instance {
    article_locations: Vec<ArticleLocation>,
    orders: Vec<Order>,
    articles: Vec<Article>,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "PascalCase")]
struct ArticleLocation {
    warehouse: usize,
    aisle: usize,
    // position: usize, // No need to parse the item position
    article_id: usize,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "PascalCase")]
struct Order {
    order_id: usize,
    article_ids: Vec<usize>,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "PascalCase")]
struct Article {
    article_id: usize,
    volume: usize,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "PascalCase")]
struct Solution {
    waves: Vec<Wave>,
    batches: Vec<Batch>,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "PascalCase")]
struct Wave {
    wave_id: usize,
    batch_ids: Vec<usize>,
    order_ids: Vec<usize>,
    wave_size: usize,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "PascalCase")]
struct Batch {
    batch_id: usize,
    items: Vec<Item>,
    batch_volume: usize,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "PascalCase")]
struct Item {
    order_id: usize,
    article_id: usize,
}

impl Batch {
    fn cost(&self, instance: &Instance) -> usize {
        let mut warehouses: HashMap<usize, HashSet<usize>> = HashMap::new();

        for item in &self.items {
            let location = instance
                .article_locations
                .iter()
                .filter(|article| article.article_id == item.article_id)
                .next()
                .expect("Unknown article id");

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
    fn cost(&self, instance: &Instance) -> usize {
        let tour_cost = self
            .batches
            .iter()
            .map(|batch| batch.cost(instance))
            .sum::<usize>();

        let rest_cost = self.waves.len() * 10 + self.batches.len() * 5;

        tour_cost + rest_cost
    }
}

fn heuristic_solution(_instance: &Instance) -> Solution {
    // Generate batches with volume < BATCH_VOLUME_MAX
    // Priority: group orders by warehouse and aisles
    // Secondary: minimal count of batches
    // Orders can be split into multiple batches as long as the batches-wave-size is < WAVE_SIZE_MAX articles

    // Generate waves - minimize count of waves

    Solution {
        waves: Vec::new(),
        batches: Vec::new(),
    }
}

fn greedy_solution(_instance: &Instance) -> Solution {
    // In theory, if the order count is low enough, we can just check every possible solution

    Solution {
        waves: Vec::new(),
        batches: Vec::new(),
    }
}

fn main() {
    let mut args = env::args();
    let b = args.next().unwrap();
    let instance_path = args.next();
    let solution_path = args.next();
    if instance_path.is_none() || solution_path.is_none() {
        eprintln!("{} <instance.json> <solution.json>", b);
        exit(2);
    }

    let instance_file =
        std::fs::File::open(instance_path.unwrap()).expect("Failed to open instance JSON file");

    let instance: Instance =
        serde_json::from_reader(instance_file).expect("Failed to parse instance JSON");

    let _greedy_solution = greedy_solution(&instance);
    let _heuristic_solution = heuristic_solution(&instance);

    //println!("Storing best solution with a cost of {}.", solution.cost());

    //let solution_file = std::fs::File::create(solution_path.unwrap()).expect("Failed to create solution JSON file");

    /*
    let result = serde_json::to_writer(
        solution_file,
        solution,
    );

    if result.is_err() {
        eprintln!(
            "Failed to write solution JSON file: {}",
            result.unwrap_err()
        );
        exit(3);
    }
     */
}

#[test]
fn cost_calculation() {
    let instance_file =
        std::fs::File::open("instances/instance0.json").expect("Failed to open instance JSON file");

    let instance: Instance =
        serde_json::from_reader(instance_file).expect("Failed to parse instance JSON");

    let solution_file =
        std::fs::File::open("solutions/solution0.json").expect("Failed to open solution JSON file");

    let solution: Solution =
        serde_json::from_reader(solution_file).expect("Failed to parse solution JSON");

    assert_eq!(solution.cost(&instance), 665);
}
