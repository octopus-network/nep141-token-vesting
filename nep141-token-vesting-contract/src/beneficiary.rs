use crate::events::{EventEmit, UserAction, VestingEvent};
use crate::interfaces::BeneficiaryAction;
use crate::vesting::traits::{Beneficiary, Claimable, Finish};
use crate::*;
use crate::{TokenVestingContract, VestingId};

#[near_bindgen]
impl BeneficiaryAction for TokenVestingContract {
    #[payable]
    fn change_beneficiary(&mut self, vesting_id: VestingId, new_beneficiary: AccountId) {
        let prev_storage = env::storage_usage();

        let mut vesting = self
            .internal_get_vesting(&vesting_id)
            .expect("No such vesting.");
        assert!(
            env::predecessor_account_id().eq(&vesting.get_beneficiary())
                || env::predecessor_account_id().eq(&self.owner),
            "Only owner and vesting beneficiary can set a new beneficiary."
        );
        let old_beneficiary = vesting.get_beneficiary();

        self.internal_register_legacy(&new_beneficiary);

        vesting.set_beneficiary(new_beneficiary);

        self.internal_save_vesting(&vesting);

        self.internal_check_storage(prev_storage);

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
        let mut vesting = self
            .internal_get_vesting(&vesting_id)
            .expect(format!("Failed to claim, no such vesting id: #{}", vesting_id.0).as_str());
        let beneficiary = vesting.get_beneficiary();
        let claimable_amount = vesting.claim(amount.map(|e| e.0));

        if vesting.is_release_finish() {
            self.internal_remove_vesting(&vesting_id);
        } else {
            self.internal_save_vesting(&vesting);
        }

        // let (claimable_amount, beneficiary) = self.internal_use_vesting(&vesting_id, |vesting| {
        //     (
        //         vesting.claim(amount.map(|e| e.0)),
        //         vesting.get_beneficiary(),
        //     )
        // });
        VestingEvent::UpdateVesting {
            vesting: &self
                .internal_get_vesting(&vesting_id)
                .expect(format!("Failed to get vesting by {}.", &vesting_id.0).as_str()),
        }
        .emit();
        UserAction::Claim {
            vesting_id: &vesting_id,
            beneficiary: &beneficiary,
            token_id: &self.token_id,
            amount: &U128(claimable_amount),
        }
        .emit();
        self.internal_send_tokens(&beneficiary, &self.token_id.clone(), claimable_amount)
    }

    fn claim_all(&mut self, beneficiary: AccountId) -> Promise {
        let mut amount: u128 = 0;
        let vesting_token_id = self.token_id.clone();
        let vestings = self
            .vestings
            .values()
            .filter(|e| e.get_beneficiary().eq(&beneficiary))
            .collect_vec();

        for mut vesting in vestings {
            // for vesting in self.vestings.values().filter(|e|e.get_beneficiary().eq(&beneficiary)) {
            let vesting_id = vesting.get_vesting_id();
            let claimable_amount = vesting.claim(Option::None);
            if vesting.is_release_finish() {
                self.internal_remove_vesting(&vesting_id);
            } else {
                self.internal_save_vesting(&vesting)
            }

            VestingEvent::UpdateVesting {
                vesting: &self
                    .internal_get_vesting(&vesting_id)
                    .expect(format!("Failed to get vesting by {}.", &vesting_id.0).as_str()),
            }
            .emit();
            UserAction::Claim {
                vesting_id: &vesting_id,
                beneficiary: &beneficiary,
                token_id: &vesting_token_id,
                amount: &U128(claimable_amount),
            }
            .emit();

            amount += claimable_amount;
        }

        self.internal_send_tokens(&beneficiary, &vesting_token_id, amount);

        todo!()
    }
}
