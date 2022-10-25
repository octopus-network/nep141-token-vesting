use crate::common::util::ResultAssert;
use crate::common::util::{nano_to_seconds, register_account, setup_vesting};
use crate::common::vesting::VestingContract;
use near_sdk::json_types::{U128, U64};
use near_sdk::Timestamp;
use nep141_token_vesting_contract::vesting::cliff::{CliffVestingCheckpoint, TimeCliffVesting};
use nep141_token_vesting_contract::vesting::linear::NaturalTimeLinearVesting;
use nep141_token_vesting_contract::vesting::traits::{
    Beneficiary, Frozen, VestingAmount, VestingTokenInfoTrait,
};
use nep141_token_vesting_contract::vesting::{Vesting, VestingTokenInfo};
use nep141_token_vesting_contract::TokenVestingContract;
use workspaces::AccountId;

mod common;

#[tokio::test]
async fn test_create_linear_vesting() -> anyhow::Result<()> {
    let worker = workspaces::sandbox().await.unwrap();
    let (vesting_contract, oct_contract, root, owner, beneficiary) = setup_vesting(&worker).await;

    let block = worker.view_latest_block().await.unwrap();
    let now = nano_to_seconds(block.timestamp());

    vesting_contract
        .create_linear_vesting(
            &owner,
            beneficiary.id().clone(),
            U64(now - 1440 - 1440),
            U64(now - 1440),
            U128(100),
        )
        .await?;

    let vesting = vesting_contract
        .get_vesting(0, 1, None)
        .await
        .get(0)
        .unwrap()
        .clone();

    let linear_vesting = match vesting {
        Vesting::NaturalTimeLinearVesting(v) => v,
        Vesting::TimeCliffVesting(_) => {
            panic!("should be NaturalTimeLinearVesting")
        }
    };

    let expected_vesting = NaturalTimeLinearVesting {
        id: U64(1),
        beneficiary: near_sdk::AccountId::new_unchecked(beneficiary.id().to_string()),
        start_time: now - 1440 - 1440,
        end_time: now - 1440,
        vesting_token_info: VestingTokenInfo {
            claimed_token_amount: 0,
            total_vesting_amount: 100,
        },
        is_frozen: false,
        create_time: 0,
    };

    assert_eq!(expected_vesting.id, linear_vesting.id);
    assert_eq!(expected_vesting.beneficiary, linear_vesting.beneficiary);
    assert_eq!(expected_vesting.start_time, linear_vesting.start_time);
    assert_eq!(expected_vesting.end_time, linear_vesting.end_time);
    assert_eq!(
        expected_vesting.vesting_token_info.total_vesting_amount,
        linear_vesting.vesting_token_info.total_vesting_amount
    );
    assert_eq!(
        expected_vesting.vesting_token_info.claimed_token_amount,
        linear_vesting.vesting_token_info.claimed_token_amount
    );
    assert_eq!(expected_vesting.is_frozen, linear_vesting.is_frozen);
    assert_eq!(
        expected_vesting.get_claimable_amount(),
        linear_vesting.get_claimable_amount()
    );

    anyhow::Ok(())
}

