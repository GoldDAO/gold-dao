# Changelog

All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.0.0/).

## Versions

### [unreleased]

### [1.0.9] - 2025-07-18

#### Added

- **Support of ICRC10 standard**: ICRC-10 is a standard aimed at simplifying the discovery of supported standards by canisters on the Internet Computer. By providing a unified method, icrc10_supported_standards, canisters can easily expose the standards they implement, enhancing interoperability and easing integration efforts across the ecosystem.
- **Support of ICRC21 standard**: Added support for retrieving human-readable consent messages from canisters before executing calls. This enables wallet signers (such as OISY) to prompt users with clear, canister-defined explanations of requested actions, improving transparency and user trust.

### [1.0.8] - 2025-07-11

#### Fixed

- **Migration issue** - Fixed a deserialization bug that caused canister upgrades to fail when migrating the canister state. The updated logic now correctly handles legacy data formats to ensure smooth migration.

### [1.0.7] - 2025-06-11

#### Added

- **WTN token support** - Added full support for the WTN token in the system, including serialization, parsing, and validation logic. This allows the system to properly handle WTN-related transactions and metadata.

#### Fixed

- **Payment rounds bug** - A serialization issue caused historical payment round data to be incorrectly serialized &deserialized or stored in memory. As a result, the internal index tracking the progression of payment rounds became corrupted. Despite this, rewards were still distributed correctly - they simply occurred under a different index than expected.

### [1.0.6] - 2025-04-07

#### Improved

- cron timing - An upgrade to how weekly scheduled tasks are run between upgrades now means our scheduled jobs can run at specific times instead of between hourly ranges.

### [1.0.5] - 2025-02-21

#### Fixed

- SNS neuron disburse events create the posibility of some neurons not gaining maturity despite being a valid neuron that should receive rewards. This change allows the system to take into consideration the maturity reset that occurs when an SNS neuron has one or more disburse events active.

### [1.0.4] - 2024-10-10

#### Changed

- let any principal in permission list claim rewards to enable actual owners of neurons to also claim rewards.

### [1.0.3] - 2024-09-25

#### Added

- **State**: Include versioning and commit info in state.

### [1.0.2] - 2024-09-04

#### Changed

- **Dependencies**: Updated Rust CDK dependencies to the latest versions to improve compatibility and performance.

### [1.0.1] - 2024-07-16

#### Description

This includes fixes and improvements

#### Added

- **Consistent distribution times across upgrades** : Upgrading a canister would mean a distribution timer would get reset and so this could result in an almost 2 week delay depending on the time of the upgrade. This new feature allows the distribution to always start at a specific time of UTC 14 - 16.
- sns_rewards canister ugprade via SNS proposal.

#### Fixed

- **History overwrite** : the history would sometimes be overwritten for early distributions, although this isn't a problem now, we have changed how history is added and added unit tests to make sure historic distributions are added to the history state correctly.

### [1.0.0] - 2024-04-18

#### Description

This marks the initial release of SNS Rewards (sns_rewards canister). SNS Rewards calculates the contribution of each GOLDAO neuron as a percentage of the total over a specified period. Subsequently, it transfers the DAO's treasury neuron rewards based on these percentages.

#### Added

- **Daily GOLDAO Neuron Synchronization** : Keeps track of each GOLDAO neuron's maturity gains on a daily basis.
- **Weekly Reward Distribution** : Calculate and distribute rewards weekly based on the proportional accumulated maturity of each neuron for that week.
- **Daily Reserve Pool Transfer** : Transfers GOLDAO tokens to a dedicated reward pool within the canister daily, ensuring a consistent payout amount each week for the weekly reward distribution of GOLDAO tokens.
- **Neuron Ownership & Reward Claims** : Easily claim ownership of a neuron via hotkeys and subsequently claim any distributed rewards.
