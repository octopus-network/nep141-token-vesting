use crate::{Vesting, VestingId};
use near_sdk::json_types::U128;
use near_sdk::serde::Serialize;
use near_sdk::serde_json::{json, Value};
use near_sdk::{log, AccountId};

pub const EVENT_STANDARD: &str = "nep141_token_vesting";
pub const EVENT_STANDARD_VERSION: &str = "1.0.0";

#[derive(Serialize, Debug, Clone)]
#[serde(crate = "near_sdk::serde")]
#[serde(tag = "vesting_event", content = "data")]
#[serde(rename_all = "snake_case")]
pub enum VestingEvent<'a> {
    CreateVesting {
        vesting: &'a Vesting,
        token_id: &'a AccountId,
    },
    UpdateVesting {
        vesting: &'a Vesting,
    },
    TerminateVesting {
        vesting_id: &'a VestingId,
    },
}

#[derive(Serialize, Debug, Clone)]
#[serde(crate = "near_sdk::serde")]
#[serde(tag = "user_action", content = "data")]
#[serde(rename_all = "snake_case")]
pub enum UserAction<'a> {
    CreateVesting {
        vesting_id: &'a VestingId,
    },
    FreezeVesting {
        vesting_id: &'a VestingId,
    },
    UnfreezeVesting {
        vesting_id: &'a VestingId,
    },
    TerminateVesting {
        vesting_id: &'a VestingId,
    },
    ChangeBeneficiary {
        vesting_id: &'a VestingId,
        old_beneficiary: &'a AccountId,
        new_beneficiary: &'a AccountId,
    },
    Claim {
        vesting_id: &'a VestingId,
        beneficiary: &'a AccountId,
        token_id: &'a AccountId,
        amount: &'a U128,
    },
    Legacy {
        beneficiary: &'a AccountId,
        token_id: &'a AccountId,
        amount: &'a U128,
    },
}

pub trait EventEmit {
    fn emit(&self)
    where
        Self: Sized + Serialize,
    {
        emit_event(&self);
    }
}

impl EventEmit for VestingEvent<'_> {}
impl EventEmit for UserAction<'_> {}

// Emit event that follows NEP-297 standard: https://nomicon.io/Standards/EventsFormat
// Arguments
// * `standard`: name of standard, e.g. nep171
// * `version`: e.g. 1.0.0
// * `event`: type of the event, e.g. nft_mint
// * `data`: associate event data. Strictly typed for each set {standard, version, event} inside corresponding NEP
pub(crate) fn emit_event<T: ?Sized + Serialize>(data: &T) {
    let mut result = json!(data);
    let map = result.as_object_mut().unwrap();
    map.insert(
        "standard".to_string(),
        Value::String(EVENT_STANDARD.to_string()),
    );
    map.insert(
        "version".to_string(),
        Value::String(EVENT_STANDARD_VERSION.to_string()),
    );

    log!(format!("EVENT_JSON:{}", result.to_string()));
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::test::{usdc, usdt};
    use crate::vesting::cliff::TimeCliffVesting;
    use crate::vesting::VestingTokenInfo;
    use near_sdk::json_types::U128;
    use near_sdk::json_types::U64;
    use near_sdk::test_utils;
    use near_sdk::test_utils::test_env::bob;

    #[test]
    fn test_vesting() {
        VestingEvent::CreateVesting {
            vesting: &(Vesting::TimeCliffVesting(TimeCliffVesting {
                id: U64(1),
                beneficiary: bob(),
                time_cliff_list: vec![],
                vesting_token_info: VestingTokenInfo {
                    claimed_token_amount: 0,
                    total_vesting_amount: 0,
                },
                is_frozen: false,
            })),
            token_id: &usdt(),
        }
        .emit();
    }
}
