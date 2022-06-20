use crate::interfaces::OwnerAction;
use crate::types::VestingId;
use crate::vesting::Vesting;
use itertools::Itertools;
use near_sdk::borsh::{self, BorshDeserialize, BorshSerialize};
use near_sdk::collections::UnorderedMap;
use near_sdk::json_types::U128;
use near_sdk::serde::{Deserialize, Serialize};
use near_sdk::{
    env, log, near_bindgen, AccountId, Balance, BorshStorageKey, PanicOnDefault, Promise,
    StorageUsage,
};

mod beneficiary;
mod constants;
mod contract_viewers;
mod domain;
pub mod events;
mod fungible_token;
mod interfaces;
mod owner;
mod types;
mod utils;
mod vesting;

use crate::utils::*;

#[derive(BorshStorageKey, BorshSerialize)]
pub(crate) enum StorageKey {
    Vestings,
}

#[near_bindgen]
#[derive(BorshDeserialize, BorshSerialize, PanicOnDefault)]
pub struct TokenVestingContract {
    pub owner: AccountId,
    // pub account: LookupMap<AccountId, VAccount>,
    pub vestings: UnorderedMap<VestingId, Vesting>,
    pub vesting_id: u64,
}

#[near_bindgen]
impl TokenVestingContract {
    #[init]
    pub fn new(owner: AccountId) -> Self {
        Self {
            owner,
            vestings: UnorderedMap::new(StorageKey::Vestings),
            vesting_id: 0,
        }
    }
}

impl TokenVestingContract {
    /// Check how much storage taken costs and refund the left over back.
    fn internal_check_storage(&self, prev_storage: StorageUsage) {
        let storage_cost = env::storage_usage()
            .checked_sub(prev_storage)
            .unwrap_or_default() as Balance
            * env::storage_byte_cost();

        log!("storage cost {}", storage_cost);
        let refund = env::attached_deposit().checked_sub(storage_cost).expect(
            format!(
                "ERR_STORAGE_DEPOSIT need {}, attached {}",
                storage_cost,
                env::attached_deposit()
            )
            .as_str(),
        );
        if refund > 0 {
            Promise::new(env::predecessor_account_id()).transfer(refund);
        }
    }
}

#[cfg(not(target_arch = "wasm32"))]
#[cfg(test)]
pub mod test {
    use near_sdk::AccountId;

    pub fn usdt() -> AccountId {
        AccountId::new_unchecked("usdt".to_string())
    }

    pub fn usdc() -> AccountId {
        AccountId::new_unchecked("usdc".to_string())
    }
}
