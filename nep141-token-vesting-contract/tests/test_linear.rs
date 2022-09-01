use crate::common::util::ResultAssert;
use crate::common::util::{nano_to_seconds, register_account, setup_vesting};
use crate::common::vesting::VestingContract;
use near_sdk::json_types::{U128, U64};
use near_sdk::Timestamp;
use nep141_token_vesting_contract::vesting::traits::{
    Beneficiary, VestingAmount, VestingTokenInfoTrait,
};
use nep141_token_vesting_contract::TokenVestingContract;
use workspaces::AccountId;

mod common;

#[tokio::test]
async fn test_linear() -> anyhow::Result<()> {
    let worker = workspaces::sandbox().await.unwrap();
    let (vesting_contract, oct_contract, root, owner, beneficiary) = setup_vesting(&worker).await;

    println!("{}", vesting_contract.get_vesting_token_id().await);

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
    let claimable_amount = vesting_contract.get_claimable_amount(U64(1)).await.0;

    assert_eq!(vesting.get_vesting_id(), U64(1), "vesting id should be 1");
    assert_eq!(
        claimable_amount, 100,
        "vesting claimable amount should be 100"
    );
    assert_eq!(
        vesting.get_beneficiary().to_string(),
        beneficiary.id().to_string()
    );
    assert_eq!(vesting.get_vesting_token_info().total_vesting_amount, 100);
    assert_eq!(vesting.get_vesting_token_info().claimed_token_amount, 0);

    anyhow::Ok(())
}
