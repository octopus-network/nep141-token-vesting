use crate::interfaces::Viewer;
use crate::vesting::traits::{Beneficiary, VestingAmount};
use crate::*;

#[near_bindgen]
impl Viewer for TokenVestingContract {
    fn get_vesting_token_id(&self) -> AccountId {
        self.token_id.clone()
    }

    fn get_vesting(
        &self,
        from_index: u32,
        limit: u32,
        beneficiary: Option<AccountId>,
    ) -> Vec<Vesting> {
        // let b = beneficiary.un
        self.vestings
            .iter()
            .filter(|e| {
                if beneficiary.is_none() {
                    return true;
                }
                let beneficiary_in_vesting = match &e.1 {
                    Vesting::NaturalTimeLinearVesting(linear) => &linear.beneficiary,
                    Vesting::TimeCliffVesting(cliff) => &cliff.beneficiary,
                };
                beneficiary_in_vesting.eq(beneficiary.as_ref().unwrap())
            })
            .skip(from_index as usize)
            .take(limit as usize)
            .map(|e| e.1)
            .collect_vec()
    }

    fn get_vesting_by_id(&self, vesting_id: VestingId) -> Vesting {
        self.internal_get_vesting(&vesting_id)
            .expect(format!("Failed to get vesting, id: #{}", vesting_id.0).as_str())
    }

    fn get_claimable_amount(&self, vesting_id: VestingId) -> U128 {
        self.internal_get_vesting(&vesting_id)
            .expect("No such vesting.")
            .get_claimable_amount()
            .into()
    }

    fn get_all_claimable_amount(&self, beneficiary: Option<AccountId>) -> U128 {
        U128(
            self.vestings
                .values()
                .filter(|e| {
                    beneficiary.is_none() || e.get_beneficiary().eq(&beneficiary.as_ref().unwrap())
                })
                .map(|e| e.get_claimable_amount())
                .sum(),
        )
    }
}
