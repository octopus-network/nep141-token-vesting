use crate::constants::{T_GAS_FOR_FT_TRANSFER, T_GAS_FOR_RESOLVE_TRANSFER};
use crate::*;
use near_contract_standards::fungible_token::core::ext_ft_core;
use near_contract_standards::fungible_token::receiver::FungibleTokenReceiver;
use near_sdk::json_types::U128;
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
    ) -> Promise {
        assert!(amount > 0, "Failed to send tokens because amount is 0.");
        ext_ft_core::ext(token_id.clone())
            .with_attached_deposit(ONE_YOCTO)
            .with_static_gas(Gas::ONE_TERA.mul(T_GAS_FOR_FT_TRANSFER))
            .ft_transfer(receiver_id.clone(), U128(amount), None)
            .then(
                Self::ext(env::current_account_id())
                    .with_static_gas(Gas::ONE_TERA.mul(T_GAS_FOR_RESOLVE_TRANSFER))
                    .ft_transfer_resolved(token_id.clone(), receiver_id.clone(), U128(amount)),
            )
    }

    #[private]
    pub fn ft_transfer_resolved(
        &mut self,
        token_id: AccountId,
        sender_id: AccountId,
        amount: U128,
    ) {
        assert_eq!(
            env::promise_results_count(),
            1,
            "Expect 1 promise result for sending token."
        );
        match env::promise_result(0) {
            PromiseResult::NotReady => unreachable!(),
            PromiseResult::Successful(_) => {}
            PromiseResult::Failed => {
                // should log
                todo!()
            }
        }
    }
}
