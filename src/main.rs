mod instance;
mod solution;

use std::env;
use std::process::exit;
use std::time::Instant;

use crate::solution::Solution;

const TIMEOUT_MINUTES: u64 = 4;

fn main() {
    let start = Instant::now();

    let mut args = env::args();
    let b = args.next().unwrap();
    let instance_path = args.next();
    let solution_path = args.next();
    if instance_path.is_none() || solution_path.is_none() {
        eprintln!("{} <instance.json> <solution.json>", b);
        exit(2);
    }

    let instance_path = instance_path.unwrap();
    let solution_path = solution_path.unwrap();

    println!("Running for {} minutes.", TIMEOUT_MINUTES);
    println!("Loading instance '{}'…", instance_path);

    let mut instance = instance::Instance::new_from_file(&instance_path).unwrap();

    let mut best_solution = Solution::new();
    let mut best_cost = (std::usize::MAX / 2, std::usize::MAX / 2);

    println!("Solving…");

    'forever: loop {
        instance.shuffle_orders();

        for maximum_allowed_divergence in 0..=5 {
            let solution = instance.solve(maximum_allowed_divergence);
            let cost = solution.cost(&instance);
            if (cost.0 + cost.1) < (best_cost.0 + best_cost.1) {
                best_cost = cost;
                best_solution = solution;
            }

            if start.elapsed().as_secs() > TIMEOUT_MINUTES * 60 {
                break 'forever;
            }
        }
    }

    println!(
        "Storing solution at {} with tour cost {} + rest cost {} = {}.",
        solution_path,
        best_cost.0,
        best_cost.1,
        best_cost.0 + best_cost.1
    );

    best_solution.save_to_file(&solution_path).unwrap();
}
