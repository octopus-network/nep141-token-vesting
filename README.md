# nep141-token-vesting

The purpose of this contract is to provide vesting service for nep141 tokens.

Contents:

- [Terminology](#terminology)
- [Function specification](#function-specification)
    - [Whitelist management](#Whitelist-management)
    - [Create a conversion pool](#Create-a-conversion-pool)
    - [Delete a conversion pool](#Delete-a-conversion-pool)
    - [Transfer token to contract](#Transfer-token-to-contract)
    - [Withdraw token from pool](#Withdraw-token-from-pool)
    - [Pause and resume contract](#Pause-and-resume-contract)
    - [View functions](#View-functions)
- [Auditing](#Auditing)

## Terminology

- `nep141`: [A standard interface for fungible tokens in near network.](https://nomicon.io/Standards/FungibleToken/Core)
- `cliff vesting`: A cliff vesting contains series of time point and token amount, which means how much token will be released at the time point.
- `time linear vesting`: A time linear vesting contains a start time and end time, which means the vesting should be linear released by start time and end time.
- `beneficiary`: People who can claim tokens from a vesting.
- `claim`: Beneficiary can claim rewards from a vesting.
- `freeze a vesting`: Beneficiary is not allowed to claim the tokens when vesting is frozen.
- `terminate a vesting`: Delete a vesting and the unclaimed tokens will be canceled.
- `owner`: People who can creating, freeze and terminate a vesting.

## Function specification

### Create a vesting

The `owner` can create a vesting. And the vesting has two different types: cliff vesting and linear vesting.
- `cliff vesting`: When `owner` creating a `cliff vesting`, it'll be allowed to add a series of point which contains time and amount. Then the vesting will calculate claimable token by these points.
- `linear vesting`: When `owner` creating a `linear`, it'll be allowed to appoint start time and end time. Then the vesting will calculate claimable token by start time and end time.

### Freeze a vesting

- The `owner` can freeze a vesting, then the beneficiary will not allowed to claim the tokens when vesting is frozen.

### Terminate a vesting

- The `owner` can terminate a vesting, then the vesting will be deleted and the unclaimed tokens will belong to `owner`.

### Claim rewards from a vesting

- The `beneficiary` can claim rewards from a vesting. The vesting will calculate claimable token then send tokens to beneficiary's account.


### Change beneficiary

- The `beneficiary` and the `owner` can change a new `beneficiary` in a vesting. Then the next time claimed rewards will send to new `beneficiary`.


### View functions

This contract has a set of view functions for anyone to get the status detail of this contract.