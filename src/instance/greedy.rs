use std::collections::{HashMap, HashSet};

use crate::solution;

use super::Instance;

// Currently this only works on instance0. Maybe the todo below might improve that?

// TODO: Termination conditions: The total cost of the (fraction-)solution is higher than the best known final (!) solution

impl Instance {
    pub fn _generate_greedy_solution(&self) -> solution::Solution {
        let mut wave_cache: HashSet<Vec<Vec<usize>>> = HashSet::new();

        let order_sizes: HashMap<usize, usize> = self
            .orders
            .iter()
            .map(|o| (o.order_id, o.article_ids.len()))
            .collect();
        let remaining_orders: Vec<usize> = self.orders.iter().map(|o| o.order_id).collect();

        let _dirty_waves_variants =
            self._walk_tree(&order_sizes, &mut wave_cache, Vec::new(), remaining_orders);

        // TODO: Dirty waves variants to solution

        solution::Solution {
            waves: Vec::new(),
            batches: Vec::new(),
        }
    }

    fn _walk_tree(
        &self,
        order_sizes: &HashMap<usize, usize>,
        wave_cache: &mut HashSet<Vec<Vec<usize>>>,
        unfinished_waves: Vec<Vec<usize>>,
        remaining_orders: Vec<usize>,
    ) -> Vec<Vec<Vec<usize>>> {
        // Termination condition: No more orders left
        if remaining_orders.is_empty() {
            return vec![unfinished_waves];
        }

        let mut dirty_waves_variants = Vec::new();

        for (o_index, order_id) in remaining_orders.iter().enumerate() {
            ////////////////////////////////////////////////////////////////////////////////
            // Try to fit it inside an existing wave ///////////////////////////////////////
            ////////////////////////////////////////////////////////////////////////////////

            for (w_index, unfinished_wave) in unfinished_waves.iter().enumerate() {
                // Termination condition: The article count in waves is > WAVE_SIZE_MAX
                if unfinished_wave
                    .iter()
                    .map(|order_id| order_sizes[order_id])
                    .sum::<usize>()
                    + order_sizes[order_id]
                    > solution::WAVE_SIZE_MAX
                {
                    continue;
                }

                // Invariant: The order will fit inside the wave

                let mut unfinished_waves = unfinished_waves.clone();
                unfinished_waves[w_index].push(*order_id);

                // This is important for the cache-check - somehow I can't use HashSets here
                unfinished_waves[w_index].sort();
                unfinished_waves.sort();

                // Termination condition: We have already seen unfinished_waves
                if wave_cache.contains(&unfinished_waves) {
                    continue;
                }
                wave_cache.insert(unfinished_waves.clone());

                let mut remaining_orders = remaining_orders.clone();
                remaining_orders.swap_remove(o_index);

                dirty_waves_variants.append(&mut self._walk_tree(
                    order_sizes,
                    wave_cache,
                    unfinished_waves,
                    remaining_orders,
                ));
            }

            ////////////////////////////////////////////////////////////////////////////////
            // Fit it inside a new wave ////////////////////////////////////////////////////
            ////////////////////////////////////////////////////////////////////////////////

            let mut unfinished_waves = unfinished_waves.clone();
            unfinished_waves.push(vec![*order_id]);

            // This is important for the cache-check - somehow I can't use HashSets here
            unfinished_waves.sort();

            // Termination condition: We have already seen this Wave/Batch combination
            if wave_cache.contains(&unfinished_waves) {
                continue;
            }
            wave_cache.insert(unfinished_waves.clone());

            let mut remaining_orders = remaining_orders.clone();
            remaining_orders.swap_remove(o_index);

            dirty_waves_variants.append(&mut self._walk_tree(
                order_sizes,
                wave_cache,
                unfinished_waves,
                remaining_orders,
            ));
        }

        return dirty_waves_variants;
    }
}
