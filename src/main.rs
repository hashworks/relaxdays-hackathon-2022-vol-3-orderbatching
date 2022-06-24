// TODO: Remove
#![allow(dead_code)]

mod instance;
mod solution;

use std::env;
use std::process::exit;

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

    let instance: instance::Instance =
        serde_json::from_reader(instance_file).expect("Failed to parse instance JSON");

    let _greedy_solution = instance.generate_greedy_solution();
    let _heuristic_solution = instance.generate_heuristic_solution();

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
