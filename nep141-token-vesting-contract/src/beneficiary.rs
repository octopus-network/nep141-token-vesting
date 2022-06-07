use crate::interfaces::BeneficiaryAction;
use crate::vesting::traits::{Beneficiary, Claimable, VestingTokenInfoTrait};
use crate::*;
use crate::{TokenVestingContract, Vesting, VestingId};

#[near_bindgen]
impl BeneficiaryAction for TokenVestingContract {
    fn change_beneficiary(&mut self, vesting_id: VestingId, new_beneficiary: AccountId) {
        match self.vestings.get(&vesting_id).expect("") {
            Vesting::NaturalTimeLinearVesting(mut vest) => {
                vest.set_beneficiary(new_beneficiary);
            }
            Vesting::TimeCliffVesting(mut vest) => {
                vest.set_beneficiary(new_beneficiary);
            }
        }
    }

    fn claim(&mut self, vesting_id: VestingId, amount: Option<U128>) -> Promise {
        let (claimable_amount, beneficiary, token_id) =
            self.internal_use_vesting(&vesting_id, |vesting| match vesting {
                Vesting::NaturalTimeLinearVesting(linear_vesting) => (
                    linear_vesting.claim(amount.map(|e| e.0)),
                    linear_vesting.beneficiary.clone(),
                    linear_vesting.vesting_token_info.token_id.clone(),
                ),
                Vesting::TimeCliffVesting(cliff_vesting) => (
                    cliff_vesting.claim(amount.map(|e| e.0)),
                    cliff_vesting.beneficiary.clone(),
                    cliff_vesting.vesting_token_info.token_id.clone(),
                ),
            });
        self.internal_send_tokens(&beneficiary, &token_id, claimable_amount)
    }
}
