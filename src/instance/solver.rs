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

        // Convert map to vector so we can sort it
        // Funny thing: Iterating over this HashMap gives warehouse_order_vector a random order, giving us different scores each time
        let mut warehouse_order_vector = warehouse_order_map
            .iter()
            .map(|(warehouses, orders)| (warehouses.clone(), orders.clone()))
            .collect::<Vec<_>>();

        warehouse_order_vector.sort_unstable_by(|(warehouses_a, _), (warehouses_b, _)| {
            warehouses_a.len().cmp(&warehouses_b.len())
        });

        let mut warehouses_waves: HashMap<Vec<usize>, Vec<Wave>> = HashMap::new();

        while let Some((warehouses_a, mut orders)) = warehouse_order_vector.pop() {
            let mut wave = Wave::new();
            'orderloop: while let Some(order) = orders.pop() {
                // Try to add orders to existing waves with matching warehouses with a specific allowed divergence
                // F.e.: A wave with warehouses [1,2,3] will also take orders with warehouses [1,2,3] or [1,2] if the divergence is 1
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

        // Optimize wave and batch sizes – f.e. move small waves into other waves if they fit
        // Also: Set ids etc. for JSON export
        solution.finalize();

        solution
    }
}
