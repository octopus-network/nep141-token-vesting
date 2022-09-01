use crate::common::util::ResultAssert;
use crate::common::util::{nano_to_seconds, register_account, setup_vesting};
use crate::common::vesting::VestingContract;
use near_sdk::json_types::{U128, U64};
use near_sdk::Timestamp;
use nep141_token_vesting_contract::vesting::cliff::CliffVestingCheckpoint;
use nep141_token_vesting_contract::vesting::traits::{
    Beneficiary, VestingAmount, VestingTokenInfoTrait,
};
use nep141_token_vesting_contract::TokenVestingContract;
use workspaces::AccountId;

mod common;

#[tokio::test]
async fn test_cliff() -> anyhow::Result<()> {
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
    let claimable_amount = vesting_contract.get_claimable_amount(U64(1)).await.0;

    assert_eq!(vesting.get_vesting_id(), U64(1), "vesting id should be 1");
    assert_eq!(
        vesting.get_beneficiary().to_string(),
        beneficiary.id().to_string()
    );
    assert_eq!(claimable_amount, 2);
    assert_eq!(vesting.get_vesting_token_info().total_vesting_amount, 3);
    assert_eq!(vesting.get_vesting_token_info().claimed_token_amount, 0);

    // oct_contract
    //     .mint(vesting_contract.contract_id.clone(), U128(100))
    //     .await?;
    //
    // let claimable = vesting_contract.get_claimable_amount(U64(1)).await.0;
    // assert_eq!(claimable, 2, "claimable amount should be 2." );
    // vesting_contract.claim(&beneficiary, U64(1), None).await?;
    // let claimable = vesting_contract.get_claimable_amount(U64(1)).await.0;
    // assert_eq!(claimable, 0, "claimable amount should be 0." );

    anyhow::Ok(())
}
