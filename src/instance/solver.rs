use std::collections::HashMap;

use crate::solution::{self, Wave};

use super::{Instance, Order};

impl Instance {
    pub fn solve(&self, maximum_allowed_divergence: usize) -> solution::Solution {
        // find orders that need the same warehouses
        let mut warehouse_order_map: HashMap<Vec<usize>, Vec<Order>> = HashMap::new();
        for order in &self.orders {
            let mut warehouses = Vec::new();

            for article_id in &order.article_ids {
                let location = &self.article_id_location_map[&article_id];
                if !warehouses.contains(&location.warehouse) {
                    warehouses.push(location.warehouse);
                }
            }

            warehouses.sort_unstable();
            warehouse_order_map
                .entry(warehouses)
                .or_insert(Vec::new())
                .push(order.clone());
        }

        // Funny thing: Iterating over this HashMap gives warehouse_order_vector a random order, giving us different scores each time
        let mut warehouse_order_vector = warehouse_order_map
            .iter()
            .map(|(warehouses, orders)| (warehouses.clone(), orders.clone()))
            .collect::<Vec<_>>();

        warehouse_order_vector.sort_unstable_by(|(warehouses_a, _), (warehouses_b, _)| {
            warehouses_a.len().cmp(&warehouses_b.len())
        });

        /*
            for (warehouses, orders) in warehouse_order_vector {
                println!("W {:?}: {} orders", warehouses, orders.len());
            }

            W [0, 1, 2, 4, 6, 7, 12, 13, 15]: 1 orders
            W [1, 4, 6, 7, 10, 11, 12, 14, 15]: 1 orders
            W [4, 5, 6, 8, 9, 11, 12, 14, 15]: 1 orders
            W [0, 1, 5, 8, 9, 10, 11, 12, 14]: 1 orders
            W [1, 2, 4, 7, 8, 9, 10, 11, 12]: 1 orders
            W [0, 3, 4, 5, 8, 10, 11, 12, 14]: 1 orders
            [...]
            W [7]: 29 orders
            W [10]: 34 orders
            W [6]: 26 orders
            W [15]: 34 orders
            W [4]: 26 orders
            W [9]: 32 orders
        */

        let mut warehouses_waves: HashMap<Vec<usize>, Vec<Wave>> = HashMap::new();

        while let Some((warehouses_a, mut orders)) = warehouse_order_vector.pop() {
            let mut wave = Wave::new();
            'orderloop: while let Some(order) = orders.pop() {
                // Try to add orders to existing waves with matching warehouses
                for allowed_divergence in 0..=maximum_allowed_divergence {
                    for (warehouses_b, waves) in warehouses_waves.iter_mut() {
                        let divergence = warehouses_a
                            .iter()
                            .filter(|w_a| !warehouses_b.contains(w_a))
                            .count();
                        if divergence > allowed_divergence {
                            continue;
                        }

                        for wave in waves.iter_mut() {
                            if wave.add_order(&self, order.clone()) {
                                continue 'orderloop;
                            }
                        }
                    }
                }

                // Add to a new wave otherwise
                if !wave.add_order(self, order.clone()) {
                    warehouses_waves
                        .entry(warehouses_a.clone())
                        .or_insert(Vec::new())
                        .push(wave);

                    wave = Wave::new();
                    if !wave.add_order(self, order) {
                        panic!("Failed to add an order to a new wave, this shouldn't happen!");
                    }
                }
            }

            // Finished the orders, add the last wave if it's not empty
            if !wave.batches.is_empty() {
                warehouses_waves
                    .entry(warehouses_a)
                    .or_insert(Vec::new())
                    .push(wave);
            }
        }

        let mut solution = solution::Solution::new();
        for (_, mut waves) in warehouses_waves {
            solution.waves.append(&mut waves);
        }

        solution.finalize();

        solution
    }
}
