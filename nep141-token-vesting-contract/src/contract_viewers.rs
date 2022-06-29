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

    fn get_claimable_amount(&self, vesting_id: VestingId) -> U128 {
        self.internal_get_vesting(&vesting_id)
            .expect("No such vesting.")
            .get_claimable_amount()
            .into()
    }

    fn get_all_claimable_amount(&self, beneficiary: AccountId) -> U128 {
        U128(
            self.vestings
                .values()
                .filter(|e| e.get_beneficiary().eq(&beneficiary))
                .map(|e| e.get_claimable_amount())
                .sum(),
        )
    }

    fn get_legacy(&self, account_id: AccountId) -> U128 {
        self.legacy.get(&account_id).unwrap_or(0).into()
    }
}