#[tokio::test]
async fn test_create_cliff_vesting() -> anyhow::Result<()> {
    let worker = workspaces::sandbox().await.unwrap();
    let (vesting_contract, oct_contract, root, owner, beneficiary) = setup_vesting(&worker).await;

    let block = worker.view_latest_block().await.unwrap();
    let now = nano_to_seconds(block.timestamp());

    vesting_contract
        .create_cliff_vesting(
            &owner,
            beneficiary.id().clone(),
            vec![
                CliffVestingCheckpoint {
                    time: now - 1440,
                    amount: 1,
                },
                CliffVestingCheckpoint {
                    time: now,
                    amount: 1,
                },
                CliffVestingCheckpoint {
                    time: now + 1440,
                    amount: 1,
                },
            ],
        )
        .await?;

    let vesting = vesting_contract
        .get_vesting(0, 1, None)
        .await
        .get(0)
        .unwrap()
        .clone();
    let cliff_vesting = match vesting {
        Vesting::NaturalTimeLinearVesting(_) => {
            panic!("should be cliff vesting")
        }
        Vesting::TimeCliffVesting(v) => v,
    };

    let expected_vesting = TimeCliffVesting {
        id: U64(1),
        beneficiary: near_sdk::AccountId::new_unchecked(beneficiary.id().to_string()),
        time_cliff_list: vec![
            CliffVestingCheckpoint {
                time: now - 1440,
                amount: 1,
            },
            CliffVestingCheckpoint {
                time: now,
                amount: 1,
            },
            CliffVestingCheckpoint {
                time: now + 1440,
                amount: 1,
            },
        ],
        vesting_token_info: VestingTokenInfo {
            claimed_token_amount: 0,
            total_vesting_amount: 3,
        },
        is_frozen: false,
        create_time: 0,
    };

    assert_eq!(expected_vesting.id, cliff_vesting.id);
    assert_eq!(expected_vesting.beneficiary, cliff_vesting.beneficiary);
    assert_eq!(
        expected_vesting.time_cliff_list,
        cliff_vesting.time_cliff_list
    );
    assert_eq!(
        expected_vesting.vesting_token_info.total_vesting_amount,
        cliff_vesting.vesting_token_info.total_vesting_amount
    );
    assert_eq!(
        expected_vesting.vesting_token_info.claimed_token_amount,
        cliff_vesting.vesting_token_info.claimed_token_amount
    );
    assert_eq!(expected_vesting.is_frozen, cliff_vesting.is_frozen);
    assert_eq!(
        expected_vesting.get_claimable_amount(),
        cliff_vesting.get_claimable_amount()
    );
    Ok(())
}

#[tokio::test]
async fn test_freeze_unfreeze_vesting() -> anyhow::Result<()> {
    let worker = workspaces::sandbox().await.unwrap();
    let (vesting_contract, oct_contract, root, owner, beneficiary) = setup_vesting(&worker).await;

    let block = worker.view_latest_block().await.unwrap();
    let now = nano_to_seconds(block.timestamp());

    vesting_contract
        .create_linear_vesting(
            &owner,
            beneficiary.id().clone(),
            U64(now - 1440 - 1440),
            U64(now + 1440),
            U128(100),
        )
        .await?;

    let vesting = vesting_contract
        .get_vesting(0, 1, None)
        .await
        .get(0)
        .unwrap()
        .clone();

    assert_eq!(vesting.is_frozen(), false);

    let result = vesting_contract
        .freeze_vesting(&beneficiary, vesting.get_vesting_id())
        .await;
    assert!(result
        .unwrap_err()
        .to_string()
        .contains("Owner must be predecessor"));

    vesting_contract
        .freeze_vesting(&owner, vesting.get_vesting_id())
        .await?;

    let vesting = vesting_contract
        .get_vesting(0, 1, None)
        .await
        .get(0)
        .unwrap()
        .clone();

    assert_eq!(vesting.is_frozen(), true);

    let result = vesting_contract
        .claim(&beneficiary, vesting.get_vesting_id(), None)
        .await;
    assert!(result
        .unwrap_err()
        .to_string()
        .contains("Failed to claim because this vesting is frozen."));

    Ok(())
}

#[tokio::test]
async fn test_terminate_vesting() -> anyhow::Result<()> {
    let worker = workspaces::sandbox().await.unwrap();
    let (vesting_contract, oct_contract, root, owner, beneficiary) = setup_vesting(&worker).await;

    let block = worker.view_latest_block().await.unwrap();
    let now = nano_to_seconds(block.timestamp());

    vesting_contract
        .create_linear_vesting(
            &owner,
            beneficiary.id().clone(),
            U64(now - 1440 - 1440),
            U64(now + 1440),
            U128(100),
        )
        .await?;

    let vesting = vesting_contract
        .get_vesting(0, 1, None)
        .await
        .get(0)
        .unwrap()
        .clone();

    let result = vesting_contract
        .terminate_vesting(&beneficiary, vesting.get_vesting_id())
        .await;
    assert!(result
        .unwrap_err()
        .to_string()
        .contains("Owner must be predecessor"));

    vesting_contract
        .terminate_vesting(&owner, vesting.get_vesting_id())
        .await?;

    let result1 = vesting_contract
        .claim(&beneficiary, vesting.get_vesting_id(), None)
        .await;
    assert!(result1
        .unwrap_err()
        .to_string()
        .contains("No such vesting id: #1"));

    Ok(())
}
