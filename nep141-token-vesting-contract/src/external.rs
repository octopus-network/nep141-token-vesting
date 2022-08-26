use near_contract_standards::storage_management::{StorageBalance, StorageBalanceBounds};
use near_sdk::ext_contract;
use near_sdk::AccountId;

#[ext_contract(ext_storage_management)]
pub trait StorageManagement {
    fn storage_balance_of(&self, account_id: AccountId) -> Option<StorageBalance>;
}
