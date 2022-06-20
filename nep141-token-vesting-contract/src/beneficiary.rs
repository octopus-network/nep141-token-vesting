use crate::events::{EventEmit, UserAction, VestingEvent};
use crate::interfaces::BeneficiaryAction;
use crate::vesting::traits::{Beneficiary, Claimable, VestingTokenInfoTrait};
use crate::*;
use crate::{TokenVestingContract, VestingId};

#[near_bindgen]
impl BeneficiaryAction for TokenVestingContract {
    fn change_beneficiary(&mut self, vesting_id: VestingId, new_beneficiary: AccountId) {
        let mut vesting = self
            .internal_get_vesting(&vesting_id)
            .expect("No such vesting.");
        assert!(
            env::predecessor_account_id().eq(&vesting.get_beneficiary())
                || env::predecessor_account_id().eq(&self.owner),
            "Only owner and vesting beneficiary can set a new beneficiary."
        );
        let old_beneficiary = vesting.get_beneficiary();
        vesting.set_beneficiary(new_beneficiary);
        self.internal_save_vesting(&vesting_id, &vesting);

        VestingEvent::UpdateVesting {
            vesting: &self
                .internal_get_vesting(&vesting_id)
                .expect(format!("Failed to get vesting by {}.", &vesting_id.0).as_str()),
        }
        .emit();

        UserAction::ChangeBeneficiary {
            vesting_id: &vesting_id,
            old_beneficiary: &old_beneficiary,
            new_beneficiary: &vesting.get_beneficiary(),
        }
        .emit();
    }

    fn claim(&mut self, vesting_id: VestingId, amount: Option<U128>) -> Promise {
        let (claimable_amount, beneficiary, token_id) =
            self.internal_use_vesting(&vesting_id, |vesting| {
                (
                    vesting.claim(amount.map(|e| e.0)),
                    vesting.get_beneficiary(),
                    vesting.get_vesting_token_info().token_id.clone(),
                )
            });
        VestingEvent::UpdateVesting {
            vesting: &self
                .internal_get_vesting(&vesting_id)
                .expect(format!("Failed to get vesting by {}.", &vesting_id.0).as_str()),
        }
        .emit();
        UserAction::Claim {
            vesting_id: &vesting_id,
            beneficiary: &beneficiary,
            token_id: &token_id,
            amount: &U128(claimable_amount),
        }
        .emit();
        self.internal_send_tokens(&beneficiary, &token_id, claimable_amount, Some(vesting_id))
    }
}
