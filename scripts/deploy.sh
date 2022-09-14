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
elif [ "$1" == "deploy-token" ]; then
  near deploy $TOKEN_ID ../res/$TEST_TOKEN_WASM_NAME new $'{"metadata": {"spec": "ft-1.0.0", "name": "Octopus Network Token", "symbol": "OCT", "decimals": 18, "icon": "data:image/svg+xml,%3Csvg version=\'1.1\' id=\'O\' xmlns=\'http://www.w3.org/2000/svg\' xmlns:xlink=\'http://www.w3.org/1999/xlink\' x=\'0px\' y=\'0px\' viewBox=\'0 0 113.39 113.39\' style=\'enable-background:new 0 0 113.39 113.39;\' xml:space=\'preserve\'%3E%3Cstyle type=\'text/css\'%3E .st0%7Bfill:%23014299;%7D .st1%7Bfill:%23FFFFFF;%7D %3C/style%3E%3Ccircle class=\'st0\' cx=\'56.69\' cy=\'56.69\' r=\'56.69\'/%3E%3Cg%3E%3Cpath class=\'st1\' d=\'M44.25,59.41c-1.43,0-2.59,1.16-2.59,2.59v20.28c0,1.43,1.16,2.59,2.59,2.59c1.43,0,2.59-1.16,2.59-2.59V62 C46.84,60.57,45.68,59.41,44.25,59.41z\'/%3E%3Cpath class=\'st1\' d=\'M56.69,59.41c-1.45,0-2.62,1.17-2.62,2.62v26.47c0,1.45,1.17,2.62,2.62,2.62s2.62-1.17,2.62-2.62V62.02 C59.31,60.58,58.14,59.41,56.69,59.41z\'/%3E%3Cpath class=\'st1\' d=\'M79.26,78.87c-0.33,0.15-0.64,0.28-0.95,0.38c0,0-0.01,0-0.01,0c-0.59,0.19-1.13,0.29-1.63,0.31h-0.06 c-1,0.03-1.84-0.27-2.59-0.75c-0.49-0.32-0.91-0.73-1.25-1.23c-0.3-0.43-0.53-0.93-0.71-1.51c0-0.01-0.01-0.02-0.01-0.03 c-0.22-0.74-0.34-1.61-0.34-2.59V62.02c0-1.45-1.17-2.62-2.62-2.62c-1.45,0-2.62,1.17-2.62,2.62v11.43c0,4.5,1.64,8.03,4.63,9.96 c1.5,0.97,3.21,1.45,5.04,1.45c1.68,0,3.45-0.41,5.25-1.22c1.32-0.59,1.9-2.14,1.31-3.46C82.13,78.86,80.57,78.27,79.26,78.87z\'/%3E%3Cpath class=\'st1\' d=\'M68.33,45.9c0-2.15-1.75-3.9-3.9-3.9c-2.15,0-3.9,1.75-3.9,3.9s1.75,3.9,3.9,3.9 C66.58,49.8,68.33,48.05,68.33,45.9z\'/%3E%3Cpath class=\'st1\' d=\'M48.96,41.99c-2.15,0-3.9,1.75-3.9,3.9s1.75,3.9,3.9,3.9s3.9-1.75,3.9-3.9S51.11,41.99,48.96,41.99z\'/%3E%3Cpath class=\'st1\' d=\'M56.69,22.28c-15.17,0-27.52,12.34-27.52,27.52v15.09c0,1.46,1.18,2.64,2.64,2.64s2.64-1.18,2.64-2.64V49.8 c0-12.26,9.98-22.24,22.24-22.24c12.26,0,22.24,9.98,22.24,22.24v15.09c0,1.46,1.18,2.64,2.64,2.64s2.64-1.18,2.64-2.64V49.8 C84.21,34.62,71.87,22.28,56.69,22.28z\'/%3E%3C/g%3E%3C/svg%3E"}}'
fi
