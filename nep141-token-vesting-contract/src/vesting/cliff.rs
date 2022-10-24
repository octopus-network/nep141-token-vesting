use super::*;
use crate::types::SecondTimeStamp;
use crate::utils::get_block_second_time;
use crate::vesting::traits::{Beneficiary, Finish, VestingAmount, VestingTokenInfoTrait};
use crate::vesting::VestingTokenInfo;
use near_sdk::{AccountId, Balance};

#[derive(BorshSerialize, BorshDeserialize, Debug, Serialize, Deserialize, Clone)]
#[serde(crate = "near_sdk::serde")]
pub struct TimeCliffVesting {
    pub id: VestingId,
    pub beneficiary: AccountId,
    pub time_cliff_list: Vec<CliffVestingCheckpoint>,
    pub vesting_token_info: VestingTokenInfo,
    pub is_frozen: bool,
    #[serde(default)]
    #[serde(with = "u64_dec_format")]
    pub create_time: SecondTimeStamp,
}

#[derive(BorshSerialize, BorshDeserialize, Debug, Serialize, Deserialize, Clone)]
#[serde(crate = "near_sdk::serde")]
pub struct CliffVestingCheckpoint {
    #[serde(default)]
    #[serde(with = "u64_dec_format")]
    pub time: SecondTimeStamp,
    #[serde(default)]
    #[serde(with = "u128_dec_format")]
    pub amount: Balance,
}

impl Finish for TimeCliffVesting {
    fn is_release_finish(&self) -> bool {
        let max_time = self
            .time_cliff_list
            .iter()
            .map(|e| e.time)
            .max()
            .unwrap_or(0);
        return max_time <= get_block_second_time();
    }
}

impl Frozen for TimeCliffVesting {
    fn freeze(&mut self) {
        self.is_frozen = true
    }

    fn unfreeze(&mut self) {
        self.is_frozen = false
    }

    fn is_frozen(&self) -> bool {
        self.is_frozen
    }
}

impl Beneficiary for TimeCliffVesting {
    fn get_beneficiary(&self) -> AccountId {
        self.beneficiary.clone()
    }

    fn set_beneficiary(&mut self, account: AccountId) {
        self.beneficiary = account;
    }
}

impl VestingTokenInfoTrait for TimeCliffVesting {
    fn get_vesting_token_info(&self) -> &VestingTokenInfo {
        &self.vesting_token_info
    }

    fn set_claimed_token_amount(&mut self, amount: Balance) {
        assert!(
            amount <= self.vesting_token_info.total_vesting_amount,
            "Failed to claim {} amount of token, should less or eq than total vesting amount:{}",
            amount,
            self.vesting_token_info.total_vesting_amount
        );
        self.vesting_token_info.claimed_token_amount = amount;
    }
}

impl VestingAmount for TimeCliffVesting {
    fn get_unreleased_amount(&self) -> Balance {
        let block_second_time = get_block_second_time();
        self.time_cliff_list
            .iter()
            .map(|e| {
                if e.time > block_second_time {
                    e.amount
                } else {
                    0
                }
            })
            .sum()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::test::{usdc, usdt};
    use crate::vesting::traits::Claimable;
    use near_sdk::test_utils::test_env::bob;
    use near_sdk::test_utils::VMContextBuilder;
    use near_sdk::testing_env;

    #[test]
    fn test_cliff_claim() {
        let mut context = VMContextBuilder::new();
        testing_env!(context.block_timestamp(2 * 1000_000_000).build());

        let mut vesting = TimeCliffVesting {
            id: U64(1),
            beneficiary: bob(),
            time_cliff_list: vec![
                CliffVestingCheckpoint { time: 1, amount: 1 },
                CliffVestingCheckpoint { time: 2, amount: 1 },
                CliffVestingCheckpoint { time: 3, amount: 1 },
            ],
            vesting_token_info: VestingTokenInfo {
                claimed_token_amount: 0,
                total_vesting_amount: 3,
            },
            is_frozen: false,
            create_time: get_block_second_time(),
        };
        assert_eq!(vesting.get_claimable_amount(), 2);
        vesting.claim();
        assert_eq!(vesting.get_claimable_amount(), 0);
    }
}
