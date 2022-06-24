use crate::solution;

use super::Instance;

impl Instance {
    pub fn generate_greedy_solution(&self) -> solution::Solution {
        // In theory, if the order count is low enough, we can just check every possible solution

        // Termination conditions:
        // A) We have already seen this Wave/Batch combination regardless of order [cache with HashSet<HashSet>]
        // B) The article count in waves is > WAVE_SIZE_MAX
        // C) The total cost of the (fraction-)solution is higher than the best known final (!) solution

        solution::Solution {
            waves: Vec::new(),
            batches: Vec::new(),
        }
    }
}
