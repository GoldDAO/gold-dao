# Changelog

All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.0.0/).

## Versions

### [unreleased]

### [1.0.16] - 2025-12-10

#### Changed

- **Disable Proposal Voting Job**: Stop the automatic voting of the 100 million GOLDAO neuron

### [1.0.15] - 2025-10-13

#### Changed

- **Add APY limit**: Introduce APY limitation on the rewards processing in order to stabilize the rewards

### [1.0.14] - 2025-09-16

#### Changed

- **Updated ICRC21 types**: Aligned `Icrc21Args` and `Icrc21Response` with the latest spec, ensuring correct parsing and validation of consent messages

### [1.0.13] - 2025-09-12

#### Added

- Added ICRC21 for getPosition call

### [1.0.12] - 2025-09-10

#### Changed

- Fetch USD tokens price from another reliable source

### [1.0.11] - 2025-09-05

#### Changed

- Add additional field to ICRC3 transaction
- Run instant_dissolve_fees at each upgrade
- Hide testing endpoints

### [1.0.10] - 2025-09-02

#### Changed

- Change the list of rewards
- Deactivate automatic reward claiming
- Daily analytics are now returned in reverse chronological order (newest → oldest). Previously they were returned oldest → newest due to StableBTreeMap iteration order. Also adjusted other analytics endpoints

### [1.0.9] - 2025-08-29

#### Changed

- Lower the rewards transfer threshold
- Adjust the daily analytics to be returned reversed

### [1.0.8] - 2025-08-28

#### Added

- Add manual process call

### [1.0.7] - 2025-08-28

#### Modified

- **DailyAnalyticsSystem**:
  - Daily analytics are now updated instantly (removed 24h delay)
  - Metrics adjusted for improved accuracy
- **Processing rewards transfer**: Adjust the transfer amount threshold

### [1.0.6] - 2025-08-27

#### Modified

- **Authorized principals**: Adjust the list of authorized principals

### [1.0.5] - 2025-08-26

#### Removed

- **Whitelist**: remove the whitelist guard from manage_stake_position method

### [1.0.4] - 2025-08-25

#### Added

- **Daily Analytics Tracking**:  
  The `gldt_stake` canister now stores historical analytics on a **per-day basis**, including:
  - APYs
  - Staked GLDT amounts
  - Rewards per token

#### Fixed

- **APY Calculation**: APY is now calculated correctly on a **daily** basis (previously was still using weekly logic).
- **Funds Flow**: All fund movements between pools are now tracked and executed correctly.
- **General Stability**: Multiple bugs fixed, improving efficiency and reliability of the canister.

### [1.0.3] - 2025-06-05

#### Changed

- Simplified staking model: GLDT staking now enforces a single stake position per user:
  - Users can now increase their stake via the increase_stake endpoint.
  - Users can also initiate partial dissolves of their stake by specifying a dissolve fraction.
  - A maximum of 5 concurrent dissolve events per stake position ensures the system remains robust and spam-resistant.
- To reduce APY volatility and smooth out fluctuations in GOLDAO and ICP rewards, only 33% of the available rewards are distributed each week. This creates a more stable and predictable reward experience for users.

#### Added

- **Support of ICRC10 standard**: ICRC-10 is a standard aimed at simplifying the discovery of supported standards by canisters on the Internet Computer. By providing a unified method, icrc10_supported_standards, canisters can easily expose the standards they implement, enhancing interoperability and easing integration efforts across the ecosystem.
- **Support of ICRC21 standard**: Added support for retrieving human-readable consent messages from canisters before executing calls. This enables wallet signers (such as OISY) to prompt users with clear, canister-defined explanations of requested actions, improving transparency and user trust.

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
