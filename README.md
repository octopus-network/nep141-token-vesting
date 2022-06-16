# nep141-token-vesting

The purpose of this contract is to provide vesting service for nep141 tokens.

Contents:

- [Terminology](#terminology)
- [Function specification](#function-specification)
  - [Create a vesting](#Create-a-vesting)
  - [Freeze a vesting](#Freeze-a-vesting)
  - [Terminate a vesting](#Terminate-a-vesting)
  - [Claim tokens](#Claim-tokens)
  - [Change beneficiary](#Change-beneficiary)
  - [View functions](#View-functions)

## Terminology

- `nep141`: [A standard interface for fungible tokens in near network.](https://nomicon.io/Standards/FungibleToken/Core)
- `cliff vesting`: A cliff vesting contains a series of time points and token amount, which means how many tokens will be released at the time point.
- `time linear vesting`: A time linear vesting contains a start time and end time, which means the vesting should be linearly released between start time and end time.
- `beneficiary`: People who can claim tokens from a vesting.
- `owner`: People who can create, freeze and terminate a vesting.

## Function specification

### Create a vesting

The `owner` can create a vesting. And the vesting has two different types: cliff vesting and linear vesting.
- `cliff vesting`: When the `owner` creating a `cliff vesting`, it'll be allowed to add a series of point which contains time and amount. Then the vesting will calculate the claimable tokens by these points.
- `linear vesting`: When the `owner` creating a `linear vesting`, it'll be allowed to set start time and end time. Then the vesting will calculate the claimable tokens by start time and end time.

### Freeze a vesting

- The `owner` can freeze a vesting, then the beneficiary will not be allowed to claim the tokens when the vesting is frozen.

### Terminate a vesting

- The `owner` can terminate a vesting, then the vesting will be deleted and the unclaimed tokens will belong to the `owner`.

### Claim tokens

- The `beneficiary` can claim tokens from a vesting. The vesting will calculate the claimable token and then send tokens to the beneficiary's account.


### Change beneficiary

- The `beneficiary` and the `owner` can set a new `beneficiary` in a vesting. Then the next time claimed tokens will send to the new `beneficiary`.


### View functions

This contract has a set of view functions for anyone to get the status detail of this contract.