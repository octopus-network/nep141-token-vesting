use near_sdk::json_types::{U128, U64};
use near_sdk::serde_json::json;
use near_sdk::ONE_NEAR;
use nep141_token_vesting_contract::interfaces::OwnerAction;
use nep141_token_vesting_contract::types::VestingId;
use nep141_token_vesting_contract::vesting::cliff::CliffVestingCheckpoint;
use workspaces::network::Sandbox;
use workspaces::result::CallExecutionDetails;
use workspaces::AccountId;
use workspaces::{Account, Worker};

pub const VESTING_WASM_BYTES: &[u8] =
    include_bytes!("../../../res/nep141_token_vesting_contract.wasm");

pub struct VestingContract<'s> {
    pub contract_id: workspaces::AccountId,
    pub worker: &'s Worker<Sandbox>,
    pub account: Account,
}

impl<'s> VestingContract<'s> {
    pub async fn deploy(
        worker: &'s Worker<Sandbox>,
        deploy_account: Account,
        owner_account: AccountId,
        vesting_token: AccountId,
    ) -> VestingContract<'s> {
        deploy_account
            .deploy(worker, VESTING_WASM_BYTES)
            .await
            .unwrap();
        deploy_account
            .call(worker, deploy_account.id(), "new")
            .args_json((owner_account, vesting_token))
            .unwrap()
            .transact()
            .await
            .unwrap();
        Self {
            contract_id: deploy_account.id().clone(),
            worker,
            account: deploy_account,
        }
    }

    pub async fn get_vesting_token_id(&self) -> AccountId {
        self.worker
            .view(
                &self.contract_id,
                "get_vesting_token_id",
                json!(()).to_string().into_bytes(),
            )
            .await
            .unwrap()
            .json()
            .unwrap()
    }

    pub async fn get_vesting(
        &self,
        from_index: u32,
        limit: u32,
        beneficiary: Option<AccountId>,
    ) -> Vec<nep141_token_vesting_contract::vesting::Vesting> {
        self.worker
            .view(
                &self.contract_id,
                "get_vesting",
                json!({
                    "from_index": from_index,
                    "limit": limit,
                    "beneficiary": beneficiary
                })
                .to_string()
                .into_bytes(),
            )
            .await
            .unwrap()
            .json()
            .unwrap()
    }

    pub async fn get_vesting_by_id(
        &self,
        id: VestingId,
    ) -> nep141_token_vesting_contract::vesting::Vesting {
        self.worker
            .view(
                &self.contract_id,
                "get_vesting_by_id",
                json!({
                    "vesting_id": id,
                })
                .to_string()
                .into_bytes(),
            )
            .await
            .unwrap()
            .json()
            .unwrap()
    }

    pub async fn get_claimable_amount(
        &self,
        vesting_id: nep141_token_vesting_contract::types::VestingId,
    ) -> U128 {
        self.worker
            .view(
                &self.contract_id,
                "get_claimable_amount",
                json!({
                    "vesting_id": vesting_id,
                })
                .to_string()
                .into_bytes(),
            )
            .await
            .unwrap()
            .json()
            .unwrap()
    }

    pub async fn get_all_claimable_amount(&self, beneficiary: Option<AccountId>) -> U128 {
        self.worker
            .view(
                &self.contract_id,
                "get_all_claimable_amount",
                json!({
                    "beneficiary": beneficiary,
                })
                .to_string()
                .into_bytes(),
            )
            .await
            .unwrap()
            .json()
            .unwrap()
    }

    pub async fn create_linear_vesting(
        &self,
        signer: &workspaces::Account,
        beneficiary: AccountId,
        start_time: U64,
        end_time: U64,
        total_vesting_amount: U128,
    ) -> anyhow::Result<CallExecutionDetails> {
        signer
            .call(self.worker, &self.contract_id, "create_linear_vesting")
            .deposit(ONE_NEAR)
            .args_json(json!({
                "beneficiary": beneficiary,
                "start_time": start_time,
                "end_time": end_time ,
                "total_vesting_amount": total_vesting_amount
            }))?
            .transact()
            .await
    }

    pub async fn create_cliff_vesting(
        &self,
        signer: &workspaces::Account,
        beneficiary: AccountId,
        time_cliff_list: Vec<CliffVestingCheckpoint>,
    ) -> anyhow::Result<CallExecutionDetails> {
        signer
            .call(self.worker, &self.contract_id, "create_cliff_vesting")
            .deposit(ONE_NEAR)
            .args_json(json!({
                "beneficiary": beneficiary,
                "time_cliff_list": time_cliff_list,
            }))?
            .transact()
            .await
    }

    pub async fn freeze_vesting(
        &self,
        signer: &workspaces::Account,
        vesting_id: VestingId,
    ) -> anyhow::Result<CallExecutionDetails> {
        signer
            .call(self.worker, &self.contract_id, "freeze_vesting")
            .args_json(json!({
                "vesting_id": vesting_id,
            }))?
            .transact()
            .await
    }

    pub async fn unfreeze_vesting(
        &self,
        signer: &workspaces::Account,
        vesting_id: VestingId,
    ) -> anyhow::Result<CallExecutionDetails> {
        signer
            .call(self.worker, &self.contract_id, "unfreeze_vesting")
            .args_json(json!({
                "vesting_id": vesting_id,
            }))?
            .transact()
            .await
    }

    pub async fn terminate_vesting(
        &self,
        signer: &workspaces::Account,
        vesting_id: VestingId,
    ) -> anyhow::Result<CallExecutionDetails> {
        signer
            .call(self.worker, &self.contract_id, "terminate_vesting")
            .args_json(json!({
                "vesting_id": vesting_id,
            }))?
            .transact()
            .await
    }

    pub async fn change_beneficiary(
        &self,
        signer: &workspaces::Account,
        vesting_id: VestingId,
        new_beneficiary: AccountId,
    ) -> anyhow::Result<CallExecutionDetails> {
        signer
            .call(self.worker, &self.contract_id, "change_beneficiary")
            .deposit(ONE_NEAR)
            .args_json(json!({
                "vesting_id": vesting_id,
                "new_beneficiary": new_beneficiary
            }))?
            .transact()
            .await
    }

    pub async fn claim(
        &self,
        signer: &workspaces::Account,
        vesting_id: VestingId,
        amount: Option<U128>,
    ) -> anyhow::Result<CallExecutionDetails> {
        signer
            .call(self.worker, &self.contract_id, "claim")
            .max_gas()
            .args_json(json!({
                "vesting_id": vesting_id,
                "amount": amount
            }))?
            .transact()
            .await
    }

    pub async fn claim_all(
        &self,
        signer: &workspaces::Account,
        beneficiary: Option<AccountId>,
    ) -> anyhow::Result<CallExecutionDetails> {
        signer
            .call(self.worker, &self.contract_id, "claim_all")
            .max_gas()
            .args_json(json!({
                "beneficiary": beneficiary,
            }))?
            .transact()
            .await
    }
}
