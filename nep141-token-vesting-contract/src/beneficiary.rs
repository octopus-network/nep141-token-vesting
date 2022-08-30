use crate::events::{EventEmit, UserAction, VestingEvent};
use crate::external::*;
use crate::interfaces::{BeneficiaryAction, Viewer};
use crate::vesting::traits::{Beneficiary, Claimable, Finish};
use crate::*;
use crate::{TokenVestingContract, VestingId};
use near_contract_standards::fungible_token::core::ext_ft_core;
use near_contract_standards::storage_management::StorageBalance;
use near_sdk::env::current_account_id;
use near_sdk::{assert_one_yocto, ext_contract, PromiseOrValue};

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

    fn claim(&mut self, vesting_id: VestingId, amount: Option<U128>) -> PromiseOrValue<U128> {
        let vesting = self
            .internal_get_vesting(&vesting_id)
            .expect("No such vesting,id: #{}.");

        PromiseOrValue::Promise(
            ext_ft_core::ext(self.token_id.clone())
                .ft_balance_of(current_account_id())
                .and(
                    ext_storage_management::ext(self.token_id.clone())
                        .storage_balance_of(vesting.get_beneficiary()),
                )
                .then(Self::ext(env::current_account_id()).claim_callback(vesting_id, amount)),
        )
    }

    fn claim_all(&mut self, beneficiary: Option<AccountId>) -> PromiseOrValue<U128> {
        let beneficiary = beneficiary.unwrap_or(env::predecessor_account_id());

        PromiseOrValue::Promise(
            ext_ft_core::ext(self.token_id.clone())
                .ft_balance_of(current_account_id())
                .and(
                    ext_storage_management::ext(self.token_id.clone())
                        .storage_balance_of(beneficiary.clone()),
                )
                .then(Self::ext(env::current_account_id()).claim_all_callback(beneficiary)),
        )
    }
}

#[near_bindgen]
impl TokenVestingContract {
    #[private]
    pub fn claim_callback(
        &mut self,
        vesting_id: VestingId,
        amount: Option<U128>,
        #[callback_unwrap] ft_balance: U128,
        #[callback_unwrap] storage_balance: Option<StorageBalance>,
    ) -> U128 {
        assert!(
            storage_balance.is_some(),
            "Failed to claim because the beneficiary hasn't registered in vesting token contract."
        );

        let mut vesting = self
            .internal_get_vesting(&vesting_id)
            .expect(format!("Failed to claim, no such vesting id: #{}", vesting_id.0).as_str());
        let beneficiary = vesting.get_beneficiary();
        let claimable_amount = vesting.claim(amount.map(|e| e.0));

        assert!(
            ft_balance.0 >= claimable_amount,
            "Failed to claim because the contract balance is not enough."
        );

        if vesting.is_release_finish() {
            self.internal_remove_vesting(&vesting_id);
            VestingEvent::FinishVesting {
                vesting_id: &vesting_id,
            }
            .emit();
        } else {
            self.internal_save_vesting(&vesting);
        }

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
        U128(claimable_amount)
    }

    #[private]
    pub fn claim_all_callback(
        &mut self,
        beneficiary: AccountId,
        #[callback_unwrap] ft_balance: U128,
        #[callback_unwrap] storage_balance: Option<StorageBalance>,
    ) -> U128 {
        assert!(
            storage_balance.is_some(),
            "Failed to claim because the beneficiary hasn't registered in vesting token contract."
        );

        let mut amount: u128 = 0;
        let vestings = self
            .vestings
            .values()
            .filter(|e| e.get_beneficiary().eq(&beneficiary))
            .collect_vec();

        let mut claimed_vesting_ids: Vec<VestingId> = vec![];
        for mut vesting in vestings {
            let vesting_id = vesting.get_vesting_id();
            let claimable_amount = vesting.claim(Option::None);

            if claimable_amount == 0 {
                continue;
            }

            if vesting.is_release_finish() {
                self.internal_remove_vesting(&vesting_id);
                VestingEvent::FinishVesting {
                    vesting_id: &vesting_id,
                }
                .emit();
            } else {
                self.internal_save_vesting(&vesting)
            }

            VestingEvent::UpdateVesting { vesting: &vesting }.emit();

            amount += claimable_amount;
            claimed_vesting_ids.push(vesting_id);
        }

        if amount > 0 {

            assert!(
                ft_balance.0 >= amount,
                "Failed to claim because the contract balance is not enough."
            );

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
        U128(amount)
    }
}
