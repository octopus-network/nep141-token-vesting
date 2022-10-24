use crate::constants::{T_GAS_FOR_FT_TRANSFER, T_GAS_FOR_RESOLVE_TRANSFER};
use crate::events::{ActionStatus, EventEmit};
use crate::types::TransferId;
use crate::*;
use near_contract_standards::fungible_token::core::ext_ft_core;
use near_sdk::json_types::U128;
use near_sdk::{Gas, PromiseResult, ONE_YOCTO};
use std::ops::Mul;

#[near_bindgen]
impl TokenVestingContract {
    pub(crate) fn internal_send_tokens(
        &mut self,
        receiver_id: &AccountId,
        token_id: &AccountId,
        amount: Balance,
        transfer_id: TransferId,
    ) {
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
                        transfer_id,
                    ),
            );
    }

    #[private]
    pub fn ft_transfer_resolved(
        &mut self,
        token_id: AccountId,
        receiver_id: AccountId,
        amount: U128,
        transfer_id: TransferId,
    ) {
        assert_eq!(
            env::promise_results_count(),
            1,
            "Expect 1 promise result for ft_transfer_resolved."
        );
        log!(
            "ft_transfer_resolved, token_id: {}, receiver_id: {}, amount: {}",
            token_id,
            receiver_id,
            amount.0
        );
        match env::promise_result(0) {
            PromiseResult::NotReady => unreachable!(),
            PromiseResult::Successful(_) => ActionStatus::FtTransferResult {
                transfer_id: &transfer_id,
                is_success: &true,
            }
            .emit(),
            PromiseResult::Failed => {
                ActionStatus::FtTransferResult {
                    transfer_id: &transfer_id,
                    is_success: &false,
                }
                .emit();

                UserAction::Legacy {
                    account_id: &receiver_id,
                    token_id: &self.token_id,
                    amount: &amount,
                }
                .emit();
            }
        }
    }
}
