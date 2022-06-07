use crate::types::SecondTimeStamp;
use crate::vesting::cliff::CliffVestingCheckpoint;
use crate::vesting::VestingCreateParam;
use crate::{Vesting, VestingId};
use near_sdk::json_types::{U128, U64};
use near_sdk::{AccountId, Balance, Promise};

pub trait Viewer {
    fn get_vesting(&self, from_index: u32, limit: u32) -> Vec<Vesting>;
}

pub trait OwnerAction {
    fn create_linear_vesting(
        &mut self,
        beneficiary: AccountId,
        start_time: U64,
        end_time: U64,
        total_vesting_amount: U128,
        token_id: AccountId,
    ) -> VestingId;

    fn create_cliff_vesting(
        &mut self,
        beneficiary: AccountId,
        time_cliff_list: Vec<CliffVestingCheckpoint>,
        token_id: AccountId,
    ) -> VestingId;

    fn freeze_vesting(&mut self, vesting_id: VestingId);

    fn unfreeze_vesting(&mut self, vesting_id: VestingId);

    fn terminate_vesting(&mut self, vesting_id: VestingId);
}

pub trait BeneficiaryAction {
    fn change_beneficiary(&mut self, vesting_id: VestingId, new_beneficiary: AccountId);

    fn claim(&mut self, vesting_id: VestingId, amount: Option<U128>) -> Promise;
}
