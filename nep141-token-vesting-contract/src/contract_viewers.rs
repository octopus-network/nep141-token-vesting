use crate::interfaces::Viewer;
use crate::*;

#[near_bindgen]
impl Viewer for TokenVestingContract {
    fn get_vesting(&self, from_index: u32, limit: u32) -> Vec<Vesting> {
        self.vestings
            .iter()
            .skip(from_index as usize)
            .take(limit as usize)
            .map(|e| e.1)
            .collect_vec()
    }
}
