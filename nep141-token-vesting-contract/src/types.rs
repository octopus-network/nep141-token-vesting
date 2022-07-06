use near_sdk::json_types::U64;
use uint::construct_uint;

construct_uint! {
    /// 256-bit unsigned integer.
    pub struct U256(4);
}

pub type SecondTimeStamp = u64;
pub type VestingId = U64;
pub type TransferId = U64;
