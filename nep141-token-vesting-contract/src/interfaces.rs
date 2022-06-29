use crate::vesting::cliff::CliffVestingCheckpoint;
use crate::{Vesting, VestingId};
use near_sdk::json_types::{U128, U64};
use near_sdk::{AccountId, Promise};

pub trait Viewer {
    fn get_vesting_token_id(&self) -> AccountId;

    fn get_vesting(
        &self,
        from_index: u32,
        limit: u32,
        beneficiary: Option<AccountId>,
    ) -> Vec<Vesting>;

    fn get_claimable_amount(&self, vesting_id: VestingId) -> U128;

    fn get_all_claimable_amount(&self, beneficiary: AccountId) -> U128;

    fn get_legacy(&self, account_id: AccountId) -> U128;
}

pub trait OwnerAction {
    fn create_linear_vesting(
        &mut self,
        beneficiary: AccountId,
        start_time: U64,
        end_time: U64,
        total_vesting_amount: U128,
    ) -> VestingId;

    fn create_cliff_vesting(
        &mut self,
        beneficiary: AccountId,
        time_cliff_list: Vec<CliffVestingCheckpoint>,
    ) -> VestingId;

    fn freeze_vesting(&mut self, vesting_id: VestingId);

    fn unfreeze_vesting(&mut self, vesting_id: VestingId);

    fn terminate_vesting(&mut self, vesting_id: VestingId);
}

pub trait BeneficiaryAction {
    fn change_beneficiary(&mut self, vesting_id: VestingId, new_beneficiary: AccountId);

    fn claim(&mut self, vesting_id: VestingId, amount: Option<U128>) -> Promise;

    fn claim_all(&mut self, beneficiary: AccountId) -> Promise;
}
