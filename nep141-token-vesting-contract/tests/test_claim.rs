use crate::common::util::{nano_to_seconds, setup_vesting};
use near_sdk::json_types::{U128, U64};
use nep141_token_vesting_contract::vesting::cliff::CliffVestingCheckpoint;

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

    vesting_contract
        .create_linear_vesting(
            &owner,
            beneficiary.id().clone(),
            U64(now - 1440 - 1440),
            U64(now - 1440),
            U128(5),
        )
        .await?;

    oct_contract
        .mint(vesting_contract.contract_id.clone(), U128(8))
        .await?;

    let claimable = vesting_contract.get_claimable_amount(U64(1)).await.0;
    assert_eq!(claimable, 2, "vesting #1 claimable amount should be 2.");
    let claimable = vesting_contract.get_claimable_amount(U64(2)).await.0;
    assert_eq!(claimable, 5, "vesting #2 claimable amount should be 5.");

    let contract_amount_before_claim = oct_contract
        .ft_balance_of(vesting_contract.contract_id.clone())
        .await
        .0;
    let beneficiary_amount_before_claim =
        oct_contract.ft_balance_of(beneficiary.id().clone()).await.0;
    vesting_contract.claim_all(&beneficiary, None).await?;

    let contract_amount_after_claim = oct_contract
        .ft_balance_of(vesting_contract.contract_id.clone())
        .await
        .0;
    let beneficiary_amount_after_claim =
        oct_contract.ft_balance_of(beneficiary.id().clone()).await.0;

    assert_eq!(
        contract_amount_before_claim - 7,
        contract_amount_after_claim
    );
    assert_eq!(
        beneficiary_amount_before_claim + 7,
        beneficiary_amount_after_claim
    );

    anyhow::Ok(())
}
