use crate::common::nep141::Nep141;
use crate::common::vesting::VestingContract;
use near_sdk::serde_json::json;
use near_units::parse_near;
use workspaces::network::Sandbox;
use workspaces::result::CallExecutionDetails;
use workspaces::{Account, Worker};

pub const TEST_TOKEN_WASM_BYTES: &[u8] = include_bytes!("../../../res/test_token.wasm");

pub async fn register_account(worker: &Worker<Sandbox>, root: &Account, name: &str) -> Account {
    let tt = root
        .create_subaccount(worker, name)
        .initial_balance(parse_near!("100 N"))
        .transact()
        .await;
    tt.unwrap().into_result().unwrap()
}

pub fn nano_to_seconds(nano: u64) -> u64 {
    nano / 1_000_000_000
}

pub trait ResultAssert {
    fn assert_success(&self, msg: &str);
    fn assert_fail(&self, msg: &str);
}

pub async fn setup_vesting<'s>(
    worker: &'s Worker<Sandbox>,
) -> (VestingContract<'s>, Nep141<'s>, Account, Account, Account) {
    let root = worker.root_account();

    let vesting_contract_account = register_account(&worker, &root, "vesting_contract").await;
    let owner = register_account(&worker, &root, "owner").await;

    let oct = register_account(&worker, &root, "oct").await;

    let vesting_contract = VestingContract::deploy(
        &worker,
        vesting_contract_account,
        owner.id().clone(),
        oct.id().clone(),
    )
    .await;

    let beneficiary = register_account(&worker, &root, "beneficiary").await;

    deploy_test_token_contract(
        &worker,
        &oct,
        vec![
            owner.id().to_string(),
            vesting_contract.contract_id.to_string(),
            beneficiary.id().to_string(),
        ],
    )
    .await;

    let oct_contract = Nep141 {
        contract_id: oct.id().clone(),
        worker,
        account: oct,
    };

    (vesting_contract, oct_contract, root, owner, beneficiary)
}

pub async fn deploy_test_token_contract(
    worker: &Worker<Sandbox>,
    deploy_account: &workspaces::Account,
    accounts_to_register: Vec<String>,
) {
    deploy_account
        .deploy(worker, TEST_TOKEN_WASM_BYTES)
        .await
        .unwrap();
    deploy_account
        .call(worker, deploy_account.id(), "new")
        .args_json(())
        .unwrap()
        .transact()
        .await
        .unwrap();

    let mut i = 0;
    loop {
        let account_id = accounts_to_register[i].clone();
        let result = deploy_account
            .call(worker, deploy_account.id(), "storage_deposit")
            .deposit(parse_near!("0.00125 N"))
            .args_json(json!({ "account_id": account_id }))
            .unwrap()
            .transact()
            .await
            .unwrap();
        i += 1;
        if i == accounts_to_register.len() {
            break;
        }
    }
}
