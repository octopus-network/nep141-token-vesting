# nep141-token-vesting

The purpose of this contract is to provide vesting service for nep141 tokens.

Contents:

- [Terminology](#terminology)
- [Function specification](#function-specification)
  - [Create a vesting](#create-a-vesting)
  - [Pause and resume a vesting](#pause-and-resume-a-vesting)
  - [Terminate a vesting](#terminate-a-vesting)
  - [Claim tokens](#claim-tokens)
  - [Change beneficiary of a vesting](#change-beneficiary-of-a-vesting)
  - [Withdraw remaining tokens in the contract](#withdraw-remaining-tokens-in-the-contract)
  - [View functions](#view-functions)
- [Auditing](#auditing)

## Terminology

- `nep141`: [A standard interface for fungible tokens in near network.](https://nomicon.io/Standards/FungibleToken/Core)
- `cliff vesting`: A cliff vesting contains a series of time points and token amount, which means how many tokens will be released at the time point.
- `time linear vesting`: A time linear vesting contains a start time and end time, which means the vesting should be linearly released between start time and end time.
- `beneficiary`: People who can claim tokens from a vesting.
- `owner`: People who can create, pause/resume and terminate a vesting. It is a NEAR account, which can be an account controlled by an user's keys directly or a DAO contract like [sputnik DAO](https://github.com/near-daos/sputnik-dao-contract).

## Function specification

### Create a vesting

The `owner` can create as many vesting as he/she wants. A vesting includes a beneficiary and necessary settings based on its type. The vesting types are as the following:

- `cliff vesting`: This type of vesting allows the owner to add a series of `release point` which contains release time and amount. Then the vesting will calculate the claimable tokens by these `release point`s.
- `linear vesting`: This type of vesting allows the owner to set start time and end time. Then the vesting will calculate the claimable tokens by start time and end time linearly.

### Pause and resume a vesting

- The `owner` can pause a vesting, then the beneficiary can not claim the tokens from the vesting anymore until the vesting is resumed.
- A paused vesting can be resumed by the `owner`.

### Terminate a vesting

- The `owner` can terminate a vesting, then the beneficiary can not claim the tokens from the vesting anymore.
- This is an one-time action. A termiated vesting can not be activated again.

### Claim tokens

- A `beneficiary` of a vesting in this contract can claim tokens from a vesting. The vesting will calculate the claimable token and then send tokens to the beneficiary's account.

> Is it possible to support that the beneficiary can claim tokens in all vestings which are with the same beneficiary?

### Change beneficiary of a vesting

- The `beneficiary` and the `owner` can set a new `beneficiary` in a vesting. Then the next time claimed tokens will send to the new `beneficiary`.

### Withdraw remaining tokens in the contract

The owner can withdraw the remaining tokens in this contract only if there is no active/paused vesting in this contract.

### View functions

This contract has a set of view functions for anyone to get the status detail of this contract.

> Maybe should add a view function for beneficiary to query the claimable amount of a vesting or the total claimable amount in all vestings.

## Auditing

This contract has completed auditing by [Blocksec](https://blocksec.com). The report is [here](blocksec-octopus-vesting-v1.0_signed.pdf).
