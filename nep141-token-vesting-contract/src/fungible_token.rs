use crate::constants::{T_GAS_FOR_FT_TRANSFER, T_GAS_FOR_RESOLVE_TRANSFER};
use crate::events::{EventEmit, UserAction, VestingEvent};
use crate::vesting::traits::{Finish, VestingTokenInfoTrait};
use crate::*;
use near_contract_standards::fungible_token::core::ext_ft_core;
use near_contract_standards::fungible_token::receiver::FungibleTokenReceiver;
use near_sdk::json_types::{U128, U64};
use near_sdk::{Gas, PromiseResult, ONE_YOCTO};
use std::ops::Mul;

#[near_bindgen]
impl TokenVestingContract {
    pub(crate) fn internal_send_near(&self, receiver_id: AccountId, amount: Balance) -> Promise {
        Promise::new(receiver_id).transfer(amount)
    }

    pub(crate) fn internal_send_tokens(
        &mut self,
        receiver_id: &AccountId,
        token_id: &AccountId,
        amount: Balance,
        vesting_id: Option<VestingId>,
    ) -> Promise {
        assert!(amount > 0, "Failed to send tokens because amount is 0.");
        ext_ft_core::ext(token_id.clone())
            .with_attached_deposit(ONE_YOCTO)
            .with_static_gas(Gas::ONE_TERA.mul(T_GAS_FOR_FT_TRANSFER))
            .ft_transfer(receiver_id.clone(), U128(amount), None)
            .then(
                Self::ext(env::current_account_id())
                    .with_static_gas(Gas::ONE_TERA.mul(T_GAS_FOR_RESOLVE_TRANSFER))
                    .ft_transfer_resolved(
                        token_id.clone(),
                        receiver_id.clone(),
                        U128(amount),
                        vesting_id,
                    ),
            )
    }

    #[private]
    pub fn ft_transfer_resolved(
        &mut self,
        token_id: AccountId,
        sender_id: AccountId,
        amount: U128,
        vesting_id: Option<VestingId>,
    ) {
        assert_eq!(
            env::promise_results_count(),
            1,
            "Expect 1 promise result for sending token."
        );
        log!(
            "ft_transfer_resolved, tokenKid: {}, sender_id: {}, amount: {}",
            token_id,
            sender_id,
            amount.0
        );
        match env::promise_result(0) {
            PromiseResult::NotReady => unreachable!(),
            PromiseResult::Successful(_) => {
                if vesting_id.is_some() {
                    let vesting_id = vesting_id.unwrap();
                    let vesting = self
                        .internal_get_vesting(&vesting_id)
                        .expect(format!("Failed to get vesting,id is {}", vesting_id.0).as_str());
                    VestingEvent::UpdateVesting { vesting: &vesting }.emit();
                    match vesting {
                        Vesting::NaturalTimeLinearVesting(linear) => {
                            if linear.is_vesting_finish() {
                                self.internal_remove_vesting(&vesting_id);
                            }
                        }
                        Vesting::TimeCliffVesting(cliff) => {
                            if cliff.is_vesting_finish() {
                                self.internal_remove_vesting(&vesting_id);
                            }
                        }
                    }
                }
            }
            PromiseResult::Failed => {
                if vesting_id.is_some() {
                    let vesting_id = vesting_id.unwrap();
                    self.internal_use_vesting(&vesting_id, |vesting| {
                        vesting.set_claimed_token_amount(
                            vesting.get_vesting_token_info().claimed_token_amount - amount.0,
                        )
                    });
                    VestingEvent::UpdateVesting {
                        vesting: &self.internal_get_vesting(&vesting_id).expect(
                            format!("Failed to get vesting by {}.", &vesting_id.0).as_str(),
                        ),
                    }
                    .emit();
                    UserAction::Refund {
                        vesting_id: &vesting_id,
                        token_id: &token_id,
                        amount: &amount,
                    }
                    .emit();
                }
            }
        }
    }
}
