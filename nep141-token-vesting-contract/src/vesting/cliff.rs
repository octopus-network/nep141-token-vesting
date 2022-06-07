use super::*;
use crate::types::SecondTimeStamp;
use crate::utils::get_block_second_time;
use crate::vesting::traits::{Beneficiary, VestingAmount, VestingTokenInfoTrait};
use crate::vesting::VestingTokenInfo;
use near_sdk::{AccountId, Balance};

#[derive(BorshSerialize, BorshDeserialize, Debug, Serialize, Deserialize, Clone)]
#[serde(crate = "near_sdk::serde")]
pub struct TimeCliffVesting {
    pub beneficiary: AccountId,
    pub time_cliff_list: Vec<CliffVestingCheckpoint>,
    pub vesting_token_info: VestingTokenInfo,
    pub is_frozen: bool,
}

#[derive(BorshSerialize, BorshDeserialize, Debug, Serialize, Deserialize, Clone)]
#[serde(crate = "near_sdk::serde")]
pub struct CliffVestingCheckpoint {
    pub time: SecondTimeStamp,
    #[serde(default)]
    #[serde(with = "u128_dec_format")]
    pub amount: Balance,
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
            amount < self.vesting_token_info.total_vesting_amount,
            "claimed token amount:{} should less than total vesting amount:{}",
            amount,
            self.vesting_token_info.total_vesting_amount
        );
        self.vesting_token_info.claimed_token_amount = amount;
    }
}

impl VestingAmount for TimeCliffVesting {
    fn get_unreleased_amount(&self) -> Balance {
        self.time_cliff_list
            .iter()
            .map(|e| {
                if e.time > get_block_second_time() {
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

    #[test]
    fn test_cliff_claim() {
        let mut cliff = TimeCliffVesting {
            beneficiary: bob(),
            time_cliff_list: vec![],
            vesting_token_info: VestingTokenInfo {
                token_id: usdt(),
                claimed_token_amount: 0,
                total_vesting_amount: 12345,
            },
            is_frozen: false,
        };
        cliff.claim(Some(345));
        assert_eq!(cliff.get_claimable_amount(), 12000);
    }

    #[test]
    #[should_panic]
    fn test_claim_error() {
        let mut cliff = TimeCliffVesting {
            beneficiary: bob(),
            time_cliff_list: vec![],
            vesting_token_info: VestingTokenInfo {
                token_id: usdt(),
                claimed_token_amount: 0,
                total_vesting_amount: 12345,
            },
            is_frozen: false,
        };
        cliff.claim(Some(123456));
    }
}
