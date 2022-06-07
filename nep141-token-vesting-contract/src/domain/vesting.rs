// use near_sdk::{AccountId, Balance};
// use crate::{OwnerAction, TokenVesting};
// use crate::types::SecondTimeStamp;
//
// pub enum Vesting {
//     NaturalTimeLinearVesting(NaturalTimeLinearVesting),
//     TimeCliffVesting(TimeCliffVesting),
// }
//
// pub enum CreateVestingParam  {
//     CreateNaturalTimeLinearVestingParam(NaturalTimeLinearVestingParam),
//     CreateTimeCliffVestingParam(TimeCliffVestingParam),
// }
//
// pub struct NaturalTimeLinearVestingParam {
//     owner: AccountId,
//
//     beneficiary: AccountId,
// }
//
// pub struct NaturalTimeLinearVesting {
//     owner: AccountId,
//     beneficiary: AccountId,
//     token_id: AccountId,
//     token_balance: Amount,
//     start_time: SecondTimeStamp,
//     end_time: SecondTimeStamp,
//     total_amount: Amount,
// }
//
//
//
//
// struct VestingOperateConfig {
//     freezable: bool,
//     is_frozen: bool,
//     terminable: bool,
//     is_terminated: bool,
// }
//
// pub struct VestingCreateParam {
//
//
// }
//
// pub mod vesting_traits{
//     use near_sdk::{AccountId, Balance};
//
//     pub trait Owner {
//         fn get_owner(self);
//         fn set_owner(self,account: AccountId);
//     }
//
//     pub trait Beneficiary {
//         fn get_owner(self);
//         fn set_owner(self,account: AccountId);
//     }
//
//     pub trait IBalance {
//         fn get_token_id(&self)-> &AccountId;
//         fn get_balance(&self)-> Balance;
//         fn set_balance(&mut self, balance: Balance);
//     }
//
//     pub trait VestingAmount {
//         //  released amount logically
//         fn get_released_amount(&self)->Amount {
//             assert!(self.get_total_amount()>=self.get_unreleased_amount(),"total amount should ge released amount.");
//             self.get_total_amount()-self.get_unreleased_amount()
//         }
//         fn get_unreleased_amount(&self)->Amount;
//         fn get_total_amount(&self)->Amount;
//         fn get_claimable_amount(&self)->Amount {
//             self.get_balance()-self.get_unreleased_amount()
//         }
//         fn get_claimed_amount(&self)->Amount;
//     }
//
//     pub trait
// }
//
// #[near_bindgen]
// impl OwnerAction for TokenVesting {
//     fn create_vesting(&mut self, param: VestingCreateParam) -> VestingId {
//
//     }
//
//     fn deposit_vesting(&mut self, vesting_id: VestingId, token_id: AccountId, amount: U128) {
//         todo!()
//     }
// }