use crate::types::SecondTimeStamp;
use crate::vesting::cliff::CliffVestingCheckpoint;
use crate::vesting::traits::Frozen;
use crate::vesting::VestingCreateParam;
use crate::*;
use crate::{OwnerAction, TokenVestingContract, Vesting, VestingId};
use near_contract_standards::upgrade::Ownable;
use near_sdk::json_types::U64;

#[near_bindgen]
impl Ownable for TokenVestingContract {
    fn get_owner(&self) -> AccountId {
        self.owner.clone()
    }

    fn set_owner(&mut self, owner: AccountId) {
        self.assert_owner();
        self.owner = owner;
    }
}

#[near_bindgen]
impl OwnerAction for TokenVestingContract {
    #[payable]
    fn create_linear_vesting(
        &mut self,
        beneficiary: AccountId,
        start_time: U64,
        end_time: U64,
        total_vesting_amount: U128,
        token_id: AccountId,
    ) -> VestingId {
        self.internal_create_vesting(VestingCreateParam::LinearVesting {
            beneficiary,
            start_time: start_time.0,
            end_time: end_time.0,
            total_vesting_amount: total_vesting_amount.0,
            token_id,
        })
    }

    #[payable]
    fn create_cliff_vesting(
        &mut self,
        beneficiary: AccountId,
        time_cliff_list: Vec<CliffVestingCheckpoint>,
        token_id: AccountId,
    ) -> VestingId {
        self.internal_create_vesting(VestingCreateParam::CliffVesting {
            beneficiary,
            time_cliff_list,
            token_id,
        })
    }

    fn freeze_vesting(&mut self, vesting_id: VestingId) {
        self.assert_owner();
        self.internal_use_vesting(&vesting_id, |vesting| match vesting {
            Vesting::NaturalTimeLinearVesting(linear_vesting) => {
                linear_vesting.freeze();
            }
            Vesting::TimeCliffVesting(cliff_vesting) => {
                cliff_vesting.freeze();
            }
        });
    }

    fn unfreeze_vesting(&mut self, vesting_id: VestingId) {
        self.assert_owner();
        self.internal_use_vesting(&vesting_id, |vesting| match vesting {
            Vesting::NaturalTimeLinearVesting(linear_vesting) => {
                linear_vesting.unfreeze();
            }
            Vesting::TimeCliffVesting(cliff_vesting) => {
                cliff_vesting.unfreeze();
            }
        });
    }

    fn terminate_vesting(&mut self, vesting_id: VestingId) {
        self.assert_owner();

        // todo , need log
        self.vestings.remove(&vesting_id);
    }
}
