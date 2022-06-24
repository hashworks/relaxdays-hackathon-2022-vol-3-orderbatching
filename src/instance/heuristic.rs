use crate::solution;

use super::Instance;

impl Instance {
    pub fn generate_heuristic_solution(&self) -> solution::Solution {
        // Generate batches with volume < BATCH_VOLUME_MAX
        // Priority: group orders by warehouse and aisles
        // Secondary: minimal count of batches
        // Orders can be split into multiple batches as long as the batches-wave-size is < WAVE_SIZE_MAX articles

        // Generate waves - minimize count of waves

        solution::Solution {
            waves: Vec::new(),
            batches: Vec::new(),
        }
    }
}
