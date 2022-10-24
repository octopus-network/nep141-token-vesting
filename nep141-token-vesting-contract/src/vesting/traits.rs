use crate::types::SecondTimeStamp;
use crate::vesting::VestingTokenInfo;
use near_sdk::{AccountId, Balance};

pub trait Frozen {
    fn freeze(&mut self);
    fn unfreeze(&mut self);
    fn is_frozen(&self) -> bool;
}

pub trait Beneficiary {
    fn get_beneficiary(&self) -> AccountId;
    fn set_beneficiary(&mut self, account: AccountId);
}

pub trait NaturalTime {
    fn get_start_time(&self) -> SecondTimeStamp;
    fn get_end_time(&self) -> SecondTimeStamp;
    fn get_period(&self) -> SecondTimeStamp {
        self.get_end_time() - self.get_start_time() + 1
    }
}

pub trait VestingTokenInfoTrait {
    fn get_vesting_token_info(&self) -> &VestingTokenInfo;

    fn set_claimed_token_amount(&mut self, amount: Balance);
}

pub trait VestingAmount: VestingTokenInfoTrait {
    //  released amount logically
    fn get_released_amount(&self) -> Balance {
        let total_amount = self.get_vesting_token_info().total_vesting_amount;
        assert!(
            total_amount >= self.get_unreleased_amount(),
            "total amount should ge released amount."
        );
        total_amount - self.get_unreleased_amount()
    }
    fn get_unreleased_amount(&self) -> Balance;
    fn get_claimable_amount(&self) -> Balance {
        self.get_released_amount() - self.get_vesting_token_info().claimed_token_amount
    }
}

pub trait Claimable {
    fn claim(&mut self) -> Balance;
}

pub trait Finish: VestingTokenInfoTrait {
    fn is_release_finish(&self) -> bool;
    fn is_vesting_finish(&self) -> bool {
        self.get_vesting_token_info().total_vesting_amount
            == self.get_vesting_token_info().claimed_token_amount
    }
}
