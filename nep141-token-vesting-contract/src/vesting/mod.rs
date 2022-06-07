use near_contract_standards::upgrade::Ownable;
use near_sdk::{AccountId, Balance, BorshStorageKey};
use std::cmp::min;

use super::*;
use crate::types::{SecondTimeStamp, U256};
use crate::utils::get_block_second_time;
use crate::vesting::cliff::{CliffVestingCheckpoint, TimeCliffVesting};
use crate::vesting::linear::NaturalTimeLinearVesting;
use crate::vesting::traits::{
    Claimable, Frozen, NaturalTime, VestingAmount, VestingTokenInfoTrait,
};
use crate::vesting::VestingCreateParam::LinearVesting;

pub mod cliff;
pub mod linear;
pub mod traits;

impl TokenVestingContract {
    pub(crate) fn internal_create_vesting(&mut self, param: VestingCreateParam) -> VestingId {
        self.assert_owner();
        let prev_storage = env::storage_usage();
        let id = self.internal_assign_pool_id();
        self.vestings.insert(&id, &Vesting::new(param));
        self.internal_check_storage(prev_storage);
        id
    }

    pub(crate) fn internal_assign_pool_id(&mut self) -> VestingId {
        self.vesting_id += 1;
        return self.vesting_id;
    }

    pub(crate) fn internal_get_vesting(&self, vesting_id: &VestingId) -> Option<Vesting> {
        self.vestings.get(vesting_id)
    }

    pub(crate) fn internal_use_vesting<F, R>(&mut self, vesting_id: &VestingId, mut f: F) -> R
    where
        F: FnMut(&mut Vesting) -> R,
    {
        let mut vesting = self
            .internal_get_vesting(&vesting_id)
            .expect("No such vesting");
        let r = f(&mut vesting);
        self.internal_save_vesting(vesting_id, &vesting);
        r
    }

    pub(crate) fn internal_save_vesting(&mut self, vesting_id: &VestingId, vesting: &Vesting) {
        self.vestings.insert(&vesting_id, &vesting);
    }
}

#[derive(BorshSerialize, BorshDeserialize, Debug, Serialize, Deserialize, Clone)]
#[serde(crate = "near_sdk::serde")]
#[serde(tag = "vesting_type", content = "data")]
#[serde(rename_all = "snake_case")]
pub enum Vesting {
    NaturalTimeLinearVesting(NaturalTimeLinearVesting),
    TimeCliffVesting(TimeCliffVesting),
}

#[derive(BorshSerialize, BorshDeserialize, Debug, Serialize, Deserialize, Clone)]
#[serde(crate = "near_sdk::serde")]
pub enum VestingCreateParam {
    LinearVesting {
        beneficiary: AccountId,
        start_time: SecondTimeStamp,
        end_time: SecondTimeStamp,
        total_vesting_amount: Balance,
        token_id: AccountId,
    },
    CliffVesting {
        beneficiary: AccountId,
        time_cliff_list: Vec<CliffVestingCheckpoint>,
        token_id: AccountId,
    },
}

impl Vesting {
    pub fn new(param: VestingCreateParam) -> Self {
        match param {
            VestingCreateParam::LinearVesting {
                beneficiary,
                start_time,
                end_time,
                total_vesting_amount,
                token_id,
            } => Vesting::NaturalTimeLinearVesting(NaturalTimeLinearVesting {
                beneficiary,
                start_time,
                end_time,
                vesting_token_info: VestingTokenInfo {
                    token_id,
                    claimed_token_amount: 0,
                    total_vesting_amount,
                },
                is_frozen: false,
            }),
            VestingCreateParam::CliffVesting {
                beneficiary,
                time_cliff_list,
                token_id,
            } => {
                let total_amount = time_cliff_list
                    .iter()
                    .map(|e| e.amount)
                    .reduce(|acc, item| {
                        acc.checked_add(item)
                            .expect("accumulation of cliff amount is overflow.")
                    })
                    .unwrap_or(0);
                Vesting::TimeCliffVesting(TimeCliffVesting {
                    beneficiary,
                    time_cliff_list,
                    vesting_token_info: VestingTokenInfo {
                        token_id,
                        claimed_token_amount: 0,
                        total_vesting_amount: total_amount,
                    },
                    is_frozen: false,
                })
            }
        }
    }
}

#[derive(BorshSerialize, BorshDeserialize, Debug, Serialize, Deserialize, Clone)]
#[serde(crate = "near_sdk::serde")]
pub struct VestingTokenInfo {
    pub token_id: AccountId,
    pub claimed_token_amount: Balance,
    pub total_vesting_amount: Balance,
}

impl<T: NaturalTime + VestingTokenInfoTrait> VestingAmount for T {
    fn get_unreleased_amount(&self) -> Balance {
        let period = self.get_period();
        let mut remain_time = if self.get_end_time() <= get_block_second_time() {
            0
        } else {
            get_block_second_time() - self.get_end_time()
        };
        remain_time = min(remain_time, period);
        let unreleased_amount = U256::from(self.get_vesting_token_info().total_vesting_amount)
            * U256::from(remain_time)
            / U256::from(period);
        unreleased_amount.as_u128()
    }
}

impl<T: VestingAmount + VestingTokenInfoTrait + Frozen> Claimable for T {
    fn claim(&mut self, amount: Option<Balance>) -> Balance {
        assert!(
            !self.is_frozen(),
            "Failed to claim because this vesting is frozen."
        );
        let claimable_amount = amount.unwrap_or(self.get_claimable_amount());
        self.set_claimed_token_amount(
            self.get_vesting_token_info().claimed_token_amount + claimable_amount,
        );
        claimable_amount
    }
}
