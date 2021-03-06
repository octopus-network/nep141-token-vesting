use super::*;
use crate::types::SecondTimeStamp;
use crate::vesting::traits::{
    Beneficiary, Claimable, NaturalTime, VestingAmount, VestingTokenInfoTrait,
};
use near_sdk::{AccountId, Balance};

#[derive(BorshSerialize, BorshDeserialize, Debug, Serialize, Deserialize, Clone)]
#[serde(crate = "near_sdk::serde")]
pub struct NaturalTimeLinearVesting {
    pub beneficiary: AccountId,
    pub start_time: SecondTimeStamp,
    pub end_time: SecondTimeStamp,
    pub vesting_token_info: VestingTokenInfo,
    pub is_frozen: bool,
    // operate_config: VestingOperateConfig
}

impl NaturalTimeLinearVesting {}

impl Frozen for NaturalTimeLinearVesting {
    fn freeze(&mut self) {
        self.is_frozen = true;
    }

    fn unfreeze(&mut self) {
        self.is_frozen = false;
    }

    fn is_frozen(&self) -> bool {
        self.is_frozen
    }
}

impl Beneficiary for NaturalTimeLinearVesting {
    fn get_beneficiary(&self) -> AccountId {
        self.beneficiary.clone()
    }

    fn set_beneficiary(&mut self, account: AccountId) {
        self.beneficiary = account;
    }
}

impl VestingTokenInfoTrait for NaturalTimeLinearVesting {
    fn get_vesting_token_info(&self) -> &VestingTokenInfo {
        &self.vesting_token_info
    }

    fn set_claimed_token_amount(&mut self, amount: Balance) {
        assert!(
            amount < self.vesting_token_info.total_vesting_amount,
            "Try to set claimed token with {} which greater than total amount: {} ",
            amount,
            self.vesting_token_info.total_vesting_amount
        );
        self.vesting_token_info.claimed_token_amount = amount;
    }
}

impl NaturalTime for NaturalTimeLinearVesting {
    fn get_start_time(&self) -> SecondTimeStamp {
        self.start_time
    }

    fn get_end_time(&self) -> SecondTimeStamp {
        self.end_time
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::test::usdc;
    use near_sdk::test_utils::test_env::{alice, bob};

    #[test]
    fn test_linear() {
        let mut a = NaturalTimeLinearVesting {
            beneficiary: bob(),
            start_time: 0,
            end_time: 0,
            vesting_token_info: VestingTokenInfo {
                token_id: usdc(),
                claimed_token_amount: 0,
                total_vesting_amount: 123,
            },
            is_frozen: false,
        };
        a.claim(Some(122));
        assert_eq!(a.get_claimable_amount(), 1);
    }
}
