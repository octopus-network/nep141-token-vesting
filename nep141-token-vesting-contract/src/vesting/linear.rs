use super::*;
use crate::types::SecondTimeStamp;
use crate::vesting::traits::{Beneficiary, Finish, NaturalTime, VestingTokenInfoTrait};
use near_sdk::{AccountId, Balance};

#[derive(BorshSerialize, BorshDeserialize, Debug, Serialize, Deserialize, Clone)]
#[serde(crate = "near_sdk::serde")]
pub struct NaturalTimeLinearVesting {
    pub id: VestingId,
    pub beneficiary: AccountId,
    #[serde(default)]
    #[serde(with = "u64_dec_format")]
    pub start_time: SecondTimeStamp,
    #[serde(default)]
    #[serde(with = "u64_dec_format")]
    pub end_time: SecondTimeStamp,
    pub vesting_token_info: VestingTokenInfo,
    pub is_frozen: bool,
    #[serde(default)]
    #[serde(with = "u64_dec_format")]
    pub create_time: SecondTimeStamp,
}

impl NaturalTimeLinearVesting {}

impl Finish for NaturalTimeLinearVesting {
    fn is_release_finish(&self) -> bool {
        self.end_time <= get_block_second_time()
    }
}

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
            amount <= self.vesting_token_info.total_vesting_amount,
            "Failed to set claimed token with {}, should less or eq than total amount: {} ",
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
    use near_sdk::test_utils::VMContextBuilder;
    use near_sdk::testing_env;

    #[test]
    fn test_linear() {
        let mut context = VMContextBuilder::new();
        testing_env!(context.block_timestamp(1 * 1000_000_000).build());

        let mut vesting = NaturalTimeLinearVesting {
            id: U64(0),
            beneficiary: bob(),
            start_time: 1,
            // 1654595929
            end_time: 10,
            vesting_token_info: VestingTokenInfo {
                claimed_token_amount: 0,
                total_vesting_amount: 100,
            },
            is_frozen: false,
            create_time: get_block_second_time(),
        };

        testing_env!(context.block_timestamp(1000_000_00).build());
        assert_eq!(vesting.get_claimable_amount(), 0);

        testing_env!(context.block_timestamp(1 * 1000_000_000).build());
        assert_eq!(vesting.get_claimable_amount(), 10);

        testing_env!(context.block_timestamp(2 * 1000_000_000).build());
        assert_eq!(vesting.get_claimable_amount(), 20);

        testing_env!(context.block_timestamp(3 * 1000_000_000).build());
        assert_eq!(vesting.get_claimable_amount(), 30);

        testing_env!(context.block_timestamp(4 * 1000_000_000).build());
        assert_eq!(vesting.get_claimable_amount(), 40);

        testing_env!(context.block_timestamp(5 * 1000_000_000).build());
        assert_eq!(vesting.get_claimable_amount(), 50);

        testing_env!(context.block_timestamp(6 * 1000_000_000).build());
        assert_eq!(vesting.get_claimable_amount(), 60);

        testing_env!(context.block_timestamp(7 * 1000_000_000).build());
        assert_eq!(vesting.get_claimable_amount(), 70);

        testing_env!(context.block_timestamp(8 * 1000_000_000).build());
        assert_eq!(vesting.get_claimable_amount(), 80);

        testing_env!(context.block_timestamp(9 * 1000_000_000).build());
        assert_eq!(vesting.get_claimable_amount(), 90);

        testing_env!(context.block_timestamp(10 * 1000_000_000).build());
        assert_eq!(vesting.get_claimable_amount(), 100);

        testing_env!(context.block_timestamp(11 * 1000_000_000).build());
        assert_eq!(vesting.get_claimable_amount(), 100);
    }
}
