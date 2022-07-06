#!/bin/bash
set -e

source ./variables.sh

cd ..
bash build.sh &&
cd scripts

if [ "$1" == "deploy" ]; then
  near deploy $VESTING_CONTRACT_ACCOUNT_ID ../res/$VESTING_WASM_NAME new '{"owner": "'$OWNER_ACCOUNT_ID'", "token_id": "'$TOKEN_ID'"}'
elif [ "$1" == "redeploy" ]; then
  near deploy $VESTING_CONTRACT_ACCOUNT_ID ../res/$VESTING_WASM_NAME
elif [ "$1" == "clean" ]; then
  bash clear-state.sh && near deploy $VESTING_CONTRACT_ACCOUNT_ID ../res/$VESTING_WASM_NAME new '{"owner": "'$OWNER_ACCOUNT_ID'", "token_id": "'$TOKEN_ID'"}'
fi
