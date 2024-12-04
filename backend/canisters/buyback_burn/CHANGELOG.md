# Changelog
All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.0.0/).

## Versions

### [unreleased]

### [1.0.2] - 2024-12-03

#### Changed

- **Burn frequency**: The burn process would become less frequent - once per day at 12:00 UTC.

### [1.0.1] - 2024-10-14

#### Changed

- **Burn amount calculation**: Previously, the buyback&burn amount was calculated once per week, while in current version, it's calculated dynamically every interval.

### [1.0.0] - 2024-08-29

#### Description
This marks the initial release of the buyback_burn canister. The canister is designed to support a deflationary tokenomics model by recieving ICP tokens from various sources and selling them on a decentralized exchange (DEX). The obtained GOLDGov tokens are then sent to a minting address to be burned, thereby reducing the token supply over time. Currently, the canister supports swaps through ICPSwap.

#### Added
- **DEX Integration** : Support for ICPSwap to facilitate ICP token swaps. The design is scalable, so that other DEXs could be added in future
- **Token Burning** : Mechanism to send swapped tokens to a minting address for burning.
- **Error Handling** : Basic error handling during swap and burn processes to ensure reliable operations.