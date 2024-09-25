# Changelog

All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.0.0/).

## Versions

### [unreleased]

#### New

- **State**: Include versioning and commit info in state.

### [1.0.2] - 2024-09-04

#### Changed

- **Dependencies**: Updated Rust CDK dependencies to the latest versions to improve compatibility and performance.

### [1.0.1] - 2024-07-16

#### Description

This includes fixes and improvements

#### Added

- **Consistent distribution times across upgrades** : Upgrading a canister would mean a distribution timer would get reset and so this could result in an almost 2 week delay depending on the time of the upgrade. This new feature allows the distribution to always start at a specific time of UTC 14 - 16.
- sns_rewards canister ugprade via SNS proposal

#### Fixed

- **History overwrite** : the history would sometimes be overwritten for early distributions, although this isn't a problem now, we have changed how history is added and added unit tests to make sure historic distributions are added to the history state correctly.

### [1.0.0] - 2024-04-18

#### Description

This marks the initial release of SNS Rewards (sns_rewards canister). SNS Rewards calculates the contribution of each GLDGov neuron as a percentage of the total over a specified period. Subsequently, it transfers the DAO's treasury neuron rewards based on these percentages.

#### Added

- **Daily GLDGov Neuron Synchronization** : Keeps track of each GLDGov neuron's maturity gains on a daily basis.
- **Weekly Reward Distribution** : Calculate and distribute rewards weekly based on the proportional accumulated maturity of each neuron for that week.
- **Daily Reserve Pool Transfer** : Transfers GLDGov tokens to a dedicated reward pool within the canister daily, ensuring a consistent payout amount each week for the weekly reward distribution of GLDGov tokens.
- **Neuron Ownership & Reward Claims** : Easily claim ownership of a neuron via hotkeys and subsequently claim any distributed rewards.
