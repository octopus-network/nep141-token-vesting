use crate::interfaces::Viewer;
use crate::vesting::traits::VestingAmount;
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

    fn get_claimable_amount(&self, vesting_id: VestingId) -> U128 {
        match self
            .internal_get_vesting(&vesting_id)
            .expect("no such vesting.")
        {
            Vesting::NaturalTimeLinearVesting(linear_vesting) => {
                U128(linear_vesting.get_claimable_amount())
            }
            Vesting::TimeCliffVesting(cliff_vesting) => U128(cliff_vesting.get_claimable_amount()),
        }
    }
}
