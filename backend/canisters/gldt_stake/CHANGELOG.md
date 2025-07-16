# Changelog

All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.0.0/).

## Versions

### [unreleased]

### [1.0.3] - 2025-06-05

#### Changed

- Simplified staking model: GLDT staking now enforces a single stake position per user:
    - Users can now increase their stake via the increase_stake endpoint.
    - Users can also initiate partial dissolves of their stake by specifying a dissolve fraction.
    - A maximum of 5 concurrent dissolve events per stake position ensures the system remains robust and spam-resistant.
- To reduce APY volatility and smooth out fluctuations in GOLDAO and ICP rewards, only 33% of the available rewards are distributed each week. This creates a more stable and predictable reward experience for users.

### [1.0.2] - 2025-04-23

#### Add

- Add the feature to vote with neurons based on the majority of the public's vote at the end of the voting period.
- Feature to increase stake of a position
- Add a method 'manual_token_transfer' to enable to let the DAO transfer funds from the canister in case needed.

#### Fix

- Correct the owner of a newly created SNS neuron to be the canister, not the caller

### [1.0.1] - 2025-03-25

#### Description

Changes get_apy to get_apy_overall which represents an APY in that instant. Also adds a new endpoint get_apy_timeseries which returns a set of weekly timestamps and corresponding APY snapshots

### [1.0.0] - 2025-03-10

#### Description

This marks the first version deployment of the GLDT Stake backend canister. Giving the GOLD DAO project the ability to stake GLDT to receive rewards.
