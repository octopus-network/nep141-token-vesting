use near_contract_standards::upgrade::Ownable;
use near_sdk::json_types::U64;
use near_sdk::{AccountId, Balance};
use std::cmp::min;

use crate::events::{EventEmit, UserAction, VestingEvent};
use crate::types::{SecondTimeStamp, U256};
use crate::utils::get_block_second_time;
use crate::vesting::cliff::{CliffVestingCheckpoint, TimeCliffVesting};
use crate::vesting::linear::NaturalTimeLinearVesting;
use crate::vesting::traits::{
    Beneficiary, Claimable, Finish, Frozen, NaturalTime, VestingAmount, VestingTokenInfoTrait,
};
use crate::*;

pub mod cliff;
pub mod linear;
pub mod traits;

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
    },
    CliffVesting {
        beneficiary: AccountId,
        time_cliff_list: Vec<CliffVestingCheckpoint>,
    },
}

#[derive(BorshSerialize, BorshDeserialize, Debug, Serialize, Deserialize, Clone)]
#[serde(crate = "near_sdk::serde")]
pub struct VestingTokenInfo {
    #[serde(default)]
    #[serde(with = "u128_dec_format")]
    pub claimed_token_amount: Balance,
    #[serde(default)]
    #[serde(with = "u128_dec_format")]
    pub total_vesting_amount: Balance,
}

impl<T: NaturalTime + VestingTokenInfoTrait> VestingAmount for T {
    fn get_unreleased_amount(&self) -> Balance {
        let period = self.get_period();
        let mut remain_time = if self.get_end_time() <= get_block_second_time() {
            0
        } else {
            self.get_end_time() - get_block_second_time()
        };
        remain_time = min(remain_time, period);
        // unreleased_amount / remain_time = total_vesting / period
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
        let claimable_amount = self.get_claimable_amount();
        if amount.is_some() {
            assert!(
                amount.unwrap() <= claimable_amount,
                "claimable amount is less than claim amount."
            );
        }

        self.set_claimed_token_amount(
            self.get_vesting_token_info().claimed_token_amount + amount.unwrap_or(claimable_amount),
        );
        claimable_amount
    }
}

impl Frozen for Vesting {
    fn freeze(&mut self) {
        match self {
            Vesting::NaturalTimeLinearVesting(linear) => linear.freeze(),
            Vesting::TimeCliffVesting(cliff) => cliff.freeze(),
        }
    }

    fn unfreeze(&mut self) {
        match self {
            Vesting::NaturalTimeLinearVesting(linear) => linear.unfreeze(),
            Vesting::TimeCliffVesting(cliff) => cliff.unfreeze(),
        }
    }

    fn is_frozen(&self) -> bool {
        match self {
            Vesting::NaturalTimeLinearVesting(linear) => linear.is_frozen,
            Vesting::TimeCliffVesting(cliff) => cliff.is_frozen,
        }
    }
}

impl VestingTokenInfoTrait for Vesting {
    fn get_vesting_token_info(&self) -> &VestingTokenInfo {
        match self {
            Vesting::NaturalTimeLinearVesting(linear) => linear.get_vesting_token_info(),
            Vesting::TimeCliffVesting(cliff) => cliff.get_vesting_token_info(),
        }
    }

    fn set_claimed_token_amount(&mut self, amount: Balance) {
        match self {
            Vesting::NaturalTimeLinearVesting(linear) => linear.set_claimed_token_amount(amount),
            Vesting::TimeCliffVesting(cliff) => cliff.set_claimed_token_amount(amount),
        }
    }
}

impl Beneficiary for Vesting {
    fn get_beneficiary(&self) -> AccountId {
        match self {
            Vesting::NaturalTimeLinearVesting(linear) => linear.get_beneficiary(),
            Vesting::TimeCliffVesting(cliff) => cliff.get_beneficiary(),
        }
    }

    fn set_beneficiary(&mut self, account: AccountId) {
        match self {
            Vesting::NaturalTimeLinearVesting(linear) => linear.set_beneficiary(account),
            Vesting::TimeCliffVesting(cliff) => cliff.set_beneficiary(account),
        }
    }
}

impl VestingAmount for Vesting {
    fn get_unreleased_amount(&self) -> Balance {
        match self {
            Vesting::NaturalTimeLinearVesting(linear) => linear.get_unreleased_amount(),
            Vesting::TimeCliffVesting(cliff) => cliff.get_unreleased_amount(),
        }
    }
}

impl Finish for Vesting {
    fn is_release_finish(&self) -> bool {
        match self {
            Vesting::NaturalTimeLinearVesting(linear) => linear.is_release_finish(),
            Vesting::TimeCliffVesting(cliff) => cliff.is_release_finish(),
        }
    }
}

impl Vesting {
    pub fn get_vesting_id(&self) -> VestingId {
        match self {
            Vesting::NaturalTimeLinearVesting(linear) => linear.id,
            Vesting::TimeCliffVesting(cliff) => cliff.id,
        }
    }

    pub fn new(id: VestingId, param: VestingCreateParam) -> Self {
        match param {
            VestingCreateParam::LinearVesting {
                beneficiary,
                start_time,
                end_time,
                total_vesting_amount,
            } => {
                assert!(start_time<end_time, "End time should be less than start time when creating NaturalTimeLinearVesting.");

                Vesting::NaturalTimeLinearVesting(NaturalTimeLinearVesting {
                    id,
                    beneficiary,
                    start_time,
                    end_time,
                    vesting_token_info: VestingTokenInfo {
                        claimed_token_amount: 0,
                        total_vesting_amount,
                    },
                    is_frozen: false,
                    create_time: get_block_second_time(),
                })
            }
            VestingCreateParam::CliffVesting {
                beneficiary,
                time_cliff_list,
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
                    id,
                    beneficiary,
                    time_cliff_list,
                    vesting_token_info: VestingTokenInfo {
                        claimed_token_amount: 0,
                        total_vesting_amount: total_amount,
                    },
                    is_frozen: false,
                    create_time: get_block_second_time(),
                })
            }
        }
    }
}

impl TokenVestingContract {
    pub(crate) fn internal_create_vesting(&mut self, param: VestingCreateParam) -> VestingId {
        self.assert_owner();
        let id = self.internal_assign_id();
        let prev_storage = env::storage_usage();

        self.vestings.insert(&id, &Vesting::new(id.clone(), param));
        self.internal_check_storage(prev_storage);
        VestingEvent::CreateVesting {
            vesting: &self.internal_get_vesting(&id).unwrap(),
            token_id: &self.token_id.clone(),
        }
        .emit();
        UserAction::CreateVesting { vesting_id: &id }.emit();
        id
    }

    pub(crate) fn internal_assign_id(&mut self) -> U64 {
        self.uuid += 1;
        return U64(self.uuid);
    }

    pub(crate) fn internal_remove_vesting(&mut self, vesting_id: &VestingId) {
        self.vestings.remove(&vesting_id);
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
        self.internal_save_vesting(&vesting);
        r
    }

    pub(crate) fn internal_save_vesting(&mut self, vesting: &Vesting) {
        self.vestings.insert(&vesting.get_vesting_id(), &vesting);
    }
}
