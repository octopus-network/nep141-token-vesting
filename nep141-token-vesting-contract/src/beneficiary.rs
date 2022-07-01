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
                .expect(format!("Failed to get vesting by id:  {}.", &vesting_id.0).as_str()),
        }
        .emit();

        UserAction::ChangeBeneficiary {
            vesting_id: &vesting_id,
            old_beneficiary: &old_beneficiary,
            new_beneficiary: &vesting.get_beneficiary(),
        }
        .emit();
    }

    fn claim(&mut self, vesting_id: VestingId, amount: Option<U128>) -> Balance {
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
        VestingEvent::UpdateVesting { vesting: &vesting }.emit();
        let transfer_id = self.internal_assign_id();
        UserAction::Claim {
            transfer_id: &transfer_id,
            vesting_id: &vesting_id,
            beneficiary: &beneficiary,
            token_id: &self.token_id,
            amount: &U128(claimable_amount),
        }
        .emit();
        self.internal_send_tokens(
            &beneficiary,
            &self.token_id.clone(),
            claimable_amount,
            Some(transfer_id),
        );
        claimable_amount
    }

    fn claim_all(&mut self, beneficiary: Option<AccountId>) -> Balance {
        let beneficiary = beneficiary.unwrap_or(env::predecessor_account_id());

        let mut amount: u128 = 0;
        let vestings = self
            .vestings
            .values()
            .filter(|e| e.get_beneficiary().eq(&beneficiary))
            .collect_vec();

        let mut claimed_vesting_ids: Vec<VestingId> = vec![];
        for mut vesting in vestings {
            // for vesting in self.vestings.values().filter(|e|e.get_beneficiary().eq(&beneficiary)) {
            let vesting_id = vesting.get_vesting_id();
            let claimable_amount = vesting.claim(Option::None);

            if claimable_amount == 0 {
                continue;
            }

            if vesting.is_release_finish() {
                self.internal_remove_vesting(&vesting_id);
            } else {
                self.internal_save_vesting(&vesting)
            }

            VestingEvent::UpdateVesting { vesting: &vesting }.emit();

            amount += claimable_amount;
            claimed_vesting_ids.push(vesting_id);
        }

        if amount != 0 {
            let transfer_id = self.internal_assign_id();
            UserAction::ClaimAll {
                transfer_id: &transfer_id,
                vesting_ids: &claimed_vesting_ids,
                beneficiary: &beneficiary,
                token_id: &self.token_id.clone(),
                amount: &U128(amount),
            }
            .emit();

            self.internal_send_tokens(
                &beneficiary,
                &self.token_id.clone(),
                amount,
                Some(transfer_id),
            );
        }
        amount
    }

    fn withdraw_legacy(&mut self, account_id: Option<AccountId>) {
        let account_id = account_id.unwrap_or(env::predecessor_account_id());
        let balance = self.legacy.get(&account_id).unwrap_or(0);
        let token_id = self.token_id.clone();
        assert!(
            balance > 0,
            "Failed withdraw_legacy, the balance should more than 0."
        );

        let transfer_id = self.internal_assign_id();
        UserAction::WithdrawLegacy {
            account_id: &account_id,
            token_id: &token_id,
            amount: &U128(balance),
            transfer_id: &transfer_id,
        }
        .emit();
        self.internal_send_tokens(&account_id, &token_id, balance, Some(transfer_id));
    }
}
