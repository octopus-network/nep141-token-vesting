use crate::events::{EventEmit, UserAction, VestingEvent};
use crate::vesting::cliff::CliffVestingCheckpoint;
use crate::vesting::traits::Frozen;
use crate::vesting::VestingCreateParam;
use crate::*;
use crate::{OwnerAction, TokenVestingContract, VestingId};
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
        self.internal_use_vesting(&vesting_id, |vesting| vesting.freeze());
        VestingEvent::UpdateVesting {
            vesting: &self
                .internal_get_vesting(&vesting_id)
                .expect(format!("Failed to get vesting by {}.", vesting_id.0).as_str()),
        }
        .emit();
        UserAction::FreezeVesting {
            vesting_id: &vesting_id,
        }
        .emit();
    }

    fn unfreeze_vesting(&mut self, vesting_id: VestingId) {
        self.assert_owner();

        self.internal_use_vesting(&vesting_id, |vesting| vesting.unfreeze());
        VestingEvent::UpdateVesting {
            vesting: &self
                .internal_get_vesting(&vesting_id)
                .expect(format!("Failed to get vesting by {}.", vesting_id.0).as_str()),
        }
        .emit();
        UserAction::UnfreezeVesting {
            vesting_id: &vesting_id,
        }
        .emit();
    }

    fn terminate_vesting(&mut self, vesting_id: VestingId) {
        self.assert_owner();

        self.vestings.remove(&vesting_id);

        UserAction::TerminateVesting {
            vesting_id: &vesting_id,
        }
        .emit();
    }
}
